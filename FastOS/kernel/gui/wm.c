/* FastOS v4.0 — Window Manager (PoWindow Native)
 * ADead-BIB Native OS — GUI Desktop con ventanas reales
 *
 * Manages PoWindow instances on framebuffer surface.
 * No X11, no Wayland, no GDI — direct framebuffer compositing.
 * Connected to real VESA VBE framebuffer via fb.c.
 *
 * Architecture:
 *   - 16-window pool, content surfaces at 0x800000 (2MB each)
 *   - Z-order: integer sort, focus brings to front
 *   - Decorations: titlebar 24px, border 1px, close [X]
 *   - Drag: titlebar click+hold → move (clamped to screen)
 *   - Content: apps draw into per-window fb_surface_t
 *
 * Lifecycle: wm_create() → wm_compose() → wm_handle_mouse() → wm_destroy()
 * desktop.c calls wm_init() + wm_compose() every frame.
 *
 * Compiled by: ADead-BIB (C is Master, Rust is Safety)
 */

#include "../include/types.h"

/* ================================================================
 * Window Constants
 * ================================================================ */

#define WM_MAX_WINDOWS    16    /* max simultaneous windows */
#define WM_TITLEBAR_H     24    /* titlebar height in pixels */
#define WM_BORDER_W        1    /* border width in pixels */
#define WM_CLOSE_BTN_W    20    /* close button width */
#define WM_CLOSE_BTN_H    20    /* close button height */
#define WM_MIN_W         100    /* minimum window width */
#define WM_MIN_H          60    /* minimum window height */
#define WM_TITLE_MAX      63    /* max title length */

/* Window flags */
#define WINDOW_VISIBLE     0x0001
#define WINDOW_RESIZABLE   0x0002
#define WINDOW_MODAL       0x0004
#define WINDOW_FOCUSED     0x0008
#define WINDOW_DRAGGING    0x0010
#define WINDOW_CLOSABLE    0x0020
#define WINDOW_MINIMIZED   0x0040
#define WINDOW_MAXIMIZED   0x0080
#define WINDOW_DECORATED   0x0100  /* has titlebar + border */
#define WINDOW_SHELL       0x0200  /* is the built-in shell */

/* ================================================================
 * Window Colors (matching fb.c palette)
 * ================================================================ */

#define WM_COLOR_TITLE_BG      0xFF1E3A5F  /* focused titlebar */
#define WM_COLOR_TITLE_BG_BLUR 0xFF2A2A40  /* unfocused titlebar */
#define WM_COLOR_TITLE_FG      0xFFFFFFFF  /* title text */
#define WM_COLOR_BORDER        0xFF555577  /* window border */
#define WM_COLOR_CONTENT_BG    0xFF1A1A2E  /* default content bg */
#define WM_COLOR_CLOSE_BG      0xFFCC3333  /* close button */
#define WM_COLOR_CLOSE_HOVER   0xFFFF4444  /* close button hover */
#define WM_COLOR_CLOSE_FG      0xFFFFFFFF  /* close X text */
#define WM_COLOR_SHADOW        0x40000000  /* window shadow (alpha) */

/* fb_surface_t is defined in fb.c (included inline before this file) */

/* ================================================================
 * PoWindow — Native FastOS Window
 * ================================================================ */

typedef struct {
    uint32_t    id;                    /* unique window ID (1-based, 0=unused) */
    char        title[WM_TITLE_MAX + 1];
    int32_t     x, y;                  /* screen position (top-left of decoration) */
    int32_t     w, h;                  /* total size including decorations */
    int32_t     content_w, content_h;  /* inner content area size */
    uint32_t    flags;                 /* WINDOW_* flags */
    uint32_t    bg_color;              /* content background color */
    int32_t     z_order;               /* higher = more in front */

    /* Content surface — apps draw here
     * Allocated from a fixed memory pool starting at 0x800000 (8MB)
     * Each window gets up to 800×600×4 = ~1.9MB */
    fb_surface_t content;

    /* Drag state */
    int32_t     drag_start_x;
    int32_t     drag_start_y;
    int32_t     drag_offset_x;
    int32_t     drag_offset_y;
} po_window_t;

/* ================================================================
 * Window Manager State
 * ================================================================ */

typedef struct {
    po_window_t windows[WM_MAX_WINDOWS];
    int          count;             /* active window count */
    uint32_t     next_id;           /* next window ID to assign */
    int          focused_idx;       /* index of focused window (-1=none) */
    int          dragging_idx;      /* index of window being dragged (-1=none) */
    int          initialized;
    int32_t      screen_w;          /* framebuffer width */
    int32_t      screen_h;          /* framebuffer height */
} wm_state_t;

