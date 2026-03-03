/*
 * FastOS v2.0 — Yutani-inspired Window Compositor
 * Based on: ToaruOS compositor.c by K. Lange
 * 
 * Features:
 * - Canvas-based window compositing
 * - Shared memory for client rendering
 * - Z-ordering with proper occlusion
 * - Window shadows and blur effects
 * - Mouse cursor with multiple states
 * - Keyboard focus management
 * 
 * Compile: adB cc compositor.c -o compositor.po --kernel
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ============================================================
 * Compositor Constants
 * ============================================================ */

#define COMPOSITOR_MAX_WINDOWS      256
#define COMPOSITOR_MAX_CLIENTS      64
#define COMPOSITOR_MOUSE_SCALE      1

#define WINDOW_FLAG_DECORATED       0x0001
#define WINDOW_FLAG_RESIZABLE       0x0002
#define WINDOW_FLAG_MOVABLE         0x0004
#define WINDOW_FLAG_FOCUSED         0x0008
#define WINDOW_FLAG_MINIMIZED       0x0010
#define WINDOW_FLAG_MAXIMIZED       0x0020
#define WINDOW_FLAG_FULLSCREEN      0x0040
#define WINDOW_FLAG_ALWAYS_ON_TOP   0x0080
#define WINDOW_FLAG_NO_SHADOW       0x0100
#define WINDOW_FLAG_BLUR_BEHIND     0x0200

/* Title bar dimensions */
#define TITLE_BAR_HEIGHT            24
#define BORDER_WIDTH                1
#define SHADOW_SIZE                 8
#define BUTTON_SIZE                 16

/* Colors (ARGB) */
#define COLOR_TITLE_ACTIVE          0xFF3C3C3C
#define COLOR_TITLE_INACTIVE        0xFF5C5C5C
#define COLOR_TITLE_TEXT            0xFFFFFFFF
#define COLOR_BORDER_ACTIVE         0xFF0078D7
#define COLOR_BORDER_INACTIVE       0xFF808080
#define COLOR_SHADOW                0x40000000
#define COLOR_BUTTON_CLOSE          0xFFE81123
#define COLOR_BUTTON_MAXIMIZE       0xFF00CC00
#define COLOR_BUTTON_MINIMIZE       0xFFFFBB00
#define COLOR_DESKTOP               0xFF008080

/* ============================================================
 * Compositor Structures
 * ============================================================ */

/* Rectangle */
typedef struct {
    int32_t x, y;
    int32_t width, height;
} rect_t;

/* Window structure */
typedef struct compositor_window {
    uint32_t id;
    uint32_t owner;             /* Client PID */
    
    /* Geometry */
    int32_t x, y;
    int32_t width, height;
    int32_t min_width, min_height;
    int32_t max_width, max_height;
    
    /* Saved geometry (for maximize/restore) */
    rect_t saved_geometry;
    
    /* Canvas */
    uint32_t *buffer;           /* Client-rendered content */
    uint32_t *shadow_buffer;    /* Pre-rendered shadow */
    size_t buffer_size;
    
    /* State */
    uint32_t flags;
    int32_t z_order;
    char title[128];
    
    /* Animation */
    float opacity;
    float scale;
    
    /* Linked list */
    struct compositor_window *next;
    struct compositor_window *prev;
} compositor_window_t;

/* Mouse state */
typedef struct {
    int32_t x, y;
    int32_t last_x, last_y;
    uint8_t buttons;
    uint8_t last_buttons;
    
    /* Cursor */
    uint32_t *cursor_sprite;
    int32_t cursor_hotspot_x;
    int32_t cursor_hotspot_y;
    int32_t cursor_width;
    int32_t cursor_height;
    
    /* Drag state */
    compositor_window_t *drag_window;
    int32_t drag_offset_x;
    int32_t drag_offset_y;
    int drag_mode;  /* 0=none, 1=move, 2=resize */
} mouse_state_t;

