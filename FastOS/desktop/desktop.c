/*
 * FastOS v2.0 — Desktop Environment
 * Complete desktop with windows, mouse, and applications
 * 
 * Based on: ToaruOS, Windows 11
 * Compile: adB cc desktop.c -o desktop.po --kernel
 */

#include "../include/types.h"

/* ============================================================
 * VGA Text Mode Constants
 * ============================================================ */

#define VGA_WIDTH       80
#define VGA_HEIGHT      25
#define VGA_MEMORY      ((volatile uint16_t*)0xB8000)

/* Colors */
#define COLOR_BLACK         0x0
#define COLOR_BLUE          0x1
#define COLOR_GREEN         0x2
#define COLOR_CYAN          0x3
#define COLOR_RED           0x4
#define COLOR_MAGENTA       0x5
#define COLOR_BROWN         0x6
#define COLOR_LIGHT_GRAY    0x7
#define COLOR_DARK_GRAY     0x8
#define COLOR_LIGHT_BLUE    0x9
#define COLOR_LIGHT_GREEN   0xA
#define COLOR_LIGHT_CYAN    0xB
#define COLOR_LIGHT_RED     0xC
#define COLOR_LIGHT_MAGENTA 0xD
#define COLOR_YELLOW        0xE
#define COLOR_WHITE         0xF

#define MAKE_COLOR(fg, bg)  (((bg) << 4) | (fg))
#define MAKE_ENTRY(c, color) ((uint16_t)(c) | ((uint16_t)(color) << 8))

/* ============================================================
 * Desktop State
 * ============================================================ */

typedef struct {
    int x, y;
    int width, height;
    const char *title;
    int visible;
    int focused;
    uint8_t title_color;
    uint8_t content_color;
} window_t;

typedef struct {
    int x, y;
    int prev_x, prev_y;
    uint8_t buttons;
    uint16_t saved_char;  /* Character under cursor */
} mouse_t;

typedef struct {
    int x, y;
    const char *label;
    uint8_t color;
} icon_t;

/* Global state */
static mouse_t mouse;
static window_t windows[8];
static int window_count = 0;
static icon_t icons[8];
static int icon_count = 0;

/* ============================================================
 * VGA Functions
 * ============================================================ */

static void vga_put_char(int x, int y, char c, uint8_t color) {
    if (x < 0 || x >= VGA_WIDTH || y < 0 || y >= VGA_HEIGHT) return;
    VGA_MEMORY[y * VGA_WIDTH + x] = MAKE_ENTRY(c, color);
}

static uint16_t vga_get_entry(int x, int y) {
    if (x < 0 || x >= VGA_WIDTH || y < 0 || y >= VGA_HEIGHT) return 0;
    return VGA_MEMORY[y * VGA_WIDTH + x];
}

static void vga_put_string(int x, int y, const char *str, uint8_t color) {
    while (*str && x < VGA_WIDTH) {
        vga_put_char(x++, y, *str++, color);
    }
}

static void vga_fill_rect(int x, int y, int w, int h, char c, uint8_t color) {
    for (int row = y; row < y + h && row < VGA_HEIGHT; row++) {
        for (int col = x; col < x + w && col < VGA_WIDTH; col++) {
            vga_put_char(col, row, c, color);
        }
    }
}

static void vga_clear(uint8_t color) {
    vga_fill_rect(0, 0, VGA_WIDTH, VGA_HEIGHT, ' ', color);
}

/* ============================================================
 * Window Management
 * ============================================================ */

static window_t *create_window(int x, int y, int w, int h, const char *title) {
    if (window_count >= 8) return 0;
    
    window_t *win = &windows[window_count++];
    win->x = x;
    win->y = y;
    win->width = w;
    win->height = h;
    win->title = title;
    win->visible = 1;
    win->focused = 1;
    win->title_color = MAKE_COLOR(COLOR_WHITE, COLOR_BLUE);
    win->content_color = MAKE_COLOR(COLOR_BLACK, COLOR_WHITE);
    
    return win;
}

static void draw_window(window_t *win) {
    if (!win || !win->visible) return;
    
    /* Title bar */
    vga_fill_rect(win->x, win->y, win->width, 1, ' ', win->title_color);
    vga_put_string(win->x + 1, win->y, win->title, win->title_color);
    
    /* Close button */
    vga_put_char(win->x + win->width - 3, win->y, '[', 
                 MAKE_COLOR(COLOR_WHITE, COLOR_RED));
    vga_put_char(win->x + win->width - 2, win->y, 'X', 
                 MAKE_COLOR(COLOR_WHITE, COLOR_RED));
    vga_put_char(win->x + win->width - 1, win->y, ']', 
                 MAKE_COLOR(COLOR_WHITE, COLOR_RED));
    
    /* Content area */
    vga_fill_rect(win->x, win->y + 1, win->width, win->height - 1, ' ', 
                  win->content_color);
}

