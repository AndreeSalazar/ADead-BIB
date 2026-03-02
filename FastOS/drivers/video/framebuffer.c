/*
 * FastOS v2.0 — Framebuffer/VBE Graphics Driver
 * Generic graphics driver for x86/x86-64
 * Supports VBE 2.0+ modes with linear framebuffer
 */

#include "../../include/kernel.h"
#include "../../include/types.h"

/* VBE Info Block */
typedef struct {
    char signature[4];      /* "VESA" */
    uint16_t version;
    uint32_t oem_string;
    uint32_t capabilities;
    uint32_t video_modes;
    uint16_t total_memory;  /* In 64KB blocks */
    uint16_t oem_software_rev;
    uint32_t oem_vendor;
    uint32_t oem_product;
    uint32_t oem_product_rev;
    uint8_t reserved[222];
    uint8_t oem_data[256];
} __packed vbe_info_t;

/* VBE Mode Info Block */
typedef struct {
    uint16_t attributes;
    uint8_t window_a;
    uint8_t window_b;
    uint16_t granularity;
    uint16_t window_size;
    uint16_t segment_a;
    uint16_t segment_b;
    uint32_t win_func_ptr;
    uint16_t pitch;         /* Bytes per scanline */
    
    uint16_t width;
    uint16_t height;
    uint8_t char_width;
    uint8_t char_height;
    uint8_t planes;
    uint8_t bpp;            /* Bits per pixel */
    uint8_t banks;
    uint8_t memory_model;
    uint8_t bank_size;
    uint8_t image_pages;
    uint8_t reserved0;
    
    uint8_t red_mask;
    uint8_t red_position;
    uint8_t green_mask;
    uint8_t green_position;
    uint8_t blue_mask;
    uint8_t blue_position;
    uint8_t rsv_mask;
    uint8_t rsv_position;
    uint8_t direct_color;
    
    uint32_t framebuffer;   /* Physical address of framebuffer */
    uint32_t off_screen;
    uint16_t off_screen_size;
    
    uint8_t reserved1[206];
} __packed vbe_mode_info_t;

/* Framebuffer State */
typedef struct {
    uint32_t *buffer;       /* Framebuffer address */
    uint32_t width;
    uint32_t height;
    uint32_t pitch;         /* Bytes per line */
    uint32_t bpp;           /* Bits per pixel */
    uint32_t size;          /* Total size in bytes */
} framebuffer_t;

static framebuffer_t fb = {0};

/* Color definitions (32-bit ARGB) */
#define COLOR_BLACK       0xFF000000
#define COLOR_WHITE       0xFFFFFFFF
#define COLOR_RED         0xFFFF0000
#define COLOR_GREEN       0xFF00FF00
#define COLOR_BLUE        0xFF0000FF
#define COLOR_CYAN        0xFF00FFFF
#define COLOR_MAGENTA     0xFFFF00FF
#define COLOR_YELLOW      0xFFFFFF00
#define COLOR_GRAY        0xFF808080
#define COLOR_DARK_GRAY   0xFF404040
#define COLOR_LIGHT_GRAY  0xFFC0C0C0

/* Windows-style colors */
#define COLOR_WIN_BLUE    0xFF0078D7  /* Windows 10 blue */
#define COLOR_WIN_DARK    0xFF1E1E1E  /* Dark theme */
#define COLOR_TASKBAR     0xFF2D2D30  /* Taskbar gray */
#define COLOR_START_GREEN 0xFF0F7B0F  /* Start button green */

/* Linux-style colors */
#define COLOR_UBUNTU_ORANGE  0xFFE95420
#define COLOR_UBUNTU_PURPLE  0xFF772953
#define COLOR_TERMINAL_BG    0xFF300A24

/* FastOS brand colors */
#define COLOR_FASTOS_BLUE    0xFF1A73E8
#define COLOR_FASTOS_DARK    0xFF121212
#define COLOR_FASTOS_ACCENT  0xFF00BCD4

/* Set pixel (with bounds checking) */
static inline void fb_set_pixel_fast(uint32_t x, uint32_t y, uint32_t color) {
    if (x < fb.width && y < fb.height) {
        fb.buffer[y * (fb.pitch / 4) + x] = color;
    }
}

void fb_set_pixel(uint32_t x, uint32_t y, uint32_t color) {
    fb_set_pixel_fast(x, y, color);
}

/* Get pixel */
uint32_t fb_get_pixel(uint32_t x, uint32_t y) {
    if (x < fb.width && y < fb.height) {
        return fb.buffer[y * (fb.pitch / 4) + x];
    }
    return 0;
}

/* Clear screen */
void fb_clear(uint32_t color) {
    uint32_t pixels = fb.width * fb.height;
    for (uint32_t i = 0; i < pixels; i++) {
        fb.buffer[i] = color;
    }
}

/* Fill rectangle */
void fb_fill_rect(uint32_t x, uint32_t y, uint32_t w, uint32_t h, uint32_t color) {
    for (uint32_t j = y; j < y + h && j < fb.height; j++) {
        for (uint32_t i = x; i < x + w && i < fb.width; i++) {
            fb.buffer[j * (fb.pitch / 4) + i] = color;
        }
    }
}