/* Compositor state */
typedef struct {
    /* Framebuffer */
    uint32_t *framebuffer;
    uint32_t *backbuffer;
    uint32_t width;
    uint32_t height;
    uint32_t pitch;
    
    /* Windows */
    compositor_window_t *windows;
    compositor_window_t *focused;
    compositor_window_t *top_window;
    uint32_t window_count;
    uint32_t next_window_id;
    
    /* Mouse */
    mouse_state_t mouse;
    
    /* Desktop */
    uint32_t *wallpaper;
    uint32_t desktop_color;
    
    /* Dirty regions */
    rect_t dirty_rects[64];
    int dirty_count;
    int needs_redraw;
    
    /* Statistics */
    uint64_t frames_rendered;
    uint64_t last_frame_time;
} compositor_t;

static compositor_t compositor;

/* ============================================================
 * Utility Functions
 * ============================================================ */

static inline uint32_t blend_pixel(uint32_t bg, uint32_t fg) {
    uint8_t fa = (fg >> 24) & 0xFF;
    if (fa == 255) return fg;
    if (fa == 0) return bg;
    
    uint8_t fr = (fg >> 16) & 0xFF;
    uint8_t fg_g = (fg >> 8) & 0xFF;
    uint8_t fb = fg & 0xFF;
    
    uint8_t br = (bg >> 16) & 0xFF;
    uint8_t bg_g = (bg >> 8) & 0xFF;
    uint8_t bb = bg & 0xFF;
    
    uint8_t r = (fr * fa + br * (255 - fa)) / 255;
    uint8_t g = (fg_g * fa + bg_g * (255 - fa)) / 255;
    uint8_t b = (fb * fa + bb * (255 - fa)) / 255;
    
    return 0xFF000000 | (r << 16) | (g << 8) | b;
}

static inline int rect_intersects(rect_t *a, rect_t *b) {
    return !(a->x + a->width <= b->x || b->x + b->width <= a->x ||
             a->y + a->height <= b->y || b->y + b->height <= a->y);
}

static inline int point_in_rect(int32_t px, int32_t py, rect_t *r) {
    return px >= r->x && px < r->x + r->width &&
           py >= r->y && py < r->y + r->height;
}

/* ============================================================
 * Window Management
 * ============================================================ */

compositor_window_t *compositor_create_window(uint32_t owner, int32_t x, int32_t y,
                                               int32_t width, int32_t height,
                                               uint32_t flags) {
    if (compositor.window_count >= COMPOSITOR_MAX_WINDOWS) {
        return NULL;
    }
    
    compositor_window_t *win = (compositor_window_t*)kmalloc(sizeof(compositor_window_t));
    if (!win) return NULL;
    
    kmemset(win, 0, sizeof(compositor_window_t));
    
    win->id = compositor.next_window_id++;
    win->owner = owner;
    win->x = x;
    win->y = y;
    win->width = width;
    win->height = height;
    win->flags = flags | WINDOW_FLAG_DECORATED | WINDOW_FLAG_MOVABLE | WINDOW_FLAG_RESIZABLE;
    win->opacity = 1.0f;
    win->scale = 1.0f;
    win->min_width = 100;
    win->min_height = 50;
    win->max_width = compositor.width;
    win->max_height = compositor.height;
    
    /* Allocate buffer */
    win->buffer_size = width * height * sizeof(uint32_t);
    win->buffer = (uint32_t*)kmalloc(win->buffer_size);
    if (win->buffer) {
        kmemset(win->buffer, 0xFF, win->buffer_size);  /* White background */
    }
    
    /* Add to window list */
    win->z_order = compositor.window_count;
    win->next = compositor.windows;
    if (compositor.windows) {
        compositor.windows->prev = win;
    }
    compositor.windows = win;
    compositor.window_count++;
    
    /* Focus new window */
    compositor.focused = win;
    win->flags |= WINDOW_FLAG_FOCUSED;
    
    compositor.needs_redraw = 1;
    
    return win;
}

