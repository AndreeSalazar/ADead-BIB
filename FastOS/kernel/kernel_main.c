/*
 * FastOS v2.0 — Main Kernel
 * ADead-BIB Native Operating System
 * 
 * C is the Master, Rust provides Safety
 * Integrates: VBE Graphics, Nouveau GPU, Po Format, Hybrid Syscalls
 * 
 * Compile: adB cc kernel_main.c -o kernel.bin --kernel --64bit
 */

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/po.h"
#include "../include/rust_safety.h"
#include "../include/boot_types.h"
#include "../drivers/video/nouveau/nouveau.h"

/* ============================================================
 * Rust Safety Layer Integration
 * ============================================================ */

static int rust_safety_initialized = 0;

static void rust_safety_init(void) {
    /* Initialize Rust heap allocator */
    extern u8 _heap_start[];
    extern u8 _heap_end[];
    
    /* Use 1MB for Rust heap if symbols not available */
    static u8 rust_heap[1024 * 1024];
    rust_heap_init((size_t)rust_heap, sizeof(rust_heap));
    
    rust_safety_initialized = 1;
    kputs("[RUST] Safety layer initialized\n");
    kputs("[BG] Binary Guardian: ACTIVE\n");
}

/* Safe memory operations using Rust */
static void* safe_kmalloc(size_t size) {
    if (rust_safety_initialized) {
        return rust_malloc(size);
    }
    return kmalloc(size);
}

static int safe_memcpy(void* dest, size_t dest_size, const void* src, size_t count) {
    if (rust_safety_initialized) {
        return rust_memcpy_safe(dest, dest_size, src, count);
    }
    kmemcpy(dest, src, count);
    return 1;
}

static int safe_memset(void* dest, size_t dest_size, u8 value, size_t count) {
    if (rust_safety_initialized) {
        return rust_memset_safe(dest, dest_size, value, count);
    }
    kmemset(dest, value, count);
    return 1;
}

/* ============================================================
 * Global State
 * ============================================================ */

static framebuffer_t g_framebuffer;
static nv_device_t g_gpu;
static int g_gpu_available = 0;
static uint64_t g_boot_time;

/* VGA text mode fallback */
static volatile uint16_t *vga_text = (volatile uint16_t*)0xB8000;
static int vga_x = 0, vga_y = 0;

/* ============================================================
 * Early Console (VGA Text Mode)
 * ============================================================ */

static void vga_scroll(void) {
    for (int y = 0; y < 24; y++) {
        for (int x = 0; x < 80; x++) {
            vga_text[y * 80 + x] = vga_text[(y + 1) * 80 + x];
        }
    }
    for (int x = 0; x < 80; x++) {
        vga_text[24 * 80 + x] = 0x0720;  /* Space with gray on black */
    }
}

static void vga_putchar(char c) {
    if (c == '\n') {
        vga_x = 0;
        vga_y++;
    } else if (c == '\r') {
        vga_x = 0;
    } else if (c == '\t') {
        vga_x = (vga_x + 8) & ~7;
    } else {
        vga_text[vga_y * 80 + vga_x] = (0x07 << 8) | c;
        vga_x++;
    }
    
    if (vga_x >= 80) {
        vga_x = 0;
        vga_y++;
    }
    
    if (vga_y >= 25) {
        vga_scroll();
        vga_y = 24;
    }
}

void kputs(const char *s) {
    while (*s) {
        vga_putchar(*s++);
    }
}

void kprintf(const char *fmt, ...) {
    /* Simple printf implementation */
    const char *p = fmt;
    while (*p) {
        if (*p == '%' && *(p+1)) {
            p++;
            switch (*p) {
                case 's': {
                    /* Would need va_args - simplified */
                    kputs("<str>");
                    break;
                }
                case 'd':
                case 'x':
                case 'p':
                    kputs("<num>");
                    break;
                case '%':
                    vga_putchar('%');
                    break;
            }
        } else {
            vga_putchar(*p);
        }
        p++;
    }
}

