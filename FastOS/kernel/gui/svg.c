/* FastOS v4.0 — Procedural Icon System
 * ADead-BIB Native OS — 5 built-in 32×32 ARGB icons
 *
 * Icons rendered procedurally (rect, circle, rounded_rect).
 * Cache at 0x700000 (7MB), 4096 bytes per 32×32 icon.
 * folder, terminal, settings, app (Po), adead (skull).
 *
 * Compiled by: ADead-BIB (C is Master, Rust is Safety)
 */

#include "../include/types.h"

/* fb_surface_t defined in fb.c (inline) */

/* ================================================================
 * Icon Constants
 * ================================================================ */

#define ICON_SMALL   32   /* 32×32 desktop icons */
#define ICON_LARGE   64   /* 64×64 for file manager / dialogs */
#define ICON_MAX     32   /* max cached icons */

/* ================================================================
 * Rasterized Icon Cache
 *
 * Icons are pre-rasterized to ARGB pixel buffers.
 * Each 32×32 icon = 4096 bytes. 32 icons = 128KB total.
 * Stored at fixed address 0x700000 (7MB), below WM surface pool.
 * ================================================================ */

#define ICON_CACHE_BASE   0x700000
#define ICON_CACHE_SLOT   (ICON_SMALL * ICON_SMALL * 4)  /* 4096 bytes */

typedef struct {
    uint32_t *pixels;       /* pointer into cache memory */
    uint32_t  width;
    uint32_t  height;
    int       valid;        /* 1 if rasterized */
    char      name[16];     /* icon name for lookup */
} svg_icon_t;

static svg_icon_t icon_cache[ICON_MAX];
static int icon_count = 0;

/* ================================================================
 * Built-in Icon Data — Procedural Rendering
 *
 * Instead of parsing SVG XML, we render icons procedurally.
 * Each icon is a function that draws into a 32×32 pixel buffer.
 * This is simpler, smaller, and faster than XML parsing.
 *
 * Icon style: flat design, 2-3 colors, clear silhouettes.
 * ================================================================ */

/* Set pixel in icon buffer (with bounds check) */
static void icon_pixel(uint32_t *buf, int x, int y, int w, int h, uint32_t color)
{
    if (x >= 0 && x < w && y >= 0 && y < h) {
        buf[y * w + x] = color;
    }
}

/* Fill rectangle in icon buffer */
static void icon_rect(uint32_t *buf, int bx, int by, int bw, int bh,
                      int w, int h, uint32_t color)
{
    int iy, ix;
    iy = by;
    while (iy < by + bh) {
        ix = bx;
        while (ix < bx + bw) {
            icon_pixel(buf, ix, iy, w, h, color);
            ix = ix + 1;
        }
        iy = iy + 1;
    }
}

/* Fill circle in icon buffer (Bresenham midpoint) */
static void icon_circle(uint32_t *buf, int cx, int cy, int r,
                        int w, int h, uint32_t color)
{
    int iy, ix;
    int dx, dy;
    iy = cy - r;
    while (iy <= cy + r) {
        ix = cx - r;
        while (ix <= cx + r) {
            dx = ix - cx;
            dy = iy - cy;
            if (dx * dx + dy * dy <= r * r) {
                icon_pixel(buf, ix, iy, w, h, color);
            }
            ix = ix + 1;
        }
        iy = iy + 1;
    }
}

