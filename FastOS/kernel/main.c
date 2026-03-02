/*
 * FastOS v2.0 — Kernel Main
 * ADead-BIB Native Operating System
 * 
 * Entry point for the 64-bit kernel.
 * Compiled with: adB cc kernel/main.c -o kernel.bin --flat --org=0x100000
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* VGA text mode buffer */
#define VGA_BUFFER ((volatile uint16_t*)0xB8000)
#define VGA_WIDTH  80
#define VGA_HEIGHT 25

/* VGA colors */
#define VGA_BLACK        0x0
#define VGA_BLUE         0x1
#define VGA_GREEN        0x2
#define VGA_CYAN         0x3
#define VGA_RED          0x4
#define VGA_MAGENTA      0x5
#define VGA_BROWN        0x6
#define VGA_LIGHT_GREY   0x7
#define VGA_DARK_GREY    0x8
#define VGA_LIGHT_BLUE   0x9
#define VGA_LIGHT_GREEN  0xA
#define VGA_LIGHT_CYAN   0xB
#define VGA_LIGHT_RED    0xC
#define VGA_LIGHT_MAGENTA 0xD
#define VGA_YELLOW       0xE
#define VGA_WHITE        0xF

#define VGA_COLOR(fg, bg) ((bg << 4) | fg)
#define VGA_ENTRY(c, color) ((uint16_t)(c) | ((uint16_t)(color) << 8))

/* Terminal state */
static int term_row = 0;
static int term_col = 0;
static uint8_t term_color = VGA_COLOR(VGA_WHITE, VGA_BLUE);

/* Forward declarations */
void term_init(void);
void term_clear(void);
void term_putchar(char c);
void term_write(const char *str);
void term_write_color(const char *str, uint8_t color);
void term_set_cursor(int row, int col);

/* ============================================================
 * Terminal Functions
 * ============================================================ */

void term_init(void) {
    term_row = 0;
    term_col = 0;
    term_color = VGA_COLOR(VGA_WHITE, VGA_BLUE);
    term_clear();
}

void term_clear(void) {
    for (int y = 0; y < VGA_HEIGHT; y++) {
        for (int x = 0; x < VGA_WIDTH; x++) {
            VGA_BUFFER[y * VGA_WIDTH + x] = VGA_ENTRY(' ', term_color);
        }
    }
    term_row = 0;
    term_col = 0;
}

void term_scroll(void) {
    /* Move all lines up by one */
    for (int y = 0; y < VGA_HEIGHT - 1; y++) {
        for (int x = 0; x < VGA_WIDTH; x++) {
            VGA_BUFFER[y * VGA_WIDTH + x] = VGA_BUFFER[(y + 1) * VGA_WIDTH + x];
        }
    }
    /* Clear last line */
    for (int x = 0; x < VGA_WIDTH; x++) {
        VGA_BUFFER[(VGA_HEIGHT - 1) * VGA_WIDTH + x] = VGA_ENTRY(' ', term_color);
    }
    term_row = VGA_HEIGHT - 1;
}

void term_putchar(char c) {
    if (c == '\n') {
        term_col = 0;
        term_row++;
    } else if (c == '\r') {
        term_col = 0;
    } else if (c == '\t') {
        term_col = (term_col + 8) & ~7;
    } else {
        VGA_BUFFER[term_row * VGA_WIDTH + term_col] = VGA_ENTRY(c, term_color);
        term_col++;
    }
    
    if (term_col >= VGA_WIDTH) {
        term_col = 0;
        term_row++;
    }
    
    if (term_row >= VGA_HEIGHT) {
        term_scroll();
    }
}

void term_write(const char *str) {
    while (*str) {
        term_putchar(*str++);
    }
}

void term_write_color(const char *str, uint8_t color) {
    uint8_t old_color = term_color;
    term_color = color;
    term_write(str);
    term_color = old_color;
}

void term_set_cursor(int row, int col) {
    term_row = row;
    term_col = col;
}

/* ============================================================
 * Simple kprintf (integer only for now)
 * ============================================================ */

void kprint_int(int64_t n) {
    if (n < 0) {
        term_putchar('-');
        n = -n;
    }
    if (n >= 10) {
        kprint_int(n / 10);
    }
    term_putchar('0' + (n % 10));
}

void kprint_hex(uint64_t n) {
    const char *hex = "0123456789ABCDEF";
    term_write("0x");
    for (int i = 60; i >= 0; i -= 4) {
        term_putchar(hex[(n >> i) & 0xF]);
    }
}

void kprintf(const char *fmt, ...) {
    /* Simple printf - just prints strings for now */
    term_write(fmt);
}

void kputs(const char *s) {
    term_write(s);
    term_putchar('\n');
}

/* ============================================================
 * Kernel Panic
 * ============================================================ */