/* ============================================================
 * Memory Management (C + Rust hybrid)
 * ============================================================ */

/* Simple bump allocator for early boot */
static uint8_t heap[1024 * 1024];  /* 1MB heap */
static size_t heap_pos = 0;

void *kmalloc(size_t size) {
    /* Align to 16 bytes */
    size = (size + 15) & ~15;
    
    if (heap_pos + size > sizeof(heap)) {
        return 0;  /* Out of memory */
    }
    
    void *ptr = &heap[heap_pos];
    heap_pos += size;
    return ptr;
}

void kfree(void *ptr) {
    /* Simple allocator doesn't support free */
    (void)ptr;
}

void *kzalloc(size_t size) {
    void *ptr = kmalloc(size);
    if (ptr) {
        kmemset(ptr, 0, size);
    }
    return ptr;
}

void *kmemset(void *s, int c, size_t n) {
    uint8_t *p = (uint8_t*)s;
    while (n--) {
        *p++ = (uint8_t)c;
    }
    return s;
}

void *kmemcpy(void *dest, const void *src, size_t n) {
    uint8_t *d = (uint8_t*)dest;
    const uint8_t *s = (const uint8_t*)src;
    while (n--) {
        *d++ = *s++;
    }
    return dest;
}

size_t kstrlen(const char *s) {
    size_t len = 0;
    while (*s++) len++;
    return len;
}

/* ============================================================
 * Interrupt Descriptor Table (IDT)
 * ============================================================ */

typedef struct {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t  ist;
    uint8_t  type_attr;
    uint16_t offset_mid;
    uint32_t offset_high;
    uint32_t zero;
} __packed idt_entry_t;

static idt_entry_t idt[256];
static idt_ptr_t idtr;

static void idt_set_gate(int num, uint64_t handler, uint16_t selector, uint8_t type) {
    idt[num].offset_low = handler & 0xFFFF;
    idt[num].offset_mid = (handler >> 16) & 0xFFFF;
    idt[num].offset_high = (handler >> 32) & 0xFFFFFFFF;
    idt[num].selector = selector;
    idt[num].ist = 0;
    idt[num].type_attr = type;
    idt[num].zero = 0;
}

/* Exception handlers */
void exception_handler(int num, uint64_t error) {
    kputs("\n!!! EXCEPTION ");
    vga_putchar('0' + (num / 10));
    vga_putchar('0' + (num % 10));
    kputs(" !!!\n");
    
    const char *names[] = {
        "Divide Error", "Debug", "NMI", "Breakpoint",
        "Overflow", "Bound Range", "Invalid Opcode", "No Coprocessor",
        "Double Fault", "Coprocessor Segment", "Invalid TSS", "Segment Not Present",
        "Stack Fault", "General Protection", "Page Fault", "Reserved"
    };
    
    if (num < 16) {
        kputs(names[num]);
    }
    kputs("\nSystem Halted.\n");
    
    cli();
    while (1) hlt();
}

/* IRQ handlers */
static void (*irq_handlers[16])(void) = {0};

void irq_handler(int irq) {
    if (irq_handlers[irq]) {
        irq_handlers[irq]();
    }
    
    /* Send EOI to PIC */
    if (irq >= 8) {
        outb(0xA0, 0x20);  /* Slave PIC */
    }
    outb(0x20, 0x20);  /* Master PIC */
}

void irq_register(int irq, void (*handler)(void)) {
    irq_handlers[irq] = handler;
}

/* ============================================================
 * PIC (Programmable Interrupt Controller)
 * ============================================================ */