void compositor_destroy_window(compositor_window_t *win) {
    if (!win) return;
    
    /* Remove from list */
    if (win->prev) win->prev->next = win->next;
    if (win->next) win->next->prev = win->prev;
    if (compositor.windows == win) compositor.windows = win->next;
    
    /* Free buffers */
    if (win->buffer) kfree(win->buffer);
    if (win->shadow_buffer) kfree(win->shadow_buffer);
    
    /* Update focus */
    if (compositor.focused == win) {
        compositor.focused = compositor.windows;
        if (compositor.focused) {
            compositor.focused->flags |= WINDOW_FLAG_FOCUSED;
        }
    }
    
    kfree(win);
    compositor.window_count--;
    compositor.needs_redraw = 1;
}

void compositor_focus_window(compositor_window_t *win) {
    if (!win || win == compositor.focused) return;
    
    /* Remove focus from current */
    if (compositor.focused) {
        compositor.focused->flags &= ~WINDOW_FLAG_FOCUSED;
    }
    
    /* Set new focus */
    compositor.focused = win;
    win->flags |= WINDOW_FLAG_FOCUSED;
    
    /* Bring to front */
    if (win != compositor.windows) {
        /* Remove from current position */
        if (win->prev) win->prev->next = win->next;
        if (win->next) win->next->prev = win->prev;
        
        /* Insert at front */
        win->prev = NULL;
        win->next = compositor.windows;
        if (compositor.windows) compositor.windows->prev = win;
        compositor.windows = win;
    }
    
    compositor.needs_redraw = 1;
}

/* ============================================================
 * Rendering
 * ============================================================ */

static void render_shadow(compositor_window_t *win) {
    if (win->flags & WINDOW_FLAG_NO_SHADOW) return;
    
    int32_t sx = win->x - SHADOW_SIZE;
    int32_t sy = win->y - SHADOW_SIZE;
    int32_t sw = win->width + SHADOW_SIZE * 2;
    int32_t sh = win->height + SHADOW_SIZE * 2;
    
    /* Simple shadow - darker at edges */
    for (int32_t y = 0; y < sh; y++) {
        for (int32_t x = 0; x < sw; x++) {
            int32_t px = sx + x;
            int32_t py = sy + y;
            
            if (px < 0 || px >= (int32_t)compositor.width ||
                py < 0 || py >= (int32_t)compositor.height) continue;
            
            /* Skip window area */
            if (x >= SHADOW_SIZE && x < sw - SHADOW_SIZE &&
                y >= SHADOW_SIZE && y < sh - SHADOW_SIZE) continue;
            
            /* Calculate shadow intensity */
            int dx = 0, dy = 0;
            if (x < SHADOW_SIZE) dx = SHADOW_SIZE - x;
            else if (x >= sw - SHADOW_SIZE) dx = x - (sw - SHADOW_SIZE - 1);
            if (y < SHADOW_SIZE) dy = SHADOW_SIZE - y;
            else if (y >= sh - SHADOW_SIZE) dy = y - (sh - SHADOW_SIZE - 1);
            
            int dist = (dx > dy) ? dx : dy;
            uint8_t alpha = (SHADOW_SIZE - dist) * 20;
            
            uint32_t shadow = (alpha << 24);
            uint32_t *pixel = &compositor.backbuffer[py * compositor.width + px];
            *pixel = blend_pixel(*pixel, shadow);
        }
    }
}

