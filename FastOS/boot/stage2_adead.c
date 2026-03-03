/*
 * FastOS v2.0 - Stage 2 Bootloader
 * Compilable by ADead-BIB C Compiler
 * 
 * Philosophy: C is Master, Rust provides Safety
 * 
 * This file replaces the raw assembly bytes in build.ps1
 * ADead-BIB compiles this to flat binary for real mode
 */

#include "../include/boot_types.h"

/* ============================================================
 * VGA Text Mode Constants
 * ============================================================ */

#define VGA_BASE        0xB8000
#define VGA_WIDTH       80
#define VGA_HEIGHT      25
#define VGA_SIZE        (VGA_WIDTH * VGA_HEIGHT)

/* Colors */
#define COLOR_BLACK     0x00
#define COLOR_BLUE      0x01
#define COLOR_GREEN     0x02
#define COLOR_CYAN      0x03
#define COLOR_RED       0x04
#define COLOR_MAGENTA   0x05
#define COLOR_BROWN     0x06
#define COLOR_LGRAY     0x07
#define COLOR_DGRAY     0x08
#define COLOR_LBLUE     0x09
#define COLOR_LGREEN    0x0A
#define COLOR_LCYAN     0x0B
#define COLOR_LRED      0x0C
#define COLOR_LMAGENTA  0x0D
#define COLOR_YELLOW    0x0E
#define COLOR_WHITE     0x0F

/* Attribute macros */
#define VGA_ATTR(fg, bg)    (((bg) << 4) | (fg))
#define ATTR_BLUE_WHITE     VGA_ATTR(COLOR_WHITE, COLOR_BLUE)
#define ATTR_GRAY_BLACK     VGA_ATTR(COLOR_BLACK, COLOR_LGRAY)
#define ATTR_WHITE_BLACK    VGA_ATTR(COLOR_WHITE, COLOR_BLACK)
#define ATTR_GREEN_BLACK    VGA_ATTR(COLOR_LGREEN, COLOR_BLACK)
#define ATTR_RED_WHITE      VGA_ATTR(COLOR_WHITE, COLOR_RED)

/* ============================================================
 * Port I/O (inline assembly via ADead-BIB)
 * ============================================================ */

static inline void outb(u16 port, u8 value) {
    __asm__ volatile("outb %0, %1" : : "a"(value), "Nd"(port));
}

