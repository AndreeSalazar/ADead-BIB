/* FastOS v4.0 — Native API for Po Applications
 * ADead-BIB Native OS — 256-bit GUI API
 *
 * Provides a structured API table (FastOSAPI) for .po apps.
 * Apps receive a pointer to this struct and use it to:
 *   - Create/destroy windows
 *   - Draw rectangles, text, pixels
 *   - Handle input (keyboard, mouse)
 *   - System calls (heap alloc, BG query, serial debug)
 *
 * All function pointers reference kernel-internal static functions.
 * Po apps are 64-bit compatible; native apps are 256-bit BG-verified.
 *
 * Usage in a Po app:
 *   void app_main(FastOSAPI *api) {
 *       void *win = api->create_window("MyApp", 100, 100, 400, 300, 0);
 *       api->draw_rect(win, 10, 10, 100, 50, 0xFF00FF00);
 *       api->draw_text(win, 10, 70, "Hello", 0xFFFFFFFF);
 *   }
 *
 * Compiled by: ADead-BIB (C is Master, Rust is Safety)
 */

#include "../include/types.h"

/* All GUI functions (fb_*, font_*, wm_*, etc.) are defined inline
 * in the compilation unit before this file. */

/* ================================================================
 * FastOSAPI — Function Table for Po Applications
 * ================================================================ */

typedef struct FastOSAPI {
    /* Version */
    uint32_t    version;         /* API version: 0x0400 = v4.0 */
    uint32_t    flags;           /* capabilities: bit 0=GUI, bit 1=AVX2 */

    /* Window management */
    void *    (*create_window)(const char *title, int32_t x, int32_t y,
                               int32_t w, int32_t h, uint32_t flags);
    void      (*destroy_window)(void *win);
    void *    (*get_content)(void *win);

    /* Drawing (into window content surface) */
    void      (*draw_rect)(void *win, uint32_t x, uint32_t y,
                           uint32_t w, uint32_t h, uint32_t color);
    void      (*draw_text)(void *win, uint32_t x, uint32_t y,
                           const char *str, uint32_t color);
    void      (*draw_pixel)(void *win, uint32_t x, uint32_t y, uint32_t color);
    void      (*draw_clear)(void *win, uint32_t color);

    /* Input */
    int       (*poll_key)(void);      /* returns scancode or 0 */
    int       (*key_to_ascii)(int sc);
    void      (*get_mouse)(int32_t *x, int32_t *y, int *buttons);

    /* System */
    void *    (*kmalloc)(uint32_t size);
    void      (*kfree)(void *ptr);
    void      (*serial_print)(const char *msg);
    uint32_t  (*get_ticks)(void);
} FastOSAPI;

/* ================================================================
 * API Implementation — Wrappers around kernel functions
 * ================================================================ */

static void *api_create_window(const char *title, int32_t x, int32_t y,
                                int32_t w, int32_t h, uint32_t flags)
{
    return (void *)wm_create(title, x, y, w, h, flags);
}

static void api_destroy_window(void *win)
{
    if (win) wm_destroy((po_window_t *)win);
}

static void *api_get_content(void *win)
{
    if (!win) return 0;
    return (void *)wm_get_content((po_window_t *)win);
}

static void api_draw_rect(void *win, uint32_t x, uint32_t y,
                           uint32_t w, uint32_t h, uint32_t color)
{
    fb_surface_t *content;
    if (!win) return;
    content = wm_get_content((po_window_t *)win);
    if (content) fb_fill_rect(content, x, y, w, h, color);
}

static void api_draw_text(void *win, uint32_t x, uint32_t y,
                           const char *str, uint32_t color)
{
    fb_surface_t *content;
    if (!win) return;
    content = wm_get_content((po_window_t *)win);
    if (content) font_draw_string(content, x, y, str, color, 0x00000000);
}

static void api_draw_pixel(void *win, uint32_t x, uint32_t y, uint32_t color)
{
    fb_surface_t *content;
    if (!win) return;
    content = wm_get_content((po_window_t *)win);
    if (content) fb_pixel(content, x, y, color);
}

static void api_draw_clear(void *win, uint32_t color)
{
    fb_surface_t *content;
    if (!win) return;
    content = wm_get_content((po_window_t *)win);
    if (content) fb_clear(content, color);
}

static int api_poll_key(void)
{
    int sc;
    sc = __inb(0x64);
    if (sc & 1) {
        sc = __inb(0x60);
        if (sc > 0 && sc < 128) return sc;
    }
    return 0;
}

static int api_key_to_ascii(int sc)
{
    /* Same scancode→ASCII table as kernel/desktop */
    if(sc==0x10) return 113; if(sc==0x11) return 119;
    if(sc==0x12) return 101; if(sc==0x13) return 114;
    if(sc==0x14) return 116; if(sc==0x15) return 121;
    if(sc==0x16) return 117; if(sc==0x17) return 105;
    if(sc==0x18) return 111; if(sc==0x19) return 112;
    if(sc==0x1E) return 97;  if(sc==0x1F) return 115;
    if(sc==0x20) return 100; if(sc==0x21) return 102;
    if(sc==0x22) return 103; if(sc==0x23) return 104;
    if(sc==0x24) return 106; if(sc==0x25) return 107;
    if(sc==0x26) return 108; if(sc==0x2C) return 122;
    if(sc==0x2D) return 120; if(sc==0x2E) return 99;
    if(sc==0x2F) return 118; if(sc==0x30) return 98;
    if(sc==0x31) return 110; if(sc==0x32) return 109;
    if(sc==0x39) return 32;  if(sc==0x0C) return 45;
    if(sc==0x34) return 46;
    if(sc==0x02) return 49; if(sc==0x03) return 50;
    if(sc==0x04) return 51; if(sc==0x05) return 52;
    if(sc==0x06) return 53; if(sc==0x07) return 54;
    if(sc==0x08) return 55; if(sc==0x09) return 56;
    if(sc==0x0A) return 57; if(sc==0x0B) return 48;
    if(sc==0x1C) return 13;  /* Enter */
    if(sc==0x0E) return 8;   /* Backspace */
    return 0;
}

static void api_get_mouse(int32_t *x, int32_t *y, int *buttons)
{
    mouse_get_pos(x, y);
    *buttons = mouse_left_down() ? 1 : 0;
}

static void api_serial_print(const char *msg)
{
    int i;
    i = 0;
    while (msg[i] != 0) {
        __outb(0x3F8, msg[i]);
        i = i + 1;
    }
}

/* Simple tick counter (incremented by PIT — placeholder) */
static uint32_t api_ticks = 0;
static uint32_t api_get_ticks(void)
{
    api_ticks = api_ticks + 1;
    return api_ticks;
}

/* ================================================================
 * Global API Instance
 * ================================================================ */

static FastOSAPI fastos_api = {
    0x0400,             /* version */
    0x03,               /* flags: GUI + AVX2 */
    api_create_window,
    api_destroy_window,
    api_get_content,
    api_draw_rect,
    api_draw_text,
    api_draw_pixel,
    api_draw_clear,
    api_poll_key,
    api_key_to_ascii,
    api_get_mouse,
    0,                  /* kmalloc — TODO: connect to heap.c */
    0,                  /* kfree   — TODO: connect to heap.c */
    api_serial_print,
    api_get_ticks
};