static wm_state_t wm;

/* Content surface memory pool: starts at 8MB, each window gets 2MB max */
#define WM_SURFACE_POOL_BASE  0x800000
#define WM_SURFACE_POOL_SLOT  0x200000  /* 2MB per window slot */

/* ================================================================
 * WM Init
 * ================================================================ */

static void wm_init(int32_t screen_w, int32_t screen_h)
{
    int i;

    wm.count = 0;
    wm.next_id = 1;
    wm.focused_idx = -1;
    wm.dragging_idx = -1;
    wm.screen_w = screen_w;
    wm.screen_h = screen_h;
    wm.initialized = 1;

    i = 0;
    while (i < WM_MAX_WINDOWS) {
        wm.windows[i].id = 0;
        wm.windows[i].flags = 0;
        i = i + 1;
    }
}

/* ================================================================
 * String Copy (inline — no libc)
 * ================================================================ */

static void wm_strcpy(char *dst, const char *src, int max)
{
    int i;
    i = 0;
    while (src[i] != 0 && i < max - 1) {
        dst[i] = src[i];
        i = i + 1;
    }
    dst[i] = 0;
}

/* ================================================================
 * Find Free Window Slot
 * ================================================================ */

static int wm_find_free_slot(void)
{
    int i;
    i = 0;
    while (i < WM_MAX_WINDOWS) {
        if (wm.windows[i].id == 0) return i;
        i = i + 1;
    }
    return -1;
}

/* ================================================================
 * Create Window
 *
 * Returns pointer to new window, or NULL if pool full.
 * Content surface is pre-allocated from fixed memory pool.
 * ================================================================ */

static po_window_t *wm_create(const char *title, int32_t x, int32_t y,
                               int32_t w, int32_t h, uint32_t flags)
{
    int slot;
    po_window_t *win;
    uint32_t pool_addr;

    slot = wm_find_free_slot();
    if (slot < 0) return 0;  /* NULL — pool full */

    win = &wm.windows[slot];
    win->id = wm.next_id;
    wm.next_id = wm.next_id + 1;

    wm_strcpy(win->title, title, WM_TITLE_MAX + 1);

    win->x = x;
    win->y = y;
    win->w = w;
    win->h = h;
    win->content_w = w - 2 * WM_BORDER_W;
    win->content_h = h - WM_TITLEBAR_H - WM_BORDER_W;
    if (win->content_w < 1) win->content_w = 1;
    if (win->content_h < 1) win->content_h = 1;
    win->flags = flags | WINDOW_VISIBLE | WINDOW_DECORATED | WINDOW_CLOSABLE;
    win->bg_color = WM_COLOR_CONTENT_BG;
    win->z_order = wm.count;

    win->drag_start_x = 0;
    win->drag_start_y = 0;
    win->drag_offset_x = 0;
    win->drag_offset_y = 0;

    /* Allocate content surface from pool */
    pool_addr = WM_SURFACE_POOL_BASE + (uint32_t)slot * WM_SURFACE_POOL_SLOT;
    win->content.pixels = (uint32_t *)((uintptr_t)pool_addr);
    win->content.width  = (uint32_t)win->content_w;
    win->content.height = (uint32_t)win->content_h;
    win->content.pitch  = (uint32_t)win->content_w * 4;
    win->content.bpp    = 32;
    win->content.size   = win->content.pitch * (uint32_t)win->content_h;

    wm.count = wm.count + 1;

    /* Focus the new window */
    wm.focused_idx = slot;

    return win;
}

/* ================================================================
 * Destroy Window
 * ================================================================ */

static void wm_destroy(po_window_t *win)
{
    if (!win || win->id == 0) return;

    win->id = 0;
    win->flags = 0;
    wm.count = wm.count - 1;
    if (wm.count < 0) wm.count = 0;

    /* If this was focused, clear focus */
    if (&wm.windows[wm.focused_idx] == win) {
        wm.focused_idx = -1;
    }
}

/* ================================================================
 * Find Window by ID
 * ================================================================ */

static po_window_t *wm_find(uint32_t id)
{
    int i;
    i = 0;
    while (i < WM_MAX_WINDOWS) {
        if (wm.windows[i].id == id) return &wm.windows[i];
        i = i + 1;
    }
    return 0;
}

/* ================================================================
 * Focus Window — Bring to front
 * ================================================================ */

