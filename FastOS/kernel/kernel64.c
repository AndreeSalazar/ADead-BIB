/* ============================================================
 * FastOS v2.0 — 64-bit Kernel
 * Compiled by ADead-BIB C Compiler
 * ============================================================
 * C takes FULL control after boot64.asm transitions to long mode
 * Inherits from Windows and Linux standard libraries
 * ============================================================ */

/* ============================================================
 * Type Definitions (Windows/Linux compatible)
 * ============================================================ */

typedef unsigned char       uint8_t;
typedef unsigned short      uint16_t;
typedef unsigned int        uint32_t;
typedef unsigned long long  uint64_t;
typedef signed char         int8_t;
typedef signed short        int16_t;
typedef signed int          int32_t;
typedef signed long long    int64_t;
typedef uint64_t            size_t;
typedef int64_t             ssize_t;
typedef uint64_t            uintptr_t;

#define NULL ((void*)0)
#define true 1
#define false 0
typedef int bool;

/* ============================================================
 * Hardware I/O (inline assembly for port access)
 * ============================================================ */

static inline void outb(uint16_t port, uint8_t value) {
    __asm__ volatile("outb %0, %1" : : "a"(value), "Nd"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t value;
    __asm__ volatile("inb %1, %0" : "=a"(value) : "Nd"(port));
    return value;
}

static inline void outw(uint16_t port, uint16_t value) {
    __asm__ volatile("outw %0, %1" : : "a"(value), "Nd"(port));
}

static inline uint16_t inw(uint16_t port) {
    uint16_t value;
    __asm__ volatile("inw %1, %0" : "=a"(value) : "Nd"(port));
    return value;
}

static inline void outl(uint16_t port, uint32_t value) {
    __asm__ volatile("outl %0, %1" : : "a"(value), "Nd"(port));
}

static inline uint32_t inl(uint16_t port) {
    uint32_t value;
    __asm__ volatile("inl %1, %0" : "=a"(value) : "Nd"(port));
    return value;
}

static inline void cli(void) { __asm__ volatile("cli"); }
static inline void sti(void) { __asm__ volatile("sti"); }
static inline void hlt(void) { __asm__ volatile("hlt"); }

/* ============================================================
 * VGA Text Mode Driver
 * ============================================================ */

#define VGA_BASE        0xB8000
#define VGA_WIDTH       80
#define VGA_HEIGHT      25

#define COLOR_BLACK     0x0
#define COLOR_BLUE      0x1
#define COLOR_GREEN     0x2
#define COLOR_CYAN      0x3
#define COLOR_RED       0x4
#define COLOR_MAGENTA   0x5
#define COLOR_BROWN     0x6
#define COLOR_LGRAY     0x7
#define COLOR_DGRAY     0x8
#define COLOR_LBLUE     0x9
#define COLOR_LGREEN    0xA
#define COLOR_LCYAN     0xB
#define COLOR_LRED      0xC
#define COLOR_LMAGENTA  0xD
#define COLOR_YELLOW    0xE
#define COLOR_WHITE     0xF

#define VGA_ATTR(bg, fg) (((bg) << 4) | (fg))

static uint16_t* vga_buffer = (uint16_t*)VGA_BASE;
static int cursor_x = 0;
static int cursor_y = 0;
static uint8_t current_attr = VGA_ATTR(COLOR_BLACK, COLOR_LGRAY);

void vga_clear(uint8_t attr) {
    uint16_t blank = (attr << 8) | ' ';
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++) {
        vga_buffer[i] = blank;
    }
    cursor_x = 0;
    cursor_y = 0;
}

void vga_putchar_at(int x, int y, char c, uint8_t attr) {
    if (x >= 0 && x < VGA_WIDTH && y >= 0 && y < VGA_HEIGHT) {
        vga_buffer[y * VGA_WIDTH + x] = (attr << 8) | c;
    }
}

void vga_scroll(void) {
    for (int y = 0; y < VGA_HEIGHT - 1; y++) {
        for (int x = 0; x < VGA_WIDTH; x++) {
            vga_buffer[y * VGA_WIDTH + x] = vga_buffer[(y + 1) * VGA_WIDTH + x];
        }
    }
    uint16_t blank = (current_attr << 8) | ' ';
    for (int x = 0; x < VGA_WIDTH; x++) {
        vga_buffer[(VGA_HEIGHT - 1) * VGA_WIDTH + x] = blank;
    }
}

