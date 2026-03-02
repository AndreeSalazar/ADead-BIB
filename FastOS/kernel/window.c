/*
 * FastOS v2.0 — Window Manager
 * Hybrid Windows + Linux style windowing system
 * 
 * Inspired by:
 * - Windows: Title bar, minimize/maximize/close buttons, taskbar
 * - Linux/X11: Compositing, workspaces, tiling options
 * 
 * Compile: adB cc window.c -o window.po --po
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ============================================================
 * Window System Constants
 * ============================================================ */

#define MAX_WINDOWS         64
#define TITLEBAR_HEIGHT     24
#define BORDER_WIDTH        2
#define TASKBAR_HEIGHT      40
#define MIN_WINDOW_WIDTH    100
#define MIN_WINDOW_HEIGHT   50

/* Window flags */
#define WF_VISIBLE      0x0001
#define WF_FOCUSED      0x0002
#define WF_MINIMIZED    0x0004
#define WF_MAXIMIZED    0x0008
#define WF_RESIZABLE    0x0010
#define WF_MOVABLE      0x0020
#define WF_CLOSABLE     0x0040
#define WF_BORDER       0x0080
#define WF_TITLEBAR     0x0100
#define WF_SHADOW       0x0200

/* Default window style */
#define WS_DEFAULT      (WF_VISIBLE | WF_RESIZABLE | WF_MOVABLE | WF_CLOSABLE | WF_BORDER | WF_TITLEBAR)

/* Window colors (Windows 10/11 style) */
#define COLOR_TITLEBAR_ACTIVE   0xFF0078D7  /* Windows blue */
#define COLOR_TITLEBAR_INACTIVE 0xFF2D2D30  /* Dark gray */
#define COLOR_WINDOW_BG         0xFFFFFFFF  /* White */
#define COLOR_WINDOW_BORDER     0xFF404040  /* Dark border */
#define COLOR_CLOSE_BTN         0xFFE81123  /* Red */
#define COLOR_CLOSE_BTN_HOVER   0xFFC42B1C
#define COLOR_MIN_BTN_HOVER     0xFF3D3D3D
#define COLOR_MAX_BTN_HOVER     0xFF3D3D3D

/* ============================================================
 * Window Structure
 * ============================================================ */

typedef struct window {
    uint32_t id;
    char title[64];
    int32_t x, y;
    int32_t width, height;
    int32_t min_width, min_height;
    uint32_t flags;
    uint32_t *framebuffer;      /* Window's own framebuffer */
    uint32_t fb_size;
    
    /* Saved position for maximize/restore */
    int32_t saved_x, saved_y;
    int32_t saved_width, saved_height;
    
    /* Window class/type */
    uint32_t class_id;
    
    /* Callbacks */
    void (*on_paint)(struct window *win);
    void (*on_click)(struct window *win, int32_t x, int32_t y, int button);
    void (*on_key)(struct window *win, char key);
    void (*on_close)(struct window *win);
    void (*on_resize)(struct window *win, int32_t new_w, int32_t new_h);
    
    /* User data */
    void *user_data;
    
    /* Z-order (higher = on top) */
    int32_t z_order;
    
    /* Parent/child relationships */
    struct window *parent;
    struct window *children[16];
    int child_count;
} window_t;

/* ============================================================
 * Window Manager State
 * ============================================================ */

static window_t windows[MAX_WINDOWS];
static int window_count = 0;
static window_t *focused_window = NULL;
static window_t *dragging_window = NULL;
static int drag_offset_x = 0, drag_offset_y = 0;
static int resizing = 0;

/* Screen framebuffer reference */
static framebuffer_t *screen_fb = NULL;

/* ============================================================
 * Window Manager Initialization
 * ============================================================ */