static void pic_init(void) {
    /* ICW1: Initialize + ICW4 needed */
    outb(0x20, 0x11);
    outb(0xA0, 0x11);
    io_wait();
    
    /* ICW2: Vector offsets */
    outb(0x21, 0x20);  /* Master: IRQ 0-7 -> INT 32-39 */
    outb(0xA1, 0x28);  /* Slave: IRQ 8-15 -> INT 40-47 */
    io_wait();
    
    /* ICW3: Cascade */
    outb(0x21, 0x04);  /* Master: Slave on IRQ2 */
    outb(0xA1, 0x02);  /* Slave: Cascade identity */
    io_wait();
    
    /* ICW4: 8086 mode */
    outb(0x21, 0x01);
    outb(0xA1, 0x01);
    io_wait();
    
    /* Mask all IRQs except keyboard (IRQ1) and mouse (IRQ12) */
    outb(0x21, 0xF8);  /* 11111000 - Enable IRQ0,1,2 */
    outb(0xA1, 0xEF);  /* 11101111 - Enable IRQ12 */
}

/* ============================================================
 * Timer (PIT)
 * ============================================================ */

static volatile uint64_t timer_ticks = 0;

void timer_handler(void) {
    timer_ticks++;
}

uint64_t timer_get_ticks(void) {
    return timer_ticks;
}

void timer_init(uint32_t freq) {
    uint32_t divisor = 1193180 / freq;
    
    outb(0x43, 0x36);  /* Channel 0, lobyte/hibyte, square wave */
    outb(0x40, divisor & 0xFF);
    outb(0x40, (divisor >> 8) & 0xFF);
    
    irq_register(0, timer_handler);
    kputs("[TIMER] PIT initialized at 100 Hz\n");
}

void timer_sleep(uint32_t ms) {
    uint64_t target = timer_ticks + (ms / 10);
    while (timer_ticks < target) {
        hlt();
    }
}

/* ============================================================
 * VBE Graphics Initialization
 * ============================================================ */

/* VBE info passed from bootloader */
static vbe_mode_info_t *vbe_info = (vbe_mode_info_t*)0x7E00;

static int vbe_init(void) {
    /* Check if VBE info is valid */
    if (vbe_info->framebuffer == 0) {
        kputs("[VBE] No framebuffer available, using text mode\n");
        return -1;
    }
    
    g_framebuffer.buffer = (uint32_t*)(uintptr_t)vbe_info->framebuffer;
    g_framebuffer.width = vbe_info->width;
    g_framebuffer.height = vbe_info->height;
    g_framebuffer.pitch = vbe_info->pitch;
    g_framebuffer.bpp = vbe_info->bpp;
    g_framebuffer.size = vbe_info->pitch * vbe_info->height;
    
    kputs("[VBE] Framebuffer initialized: ");
    /* Would print resolution here */
    kputs("\n");
    
    return 0;
}

/* ============================================================
 * GPU Detection (Nouveau for NVIDIA)
 * ============================================================ */

/* PCI Configuration Space Access */
static uint32_t pci_read(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t addr = (1 << 31) | (bus << 16) | (slot << 11) | (func << 8) | (offset & 0xFC);
    outl(0xCF8, addr);
    return inl(0xCFC);
}

static int gpu_scan_pci(void) {
    kputs("[GPU] Scanning PCI bus for graphics cards...\n");
    
    for (int bus = 0; bus < 256; bus++) {
        for (int slot = 0; slot < 32; slot++) {
            uint32_t vendor_device = pci_read(bus, slot, 0, 0);
            uint16_t vendor = vendor_device & 0xFFFF;
            uint16_t device = (vendor_device >> 16) & 0xFFFF;
            
            if (vendor == 0xFFFF) continue;
            
            /* Check class code (display controller = 0x03) */
            uint32_t class_rev = pci_read(bus, slot, 0, 0x08);
            uint8_t class_code = (class_rev >> 24) & 0xFF;
            
            if (class_code == 0x03) {
                kputs("[GPU] Found display controller: ");
                
                if (vendor == NVIDIA_VENDOR_ID) {
                    kputs("NVIDIA ");
                    g_gpu.vendor_id = vendor;
                    g_gpu.device_id = device;
                    g_gpu.bus = bus;
                    g_gpu.slot = slot;
                    g_gpu.func = 0;
                    g_gpu.family = nouveau_get_family(device);
                    g_gpu.name = nouveau_get_name(device);
                    
                    /* Read BARs */
                    g_gpu.bar0 = pci_read(bus, slot, 0, 0x10) & 0xFFFFFFF0;
                    g_gpu.bar1 = pci_read(bus, slot, 0, 0x14) & 0xFFFFFFF0;
                    
                    g_gpu_available = 1;
                    kputs("(Nouveau driver)\n");
                    return 0;
                } else if (vendor == 0x1002) {
                    kputs("AMD/ATI\n");
                } else if (vendor == 0x8086) {
                    kputs("Intel\n");
                } else {
                    kputs("Unknown\n");
                }
            }
        }
    }
    
    kputs("[GPU] No dedicated GPU found, using VBE fallback\n");
    return -1;
}

