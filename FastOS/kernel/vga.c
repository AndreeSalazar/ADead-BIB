/*
 * kernel/vga.c — VGA Text Mode Terminal
 * FastOS v2.0
 *
 * Provides term_init(), term_putchar(), term_write(), term_write_color(), kputs().
 * Split from main.c so unity build can include VGA before printf and subsystems,
 * and kernel_main() after all subsystems.
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* Alias locales de colores para legibilidad del banner */
#define VGA_LIGHT_GREY    VGA_LGRAY
#define VGA_DARK_GREY     VGA_DGRAY

#define VGA_ENTRY(ch, attr) ((uint16_t)((uint8_t)(ch)) | ((uint16_t)(attr) << 8))

/* Terminal estado global */
static int     term_row   = 0;
static int     term_col   = 0;
static uint8_t term_color = 0;

void term_init(void) {
    term_color = VGA_COLOR(VGA_WHITE, VGA_BLUE);
    term_row   = 0;
    term_col   = 0;
    for (int y = 0; y < VGA_HEIGHT; y++)
        for (int x = 0; x < VGA_WIDTH; x++)
            VGA_BUFFER[y * VGA_WIDTH + x] = VGA_ENTRY(' ', term_color);
}

static void term_scroll(void) {
    for (int y = 0; y < VGA_HEIGHT - 1; y++)
        for (int x = 0; x < VGA_WIDTH; x++)
            VGA_BUFFER[y * VGA_WIDTH + x] = VGA_BUFFER[(y+1) * VGA_WIDTH + x];
    uint8_t blank_color = VGA_COLOR(VGA_WHITE, VGA_BLUE);
    for (int x = 0; x < VGA_WIDTH; x++)
        VGA_BUFFER[(VGA_HEIGHT-1) * VGA_WIDTH + x] = VGA_ENTRY(' ', blank_color);
    term_row = VGA_HEIGHT - 1;
}

void term_putchar(char c) {
    if (c == '\n') { term_col = 0; term_row++; }
    else if (c == '\r') { term_col = 0; }
    else if (c == '\t') { term_col = (term_col + 8) & ~7; }
    else {
        VGA_BUFFER[term_row * VGA_WIDTH + term_col] = VGA_ENTRY(c, term_color);
        term_col++;
    }
    if (term_col >= VGA_WIDTH)  { term_col = 0; term_row++; }
    if (term_row >= VGA_HEIGHT) { term_scroll(); }
}

void term_write(const char *str) {
    while (*str) term_putchar(*str++);
}

void term_write_color(const char *str, uint8_t color) {
    uint8_t old = term_color;
    term_color = color;
    term_write(str);
    term_color = old;
}

void kputs(const char *s) {
    term_write(s);
    term_putchar('\n');
}