static void render_title_bar(compositor_window_t *win) {
    if (!(win->flags & WINDOW_FLAG_DECORATED)) return;
    
    int32_t tx = win->x;
    int32_t ty = win->y - TITLE_BAR_HEIGHT;
    int32_t tw = win->width;
    
    uint32_t title_color = (win->flags & WINDOW_FLAG_FOCUSED) ? 
                           COLOR_TITLE_ACTIVE : COLOR_TITLE_INACTIVE;
    
    /* Draw title bar background */
    for (int32_t y = 0; y < TITLE_BAR_HEIGHT; y++) {
        for (int32_t x = 0; x < tw; x++) {
            int32_t px = tx + x;
            int32_t py = ty + y;
            if (px >= 0 && px < (int32_t)compositor.width &&
                py >= 0 && py < (int32_t)compositor.height) {
                compositor.backbuffer[py * compositor.width + px] = title_color;
            }
        }
    }
    
    /* Draw window buttons (right side) */
    int32_t bx = tx + tw - BUTTON_SIZE * 3 - 6;
    int32_t by = ty + (TITLE_BAR_HEIGHT - BUTTON_SIZE) / 2;
    
    /* Minimize button (yellow) */
    for (int32_t y = 0; y < BUTTON_SIZE; y++) {
        for (int32_t x = 0; x < BUTTON_SIZE; x++) {
            int32_t px = bx + x;
            int32_t py = by + y;
            if (px >= 0 && px < (int32_t)compositor.width &&
                py >= 0 && py < (int32_t)compositor.height) {
                compositor.backbuffer[py * compositor.width + px] = COLOR_BUTTON_MINIMIZE;
            }
        }
    }
    
    /* Maximize button (green) */
    bx += BUTTON_SIZE + 2;
    for (int32_t y = 0; y < BUTTON_SIZE; y++) {
        for (int32_t x = 0; x < BUTTON_SIZE; x++) {
            int32_t px = bx + x;
            int32_t py = by + y;
            if (px >= 0 && px < (int32_t)compositor.width &&
                py >= 0 && py < (int32_t)compositor.height) {
                compositor.backbuffer[py * compositor.width + px] = COLOR_BUTTON_MAXIMIZE;
            }
        }
    }
    
    /* Close button (red) */
    bx += BUTTON_SIZE + 2;
    for (int32_t y = 0; y < BUTTON_SIZE; y++) {
        for (int32_t x = 0; x < BUTTON_SIZE; x++) {
            int32_t px = bx + x;
            int32_t py = by + y;
            if (px >= 0 && px < (int32_t)compositor.width &&
                py >= 0 && py < (int32_t)compositor.height) {
                compositor.backbuffer[py * compositor.width + px] = COLOR_BUTTON_CLOSE;
            }
        }
    }
}

static void render_window_content(compositor_window_t *win) {
    if (!win->buffer) return;
    
    for (int32_t y = 0; y < win->height; y++) {
        for (int32_t x = 0; x < win->width; x++) {
            int32_t px = win->x + x;
            int32_t py = win->y + y;
            
            if (px >= 0 && px < (int32_t)compositor.width &&
                py >= 0 && py < (int32_t)compositor.height) {
                uint32_t src = win->buffer[y * win->width + x];
                compositor.backbuffer[py * compositor.width + px] = 
                    blend_pixel(compositor.backbuffer[py * compositor.width + px], src);
            }
        }
    }
}

static void render_window(compositor_window_t *win) {
    if (win->flags & WINDOW_FLAG_MINIMIZED) return;
    
    render_shadow(win);
    render_title_bar(win);
    render_window_content(win);
}

static void render_cursor(void) {
    int32_t cx = compositor.mouse.x;
    int32_t cy = compositor.mouse.y;
    
    /* Simple arrow cursor */
    uint32_t cursor_color = 0xFFFFFFFF;
    uint32_t outline_color = 0xFF000000;
    
    /* Draw cursor (simple triangle) */
    for (int i = 0; i < 12; i++) {
        for (int j = 0; j <= i && j < 8; j++) {
            int32_t px = cx + j;
            int32_t py = cy + i;
            if (px >= 0 && px < (int32_t)compositor.width &&
                py >= 0 && py < (int32_t)compositor.height) {
                uint32_t color = (j == 0 || j == i || i == 11) ? outline_color : cursor_color;
                compositor.backbuffer[py * compositor.width + px] = color;
            }
        }
    }
}