/* ============================================================
 * Desktop Environment
 * ============================================================ */

/* Colors */
#define COLOR_DESKTOP_BG    0xFF1E3A5F  /* Dark blue gradient */
#define COLOR_TASKBAR       0xFF2D2D30  /* Dark gray */
#define COLOR_TITLEBAR      0xFF404040  /* Medium gray */
#define COLOR_START_BTN     0xFF0F7B0F  /* Green */
#define COLOR_WHITE         0xFFFFFFFF
#define COLOR_BLACK         0xFF000000

static void draw_wallpaper(void) {
    if (!g_framebuffer.buffer) return;
    
    uint32_t w = g_framebuffer.width;
    uint32_t h = g_framebuffer.height;
    
    /* Gradient background */
    for (uint32_t y = 0; y < h; y++) {
        for (uint32_t x = 0; x < w; x++) {
            /* Dark blue to darker blue gradient */
            uint8_t r = 15 + (y * 25 / h);
            uint8_t g = 30 + (y * 40 / h);
            uint8_t b = 60 + (y * 80 / h);
            
            /* Subtle grid pattern */
            if ((x % 48 == 0) || (y % 48 == 0)) {
                r += 8; g += 12; b += 20;
            }
            
            /* Diagonal accent lines */
            if ((x + y) % 96 < 2) {
                r += 15; g += 25; b += 40;
            }
            
            g_framebuffer.buffer[y * (g_framebuffer.pitch / 4) + x] = 
                0xFF000000 | (r << 16) | (g << 8) | b;
        }
    }
}

static void draw_taskbar(void) {
    if (!g_framebuffer.buffer) return;
    
    uint32_t w = g_framebuffer.width;
    uint32_t h = g_framebuffer.height;
    uint32_t taskbar_h = 40;
    uint32_t taskbar_y = h - taskbar_h;
    
    /* Taskbar background */
    for (uint32_t y = taskbar_y; y < h; y++) {
        for (uint32_t x = 0; x < w; x++) {
            g_framebuffer.buffer[y * (g_framebuffer.pitch / 4) + x] = COLOR_TASKBAR;
        }
    }
    
    /* Start button */
    for (uint32_t y = taskbar_y + 5; y < taskbar_y + 35; y++) {
        for (uint32_t x = 5; x < 60; x++) {
            g_framebuffer.buffer[y * (g_framebuffer.pitch / 4) + x] = COLOR_START_BTN;
        }
    }
    
    /* Clock area */
    for (uint32_t y = taskbar_y + 5; y < taskbar_y + 35; y++) {
        for (uint32_t x = w - 80; x < w - 5; x++) {
            g_framebuffer.buffer[y * (g_framebuffer.pitch / 4) + x] = 0xFF3D3D40;
        }
    }
}

