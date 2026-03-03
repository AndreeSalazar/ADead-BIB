// ============================================================
// FastOS v2.0 — Simple 64-bit Kernel
// Compilable by ADead-BIB with --flat
// ============================================================

// VGA text buffer address
unsigned short* vga = (unsigned short*)0xB8000;

// Screen dimensions
int screen_width = 80;
int screen_height = 25;

// Cursor position
int cursor_x = 0;
int cursor_y = 0;

// Colors
int COLOR_GREEN = 0x0A00;
int COLOR_YELLOW = 0x0E00;
int COLOR_CYAN = 0x0B00;
int COLOR_WHITE = 0x0F00;
int COLOR_RED = 0x0C00;
int COLOR_BLUE_BG = 0x1F00;

// Clear screen with color
void clear_screen(int color) {
    int i;
    for (i = 0; i < 2000; i++) {
        vga[i] = color | ' ';
    }
    cursor_x = 0;
    cursor_y = 0;
}

// Print character at position
void put_char(int x, int y, char c, int color) {
    int offset = y * 80 + x;
    vga[offset] = color | c;
}

// Print string at position
void print_at(int x, int y, char* str, int color) {
    int i = 0;
    while (str[i] != 0) {
        put_char(x + i, y, str[i], color);
        i++;
    }
}

// Print string at cursor
void print(char* str, int color) {
    int i = 0;
    while (str[i] != 0) {
        if (str[i] == '\n') {
            cursor_x = 0;
            cursor_y++;
        } else {
            put_char(cursor_x, cursor_y, str[i], color);
            cursor_x++;
            if (cursor_x >= 80) {
                cursor_x = 0;
                cursor_y++;
            }
        }
        i++;
    }
}

// Draw horizontal line
void draw_hline(int y, int color) {
    int x;
    for (x = 0; x < 80; x++) {
        put_char(x, y, '-', color);
    }
}

// Draw title bar
void draw_titlebar() {
    int x;
    for (x = 0; x < 80; x++) {
        vga[x] = 0x1720;  // White on blue, space
    }
    print_at(2, 0, "FastOS v2.0 - Desktop", 0x1700);
}

// Draw taskbar
void draw_taskbar() {
    int x;
    int offset = 24 * 80;
    for (x = 0; x < 80; x++) {
        vga[offset + x] = 0x7020;  // Gray background
    }
    print_at(1, 24, "[Start]", 0x7000);
    print_at(70, 24, "12:00", 0x7000);
}

// Draw desktop icons
void draw_icons() {
    print_at(3, 3, "[>_] Terminal", COLOR_YELLOW);
    print_at(3, 5, "[#] Files", COLOR_YELLOW);
    print_at(3, 7, "[@] Settings", COLOR_YELLOW);
    print_at(3, 9, "[i] About", COLOR_CYAN);
    print_at(3, 11, "[!] Power", COLOR_RED);
}

// Draw terminal window
void draw_terminal() {
    int y;
    int x;
    
    // Window background
    for (y = 3; y < 20; y++) {
        for (x = 20; x < 75; x++) {
            put_char(x, y, ' ', 0x0A00);
        }
    }
    
    // Title bar
    for (x = 20; x < 75; x++) {
        put_char(x, 3, ' ', 0x1700);
    }
    print_at(22, 3, "Terminal - FastOS", 0x1700);
    print_at(72, 3, "X", 0x4F00);
    
    // Content
    print_at(22, 5, "FastOS v2.0 - 64-bit Long Mode", COLOR_GREEN);
    print_at(22, 6, "Compiler: ADead-BIB", COLOR_GREEN);
    print_at(22, 7, "[BG] Binary Guardian: ACTIVE", COLOR_CYAN);
    print_at(22, 9, "Welcome to FastOS!", COLOR_WHITE);
    print_at(22, 11, "Type 'help' for commands.", COLOR_YELLOW);
    print_at(22, 13, "fastos> _", COLOR_WHITE);
}

// Kernel entry point
void kernel_main() {
    // Clear screen with blue
    clear_screen(0x1F00);
    
    // Draw UI
    draw_titlebar();
    draw_taskbar();
    draw_icons();
    draw_terminal();
    
    // Halt
    while (1) {
        // Main loop - wait for interrupts
    }
}

// Entry point for flat binary
void _start() {
    kernel_main();
}