/* Rounded rect (rect + circles at corners) */
static void icon_rounded_rect(uint32_t *buf, int bx, int by, int bw, int bh,
                               int radius, int w, int h, uint32_t color)
{
    int iy, ix, dx, dy, r2;

    r2 = radius * radius;
    iy = by;
    while (iy < by + bh) {
        ix = bx;
        while (ix < bx + bw) {
            /* Check if pixel is inside rounded rect */
            int inside = 1;

            /* Top-left corner */
            if (ix < bx + radius && iy < by + radius) {
                dx = ix - (bx + radius);
                dy = iy - (by + radius);
                if (dx * dx + dy * dy > r2) inside = 0;
            }
            /* Top-right corner */
            if (ix >= bx + bw - radius && iy < by + radius) {
                dx = ix - (bx + bw - radius - 1);
                dy = iy - (by + radius);
                if (dx * dx + dy * dy > r2) inside = 0;
            }
            /* Bottom-left corner */
            if (ix < bx + radius && iy >= by + bh - radius) {
                dx = ix - (bx + radius);
                dy = iy - (by + bh - radius - 1);
                if (dx * dx + dy * dy > r2) inside = 0;
            }
            /* Bottom-right corner */
            if (ix >= bx + bw - radius && iy >= by + bh - radius) {
                dx = ix - (bx + bw - radius - 1);
                dy = iy - (by + bh - radius - 1);
                if (dx * dx + dy * dy > r2) inside = 0;
            }

            if (inside) {
                icon_pixel(buf, ix, iy, w, h, color);
            }
            ix = ix + 1;
        }
        iy = iy + 1;
    }
}

/* ================================================================
 * Built-in Icon: Folder (📁)
 *
 * Yellow folder with tab, darker shade for front face.
 * ================================================================ */

static void icon_render_folder(uint32_t *buf, int w, int h)
{
    /* Clear to transparent */
    int i;
    i = 0; while (i < w * h) { buf[i] = 0x00000000; i = i + 1; }

    /* Folder tab (top-left bump) */
    icon_rounded_rect(buf, 3, 6, 12, 5, 2, w, h, 0xFFDDA020);

    /* Folder body */
    icon_rounded_rect(buf, 2, 9, 28, 18, 3, w, h, 0xFFEEB840);

    /* Folder front face (slightly darker) */
    icon_rounded_rect(buf, 2, 13, 28, 14, 3, w, h, 0xFFDDA020);

    /* Shadow line */
    icon_rect(buf, 4, 13, 24, 1, w, h, 0xFFCC9010);
}

/* ================================================================
 * Built-in Icon: Terminal (💻)
 *
 * Dark rectangle with "> _" prompt.
 * ================================================================ */

static void icon_render_terminal(uint32_t *buf, int w, int h)
{
    int i;
    i = 0; while (i < w * h) { buf[i] = 0x00000000; i = i + 1; }

    /* Terminal body (dark bg with rounded corners) */
    icon_rounded_rect(buf, 2, 4, 28, 24, 3, w, h, 0xFF1A1A2E);

    /* Border */
    icon_rect(buf, 2, 4, 28, 1, w, h, 0xFF555577);  /* top */
    icon_rect(buf, 2, 27, 28, 1, w, h, 0xFF555577); /* bottom */
    icon_rect(buf, 2, 4, 1, 24, w, h, 0xFF555577);  /* left */
    icon_rect(buf, 29, 4, 1, 24, w, h, 0xFF555577); /* right */

    /* Titlebar */
    icon_rect(buf, 3, 5, 26, 4, w, h, 0xFF2D2D5E);

    /* Titlebar dots (close/min/max) */
    icon_pixel(buf, 5, 7, w, h, 0xFFFF5555);   /* red */
    icon_pixel(buf, 8, 7, w, h, 0xFFFFBB33);   /* yellow */
    icon_pixel(buf, 11, 7, w, h, 0xFF55CC55);  /* green */

    /* Prompt: "> " */
    icon_rect(buf, 6, 13, 1, 1, w, h, 0xFF55FF55);  /* > tip */
    icon_rect(buf, 7, 14, 1, 1, w, h, 0xFF55FF55);
    icon_rect(buf, 6, 15, 1, 1, w, h, 0xFF55FF55);

    /* Cursor: "_" blinking block */
    icon_rect(buf, 10, 15, 4, 1, w, h, 0xFF55FF55);

    /* Sample text line */
    icon_rect(buf, 6, 19, 8, 1, w, h, 0xFFAAAAAA);
    icon_rect(buf, 6, 22, 12, 1, w, h, 0xFF888888);
}

/* ================================================================
 * Built-in Icon: Settings (⚙)
 *
 * Gear shape: circle with rectangular teeth.
 * ================================================================ */