/* Draw rectangle outline */
void fb_draw_rect(uint32_t x, uint32_t y, uint32_t w, uint32_t h, uint32_t color) {
    /* Top and bottom */
    for (uint32_t i = x; i < x + w; i++) {
        fb_set_pixel_fast(i, y, color);
        fb_set_pixel_fast(i, y + h - 1, color);
    }
    /* Left and right */
    for (uint32_t j = y; j < y + h; j++) {
        fb_set_pixel_fast(x, j, color);
        fb_set_pixel_fast(x + w - 1, j, color);
    }
}

/* Draw horizontal line */
void fb_hline(uint32_t x, uint32_t y, uint32_t len, uint32_t color) {
    for (uint32_t i = 0; i < len; i++) {
        fb_set_pixel_fast(x + i, y, color);
    }
}

/* Draw vertical line */
void fb_vline(uint32_t x, uint32_t y, uint32_t len, uint32_t color) {
    for (uint32_t i = 0; i < len; i++) {
        fb_set_pixel_fast(x, y + i, color);
    }
}

/* Alpha blend two colors */
static uint32_t blend_colors(uint32_t fg, uint32_t bg, uint8_t alpha) {
    uint8_t r1 = (fg >> 16) & 0xFF;
    uint8_t g1 = (fg >> 8) & 0xFF;
    uint8_t b1 = fg & 0xFF;
    
    uint8_t r2 = (bg >> 16) & 0xFF;
    uint8_t g2 = (bg >> 8) & 0xFF;
    uint8_t b2 = bg & 0xFF;
    
    uint8_t r = (r1 * alpha + r2 * (255 - alpha)) / 255;
    uint8_t g = (g1 * alpha + g2 * (255 - alpha)) / 255;
    uint8_t b = (b1 * alpha + b2 * (255 - alpha)) / 255;
    
    return 0xFF000000 | (r << 16) | (g << 8) | b;
}

/* Draw gradient rectangle */
void fb_gradient_rect(uint32_t x, uint32_t y, uint32_t w, uint32_t h,
                      uint32_t color1, uint32_t color2, int vertical) {
    for (uint32_t j = 0; j < h; j++) {
        for (uint32_t i = 0; i < w; i++) {
            uint8_t t = vertical ? (j * 255 / h) : (i * 255 / w);
            uint32_t color = blend_colors(color2, color1, t);
            fb_set_pixel_fast(x + i, y + j, color);
        }
    }
}

/* Simple 8x8 bitmap font */
static const uint8_t font_8x8[128][8] = {
    /* ASCII 32-126 basic characters */
    [' '] = {0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00},
    ['!'] = {0x18,0x18,0x18,0x18,0x18,0x00,0x18,0x00},
    ['"'] = {0x6C,0x6C,0x24,0x00,0x00,0x00,0x00,0x00},
    ['0'] = {0x3C,0x66,0x6E,0x76,0x66,0x66,0x3C,0x00},
    ['1'] = {0x18,0x38,0x18,0x18,0x18,0x18,0x7E,0x00},
    ['2'] = {0x3C,0x66,0x06,0x0C,0x18,0x30,0x7E,0x00},
    ['3'] = {0x3C,0x66,0x06,0x1C,0x06,0x66,0x3C,0x00},
    ['4'] = {0x0C,0x1C,0x3C,0x6C,0x7E,0x0C,0x0C,0x00},
    ['5'] = {0x7E,0x60,0x7C,0x06,0x06,0x66,0x3C,0x00},
    ['6'] = {0x1C,0x30,0x60,0x7C,0x66,0x66,0x3C,0x00},
    ['7'] = {0x7E,0x06,0x0C,0x18,0x30,0x30,0x30,0x00},
    ['8'] = {0x3C,0x66,0x66,0x3C,0x66,0x66,0x3C,0x00},
    ['9'] = {0x3C,0x66,0x66,0x3E,0x06,0x0C,0x38,0x00},
    ['A'] = {0x18,0x3C,0x66,0x66,0x7E,0x66,0x66,0x00},
    ['B'] = {0x7C,0x66,0x66,0x7C,0x66,0x66,0x7C,0x00},
    ['C'] = {0x3C,0x66,0x60,0x60,0x60,0x66,0x3C,0x00},
    ['D'] = {0x78,0x6C,0x66,0x66,0x66,0x6C,0x78,0x00},
    ['E'] = {0x7E,0x60,0x60,0x7C,0x60,0x60,0x7E,0x00},
    ['F'] = {0x7E,0x60,0x60,0x7C,0x60,0x60,0x60,0x00},
    ['a'] = {0x00,0x00,0x3C,0x06,0x3E,0x66,0x3E,0x00},
    ['s'] = {0x00,0x00,0x3E,0x60,0x3C,0x06,0x7C,0x00},
    ['t'] = {0x18,0x18,0x7E,0x18,0x18,0x18,0x0E,0x00},
    ['O'] = {0x3C,0x66,0x66,0x66,0x66,0x66,0x3C,0x00},
    ['S'] = {0x3C,0x66,0x60,0x3C,0x06,0x66,0x3C,0x00},
};