void wm_init(framebuffer_t *fb) {
    screen_fb = fb;
    window_count = 0;
    focused_window = NULL;
    dragging_window = NULL;
    
    for (int i = 0; i < MAX_WINDOWS; i++) {
        windows[i].id = 0;
        windows[i].flags = 0;
    }
    
    kprintf("[WM] Window manager initialized\n");
}

/* ============================================================
 * Window Creation/Destruction
 * ============================================================ */

window_t *window_create(const char *title, int32_t x, int32_t y, 
                        int32_t width, int32_t height, uint32_t flags) {
    if (window_count >= MAX_WINDOWS) {
        return NULL;
    }
    
    /* Find free slot */
    window_t *win = NULL;
    for (int i = 0; i < MAX_WINDOWS; i++) {
        if (windows[i].id == 0) {
            win = &windows[i];
            win->id = i + 1;
            break;
        }
    }
    
    if (!win) return NULL;
    
    /* Initialize window */
    kstrncpy(win->title, title, 63);
    win->title[63] = '\0';
    win->x = x;
    win->y = y;
    win->width = (width < MIN_WINDOW_WIDTH) ? MIN_WINDOW_WIDTH : width;
    win->height = (height < MIN_WINDOW_HEIGHT) ? MIN_WINDOW_HEIGHT : height;
    win->min_width = MIN_WINDOW_WIDTH;
    win->min_height = MIN_WINDOW_HEIGHT;
    win->flags = flags ? flags : WS_DEFAULT;
    
    /* Allocate window framebuffer */
    win->fb_size = win->width * win->height * 4;
    win->framebuffer = (uint32_t*)kmalloc(win->fb_size);
    if (win->framebuffer) {
        kmemset(win->framebuffer, 0xFF, win->fb_size);  /* White background */
    }
    
    /* Save position for restore */
    win->saved_x = x;
    win->saved_y = y;
    win->saved_width = width;
    win->saved_height = height;
    
    /* Set z-order */
    win->z_order = window_count;
    
    /* Clear callbacks */
    win->on_paint = NULL;
    win->on_click = NULL;
    win->on_key = NULL;
    win->on_close = NULL;
    win->on_resize = NULL;
    win->user_data = NULL;
    
    /* No parent/children initially */
    win->parent = NULL;
    win->child_count = 0;
    
    window_count++;
    
    /* Focus new window */
    window_focus(win);
    
    kprintf("[WM] Created window '%s' (%dx%d)\n", title, width, height);
    
    return win;
}

void window_destroy(window_t *win) {
    if (!win || win->id == 0) return;
    
    /* Call close callback */
    if (win->on_close) {
        win->on_close(win);
    }
    
    /* Free framebuffer */
    if (win->framebuffer) {
        kfree(win->framebuffer);
    }
    
    /* Clear focus if this was focused */
    if (focused_window == win) {
        focused_window = NULL;
    }
    
    /* Mark as free */
    win->id = 0;
    win->flags = 0;
    window_count--;
    
    kprintf("[WM] Destroyed window\n");
}

/* ============================================================
 * Window Focus/Z-Order
 * ============================================================ */

void window_focus(window_t *win) {
    if (!win) return;
    
    /* Remove focus from previous */
    if (focused_window && focused_window != win) {
        focused_window->flags &= ~WF_FOCUSED;
    }
    
    /* Set focus */
    win->flags |= WF_FOCUSED;
    focused_window = win;
    
    /* Bring to front (highest z-order) */
    int max_z = 0;
    for (int i = 0; i < MAX_WINDOWS; i++) {
        if (windows[i].id != 0 && windows[i].z_order > max_z) {
            max_z = windows[i].z_order;
        }
    }
    win->z_order = max_z + 1;
}

window_t *window_get_focused(void) {
    return focused_window;
}

/* ============================================================
 * Window Operations
 * ============================================================ */

void window_move(window_t *win, int32_t x, int32_t y) {
    if (!win || !(win->flags & WF_MOVABLE)) return;
    win->x = x;
    win->y = y;
}