static void icon_render_settings(uint32_t *buf, int w, int h)
{
    int i, ix, iy, dx, dy, angle_ok;
    i = 0; while (i < w * h) { buf[i] = 0x00000000; i = i + 1; }

    /* Outer gear (circle with teeth approximated as rects) */
    icon_circle(buf, 16, 16, 11, w, h, 0xFF888899);

    /* Gear teeth (8 rectangular protrusions) */
    /* Top */    icon_rect(buf, 14, 2, 4, 4, w, h, 0xFF888899);
    /* Bottom */ icon_rect(buf, 14, 26, 4, 4, w, h, 0xFF888899);
    /* Left */   icon_rect(buf, 2, 14, 4, 4, w, h, 0xFF888899);
    /* Right */  icon_rect(buf, 26, 14, 4, 4, w, h, 0xFF888899);
    /* TL */     icon_rect(buf, 5, 5, 4, 3, w, h, 0xFF888899);
    /* TR */     icon_rect(buf, 23, 5, 4, 3, w, h, 0xFF888899);
    /* BL */     icon_rect(buf, 5, 24, 4, 3, w, h, 0xFF888899);
    /* BR */     icon_rect(buf, 23, 24, 4, 3, w, h, 0xFF888899);

    /* Inner circle (cutout — darker) */
    icon_circle(buf, 16, 16, 7, w, h, 0xFF555566);

    /* Center dot */
    icon_circle(buf, 16, 16, 3, w, h, 0xFFAAAABB);
}

/* ================================================================
 * Built-in Icon: App (.po generic)
 *
 * Blue rounded square with "Po" text.
 * ================================================================ */

static void icon_render_app(uint32_t *buf, int w, int h)
{
    int i;
    i = 0; while (i < w * h) { buf[i] = 0x00000000; i = i + 1; }

    /* App body (blue rounded rect) */
    icon_rounded_rect(buf, 3, 3, 26, 26, 5, w, h, 0xFF2255AA);

    /* Inner highlight */
    icon_rounded_rect(buf, 5, 5, 22, 22, 4, w, h, 0xFF3366BB);

    /* "P" letter (simplified) */
    icon_rect(buf, 9, 9, 2, 14, w, h, 0xFFFFFFFF);  /* vertical stroke */
    icon_rect(buf, 11, 9, 5, 2, w, h, 0xFFFFFFFF);  /* top horizontal */
    icon_rect(buf, 16, 9, 2, 6, w, h, 0xFFFFFFFF);  /* right vertical */
    icon_rect(buf, 11, 15, 5, 2, w, h, 0xFFFFFFFF);  /* middle horizontal */

    /* "o" letter */
    icon_circle(buf, 23, 18, 4, w, h, 0xFFFFFFFF);
    icon_circle(buf, 23, 18, 2, w, h, 0xFF3366BB);  /* hollow center */
}

/* ================================================================
 * Built-in Icon: ADead-BIB (💀)
 *
 * Skull icon for the compiler.
 * ================================================================ */

static void icon_render_adead(uint32_t *buf, int w, int h)
{
    int i;
    i = 0; while (i < w * h) { buf[i] = 0x00000000; i = i + 1; }

    /* Skull shape */
    icon_circle(buf, 16, 13, 10, w, h, 0xFFEEEEDD);

    /* Jaw */
    icon_rounded_rect(buf, 9, 20, 14, 6, 2, w, h, 0xFFEEEEDD);

    /* Eyes */
    icon_circle(buf, 12, 12, 3, w, h, 0xFF222222);
    icon_circle(buf, 20, 12, 3, w, h, 0xFF222222);

    /* Nose */
    icon_rect(buf, 15, 16, 2, 2, w, h, 0xFF444444);

    /* Teeth */
    icon_rect(buf, 11, 21, 2, 3, w, h, 0xFF333333);
    icon_rect(buf, 14, 21, 2, 3, w, h, 0xFF333333);
    icon_rect(buf, 17, 21, 2, 3, w, h, 0xFF333333);
    icon_rect(buf, 20, 21, 2, 3, w, h, 0xFF333333);
}

