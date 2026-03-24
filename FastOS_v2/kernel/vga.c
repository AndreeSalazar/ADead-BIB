/*
 * FastOS v2.0 — VGA Text Mode Driver
 * 80x25 text mode, hardware cursor, scrolling, colors
 * Maps directly to 0xB8000 framebuffer
 */

#include "include/kernel.h"

/* VGA state */
static int vga_row = 0;
static int vga_col = 0;
static uint8_t vga_attr = VGA_COLOR(VGA_LGRAY, VGA_BLACK);

/* ── Hardware cursor control via CRTC registers ── */

void vga_update_cursor(void) {
    uint16_t pos = vga_row * VGA_WIDTH + vga_col;
    /* CRT Controller: cursor location high/low */
    outb(0x3D4, 0x0F);
    outb(0x3D5, (uint8_t)(pos & 0xFF));
    outb(0x3D4, 0x0E);
    outb(0x3D5, (uint8_t)((pos >> 8) & 0xFF));
}

/* ── Initialize VGA text mode ── */

void vga_init(void) {
    vga_row = 0;
    vga_col = 0;
    vga_attr = VGA_COLOR(VGA_LGRAY, VGA_BLACK);

    /* Enable cursor (scanline 14-15) */
    outb(0x3D4, 0x0A);
    outb(0x3D5, (inb(0x3D5) & 0xC0) | 14);
    outb(0x3D4, 0x0B);
    outb(0x3D5, (inb(0x3D5) & 0xE0) | 15);

    vga_clear();
}

/* ── Clear screen ── */

void vga_clear(void) {
    uint16_t blank = (uint16_t)vga_attr << 8 | ' ';
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++) {
        VGA_BUFFER[i] = blank;
    }
    vga_row = 0;
    vga_col = 0;
    vga_update_cursor();
}

/* ── Scroll screen up one line ── */

void vga_scroll(void) {
    if (vga_row < VGA_HEIGHT) return;

    /* Move rows 1..24 to 0..23 */
    for (int i = 0; i < (VGA_HEIGHT - 1) * VGA_WIDTH; i++) {
        VGA_BUFFER[i] = VGA_BUFFER[i + VGA_WIDTH];
    }

    /* Clear last row */
    uint16_t blank = (uint16_t)vga_attr << 8 | ' ';
    for (int i = (VGA_HEIGHT - 1) * VGA_WIDTH; i < VGA_HEIGHT * VGA_WIDTH; i++) {
        VGA_BUFFER[i] = blank;
    }

    vga_row = VGA_HEIGHT - 1;
}

/* ── Put single character ── */

void vga_putchar(char c) {
    if (c == '\n') {
        vga_col = 0;
        vga_row++;
    } else if (c == '\r') {
        vga_col = 0;
    } else if (c == '\t') {
        vga_col = (vga_col + 8) & ~7;
        if (vga_col >= VGA_WIDTH) {
            vga_col = 0;
            vga_row++;
        }
    } else if (c == '\b') {
        if (vga_col > 0) {
            vga_col--;
            VGA_BUFFER[vga_row * VGA_WIDTH + vga_col] =
                (uint16_t)vga_attr << 8 | ' ';
        }
    } else {
        VGA_BUFFER[vga_row * VGA_WIDTH + vga_col] =
            (uint16_t)vga_attr << 8 | (uint8_t)c;
        vga_col++;
        if (vga_col >= VGA_WIDTH) {
            vga_col = 0;
            vga_row++;
        }
    }

    if (vga_row >= VGA_HEIGHT) {
        vga_scroll();
    }
    vga_update_cursor();
}

/* ── Print string ── */

void vga_puts(const char *s) {
    while (*s) {
        vga_putchar(*s++);
    }
}

/* ── Print string with specific color ── */

void vga_puts_color(const char *s, uint8_t color) {
    uint8_t old = vga_attr;
    vga_attr = color;
    vga_puts(s);
    vga_attr = old;
}

/* ── Set default color attribute ── */

void vga_set_color(uint8_t color) {
    vga_attr = color;
}

/* ── Set cursor position ── */

void vga_set_cursor(int row, int col) {
    vga_row = row;
    vga_col = col;
    vga_update_cursor();
}

/* ── Get cursor position ── */

void vga_get_cursor(int *row, int *col) {
    *row = vga_row;
    *col = vga_col;
}