void window_resize(window_t *win, int32_t width, int32_t height) {
    if (!win || !(win->flags & WF_RESIZABLE)) return;
    
    if (width < win->min_width) width = win->min_width;
    if (height < win->min_height) height = win->min_height;
    
    /* Reallocate framebuffer if size changed */
    if (width != win->width || height != win->height) {
        win->width = width;
        win->height = height;
        
        if (win->framebuffer) {
            kfree(win->framebuffer);
        }
        win->fb_size = width * height * 4;
        win->framebuffer = (uint32_t*)kmalloc(win->fb_size);
        if (win->framebuffer) {
            kmemset(win->framebuffer, 0xFF, win->fb_size);
        }
        
        /* Notify resize callback */
        if (win->on_resize) {
            win->on_resize(win, width, height);
        }
    }
}

void window_minimize(window_t *win) {
    if (!win) return;
    win->flags |= WF_MINIMIZED;
    win->flags &= ~WF_VISIBLE;
}

void window_maximize(window_t *win) {
    if (!win || !screen_fb) return;
    
    if (win->flags & WF_MAXIMIZED) {
        /* Restore */
        win->x = win->saved_x;
        win->y = win->saved_y;
        window_resize(win, win->saved_width, win->saved_height);
        win->flags &= ~WF_MAXIMIZED;
    } else {
        /* Save current position */
        win->saved_x = win->x;
        win->saved_y = win->y;
        win->saved_width = win->width;
        win->saved_height = win->height;
        
        /* Maximize */
        win->x = 0;
        win->y = 0;
        window_resize(win, screen_fb->width, screen_fb->height - TASKBAR_HEIGHT);
        win->flags |= WF_MAXIMIZED;
    }
}

void window_restore(window_t *win) {
    if (!win) return;
    win->flags &= ~WF_MINIMIZED;
    win->flags |= WF_VISIBLE;
}

void window_show(window_t *win) {
    if (!win) return;
    win->flags |= WF_VISIBLE;
}

void window_hide(window_t *win) {
    if (!win) return;
    win->flags &= ~WF_VISIBLE;
}

/* ============================================================
 * Window Drawing
 * ============================================================ */

static void draw_titlebar(window_t *win) {
    if (!screen_fb || !(win->flags & WF_TITLEBAR)) return;
    
    uint32_t color = (win->flags & WF_FOCUSED) ? 
                     COLOR_TITLEBAR_ACTIVE : COLOR_TITLEBAR_INACTIVE;
    
    /* Draw titlebar background */
    for (int y = win->y; y < win->y + TITLEBAR_HEIGHT && y < (int)screen_fb->height; y++) {
        for (int x = win->x; x < win->x + win->width && x < (int)screen_fb->width; x++) {
            if (x >= 0 && y >= 0) {
                screen_fb->buffer[y * (screen_fb->pitch / 4) + x] = color;
            }
        }
    }
    
    /* Draw title text (simplified - would use font) */
    /* Title would be drawn here with fb_string() */
    
    /* Draw window buttons (close, maximize, minimize) */
    int btn_size = 16;
    int btn_y = win->y + (TITLEBAR_HEIGHT - btn_size) / 2;
    
    /* Close button (red X) */
    int close_x = win->x + win->width - btn_size - 8;
    for (int y = btn_y; y < btn_y + btn_size; y++) {
        for (int x = close_x; x < close_x + btn_size; x++) {
            if (x >= 0 && y >= 0 && x < (int)screen_fb->width && y < (int)screen_fb->height) {
                screen_fb->buffer[y * (screen_fb->pitch / 4) + x] = COLOR_CLOSE_BTN;
            }
        }
    }
    
    /* Maximize button */
    int max_x = close_x - btn_size - 4;
    for (int y = btn_y; y < btn_y + btn_size; y++) {
        for (int x = max_x; x < max_x + btn_size; x++) {
            if (x >= 0 && y >= 0 && x < (int)screen_fb->width && y < (int)screen_fb->height) {
                screen_fb->buffer[y * (screen_fb->pitch / 4) + x] = COLOR_TITLEBAR_INACTIVE;
            }
        }
    }
    
    /* Minimize button */
    int min_x = max_x - btn_size - 4;
    for (int y = btn_y; y < btn_y + btn_size; y++) {
        for (int x = min_x; x < min_x + btn_size; x++) {
            if (x >= 0 && y >= 0 && x < (int)screen_fb->width && y < (int)screen_fb->height) {
                screen_fb->buffer[y * (screen_fb->pitch / 4) + x] = COLOR_TITLEBAR_INACTIVE;
            }
        }
    }
}