static void draw_cursor(int32_t cx, int32_t cy) {
    if (!g_framebuffer.buffer) return;
    
    /* Simple arrow cursor */
    static const uint8_t cursor_data[16] = {
        0x80, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC, 0xFE, 0xFF,
        0xFC, 0xF8, 0xD8, 0x8C, 0x0C, 0x06, 0x06, 0x00
    };
    
    for (int y = 0; y < 16; y++) {
        for (int x = 0; x < 8; x++) {
            if (cursor_data[y] & (0x80 >> x)) {
                int px = cx + x;
                int py = cy + y;
                if (px >= 0 && px < (int)g_framebuffer.width &&
                    py >= 0 && py < (int)g_framebuffer.height) {
                    g_framebuffer.buffer[py * (g_framebuffer.pitch / 4) + px] = COLOR_WHITE;
                }
            }
        }
    }
}

/* ============================================================
 * Kernel Main Entry Point
 * ============================================================ */

void kernel_main(void) {
    /* Clear screen */
    for (int i = 0; i < 80 * 25; i++) {
        vga_text[i] = 0x0720;
    }
    
    kputs("FastOS v2.0 - ADead-BIB Native Operating System\n");
    kputs("================================================\n\n");
    
    kputs("[BOOT] Kernel loaded at 0x100000\n");
    kputs("[BOOT] C is Master, Rust provides Safety\n\n");
    
    /* Initialize Rust Safety Layer */
    kputs("[INIT] Initializing Rust Safety Layer...\n");
    rust_safety_init();
    
    /* Initialize PIC */
    kputs("[INIT] Initializing PIC...\n");
    pic_init();
    
    /* Initialize Timer */
    kputs("[INIT] Initializing Timer...\n");
    timer_init(100);  /* 100 Hz */
    
    /* Initialize Keyboard */
    kputs("[INIT] Initializing Keyboard...\n");
    keyboard_init();
    irq_register(1, keyboard_handler);
    
    /* Initialize Mouse */
    kputs("[INIT] Initializing Mouse...\n");
    mouse_init();
    irq_register(12, mouse_handler);
    
    /* Scan for GPU */
    kputs("[INIT] Scanning for GPU...\n");
    gpu_scan_pci();
    
    /* Initialize VBE/Framebuffer */
    kputs("[INIT] Initializing Graphics...\n");
    if (vbe_init() == 0) {
        /* Graphics mode available */
        mouse_set_bounds(g_framebuffer.width, g_framebuffer.height);
        
        /* Draw desktop */
        draw_wallpaper();
        draw_taskbar();
    }
    
    /* Initialize Scheduler */
    kputs("[INIT] Initializing Scheduler (64 process slots)...\n");
    scheduler_init();
    
    /* Enable interrupts */
    kputs("[INIT] Enabling interrupts...\n");
    sti();
    
    kputs("\n");
    kputs("========================================\n");
    kputs("  FastOS is ready!\n");
    kputs("========================================\n");
    kputs("\n");
    kputs("  Architecture: x86-64 (Long Mode)\n");
    kputs("  Compiler: ADead-BIB\n");
    kputs("  Format: Po (Portable Object)\n");
    
    if (g_gpu_available) {
        kputs("  GPU: NVIDIA (Nouveau driver)\n");
    } else {
        kputs("  GPU: VBE Fallback\n");
    }
    
    kputs("\n");
    kputs("Press any key to continue...\n");
    
    /* Main loop */
    int32_t mouse_x = 0, mouse_y = 0;
    
    while (1) {
        /* Handle keyboard input */
        char c;
        if (kb_getchar_nonblock(&c)) {
            vga_putchar(c);
        }
        
        /* Update mouse cursor */
        if (g_framebuffer.buffer) {
            int32_t new_x, new_y;
            mouse_get_position(&new_x, &new_y);
            
            if (new_x != mouse_x || new_y != mouse_y) {
                /* Redraw area under old cursor (simplified) */
                /* In real implementation, save/restore background */
                
                mouse_x = new_x;
                mouse_y = new_y;
                draw_cursor(mouse_x, mouse_y);
            }
        }
        
        /* Yield CPU */
        hlt();
    }
}

/* ============================================================
 * Kernel Entry (called from bootloader)
 * ============================================================ */

void _start(void) {
    kernel_main();
    
    /* Should never reach here */
    cli();
    while (1) hlt();
}