void __noreturn kernel_panic(const char *msg) {
    cli();
    term_color = VGA_COLOR(VGA_WHITE, VGA_RED);
    term_clear();
    term_write("\n\n");
    term_write("  *** KERNEL PANIC ***\n\n");
    term_write("  ");
    term_write(msg);
    term_write("\n\n");
    term_write("  System halted.\n");
    
    while (1) {
        hlt();
    }
}

/* ============================================================
 * PCI Functions (for GPU detection)
 * ============================================================ */

#define PCI_CONFIG_ADDRESS 0xCF8
#define PCI_CONFIG_DATA    0xCFC

uint32_t pci_read(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t address = (1 << 31) | (bus << 16) | (slot << 11) | 
                       (func << 8) | (offset & 0xFC);
    outl(PCI_CONFIG_ADDRESS, address);
    return inl(PCI_CONFIG_DATA);
}

void pci_scan_bus(void) {
    term_write("\n[PCI] Scanning bus...\n");
    
    for (int bus = 0; bus < 256; bus++) {
        for (int slot = 0; slot < 32; slot++) {
            uint32_t vendor_device = pci_read(bus, slot, 0, 0);
            uint16_t vendor = vendor_device & 0xFFFF;
            uint16_t device = (vendor_device >> 16) & 0xFFFF;
            
            if (vendor != 0xFFFF) {
                uint32_t class_code = pci_read(bus, slot, 0, 8);
                uint8_t base_class = (class_code >> 24) & 0xFF;
                uint8_t sub_class = (class_code >> 16) & 0xFF;
                
                /* Check for VGA controller (class 0x03) */
                if (base_class == 0x03) {
                    term_write("  [GPU] Found: ");
                    
                    /* NVIDIA vendor ID */
                    if (vendor == 0x10DE) {
                        term_write_color("NVIDIA", VGA_COLOR(VGA_GREEN, VGA_BLUE));
                        term_write(" GPU (Nouveau compatible)\n");
                    }
                    /* AMD vendor ID */
                    else if (vendor == 0x1002) {
                        term_write_color("AMD", VGA_COLOR(VGA_RED, VGA_BLUE));
                        term_write(" GPU\n");
                    }
                    /* Intel vendor ID */
                    else if (vendor == 0x8086) {
                        term_write_color("Intel", VGA_COLOR(VGA_CYAN, VGA_BLUE));
                        term_write(" GPU\n");
                    }
                    else {
                        term_write("Unknown GPU\n");
                    }
                }
            }
        }
    }
}

/* ============================================================
 * Kernel Entry Point
 * ============================================================ */

void kernel_main(void) {
    /* Initialize terminal */
    term_init();
    
    /* Display boot banner */
    term_write_color("\n", VGA_COLOR(VGA_WHITE, VGA_BLUE));
    term_write_color("  ╔══════════════════════════════════════════════════════════════════╗\n", 
                     VGA_COLOR(VGA_CYAN, VGA_BLUE));
    term_write_color("  ║", VGA_COLOR(VGA_CYAN, VGA_BLUE));
    term_write_color("                      FastOS v2.0                                  ", 
                     VGA_COLOR(VGA_WHITE, VGA_BLUE));
    term_write_color("║\n", VGA_COLOR(VGA_CYAN, VGA_BLUE));
    term_write_color("  ║", VGA_COLOR(VGA_CYAN, VGA_BLUE));
    term_write_color("              ADead-BIB Native Operating System                    ", 
                     VGA_COLOR(VGA_LIGHT_GREY, VGA_BLUE));
    term_write_color("║\n", VGA_COLOR(VGA_CYAN, VGA_BLUE));
    term_write_color("  ╚══════════════════════════════════════════════════════════════════╝\n\n", 
                     VGA_COLOR(VGA_CYAN, VGA_BLUE));
    
    /* System info */
    term_write("[BOOT] Kernel loaded at 0x100000\n");
    term_write("[BOOT] Compiled with ADead-BIB C compiler\n");
    term_write("[BOOT] Architecture: x86-64\n\n");
    
    /* CPU info */
    uint32_t eax, ebx, ecx, edx;
    cpuid(0, &eax, &ebx, &ecx, &edx);
    
    char vendor[13];
    *((uint32_t*)&vendor[0]) = ebx;
    *((uint32_t*)&vendor[4]) = edx;
    *((uint32_t*)&vendor[8]) = ecx;
    vendor[12] = '\0';
    
    term_write("[CPU] Vendor: ");
    term_write(vendor);
    term_write("\n");
    
    /* PCI scan for GPUs */
    pci_scan_bus();
    
    /* Memory info */
    term_write("\n[MEM] Initializing memory manager...\n");
    term_write("[MEM] Heap starts at 0x200000\n");
    
    /* Ready */
    term_write("\n");
    term_write_color("[OK]", VGA_COLOR(VGA_GREEN, VGA_BLUE));
    term_write(" FastOS kernel initialized successfully!\n\n");
    
    term_write("Type 'help' for available commands.\n");
    term_write_color("fastos> ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    
    /* Halt - in real OS, would start scheduler */
    while (1) {
        hlt();
    }
}