static void wm_focus(int slot)
{
    int i;
    int32_t old_z;

    if (slot < 0 || slot >= WM_MAX_WINDOWS) return;
    if (wm.windows[slot].id == 0) return;

    /* Remove FOCUSED flag from current */
    if (wm.focused_idx >= 0 && wm.focused_idx < WM_MAX_WINDOWS) {
        wm.windows[wm.focused_idx].flags &= ~WINDOW_FOCUSED;
    }

    /* Bring clicked window to top z-order */
    old_z = wm.windows[slot].z_order;
    i = 0;
    while (i < WM_MAX_WINDOWS) {
        if (wm.windows[i].id != 0 && wm.windows[i].z_order > old_z) {
            wm.windows[i].z_order = wm.windows[i].z_order - 1;
        }
        i = i + 1;
    }
    wm.windows[slot].z_order = wm.count - 1;
    wm.windows[slot].flags |= WINDOW_FOCUSED;
    wm.focused_idx = slot;
}

/* ================================================================
 * Draw Window Decorations onto target surface
 *
 * Draws: border, titlebar, title text, close button.
 * The content area is drawn by the app (or cleared to bg_color).
 * ================================================================ */

/* fb_fill_rect, fb_rect_outline, fb_blit defined in fb.c (inline) */
/* font_draw_string, font_draw_char defined in font.c (inline) */

static void wm_draw_window(fb_surface_t *target, po_window_t *win)
{
    uint32_t title_bg;
    int32_t title_x, title_y;
    int32_t close_x, close_y;
    int32_t content_x, content_y;

    if (!(win->flags & WINDOW_VISIBLE)) return;
    if (win->flags & WINDOW_MINIMIZED) return;

    /* Border */
    fb_rect_outline(target, (uint32_t)win->x, (uint32_t)win->y,
                    (uint32_t)win->w, (uint32_t)win->h, WM_COLOR_BORDER);

    /* Titlebar background */
    title_bg = (win->flags & WINDOW_FOCUSED) ? WM_COLOR_TITLE_BG : WM_COLOR_TITLE_BG_BLUR;
    fb_fill_rect(target,
                 (uint32_t)(win->x + WM_BORDER_W),
                 (uint32_t)(win->y + WM_BORDER_W),
                 (uint32_t)(win->w - 2 * WM_BORDER_W),
                 (uint32_t)WM_TITLEBAR_H,
                 title_bg);

    /* Title text (left-aligned, vertically centered in titlebar) */
    title_x = win->x + WM_BORDER_W + 8;
    title_y = win->y + WM_BORDER_W + (WM_TITLEBAR_H - 16) / 2;
    font_draw_string(target, (uint32_t)title_x, (uint32_t)title_y,
                     win->title, WM_COLOR_TITLE_FG, 0x00000000);

    /* Close button [X] */
    if (win->flags & WINDOW_CLOSABLE) {
        close_x = win->x + win->w - WM_BORDER_W - WM_CLOSE_BTN_W - 2;
        close_y = win->y + WM_BORDER_W + (WM_TITLEBAR_H - WM_CLOSE_BTN_H) / 2;
        fb_fill_rect(target, (uint32_t)close_x, (uint32_t)close_y,
                     WM_CLOSE_BTN_W, WM_CLOSE_BTN_H, WM_COLOR_CLOSE_BG);
        /* Draw 'X' centered in close button */
        font_draw_char(target,
                       (uint32_t)(close_x + (WM_CLOSE_BTN_W - 8) / 2),
                       (uint32_t)(close_y + (WM_CLOSE_BTN_H - 16) / 2),
                       'X', WM_COLOR_CLOSE_FG, 0x00000000);
    }

    /* Content area background */
    content_x = win->x + WM_BORDER_W;
    content_y = win->y + WM_BORDER_W + WM_TITLEBAR_H;
    fb_fill_rect(target, (uint32_t)content_x, (uint32_t)content_y,
                 (uint32_t)win->content_w, (uint32_t)win->content_h,
                 win->bg_color);

    /* Blit window content surface onto target */
    fb_blit(target, (uint32_t)content_x, (uint32_t)content_y,
            &win->content, 0, 0,
            (uint32_t)win->content_w, (uint32_t)win->content_h);
}

/* ================================================================
 * Compose All Windows (back-to-front by z_order)
 *
 * Called every frame by desktop.c before fb_flip().
 * Draws windows in z-order: lowest z first, highest last (on top).
 * ================================================================ */

static void wm_compose(fb_surface_t *target)
{
    int z, i;

    z = 0;
    while (z < wm.count) {
        /* Find window with z_order == z */
        i = 0;
        while (i < WM_MAX_WINDOWS) {
            if (wm.windows[i].id != 0 && wm.windows[i].z_order == z) {
                wm_draw_window(target, &wm.windows[i]);
            }
            i = i + 1;
        }
        z = z + 1;
    }
}