void compositor_render(void) {
    /* Clear to desktop color or wallpaper */
    if (compositor.wallpaper) {
        kmemcpy(compositor.backbuffer, compositor.wallpaper, 
                compositor.width * compositor.height * 4);
    } else {
        for (uint32_t i = 0; i < compositor.width * compositor.height; i++) {
            compositor.backbuffer[i] = compositor.desktop_color;
        }
    }
    
    /* Render windows back to front */
    compositor_window_t *win = compositor.windows;
    /* Find last window */
    while (win && win->next) win = win->next;
    /* Render from back to front */
    while (win) {
        render_window(win);
        win = win->prev;
    }
    
    /* Render cursor on top */
    render_cursor();
    
    /* Copy to framebuffer */
    kmemcpy(compositor.framebuffer, compositor.backbuffer,
            compositor.width * compositor.height * 4);
    
    compositor.frames_rendered++;
    compositor.needs_redraw = 0;
}

/* ============================================================
 * Input Handling
 * ============================================================ */

void compositor_mouse_move(int32_t dx, int32_t dy) {
    compositor.mouse.last_x = compositor.mouse.x;
    compositor.mouse.last_y = compositor.mouse.y;
    
    compositor.mouse.x += dx;
    compositor.mouse.y += dy;
    
    /* Clamp to screen */
    if (compositor.mouse.x < 0) compositor.mouse.x = 0;
    if (compositor.mouse.y < 0) compositor.mouse.y = 0;
    if (compositor.mouse.x >= (int32_t)compositor.width) 
        compositor.mouse.x = compositor.width - 1;
    if (compositor.mouse.y >= (int32_t)compositor.height) 
        compositor.mouse.y = compositor.height - 1;
    
    /* Handle drag */
    if (compositor.mouse.drag_window && compositor.mouse.drag_mode == 1) {
        compositor_window_t *win = compositor.mouse.drag_window;
        win->x = compositor.mouse.x - compositor.mouse.drag_offset_x;
        win->y = compositor.mouse.y - compositor.mouse.drag_offset_y;
    }
    
    compositor.needs_redraw = 1;
}

void compositor_mouse_button(uint8_t buttons) {
    compositor.mouse.last_buttons = compositor.mouse.buttons;
    compositor.mouse.buttons = buttons;
    
    /* Left button pressed */
    if ((buttons & 1) && !(compositor.mouse.last_buttons & 1)) {
        /* Find window under cursor */
        compositor_window_t *win = compositor.windows;
        while (win) {
            rect_t wr = { win->x, win->y - TITLE_BAR_HEIGHT, 
                         win->width, win->height + TITLE_BAR_HEIGHT };
            if (point_in_rect(compositor.mouse.x, compositor.mouse.y, &wr)) {
                compositor_focus_window(win);
                
                /* Check if in title bar */
                if (compositor.mouse.y < win->y) {
                    compositor.mouse.drag_window = win;
                    compositor.mouse.drag_mode = 1;
                    compositor.mouse.drag_offset_x = compositor.mouse.x - win->x;
                    compositor.mouse.drag_offset_y = compositor.mouse.y - win->y;
                }
                break;
            }
            win = win->next;
        }
    }
    
    /* Left button released */
    if (!(buttons & 1) && (compositor.mouse.last_buttons & 1)) {
        compositor.mouse.drag_window = NULL;
        compositor.mouse.drag_mode = 0;
    }
    
    compositor.needs_redraw = 1;
}

/* ============================================================
 * Initialization
 * ============================================================ */

int compositor_init(uint32_t *framebuffer, uint32_t width, uint32_t height) {
    kmemset(&compositor, 0, sizeof(compositor));
    
    compositor.framebuffer = framebuffer;
    compositor.width = width;
    compositor.height = height;
    compositor.pitch = width * 4;
    compositor.desktop_color = COLOR_DESKTOP;
    compositor.next_window_id = 1;
    
    /* Allocate backbuffer */
    compositor.backbuffer = (uint32_t*)kmalloc(width * height * 4);
    if (!compositor.backbuffer) {
        kprintf("[COMPOSITOR] Failed to allocate backbuffer\n");
        return -1;
    }
    
    /* Initialize mouse */
    compositor.mouse.x = width / 2;
    compositor.mouse.y = height / 2;
    
    kprintf("[COMPOSITOR] Initialized %dx%d\n", width, height);
    kprintf("[COMPOSITOR] ToaruOS-inspired Yutani compositor\n");
    
    return 0;
}