/* ================================================================
 * Icon Cache Management
 * ================================================================ */

/* String compare (inline — no libc) */
static int svg_strcmp(const char *a, const char *b)
{
    int i;
    i = 0;
    while (a[i] != 0 && b[i] != 0) {
        if (a[i] != b[i]) return 1;
        i = i + 1;
    }
    return (a[i] != b[i]) ? 1 : 0;
}

/* String copy */
static void svg_strcpy(char *dst, const char *src, int max)
{
    int i;
    i = 0;
    while (src[i] != 0 && i < max - 1) { dst[i] = src[i]; i = i + 1; }
    dst[i] = 0;
}

/* Register a built-in icon */
static svg_icon_t *icon_register(const char *name, int w, int h)
{
    svg_icon_t *icon;
    uint32_t addr;

    if (icon_count >= ICON_MAX) return 0;

    icon = &icon_cache[icon_count];
    addr = ICON_CACHE_BASE + (uint32_t)icon_count * ICON_CACHE_SLOT;
    icon->pixels = (uint32_t *)((uintptr_t)addr);
    icon->width  = (uint32_t)w;
    icon->height = (uint32_t)h;
    icon->valid  = 1;
    svg_strcpy(icon->name, name, 16);

    icon_count = icon_count + 1;
    return icon;
}

/* ================================================================
 * Init All Built-in Icons
 *
 * Called during desktop init. Renders all system icons to cache.
 * ================================================================ */

static void icon_init_builtin(void)
{
    svg_icon_t *ico;

    ico = icon_register("folder", ICON_SMALL, ICON_SMALL);
    if (ico) icon_render_folder(ico->pixels, ICON_SMALL, ICON_SMALL);

    ico = icon_register("terminal", ICON_SMALL, ICON_SMALL);
    if (ico) icon_render_terminal(ico->pixels, ICON_SMALL, ICON_SMALL);

    ico = icon_register("settings", ICON_SMALL, ICON_SMALL);
    if (ico) icon_render_settings(ico->pixels, ICON_SMALL, ICON_SMALL);

    ico = icon_register("app", ICON_SMALL, ICON_SMALL);
    if (ico) icon_render_app(ico->pixels, ICON_SMALL, ICON_SMALL);

    ico = icon_register("adead", ICON_SMALL, ICON_SMALL);
    if (ico) icon_render_adead(ico->pixels, ICON_SMALL, ICON_SMALL);
}

/* ================================================================
 * Find Icon by Name
 * ================================================================ */

static svg_icon_t *icon_find(const char *name)
{
    int i;
    i = 0;
    while (i < icon_count) {
        if (svg_strcmp(icon_cache[i].name, name) == 0) {
            return &icon_cache[i];
        }
        i = i + 1;
    }
    return 0;
}

/* ================================================================
 * Draw Icon onto Framebuffer Surface (with alpha)
 *
 * Blits the icon at (x, y) onto the target surface.
 * Pixels with alpha=0 are transparent (skip).
 * ================================================================ */

static void icon_draw(fb_surface_t *target, int32_t x, int32_t y,
                      svg_icon_t *icon)
{
    int iy, ix;
    uint32_t *src_row;
    uint32_t *dst_row;
    uint32_t pixel;

    if (!icon || !icon->valid) return;

    iy = 0;
    while (iy < (int)icon->height) {
        if (y + iy < 0 || y + iy >= (int)target->height) {
            iy = iy + 1;
            continue;
        }
        src_row = icon->pixels + iy * (int)icon->width;
        dst_row = (uint32_t *)((uint8_t *)target->pixels + (y + iy) * target->pitch);

        ix = 0;
        while (ix < (int)icon->width) {
            if (x + ix >= 0 && x + ix < (int)target->width) {
                pixel = src_row[ix];
                if ((pixel >> 24) > 0) {  /* non-transparent */
                    dst_row[x + ix] = pixel;
                }
            }
            ix = ix + 1;
        }
        iy = iy + 1;
    }
}