static void draw_window_content(window_t *win, int line, const char *text) {
    if (!win || line >= win->height - 1) return;
    vga_put_string(win->x + 1, win->y + 1 + line, text, win->content_color);
}

/* ============================================================
 * Desktop Icons
 * ============================================================ */

static void add_icon(int x, int y, const char *label, uint8_t color) {
    if (icon_count >= 8) return;
    
    icon_t *icon = &icons[icon_count++];
    icon->x = x;
    icon->y = y;
    icon->label = label;
    icon->color = color;
}

static void draw_icons(void) {
    for (int i = 0; i < icon_count; i++) {
        vga_put_string(icons[i].x, icons[i].y, icons[i].label, icons[i].color);
    }
}

/* ============================================================
 * Taskbar
 * ============================================================ */

static void draw_taskbar(void) {
    /* Taskbar background */
    vga_fill_rect(0, VGA_HEIGHT - 1, VGA_WIDTH, 1, ' ', 
                  MAKE_COLOR(COLOR_WHITE, COLOR_DARK_GRAY));
    
    /* Start button */
    vga_put_string(1, VGA_HEIGHT - 1, "[Start]", 
                   MAKE_COLOR(COLOR_WHITE, COLOR_DARK_GRAY));
    
    /* Window buttons */
    int btn_x = 10;
    for (int i = 0; i < window_count; i++) {
        if (windows[i].visible) {
            uint8_t color = windows[i].focused ? 
                MAKE_COLOR(COLOR_WHITE, COLOR_BLUE) :
                MAKE_COLOR(COLOR_LIGHT_GRAY, COLOR_DARK_GRAY);
            vga_put_char(btn_x, VGA_HEIGHT - 1, '[', color);
            vga_put_string(btn_x + 1, VGA_HEIGHT - 1, windows[i].title, color);
            btn_x += 12;
        }
    }
    
    /* Clock */
    vga_put_string(VGA_WIDTH - 8, VGA_HEIGHT - 1, "12:00", 
                   MAKE_COLOR(COLOR_WHITE, COLOR_DARK_GRAY));
}

/* ============================================================
 * Mouse Handling
 * ============================================================ */

/* PS/2 Mouse ports */
#define MOUSE_DATA_PORT     0x60
#define MOUSE_STATUS_PORT   0x64
#define MOUSE_CMD_PORT      0x64

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile("outb %0, %1" : : "a"(val), "Nd"(port));
}

static void mouse_wait_write(void) {
    int timeout = 100000;
    while ((inb(MOUSE_STATUS_PORT) & 0x02) && timeout--);
}

static void mouse_wait_read(void) {
    int timeout = 100000;
    while (!(inb(MOUSE_STATUS_PORT) & 0x01) && timeout--);
}

static void mouse_write(uint8_t data) {
    mouse_wait_write();
    outb(MOUSE_CMD_PORT, 0xD4);  /* Send to mouse */
    mouse_wait_write();
    outb(MOUSE_DATA_PORT, data);
}

static uint8_t mouse_read(void) {
    mouse_wait_read();
    return inb(MOUSE_DATA_PORT);
}

static void mouse_init(void) {
    /* Enable auxiliary device (mouse) */
    mouse_wait_write();
    outb(MOUSE_CMD_PORT, 0xA8);
    
    /* Enable interrupts */
    mouse_wait_write();
    outb(MOUSE_CMD_PORT, 0x20);
    mouse_wait_read();
    uint8_t status = inb(MOUSE_DATA_PORT);
    status |= 0x02;  /* Enable IRQ12 */
    mouse_wait_write();
    outb(MOUSE_CMD_PORT, 0x60);
    mouse_wait_write();
    outb(MOUSE_DATA_PORT, status);
    
    /* Set defaults */
    mouse_write(0xF6);
    mouse_read();  /* ACK */
    
    /* Enable mouse */
    mouse_write(0xF4);
    mouse_read();  /* ACK */
    
    /* Initialize position */
    mouse.x = VGA_WIDTH / 2;
    mouse.y = VGA_HEIGHT / 2;
    mouse.prev_x = mouse.x;
    mouse.prev_y = mouse.y;
    mouse.buttons = 0;
    mouse.saved_char = MAKE_ENTRY(' ', MAKE_COLOR(COLOR_WHITE, COLOR_BLUE));
}

static void mouse_draw(void) {
    /* Restore previous position */
    VGA_MEMORY[mouse.prev_y * VGA_WIDTH + mouse.prev_x] = mouse.saved_char;
    
    /* Save character at new position */
    mouse.saved_char = VGA_MEMORY[mouse.y * VGA_WIDTH + mouse.x];
    
    /* Draw cursor */
    VGA_MEMORY[mouse.y * VGA_WIDTH + mouse.x] = 
        MAKE_ENTRY(0x1E, MAKE_COLOR(COLOR_YELLOW, COLOR_RED));  /* Arrow */
    
    mouse.prev_x = mouse.x;
    mouse.prev_y = mouse.y;
}