static void draw_border(window_t *win) {
    if (!screen_fb || !(win->flags & WF_BORDER)) return;
    
    uint32_t color = COLOR_WINDOW_BORDER;
    
    /* Top border */
    for (int x = win->x; x < win->x + win->width; x++) {
        if (x >= 0 && x < (int)screen_fb->width && win->y >= 0) {
            screen_fb->buffer[win->y * (screen_fb->pitch / 4) + x] = color;
        }
    }
    
    /* Bottom border */
    int bottom = win->y + win->height - 1;
    for (int x = win->x; x < win->x + win->width; x++) {
        if (x >= 0 && x < (int)screen_fb->width && bottom < (int)screen_fb->height) {
            screen_fb->buffer[bottom * (screen_fb->pitch / 4) + x] = color;
        }
    }
    
    /* Left border */
    for (int y = win->y; y < win->y + win->height; y++) {
        if (win->x >= 0 && y >= 0 && y < (int)screen_fb->height) {
            screen_fb->buffer[y * (screen_fb->pitch / 4) + win->x] = color;
        }
    }
    
    /* Right border */
    int right = win->x + win->width - 1;
    for (int y = win->y; y < win->y + win->height; y++) {
        if (right < (int)screen_fb->width && y >= 0 && y < (int)screen_fb->height) {
            screen_fb->buffer[y * (screen_fb->pitch / 4) + right] = color;
        }
    }
}

void window_draw(window_t *win) {
    if (!win || !screen_fb || !(win->flags & WF_VISIBLE)) return;
    if (win->flags & WF_MINIMIZED) return;
    
    int content_y = win->y;
    if (win->flags & WF_TITLEBAR) {
        content_y += TITLEBAR_HEIGHT;
    }
    
    /* Draw window content (white background) */
    for (int y = content_y; y < win->y + win->height && y < (int)screen_fb->height; y++) {
        for (int x = win->x; x < win->x + win->width && x < (int)screen_fb->width; x++) {
            if (x >= 0 && y >= 0) {
                screen_fb->buffer[y * (screen_fb->pitch / 4) + x] = COLOR_WINDOW_BG;
            }
        }
    }
    
    /* Copy window's framebuffer to screen */
    if (win->framebuffer) {
        int content_height = win->height - (win->flags & WF_TITLEBAR ? TITLEBAR_HEIGHT : 0);
        for (int y = 0; y < content_height && (content_y + y) < (int)screen_fb->height; y++) {
            for (int x = 0; x < win->width && (win->x + x) < (int)screen_fb->width; x++) {
                if (win->x + x >= 0 && content_y + y >= 0) {
                    uint32_t pixel = win->framebuffer[y * win->width + x];
                    screen_fb->buffer[(content_y + y) * (screen_fb->pitch / 4) + (win->x + x)] = pixel;
                }
            }
        }
    }
    
    /* Draw titlebar */
    draw_titlebar(win);
    
    /* Draw border */
    draw_border(win);
    
    /* Call paint callback */
    if (win->on_paint) {
        win->on_paint(win);
    }
}

/* ============================================================
 * Window Manager Drawing (all windows)
 * ============================================================ */