/* ================================================================
 * Hit Test — Which window (and which part) is under (mx, my)?
 *
 * Returns window slot index, or -1 if no window hit.
 * Sets *hit_titlebar = 1 if click is on titlebar.
 * Sets *hit_close = 1 if click is on close button.
 * Tests front-to-back (highest z_order first).
 * ================================================================ */

static int wm_hit_test(int32_t mx, int32_t my,
                       int *hit_titlebar, int *hit_close)
{
    int z, i;
    po_window_t *win;
    int32_t close_x, close_y;

    *hit_titlebar = 0;
    *hit_close = 0;

    /* Search from top (front) to bottom (back) */
    z = wm.count - 1;
    while (z >= 0) {
        i = 0;
        while (i < WM_MAX_WINDOWS) {
            win = &wm.windows[i];
            if (win->id != 0 && win->z_order == z) {
                if (!(win->flags & WINDOW_VISIBLE)) { i = i + 1; continue; }
                if (win->flags & WINDOW_MINIMIZED) { i = i + 1; continue; }

                /* Check if mouse is inside window bounds */
                if (mx >= win->x && mx < win->x + win->w &&
                    my >= win->y && my < win->y + win->h) {

                    /* Check close button */
                    if (win->flags & WINDOW_CLOSABLE) {
                        close_x = win->x + win->w - WM_BORDER_W - WM_CLOSE_BTN_W - 2;
                        close_y = win->y + WM_BORDER_W + (WM_TITLEBAR_H - WM_CLOSE_BTN_H) / 2;
                        if (mx >= close_x && mx < close_x + WM_CLOSE_BTN_W &&
                            my >= close_y && my < close_y + WM_CLOSE_BTN_H) {
                            *hit_close = 1;
                            return i;
                        }
                    }

                    /* Check titlebar */
                    if (my >= win->y + WM_BORDER_W &&
                        my < win->y + WM_BORDER_W + WM_TITLEBAR_H) {
                        *hit_titlebar = 1;
                        return i;
                    }

                    /* Content area hit */
                    return i;
                }
            }
            i = i + 1;
        }
        z = z - 1;
    }

    return -1;  /* no window hit */
}

/* ================================================================
 * Handle Mouse Events
 *
 * Called from desktop event loop with current mouse state.
 * Handles: focus, drag, close, click dispatch.
 * Returns 1 if event was consumed by a window.
 * ================================================================ */

static int wm_handle_mouse(int32_t mx, int32_t my,
                           int left_down, int left_clicked,
                           int left_released)
{
    int slot;
    int hit_title, hit_close;
    po_window_t *win;

    /* Handle ongoing drag */
    if (wm.dragging_idx >= 0) {
        win = &wm.windows[wm.dragging_idx];
        if (left_down) {
            /* Update position */
            win->x = mx - win->drag_offset_x;
            win->y = my - win->drag_offset_y;

            /* Clamp to screen */
            if (win->x < 0) win->x = 0;
            if (win->y < 0) win->y = 0;
            if (win->x + win->w > wm.screen_w) win->x = wm.screen_w - win->w;
            if (win->y + win->h > wm.screen_h) win->y = wm.screen_h - win->h;
            return 1;
        } else {
            /* Drag ended */
            win->flags &= ~WINDOW_DRAGGING;
            wm.dragging_idx = -1;
            return 1;
        }
    }

    /* New click? */
    if (!left_clicked) return 0;

    slot = wm_hit_test(mx, my, &hit_title, &hit_close);
    if (slot < 0) return 0;

    win = &wm.windows[slot];

    /* Focus this window */
    wm_focus(slot);

    /* Close button clicked */
    if (hit_close) {
        wm_destroy(win);
        return 1;
    }

    /* Titlebar — start drag */
    if (hit_title) {
        win->flags |= WINDOW_DRAGGING;
        win->drag_offset_x = mx - win->x;
        win->drag_offset_y = my - win->y;
        wm.dragging_idx = slot;
        return 1;
    }

    /* Content area click — could dispatch to app later */
    return 1;
}

/* ================================================================
 * Get Content Surface — For apps to draw into
 * ================================================================ */

static fb_surface_t *wm_get_content(po_window_t *win)
{
    if (!win || win->id == 0) return 0;
    return &win->content;
}

/* ================================================================
 * Get Focused Window
 * ================================================================ */

static po_window_t *wm_get_focused(void)
{
    if (wm.focused_idx < 0 || wm.focused_idx >= WM_MAX_WINDOWS) return 0;
    if (wm.windows[wm.focused_idx].id == 0) return 0;
    return &wm.windows[wm.focused_idx];
}

/* ================================================================
 * Get Window Count
 * ================================================================ */

static int wm_get_count(void)
{
    return wm.count;
}