static void mouse_poll(void) {
    /* Check if data available */
    if (!(inb(MOUSE_STATUS_PORT) & 0x01)) return;
    if (!(inb(MOUSE_STATUS_PORT) & 0x20)) return;  /* Must be mouse data */
    
    /* Read packet */
    uint8_t flags = inb(MOUSE_DATA_PORT);
    
    /* Wait for more data */
    mouse_wait_read();
    int8_t dx = (int8_t)inb(MOUSE_DATA_PORT);
    
    mouse_wait_read();
    int8_t dy = (int8_t)inb(MOUSE_DATA_PORT);
    
    /* Check for valid packet */
    if (!(flags & 0x08)) return;  /* Bit 3 must be set */
    
    /* Apply movement (scale down for text mode) */
    mouse.x += dx / 8;
    mouse.y -= dy / 8;  /* Y is inverted */
    
    /* Clamp to screen */
    if (mouse.x < 0) mouse.x = 0;
    if (mouse.x >= VGA_WIDTH) mouse.x = VGA_WIDTH - 1;
    if (mouse.y < 0) mouse.y = 0;
    if (mouse.y >= VGA_HEIGHT - 1) mouse.y = VGA_HEIGHT - 2;  /* Above taskbar */
    
    /* Update buttons */
    mouse.buttons = flags & 0x07;
}

/* ============================================================
 * Keyboard Handling
 * ============================================================ */

static int keyboard_poll(void) {
    if (!(inb(0x64) & 0x01)) return 0;
    return inb(0x60);
}

/* ============================================================
 * Desktop Main
 * ============================================================ */

static void draw_desktop(void) {
    /* Desktop background - blue */
    vga_clear(MAKE_COLOR(COLOR_WHITE, COLOR_BLUE));
    
    /* Draw icons */
    draw_icons();
    
    /* Draw windows */
    for (int i = 0; i < window_count; i++) {
        draw_window(&windows[i]);
    }
    
    /* Draw taskbar */
    draw_taskbar();
}

void desktop_init(void) {
    /* Initialize mouse */
    mouse_init();
    
    /* Create desktop icons */
    add_icon(2, 2, "[>] Terminal", MAKE_COLOR(COLOR_YELLOW, COLOR_BLUE));
    add_icon(2, 4, "[D] Files", MAKE_COLOR(COLOR_YELLOW, COLOR_BLUE));
    add_icon(2, 6, "[*] Settings", MAKE_COLOR(COLOR_YELLOW, COLOR_BLUE));
    add_icon(2, 8, "[~] Browser", MAKE_COLOR(COLOR_YELLOW, COLOR_BLUE));
    
    /* Create Terminal window */
    window_t *term = create_window(15, 3, 45, 12, "Terminal - FastOS");
    if (term) {
        draw_window(term);
        draw_window_content(term, 0, "FastOS v2.0 [C Master + Rust Safety]");
        draw_window_content(term, 2, "C:\\> [BG] Binary Guardian: ACTIVE");
        draw_window_content(term, 3, "C:\\> [musl] libc: LOADED");
        draw_window_content(term, 4, "C:\\> [Po] PE+ELF+Win32 ready");
        draw_window_content(term, 5, "C:\\> [Nouveau] GPU driver OK");
        draw_window_content(term, 7, "C:\\> _");
    }
    
    /* Initial draw */
    draw_desktop();
    mouse_draw();
}

void desktop_run(void) {
    int running = 1;
    int redraw_needed = 0;
    
    while (running) {
        /* Poll mouse */
        int old_x = mouse.x;
        int old_y = mouse.y;
        mouse_poll();
        
        /* Check if mouse moved */
        if (mouse.x != old_x || mouse.y != old_y) {
            mouse_draw();
        }
        
        /* Handle mouse clicks */
        if (mouse.buttons & 0x01) {  /* Left button */
            /* Check if clicking on window title bar */
            for (int i = window_count - 1; i >= 0; i--) {
                window_t *win = &windows[i];
                if (mouse.x >= win->x && mouse.x < win->x + win->width &&
                    mouse.y == win->y) {
                    /* Check close button */
                    if (mouse.x >= win->x + win->width - 3) {
                        win->visible = 0;
                        redraw_needed = 1;
                    }
                    break;
                }
            }
        }
        
        /* Poll keyboard */
        int key = keyboard_poll();
        if (key == 0x01) {  /* ESC */
            running = 0;
        }
        
        /* Redraw if needed */
        if (redraw_needed) {
            draw_desktop();
            mouse_draw();
            redraw_needed = 0;
        }
        
        /* Small delay */
        for (volatile int i = 0; i < 10000; i++);
    }
}

/* Entry point */
void _start(void) {
    desktop_init();
    desktop_run();
    
    /* Halt */
    while (1) {
        __asm__ volatile("hlt");
    }
}