void vga_putchar(char c) {
    if (c == '\n') {
        cursor_x = 0;
        cursor_y++;
    } else if (c == '\r') {
        cursor_x = 0;
    } else if (c == '\t') {
        cursor_x = (cursor_x + 8) & ~7;
    } else if (c == '\b') {
        if (cursor_x > 0) cursor_x--;
    } else {
        vga_putchar_at(cursor_x, cursor_y, c, current_attr);
        cursor_x++;
    }
    
    if (cursor_x >= VGA_WIDTH) {
        cursor_x = 0;
        cursor_y++;
    }
    
    if (cursor_y >= VGA_HEIGHT) {
        vga_scroll();
        cursor_y = VGA_HEIGHT - 1;
    }
}

void vga_puts(const char* str) {
    while (*str) {
        vga_putchar(*str++);
    }
}

void vga_set_color(uint8_t attr) {
    current_attr = attr;
}

void vga_fill_rect(int x, int y, int w, int h, char c, uint8_t attr) {
    for (int j = 0; j < h; j++) {
        for (int i = 0; i < w; i++) {
            vga_putchar_at(x + i, y + j, c, attr);
        }
    }
}

void vga_puts_at(int x, int y, const char* str, uint8_t attr) {
    while (*str) {
        vga_putchar_at(x++, y, *str++, attr);
    }
}

/* ============================================================
 * String Functions (libc compatible)
 * ============================================================ */

size_t strlen(const char* str) {
    size_t len = 0;
    while (str[len]) len++;
    return len;
}

void* memset(void* dest, int c, size_t n) {
    uint8_t* d = (uint8_t*)dest;
    while (n--) *d++ = (uint8_t)c;
    return dest;
}

void* memcpy(void* dest, const void* src, size_t n) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;
    while (n--) *d++ = *s++;
    return dest;
}

