/* ============================================================
 * FastOS v2.0 — Simple 64-bit Kernel
 * Compatible with ADead-BIB C Compiler (no __asm__)
 * Writes directly to VGA text buffer at 0xB8000
 * ============================================================ */

void kernel_main();

void _start() {
    kernel_main();
    while (1) {
    }
}

#define VGA_BASE  0xB8000
#define SCREEN_W  80
#define SCREEN_H  25

#define COLOR_BLUE_WHITE 0x1F
#define COLOR_WHITE_BLUE 0x17
#define COLOR_GRAY_BLACK 0x70
#define COLOR_GREEN      0x0A
#define COLOR_CYAN       0x0B
#define COLOR_YELLOW     0x0E
#define COLOR_RED        0x0C
#define COLOR_MAGENTA    0x0D

void vga_clear(int attr) {
    int i = 0;
    int val = (attr * 256) + 32;
    while (i < 2000) {
        *((int*)(VGA_BASE + i * 2)) = val;
        i = i + 1;
    }
}

void vga_putchar(int x, int y, int ch, int attr) {
    int offset = (y * 80 + x) * 2;
    int val = (attr * 256) + ch;
    *((int*)(VGA_BASE + offset)) = val;
}

void vga_puts(int x, int y, int attr, char* str) {
    int i = 0;
    while (str[i] != 0) {
        vga_putchar(x + i, y, str[i], attr);
        i = i + 1;
    }
}

void draw_titlebar() {
    int i = 0;
    while (i < 80) {
        vga_putchar(i, 0, 32, COLOR_WHITE_BLUE);
        i = i + 1;
    }
    vga_puts(2, 0, COLOR_WHITE_BLUE, "FastOS v2.0 - 64-bit Long Mode");
}

void draw_taskbar() {
    int i = 0;
    while (i < 80) {
        vga_putchar(i, 24, 32, COLOR_GRAY_BLACK);
        i = i + 1;
    }
    vga_puts(1, 24, COLOR_GRAY_BLACK, "[Start]");
    vga_puts(60, 24, COLOR_GRAY_BLACK, "12:00 PM");
}

void draw_desktop() {
    vga_puts(2, 2, COLOR_YELLOW, "Desktop Icons:");
    vga_puts(4, 4, COLOR_GREEN, "[Documents]");
    vga_puts(4, 5, COLOR_GREEN, "[Terminal]");
    vga_puts(4, 6, COLOR_GREEN, "[Settings]");
    vga_puts(4, 7, COLOR_CYAN, "[ADead-BIB]");
}

void draw_info() {
    vga_puts(2, 10, COLOR_CYAN, "C Kernel: Ready for ADead-BIB compilation");
    vga_puts(2, 11, COLOR_GREEN, "[BG] Binary Guardian: ACTIVE");
    vga_puts(2, 12, COLOR_MAGENTA, "Compiler: ADead-BIB (No GCC, No LLVM, No Clang)");
    vga_puts(2, 14, COLOR_YELLOW, "Kernel is running in 64-bit Long Mode!");
    vga_puts(2, 15, COLOR_RED, "C takes FULL control.");
}

void draw_prompt() {
    vga_puts(2, 18, COLOR_GREEN, "fastos>");
    vga_putchar(10, 18, 95, COLOR_GREEN);
}

void kernel_main() {
    vga_clear(COLOR_BLUE_WHITE);
    draw_titlebar();
    draw_taskbar();
    draw_desktop();
    draw_info();
    draw_prompt();
}