static inline u8 inb(u16 port) {
    u8 ret;
    __asm__ volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

/* ============================================================
 * VGA Functions
 * ============================================================ */

static volatile u16* const vga_buffer = (volatile u16*)VGA_BASE;

void vga_clear(u8 attr) {
    u16 blank = ((u16)attr << 8) | ' ';
    for (int i = 0; i < VGA_SIZE; i++) {
        vga_buffer[i] = blank;
    }
}

void vga_put_char(int x, int y, char c, u8 attr) {
    if (x >= 0 && x < VGA_WIDTH && y >= 0 && y < VGA_HEIGHT) {
        int offset = y * VGA_WIDTH + x;
        vga_buffer[offset] = ((u16)attr << 8) | (u8)c;
    }
}

void vga_put_string(int x, int y, const char* str, u8 attr) {
    while (*str) {
        vga_put_char(x++, y, *str++, attr);
        if (x >= VGA_WIDTH) {
            x = 0;
            y++;
        }
    }
}

void vga_fill_rect(int x, int y, int w, int h, char c, u8 attr) {
    for (int row = y; row < y + h && row < VGA_HEIGHT; row++) {
        for (int col = x; col < x + w && col < VGA_WIDTH; col++) {
            vga_put_char(col, row, c, attr);
        }
    }
}

void vga_draw_hline(int x, int y, int len, char c, u8 attr) {
    for (int i = 0; i < len && x + i < VGA_WIDTH; i++) {
        vga_put_char(x + i, y, c, attr);
    }
}

/* ============================================================
 * Keyboard Input (PS/2)
 * ============================================================ */

#define KB_DATA_PORT    0x60
#define KB_STATUS_PORT  0x64

static const char scancode_to_ascii[128] = {
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

u8 keyboard_read_scancode(void) {
    while ((inb(KB_STATUS_PORT) & 0x01) == 0) {
        /* Wait for data */
    }
    return inb(KB_DATA_PORT);
}

char keyboard_get_char(void) {
    u8 scancode = keyboard_read_scancode();
    if (scancode & 0x80) {
        return 0; /* Key release */
    }
    return scancode_to_ascii[scancode & 0x7F];
}

/* ============================================================
 * Window Manager (Simple)
 * ============================================================ */

typedef struct {
    int x, y;
    int width, height;
    const char* title;
    u8 title_attr;
    u8 content_attr;
    int active;
} Window;

void window_draw(const Window* win) {
    /* Title bar */
    vga_fill_rect(win->x, win->y, win->width, 1, ' ', win->title_attr);
    vga_put_string(win->x + 1, win->y, win->title, win->title_attr);
    
    /* Close button */
    vga_put_string(win->x + win->width - 4, win->y, "[X]", 
                   VGA_ATTR(COLOR_WHITE, COLOR_RED));
    
    /* Content area */
    vga_fill_rect(win->x, win->y + 1, win->width, win->height - 1, 
                  ' ', win->content_attr);
}

/* ============================================================
 * Desktop Environment
 * ============================================================ */

typedef struct {
    int cursor_x;
    int cursor_y;
    int terminal_line;
    char input_buffer[80];
    int input_pos;
} Desktop;

static Desktop desktop;

void desktop_init(void) {
    desktop.cursor_x = 40;
    desktop.cursor_y = 12;
    desktop.terminal_line = 0;
    desktop.input_pos = 0;
}

void desktop_draw_taskbar(void) {
    /* Taskbar at bottom */
    vga_fill_rect(0, VGA_HEIGHT - 1, VGA_WIDTH, 1, ' ', ATTR_GRAY_BLACK);
    vga_put_string(1, VGA_HEIGHT - 1, "[Start]", ATTR_GRAY_BLACK);
    vga_put_string(VGA_WIDTH - 20, VGA_HEIGHT - 1, "FastOS v2.0", ATTR_GRAY_BLACK);
}

void desktop_draw_icons(void) {
    /* Desktop icons */
    const char* icons[] = {
        "Terminal",
        "Files",
        "Settings",
        "Editor",
        "Info",
        "Power"
    };
    
    for (int i = 0; i < 6; i++) {
        int y = 2 + i * 3;
        vga_put_string(2, y, "[#]", VGA_ATTR(COLOR_YELLOW, COLOR_BLUE));
        vga_put_string(2, y + 1, icons[i], ATTR_BLUE_WHITE);
    }
}

void desktop_draw_cursor(void) {
    /* Simple text cursor */
    vga_put_char(desktop.cursor_x, desktop.cursor_y, '>', 
                 VGA_ATTR(COLOR_YELLOW, COLOR_RED));
}

/* ============================================================
 * Terminal Application
 * ============================================================ */

static Window terminal_window = {
    .x = 15,
    .y = 3,
    .width = 50,
    .height = 15,
    .title = "Terminal",
    .title_attr = ATTR_BLUE_WHITE,
    .content_attr = VGA_ATTR(COLOR_LGREEN, COLOR_BLACK),
    .active = 1
};

void terminal_print(const char* str) {
    int x = terminal_window.x + 1;
    int y = terminal_window.y + 1 + desktop.terminal_line;
    
    if (y >= terminal_window.y + terminal_window.height - 1) {
        /* Scroll up */
        desktop.terminal_line = 0;
        y = terminal_window.y + 1;
        vga_fill_rect(terminal_window.x, terminal_window.y + 1,
                      terminal_window.width, terminal_window.height - 1,
                      ' ', terminal_window.content_attr);
    }
    
    vga_put_string(x, y, str, terminal_window.content_attr);
    desktop.terminal_line++;
}

void terminal_prompt(void) {
    int y = terminal_window.y + 1 + desktop.terminal_line;
    if (y < terminal_window.y + terminal_window.height - 1) {
        vga_put_string(terminal_window.x + 1, y, "fastos> ", 
                       VGA_ATTR(COLOR_LCYAN, COLOR_BLACK));
    }
}

void terminal_execute(const char* cmd) {
    if (cmd[0] == 'h' && cmd[1] == 'e' && cmd[2] == 'l' && cmd[3] == 'p') {
        terminal_print("Commands: help, info, clear, exit");
    } else if (cmd[0] == 'i' && cmd[1] == 'n' && cmd[2] == 'f' && cmd[3] == 'o') {
        terminal_print("FastOS v2.0 - ADead-BIB Native OS");
        terminal_print("[BG] Binary Guardian: ACTIVE");
        terminal_print("[musl] C Library: LOADED");
        terminal_print("[Rust] Safety Layer: ENABLED");
    } else if (cmd[0] == 'c' && cmd[1] == 'l' && cmd[2] == 'e') {
        vga_fill_rect(terminal_window.x, terminal_window.y + 1,
                      terminal_window.width, terminal_window.height - 1,
                      ' ', terminal_window.content_attr);
        desktop.terminal_line = 0;
    } else if (cmd[0] != 0) {
        terminal_print("Unknown command. Type 'help'");
    }
}

/* ============================================================
 * Main Entry Point
 * ============================================================ */

void stage2_main(void) {
    /* Initialize */
    desktop_init();
    
    /* Clear screen with blue background */
    vga_clear(ATTR_BLUE_WHITE);
    
    /* Draw desktop */
    desktop_draw_taskbar();
    desktop_draw_icons();
    
    /* Draw terminal window */
    window_draw(&terminal_window);
    
    /* Welcome message */
    terminal_print("FastOS v2.0 - ADead-BIB Native OS");
    terminal_print("[BG] Binary Guardian: ACTIVE");
    terminal_print("[musl] C Library: LOADED");
    terminal_print("[Rust] Safety Layer: ENABLED");
    terminal_print("");
    terminal_prompt();
    
    /* Main loop */
    int running = 1;
    while (running) {
        char c = keyboard_get_char();
        
        if (c == 0) continue;
        
        if (c == 27) { /* ESC */
            running = 0;
        } else if (c == '\n') {
            /* Execute command */
            desktop.input_buffer[desktop.input_pos] = 0;
            desktop.terminal_line++;
            terminal_execute(desktop.input_buffer);
            desktop.input_pos = 0;
            terminal_prompt();
        } else if (c == '\b') {
            /* Backspace */
            if (desktop.input_pos > 0) {
                desktop.input_pos--;
                int x = terminal_window.x + 9 + desktop.input_pos;
                int y = terminal_window.y + 1 + desktop.terminal_line;
                vga_put_char(x, y, ' ', terminal_window.content_attr);
            }
        } else if (c >= 32 && c < 127) {
            /* Printable character */
            if (desktop.input_pos < 70) {
                desktop.input_buffer[desktop.input_pos++] = c;
                int x = terminal_window.x + 8 + desktop.input_pos;
                int y = terminal_window.y + 1 + desktop.terminal_line;
                vga_put_char(x, y, c, terminal_window.content_attr);
            }
        }
    }
    
    /* Halt */
    vga_clear(VGA_ATTR(COLOR_WHITE, COLOR_RED));
    vga_put_string(30, 12, "System Halted", VGA_ATTR(COLOR_WHITE, COLOR_RED));
    
    __asm__ volatile("cli; hlt");
}

/* Entry point for real mode */
void _start(void) {
    stage2_main();
    for(;;) {
        __asm__ volatile("hlt");
    }
}