/* Draw character */
void fb_draw_char(uint32_t x, uint32_t y, char c, uint32_t fg, uint32_t bg) {
    if (c < 0 || c > 127) c = '?';
    const uint8_t *glyph = font_8x8[(int)c];
    
    for (int row = 0; row < 8; row++) {
        for (int col = 0; col < 8; col++) {
            uint32_t color = (glyph[row] & (0x80 >> col)) ? fg : bg;
            if (color != 0) {  /* 0 = transparent */
                fb_set_pixel_fast(x + col, y + row, color);
            }
        }
    }
}

/* Draw string */
void fb_draw_string(uint32_t x, uint32_t y, const char *str, uint32_t fg, uint32_t bg) {
    while (*str) {
        fb_draw_char(x, y, *str++, fg, bg);
        x += 8;
    }
}

/* Draw procedural wallpaper (gradient with pattern) */
void fb_draw_wallpaper(void) {
    /* Dark gradient background */
    for (uint32_t y = 0; y < fb.height; y++) {
        for (uint32_t x = 0; x < fb.width; x++) {
            /* Base gradient from dark blue to darker blue */
            uint8_t r = 10 + (y * 20 / fb.height);
            uint8_t g = 20 + (y * 30 / fb.height);
            uint8_t b = 40 + (y * 60 / fb.height);
            
            /* Add subtle grid pattern */
            if ((x % 32 == 0) || (y % 32 == 0)) {
                r += 5;
                g += 8;
                b += 12;
            }
            
            /* Add diagonal lines */
            if ((x + y) % 64 < 2) {
                r += 10;
                g += 15;
                b += 25;
            }
            
            fb.buffer[y * (fb.pitch / 4) + x] = 0xFF000000 | (r << 16) | (g << 8) | b;
        }
    }
    
    /* Draw FastOS logo in center */
    uint32_t cx = fb.width / 2;
    uint32_t cy = fb.height / 2 - 50;
    
    /* Logo box */
    fb_fill_rect(cx - 150, cy - 40, 300, 80, 0x40000000);  /* Semi-transparent */
    fb_draw_rect(cx - 150, cy - 40, 300, 80, COLOR_FASTOS_ACCENT);
    
    /* "FastOS" text would go here with proper font */
}

/* Draw Windows-style taskbar */
void fb_draw_taskbar(void) {
    uint32_t taskbar_y = fb.height - 40;
    
    /* Taskbar background */
    fb_fill_rect(0, taskbar_y, fb.width, 40, COLOR_TASKBAR);
    
    /* Start button */
    fb_fill_rect(5, taskbar_y + 5, 50, 30, COLOR_START_GREEN);
    fb_draw_rect(5, taskbar_y + 5, 50, 30, 0xFF0A5F0A);
    
    /* Clock area */
    fb_fill_rect(fb.width - 80, taskbar_y + 5, 75, 30, 0xFF3D3D40);
}

/* Draw mouse cursor */
void fb_draw_cursor(int32_t x, int32_t y) {
    /* Simple arrow cursor */
    static const uint8_t cursor[16] = {
        0x80, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC, 0xFE, 0xFF,
        0xFC, 0xF8, 0xD8, 0x8C, 0x0C, 0x06, 0x06, 0x00
    };
    
    for (int row = 0; row < 16; row++) {
        for (int col = 0; col < 8; col++) {
            if (cursor[row] & (0x80 >> col)) {
                fb_set_pixel_fast(x + col, y + row, COLOR_WHITE);
            }
        }
    }
}

/* Get framebuffer info */
uint32_t fb_get_width(void)  { return fb.width; }
uint32_t fb_get_height(void) { return fb.height; }
uint32_t fb_get_bpp(void)    { return fb.bpp; }
uint32_t *fb_get_buffer(void) { return fb.buffer; }

/* Initialize framebuffer (called after VBE mode is set) */
void framebuffer_init(uint32_t *buffer, uint32_t width, uint32_t height, 
                      uint32_t pitch, uint32_t bpp) {
    fb.buffer = buffer;
    fb.width = width;
    fb.height = height;
    fb.pitch = pitch;
    fb.bpp = bpp;
    fb.size = pitch * height;
    
    kprintf("[FB] Framebuffer initialized: %dx%d, %d bpp\n", width, height, bpp);
    kprintf("[FB] Buffer at 0x%08X, pitch=%d\n", (uint32_t)(uintptr_t)buffer, pitch);
}

/* Initialize with default VGA text mode fallback */
void framebuffer_init_text(void) {
    /* Use VGA text mode buffer */
    fb.buffer = (uint32_t*)0xB8000;
    fb.width = 80;
    fb.height = 25;
    fb.pitch = 160;
    fb.bpp = 16;
    fb.size = 80 * 25 * 2;
    
    kprintf("[FB] Text mode fallback: 80x25\n");
}