void wm_draw_all(void) {
    if (!screen_fb) return;
    
    /* Sort windows by z-order and draw */
    for (int z = 0; z <= window_count + 10; z++) {
        for (int i = 0; i < MAX_WINDOWS; i++) {
            if (windows[i].id != 0 && windows[i].z_order == z) {
                window_draw(&windows[i]);
            }
        }
    }
}

/* ============================================================
 * Window Hit Testing
 * ============================================================ */

window_t *wm_window_at(int32_t x, int32_t y) {
    window_t *result = NULL;
    int highest_z = -1;
    
    for (int i = 0; i < MAX_WINDOWS; i++) {
        window_t *win = &windows[i];
        if (win->id == 0 || !(win->flags & WF_VISIBLE)) continue;
        if (win->flags & WF_MINIMIZED) continue;
        
        if (x >= win->x && x < win->x + win->width &&
            y >= win->y && y < win->y + win->height) {
            if (win->z_order > highest_z) {
                highest_z = win->z_order;
                result = win;
            }
        }
    }
    
    return result;
}

/* Check if point is on titlebar */
int wm_on_titlebar(window_t *win, int32_t x, int32_t y) {
    if (!win || !(win->flags & WF_TITLEBAR)) return 0;
    
    return (x >= win->x && x < win->x + win->width &&
            y >= win->y && y < win->y + TITLEBAR_HEIGHT);
}

/* Check if point is on close button */
int wm_on_close_button(window_t *win, int32_t x, int32_t y) {
    if (!win || !(win->flags & WF_CLOSABLE)) return 0;
    
    int btn_size = 16;
    int btn_y = win->y + (TITLEBAR_HEIGHT - btn_size) / 2;
    int close_x = win->x + win->width - btn_size - 8;
    
    return (x >= close_x && x < close_x + btn_size &&
            y >= btn_y && y < btn_y + btn_size);
}

/* ============================================================
 * Window Manager Input Handling
 * ============================================================ */

void wm_handle_mouse(int32_t x, int32_t y, int buttons, int prev_buttons) {
    int left_pressed = (buttons & 1) && !(prev_buttons & 1);
    int left_released = !(buttons & 1) && (prev_buttons & 1);
    int left_held = buttons & 1;
    
    /* Handle window dragging */
    if (dragging_window && left_held) {
        window_move(dragging_window, x - drag_offset_x, y - drag_offset_y);
        return;
    }
    
    if (left_released && dragging_window) {
        dragging_window = NULL;
    }
    
    /* Find window under cursor */
    window_t *win = wm_window_at(x, y);
    
    if (left_pressed && win) {
        /* Focus window */
        window_focus(win);
        
        /* Check for button clicks */
        if (wm_on_close_button(win, x, y)) {
            window_destroy(win);
            return;
        }
        
        /* Start dragging if on titlebar */
        if (wm_on_titlebar(win, x, y)) {
            dragging_window = win;
            drag_offset_x = x - win->x;
            drag_offset_y = y - win->y;
            return;
        }
        
        /* Pass click to window */
        if (win->on_click) {
            int local_x = x - win->x;
            int local_y = y - win->y - (win->flags & WF_TITLEBAR ? TITLEBAR_HEIGHT : 0);
            win->on_click(win, local_x, local_y, 0);
        }
    }
}

void wm_handle_key(char key) {
    if (focused_window && focused_window->on_key) {
        focused_window->on_key(focused_window, key);
    }
}

/* ============================================================
 * Window Enumeration
 * ============================================================ */

int wm_get_window_count(void) {
    return window_count;
}

window_t *wm_get_window(int index) {
    int count = 0;
    for (int i = 0; i < MAX_WINDOWS; i++) {
        if (windows[i].id != 0) {
            if (count == index) {
                return &windows[i];
            }
            count++;
        }
    }
    return NULL;
}