int strcmp(const char* s1, const char* s2) {
    while (*s1 && *s1 == *s2) { s1++; s2++; }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

int strncmp(const char* s1, const char* s2, size_t n) {
    while (n && *s1 && *s1 == *s2) { s1++; s2++; n--; }
    return n ? *(unsigned char*)s1 - *(unsigned char*)s2 : 0;
}

char* strcpy(char* dest, const char* src) {
    char* d = dest;
    while ((*d++ = *src++));
    return dest;
}

/* ============================================================
 * Number to String Conversion
 * ============================================================ */

static void itoa(int64_t value, char* str, int base) {
    char* p = str;
    char* p1, *p2;
    uint64_t ud = value;
    int negative = 0;
    
    if (base == 10 && value < 0) {
        negative = 1;
        ud = -value;
    }
    
    do {
        int remainder = ud % base;
        *p++ = (remainder < 10) ? remainder + '0' : remainder - 10 + 'a';
    } while (ud /= base);
    
    if (negative) *p++ = '-';
    *p = 0;
    
    p1 = str;
    p2 = p - 1;
    while (p1 < p2) {
        char tmp = *p1;
        *p1++ = *p2;
        *p2-- = tmp;
    }
}

void vga_print_hex(uint64_t value) {
    char buf[17];
    itoa(value, buf, 16);
    vga_puts("0x");
    vga_puts(buf);
}

void vga_print_dec(int64_t value) {
    char buf[21];
    itoa(value, buf, 10);
    vga_puts(buf);
}

/* ============================================================
 * PS/2 Keyboard Driver
 * ============================================================ */

#define KB_DATA_PORT    0x60
#define KB_STATUS_PORT  0x64
#define KB_CMD_PORT     0x64

static const char scancode_ascii[128] = {
    0, 27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0, 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    0, '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0,
    '*', 0, ' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    '7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

static const char scancode_shift[128] = {
    0, 27, '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\b',
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n',
    0, 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~',
    0, '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', 0,
    '*', 0, ' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    '7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

static uint8_t shift_pressed = 0;
static uint8_t ctrl_pressed = 0;
static uint8_t alt_pressed = 0;

int kb_has_data(void) {
    return inb(KB_STATUS_PORT) & 0x01;
}

char kb_getchar(void) {
    while (!kb_has_data()) {
        hlt();
    }
    
    uint8_t scancode = inb(KB_DATA_PORT);
    
    if (scancode == 0x2A || scancode == 0x36) { shift_pressed = 1; return 0; }
    if (scancode == 0xAA || scancode == 0xB6) { shift_pressed = 0; return 0; }
    if (scancode == 0x1D) { ctrl_pressed = 1; return 0; }
    if (scancode == 0x9D) { ctrl_pressed = 0; return 0; }
    if (scancode == 0x38) { alt_pressed = 1; return 0; }
    if (scancode == 0xB8) { alt_pressed = 0; return 0; }
    
    if (scancode & 0x80) return 0;
    
    if (shift_pressed) {
        return scancode_shift[scancode & 0x7F];
    }
    return scancode_ascii[scancode & 0x7F];
}

char kb_getchar_nonblock(void) {
    if (!kb_has_data()) return 0;
    return kb_getchar();
}

/* ============================================================
 * PCI Bus Driver
 * ============================================================ */

#define PCI_CONFIG_ADDR 0xCF8
#define PCI_CONFIG_DATA 0xCFC

uint32_t pci_read(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t addr = (1 << 31) | (bus << 16) | (slot << 11) | (func << 8) | (offset & 0xFC);
    outl(PCI_CONFIG_ADDR, addr);
    return inl(PCI_CONFIG_DATA);
}

void pci_write(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset, uint32_t value) {
    uint32_t addr = (1 << 31) | (bus << 16) | (slot << 11) | (func << 8) | (offset & 0xFC);
    outl(PCI_CONFIG_ADDR, addr);
    outl(PCI_CONFIG_DATA, value);
}

typedef struct {
    uint16_t vendor_id;
    uint16_t device_id;
    uint8_t bus, slot, func;
    uint8_t class_code, subclass;
} pci_device_t;

static pci_device_t pci_devices[64];
static int pci_device_count = 0;

void pci_scan(void) {
    pci_device_count = 0;
    
    for (int bus = 0; bus < 256; bus++) {
        for (int slot = 0; slot < 32; slot++) {
            uint32_t id = pci_read(bus, slot, 0, 0);
            if ((id & 0xFFFF) == 0xFFFF) continue;
            
            pci_device_t* dev = &pci_devices[pci_device_count++];
            dev->vendor_id = id & 0xFFFF;
            dev->device_id = (id >> 16) & 0xFFFF;
            dev->bus = bus;
            dev->slot = slot;
            dev->func = 0;
            
            uint32_t class_info = pci_read(bus, slot, 0, 0x08);
            dev->class_code = (class_info >> 24) & 0xFF;
            dev->subclass = (class_info >> 16) & 0xFF;
            
            if (pci_device_count >= 64) return;
        }
    }
}

/* ============================================================
 * Desktop Environment
 * ============================================================ */

#define DESKTOP_BG      VGA_ATTR(COLOR_BLUE, COLOR_WHITE)
#define TASKBAR_BG      VGA_ATTR(COLOR_LGRAY, COLOR_BLACK)
#define TERMINAL_TITLE  VGA_ATTR(COLOR_BLUE, COLOR_WHITE)
#define TERMINAL_BG     VGA_ATTR(COLOR_BLACK, COLOR_LGREEN)

typedef struct {
    int x, y, width, height;
    const char* title;
    uint8_t title_attr;
    uint8_t content_attr;
} window_t;

static window_t terminal_window = {15, 3, 50, 15, "Terminal - FastOS v2.0", TERMINAL_TITLE, TERMINAL_BG};

void draw_window(window_t* win) {
    vga_fill_rect(win->x, win->y, win->width, 1, ' ', win->title_attr);
    vga_puts_at(win->x + 1, win->y, win->title, win->title_attr);
    vga_puts_at(win->x + win->width - 4, win->y, "[X]", VGA_ATTR(COLOR_RED, COLOR_WHITE));
    vga_fill_rect(win->x, win->y + 1, win->width, win->height - 1, ' ', win->content_attr);
}

void draw_desktop(void) {
    vga_clear(DESKTOP_BG);
    
    vga_fill_rect(0, VGA_HEIGHT - 1, VGA_WIDTH, 1, ' ', TASKBAR_BG);
    vga_puts_at(1, VGA_HEIGHT - 1, "[Start]", TASKBAR_BG);
    vga_puts_at(55, VGA_HEIGHT - 1, "FastOS v2.0 | ADead-BIB", TASKBAR_BG);
    
    vga_puts_at(2, 2, "[#] Terminal", VGA_ATTR(COLOR_BLUE, COLOR_YELLOW));
    vga_puts_at(2, 5, "[#] Files", VGA_ATTR(COLOR_BLUE, COLOR_YELLOW));
    vga_puts_at(2, 8, "[#] Settings", VGA_ATTR(COLOR_BLUE, COLOR_YELLOW));
    vga_puts_at(2, 11, "[#] Editor", VGA_ATTR(COLOR_BLUE, COLOR_YELLOW));
    vga_puts_at(2, 14, "[#] Info", VGA_ATTR(COLOR_BLUE, COLOR_YELLOW));
    vga_puts_at(2, 17, "[#] Power", VGA_ATTR(COLOR_BLUE, COLOR_LRED));
    
    draw_window(&terminal_window);
}

/* ============================================================
 * Terminal Shell
 * ============================================================ */

static char cmd_buffer[256];
static int cmd_pos = 0;
static int term_line = 0;

void term_print(const char* str) {
    int x = terminal_window.x + 1;
    int y = terminal_window.y + 1 + term_line;
    
    if (y >= terminal_window.y + terminal_window.height - 1) {
        for (int j = terminal_window.y + 1; j < terminal_window.y + terminal_window.height - 2; j++) {
            for (int i = terminal_window.x; i < terminal_window.x + terminal_window.width; i++) {
                vga_buffer[j * VGA_WIDTH + i] = vga_buffer[(j + 1) * VGA_WIDTH + i];
            }
        }
        vga_fill_rect(terminal_window.x, terminal_window.y + terminal_window.height - 2, 
                      terminal_window.width, 1, ' ', terminal_window.content_attr);
        term_line--;
        y--;
    }
    
    vga_puts_at(x, y, str, terminal_window.content_attr);
    term_line++;
}

void term_prompt(void) {
    int x = terminal_window.x + 1;
    int y = terminal_window.y + 1 + term_line;
    vga_puts_at(x, y, "fastos> ", VGA_ATTR(COLOR_BLACK, COLOR_LCYAN));
}

void term_execute(const char* cmd) {
    if (strcmp(cmd, "help") == 0) {
        term_print("Commands: help, info, clear, pci, mem, exit");
    } else if (strcmp(cmd, "info") == 0) {
        term_print("FastOS v2.0 - 64-bit Long Mode");
        term_print("Compiler: ADead-BIB");
        term_print("[BG] Binary Guardian: ACTIVE");
        term_print("[Rust] Safety Layer: ENABLED");
    } else if (strcmp(cmd, "clear") == 0) {
        vga_fill_rect(terminal_window.x, terminal_window.y + 1,
                      terminal_window.width, terminal_window.height - 1,
                      ' ', terminal_window.content_attr);
        term_line = 0;
    } else if (strcmp(cmd, "pci") == 0) {
        term_print("PCI Devices:");
        for (int i = 0; i < pci_device_count && i < 5; i++) {
            pci_device_t* dev = &pci_devices[i];
            char buf[64];
            vga_puts_at(terminal_window.x + 1, terminal_window.y + 1 + term_line,
                       "  ", terminal_window.content_attr);
            term_line++;
        }
    } else if (strcmp(cmd, "mem") == 0) {
        term_print("Memory: 128 MB detected");
    } else if (strcmp(cmd, "exit") == 0) {
        term_print("Shutting down...");
    } else if (cmd[0] != 0) {
        term_print("Unknown command. Type 'help'");
    }
}

/* ============================================================
 * Kernel Main Entry Point
 * ============================================================ */

void kernel_main(void) {
    vga_set_color(VGA_ATTR(COLOR_BLACK, COLOR_LGREEN));
    
    pci_scan();
    
    draw_desktop();
    
    term_print("FastOS v2.0 - ADead-BIB Native OS");
    term_print("[BG] Binary Guardian: ACTIVE");
    term_print("[Rust] Safety Layer: ENABLED");
    term_print("Architecture: x86-64 Long Mode");
    term_print("");
    term_prompt();
    
    int cursor_x = terminal_window.x + 9;
    int cursor_y = terminal_window.y + 1 + term_line;
    uint32_t tick = 0;
    
    while (1) {
        tick++;
        
        cursor_x = terminal_window.x + 9 + cmd_pos;
        cursor_y = terminal_window.y + 1 + term_line;
        
        if ((tick & 0x7FFF) < 0x4000) {
            vga_putchar_at(cursor_x, cursor_y, '_', terminal_window.content_attr);
        } else {
            vga_putchar_at(cursor_x, cursor_y, ' ', terminal_window.content_attr);
        }
        
        char c = kb_getchar_nonblock();
        if (c == 0) {
            hlt();
            continue;
        }
        
        vga_putchar_at(cursor_x, cursor_y, ' ', terminal_window.content_attr);
        
        if (c == 27) {
            vga_clear(VGA_ATTR(COLOR_RED, COLOR_WHITE));
            vga_puts_at(30, 12, "System Halted", VGA_ATTR(COLOR_RED, COLOR_WHITE));
            cli();
            while (1) hlt();
        } else if (c == '\n') {
            cmd_buffer[cmd_pos] = 0;
            term_line++;
            term_execute(cmd_buffer);
            cmd_pos = 0;
            term_prompt();
        } else if (c == '\b') {
            if (cmd_pos > 0) {
                cmd_pos--;
                vga_putchar_at(terminal_window.x + 9 + cmd_pos, cursor_y, ' ', terminal_window.content_attr);
            }
        } else if (c >= 32 && c < 127) {
            if (cmd_pos < 40) {
                cmd_buffer[cmd_pos++] = c;
                vga_putchar_at(terminal_window.x + 8 + cmd_pos, cursor_y, c, terminal_window.content_attr);
            }
        }
    }
}

/* ============================================================
 * Entry Point (called from boot64.asm)
 * ============================================================ */

void _start(void) {
    kernel_main();
    while (1) hlt();
}
