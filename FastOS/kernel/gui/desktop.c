/* FastOS v3.0 — Desktop Environment
 * ADead-BIB Native OS
 *
 * The main GUI composition layer. Ties together:
 *   - Framebuffer (drivers/fb.c)
 *   - Font renderer (gui/font.c)
 *   - Mouse driver (drivers/mouse_drv.c)
 *   - Window manager (gui/wm.c)
 *   - Icon system (gui/svg.c)
 *
 * Desktop layout (1024×768):
 *   Row 0-23:     Title bar (gradient, "FastOS v3.0" + clock area)
 *   Row 24-727:   Desktop area (gradient bg, icons, windows)
 *   Row 728-767:  Taskbar (dark, running apps, system tray)
 *
 * Desktop icons are clickable — double-click launches .po apps.
 * Single-click selects icon (highlighted border).
 *
 * Event loop:
 *   1. Poll keyboard (i8042 port 0x60)
 *   2. Poll mouse (mouse_poll())
 *   3. Dispatch events to WM / desktop
 *   4. Compose: bg → icons → windows → cursor → taskbar
 *   5. fb_flip() — present frame
 *
 * This replaces the VGA text mode TUI from kernel v2.2.
 * VGA text mode is still used for serial debug output.
 *
 * Compiled by: ADead-BIB (C is Master, Rust is Safety)
 */

#include "../include/types.h"

/* ================================================================
 * Desktop Constants
 * ================================================================ */

#define DESKTOP_TITLEBAR_H   24     /* top title bar height */
#define DESKTOP_TASKBAR_H    40     /* bottom taskbar height */
#define DESKTOP_ICON_SIZE    32     /* icon pixel size */
#define DESKTOP_ICON_SPACING 80     /* grid spacing between icons */
#define DESKTOP_ICON_MARGIN  20     /* margin from edges */
#define DESKTOP_ICON_LABEL_H 16     /* label height below icon */
#define DESKTOP_MAX_ICONS    16     /* max desktop shortcuts */

/* Desktop colors */
#define DESKTOP_BG_TOP       0xFF0A0A2E  /* gradient top (very dark blue) */
#define DESKTOP_BG_BOT       0xFF1A1A4E  /* gradient bottom (dark blue) */
#define DESKTOP_TITLEBAR_BG  0xFF151530  /* title bar bg */
#define DESKTOP_TITLEBAR_FG  0xFFCCCCDD  /* title bar text */
#define DESKTOP_TASKBAR_BG   0xFF1C1C1C  /* taskbar bg */
#define DESKTOP_TASKBAR_FG   0xFFDDDDDD  /* taskbar text */
#define DESKTOP_ICON_SEL     0x40FFFFFF  /* selected icon highlight */
#define DESKTOP_ICON_LABEL   0xFFCCCCCC  /* icon label text */

/* ================================================================
 * Desktop Icon Entry
 * ================================================================ */

typedef struct {
    char     name[16];       /* display name */
    char     icon_name[16];  /* icon cache name (from svg.c) */
    char     action[32];     /* what to do on double-click */
    int32_t  x, y;           /* pixel position on desktop */
    int      selected;       /* 1 if currently selected */
    int      active;         /* 1 if this slot is used */
} desktop_icon_t;

/* ================================================================
 * Desktop State
 * ================================================================ */

typedef struct {
    int              initialized;
    int32_t          screen_w;
    int32_t          screen_h;
    desktop_icon_t   icons[DESKTOP_MAX_ICONS];
    int              icon_count;
    int              selected_icon;    /* -1 = none */
    int              shell_open;       /* 1 if shell window exists */
    uint32_t         shell_win_id;     /* WM window ID for shell */

    /* Shell state (built-in terminal) */
    int              shell_cursor_x;   /* character column in shell */
    int              shell_cursor_y;   /* character row in shell */
    int              shell_cols;       /* terminal columns */
    int              shell_rows;       /* terminal rows */
    char             shell_cmd[64];    /* current command buffer */
    int              shell_cmd_len;    /* command length */
} desktop_state_t;

static desktop_state_t desktop;

/* ================================================================
 * Forward Declarations (from other GUI modules)
 * ================================================================ */

/* fb.c */
static void fb_init(void);
static void fb_clear(fb_surface_t *s, uint32_t color);
static void fb_fill_rect(fb_surface_t *s, uint32_t x, uint32_t y,
                         uint32_t w, uint32_t h, uint32_t color);
static void fb_gradient_v(fb_surface_t *s, uint32_t x, uint32_t y,
                          uint32_t w, uint32_t h,
                          uint32_t color_top, uint32_t color_bot);
static void fb_flip(void);
static void fb_cursor_restore(fb_surface_t *s);
static void fb_cursor_draw(fb_surface_t *s, int32_t mx, int32_t my);
static void fb_rect_outline(fb_surface_t *s, uint32_t x, uint32_t y,
                            uint32_t w, uint32_t h, uint32_t color);

/* font.c */
static uint32_t font_draw_string(fb_surface_t *s, uint32_t x, uint32_t y,
                                 const char *str, uint32_t fg, uint32_t bg);
static void font_draw_centered(fb_surface_t *s, uint32_t y,
                                uint32_t area_x, uint32_t area_w,
                                const char *str, uint32_t fg, uint32_t bg);
static void font_draw_char(fb_surface_t *s, uint32_t x, uint32_t y,
                            char c, uint32_t fg, uint32_t bg);
static void font_int_to_str(int val, char *buf, int buf_size);

/* mouse_drv.c */
static int  mouse_init(int32_t screen_w, int32_t screen_h);
static int  mouse_poll(void);
static int  mouse_left_clicked(void);
static int  mouse_left_down(void);
static int  mouse_left_released(void);
static void mouse_get_pos(int32_t *out_x, int32_t *out_y);
static int  mouse_in_rect(int32_t rx, int32_t ry, int32_t rw, int32_t rh);

/* wm.c */
static void wm_init(int32_t screen_w, int32_t screen_h);
static po_window_t *wm_create(const char *title, int32_t x, int32_t y,
                               int32_t w, int32_t h, uint32_t flags);
static void wm_compose(fb_surface_t *target);
static int  wm_handle_mouse(int32_t mx, int32_t my,
                            int left_down, int left_clicked,
                            int left_released);
static fb_surface_t *wm_get_content(po_window_t *win);
static po_window_t *wm_get_focused(void);

/* svg.c */
static void icon_init_builtin(void);
static svg_icon_t *icon_find(const char *name);
static void icon_draw(fb_surface_t *target, int32_t x, int32_t y,
                      svg_icon_t *icon);

/* fb state (from fb.c) */
typedef struct {
    fb_surface_t front;
    fb_surface_t back;
    int          double_buffered;
    uint32_t     bg_color;
} fb_state_t;
extern fb_state_t fb;

/* ================================================================
 * String helpers (inline, no libc)
 * ================================================================ */

static void dt_strcpy(char *dst, const char *src, int max)
{
    int i = 0;
    while (src[i] != 0 && i < max - 1) { dst[i] = src[i]; i = i + 1; }
    dst[i] = 0;
}

/* ================================================================
 * Add Desktop Icon
 * ================================================================ */

static void desktop_add_icon(const char *name, const char *icon_name,
                             const char *action)
{
    desktop_icon_t *di;
    int row, col;

    if (desktop.icon_count >= DESKTOP_MAX_ICONS) return;

    di = &desktop.icons[desktop.icon_count];
    dt_strcpy(di->name, name, 16);
    dt_strcpy(di->icon_name, icon_name, 16);
    dt_strcpy(di->action, action, 32);

    /* Grid layout: icons arranged in columns, top to bottom */
    col = desktop.icon_count / 6;
    row = desktop.icon_count % 6;
    di->x = DESKTOP_ICON_MARGIN + col * DESKTOP_ICON_SPACING;
    di->y = DESKTOP_TITLEBAR_H + DESKTOP_ICON_MARGIN + row * (DESKTOP_ICON_SIZE + DESKTOP_ICON_LABEL_H + 12);
    di->selected = 0;
    di->active = 1;

    desktop.icon_count = desktop.icon_count + 1;
}

/* ================================================================
 * Desktop Init
 *
 * Called from kernel_main() to transition from TUI to GUI.
 * Initializes all subsystems and sets up default desktop.
 * ================================================================ */

static void desktop_init(int32_t screen_w, int32_t screen_h)
{
    int i;

    desktop.screen_w = screen_w;
    desktop.screen_h = screen_h;
    desktop.icon_count = 0;
    desktop.selected_icon = -1;
    desktop.shell_open = 0;
    desktop.shell_win_id = 0;
    desktop.shell_cursor_x = 0;
    desktop.shell_cursor_y = 0;
    desktop.shell_cmd_len = 0;

    i = 0;
    while (i < DESKTOP_MAX_ICONS) {
        desktop.icons[i].active = 0;
        i = i + 1;
    }

    /* Init subsystems */
    fb_init();
    mouse_init(screen_w, screen_h);
    wm_init(screen_w, screen_h);
    icon_init_builtin();

    /* Add default desktop icons */
    desktop_add_icon("Files",    "folder",   "files");
    desktop_add_icon("Terminal", "terminal", "terminal");
    desktop_add_icon("Settings", "settings", "settings");
    desktop_add_icon("ADead-BIB","adead",    "adead");

    desktop.initialized = 1;

    /* Serial: "GUI:OK\r\n" */
    __outb(0x3F8, 71); __outb(0x3F8, 85); __outb(0x3F8, 73);
    __outb(0x3F8, 58); __outb(0x3F8, 79); __outb(0x3F8, 75);
    __outb(0x3F8, 13); __outb(0x3F8, 10);
}

/* ================================================================
 * Draw Title Bar
 * ================================================================ */

static void desktop_draw_titlebar(fb_surface_t *target)
{
    /* Title bar background */
    fb_fill_rect(target, 0, 0,
                 (uint32_t)desktop.screen_w, DESKTOP_TITLEBAR_H,
                 DESKTOP_TITLEBAR_BG);

    /* "FastOS v3.0" left-aligned */
    font_draw_string(target, 8, 4,
                     "FastOS v3.0", DESKTOP_TITLEBAR_FG, 0x00000000);

    /* "ADead-BIB" right-aligned */
    font_draw_string(target, (uint32_t)(desktop.screen_w - 80), 4,
                     "ADead-BIB", 0xFF888899, 0x00000000);

    /* BG status: green dot + "BG" */
    fb_fill_rect(target, (uint32_t)(desktop.screen_w - 160), 8, 8, 8, 0xFF00AA00);
    font_draw_string(target, (uint32_t)(desktop.screen_w - 148), 4,
                     "BG", 0xFF00CC00, 0x00000000);

    /* Separator line */
    fb_fill_rect(target, 0, DESKTOP_TITLEBAR_H - 1,
                 (uint32_t)desktop.screen_w, 1, 0xFF333355);
}

/* ================================================================
 * Draw Desktop Icons
 * ================================================================ */

static void desktop_draw_icons(fb_surface_t *target)
{
    int i;
    desktop_icon_t *di;
    svg_icon_t *ico;
    uint32_t label_x;

    i = 0;
    while (i < desktop.icon_count) {
        di = &desktop.icons[i];
        if (!di->active) { i = i + 1; continue; }

        /* Selection highlight */
        if (di->selected) {
            fb_fill_rect(target,
                         (uint32_t)(di->x - 4), (uint32_t)(di->y - 4),
                         DESKTOP_ICON_SIZE + 8, DESKTOP_ICON_SIZE + DESKTOP_ICON_LABEL_H + 12,
                         DESKTOP_ICON_SEL);
            fb_rect_outline(target,
                            (uint32_t)(di->x - 4), (uint32_t)(di->y - 4),
                            DESKTOP_ICON_SIZE + 8, DESKTOP_ICON_SIZE + DESKTOP_ICON_LABEL_H + 12,
                            0x80AAAAFF);
        }

        /* Draw icon */
        ico = icon_find(di->icon_name);
        if (ico) {
            icon_draw(target, di->x, di->y, ico);
        }

        /* Draw label centered below icon */
        label_x = (uint32_t)di->x + (DESKTOP_ICON_SIZE / 2) -
                  (uint32_t)(font_strlen(di->name) * 8 / 2);
        font_draw_string(target, label_x,
                         (uint32_t)(di->y + DESKTOP_ICON_SIZE + 4),
                         di->name, DESKTOP_ICON_LABEL, 0x00000000);

        i = i + 1;
    }
}

/* font_strlen forward decl */
static uint32_t font_strlen(const char *s);

/* ================================================================
 * Draw Taskbar
 * ================================================================ */

static void desktop_draw_taskbar(fb_surface_t *target)
{
    uint32_t ty;
    int i;

    ty = (uint32_t)(desktop.screen_h - DESKTOP_TASKBAR_H);

    /* Taskbar background */
    fb_fill_rect(target, 0, ty,
                 (uint32_t)desktop.screen_w, DESKTOP_TASKBAR_H,
                 DESKTOP_TASKBAR_BG);

    /* Top separator */
    fb_fill_rect(target, 0, ty, (uint32_t)desktop.screen_w, 1, 0xFF333344);

    /* "FastOS" button (left) */
    fb_fill_rect(target, 4, ty + 6, 72, 28, 0xFF2A2A4A);
    fb_rect_outline(target, 4, ty + 6, 72, 28, 0xFF444466);
    font_draw_string(target, 12, ty + 12,
                     "FastOS", 0xFFCCCCEE, 0x00000000);

    /* Running windows in taskbar */
    /* (placeholder — will show window titles when WM is integrated) */

    /* Right side: "BG:APPROVE  256-bit" */
    font_draw_string(target, (uint32_t)(desktop.screen_w - 200), ty + 12,
                     "BG:APPROVE", 0xFF00CC00, 0x00000000);
    font_draw_string(target, (uint32_t)(desktop.screen_w - 90), ty + 12,
                     "256-bit", 0xFF8888AA, 0x00000000);
}

/* ================================================================
 * Shell Window — Built-in Terminal
 *
 * Creates a WM window with an embedded terminal emulator.
 * Handles keyboard input, command parsing, output display.
 * Uses font renderer to draw text into window content surface.
 * ================================================================ */

static void desktop_open_shell(void)
{
    po_window_t *win;
    fb_surface_t *content;

    if (desktop.shell_open) return;

    win = wm_create("Terminal", 200, 100, 600, 400, WINDOW_RESIZABLE);
    if (!win) return;

    desktop.shell_open = 1;
    desktop.shell_win_id = win->id;
    desktop.shell_cursor_x = 0;
    desktop.shell_cursor_y = 0;
    desktop.shell_cmd_len = 0;

    /* Calculate terminal dimensions */
    desktop.shell_cols = win->content_w / 8;   /* 8px per char */
    desktop.shell_rows = win->content_h / 16;  /* 16px per char */

    /* Draw initial content: clear + prompt */
    content = wm_get_content(win);
    if (content) {
        fb_clear(content, 0xFF0A0A1E);

        /* Welcome message */
        font_draw_string(content, 0, 0,
                         "FastOS v3.0 Terminal", 0xFF55FF55, 0x00000000);
        font_draw_string(content, 0, 16,
                         "Type 'help' for commands", 0xFF888888, 0x00000000);

        /* Prompt */
        desktop.shell_cursor_y = 2;
        font_draw_string(content, 0, (uint32_t)(desktop.shell_cursor_y * 16),
                         "> ", 0xFF55FF55, 0x00000000);
        desktop.shell_cursor_x = 2;
    }
}

/* ================================================================
 * Shell: Write character to terminal content
 * ================================================================ */

static void shell_putchar(fb_surface_t *content, char c)
{
    if (c == '\n') {
        desktop.shell_cursor_x = 0;
        desktop.shell_cursor_y = desktop.shell_cursor_y + 1;

        /* Scroll if past bottom */
        if (desktop.shell_cursor_y >= desktop.shell_rows) {
            /* Simple scroll: clear and reset to top
             * (proper scrolling would memmove the content surface) */
            fb_clear(content, 0xFF0A0A1E);
            desktop.shell_cursor_y = 0;
        }
        return;
    }

    if (desktop.shell_cursor_x < desktop.shell_cols) {
        font_draw_char(content,
                       (uint32_t)(desktop.shell_cursor_x * 8),
                       (uint32_t)(desktop.shell_cursor_y * 16),
                       c, 0xFFCCCCCC, 0x00000000);
        desktop.shell_cursor_x = desktop.shell_cursor_x + 1;
    }
}

/* Shell: Write string */
static void shell_puts(fb_surface_t *content, const char *s)
{
    int i;
    i = 0;
    while (s[i] != 0) {
        shell_putchar(content, s[i]);
        i = i + 1;
    }
}

/* ================================================================
 * Shell: Execute Command
 * ================================================================ */

static void shell_exec(fb_surface_t *content, const char *cmd, int len)
{
    /* Newline after command */
    shell_putchar(content, '\n');

    /* help */
    if (len == 4 && cmd[0] == 'h' && cmd[1] == 'e' &&
        cmd[2] == 'l' && cmd[3] == 'p') {
        shell_puts(content, "Commands:\n");
        shell_puts(content, " help    this message\n");
        shell_puts(content, " ver     version info\n");
        shell_puts(content, " cpu     CPU info\n");
        shell_puts(content, " mem     memory info\n");
        shell_puts(content, " bg      Binary Guardian\n");
        shell_puts(content, " clear   clear screen\n");
        shell_puts(content, " exit    close terminal\n");
    }
    /* ver */
    else if (len == 3 && cmd[0] == 'v' && cmd[1] == 'e' && cmd[2] == 'r') {
        shell_puts(content, "FastOS v3.0 ADead-BIB\n");
        shell_puts(content, "256-bit native YMM/AVX2\n");
        shell_puts(content, "Po magic: 506F4F53\n");
    }
    /* bg */
    else if (len == 2 && cmd[0] == 'b' && cmd[1] == 'g') {
        shell_puts(content, "BG - Binary Guardian\n");
        shell_puts(content, " Level: KERNEL Ring0\n");
        shell_puts(content, " Verdict: APPROVE\n");
    }
    /* clear */
    else if (len == 5 && cmd[0] == 'c' && cmd[1] == 'l' &&
             cmd[2] == 'e' && cmd[3] == 'a' && cmd[4] == 'r') {
        fb_clear(content, 0xFF0A0A1E);
        desktop.shell_cursor_x = 0;
        desktop.shell_cursor_y = 0;
    }
    /* exit */
    else if (len == 4 && cmd[0] == 'e' && cmd[1] == 'x' &&
             cmd[2] == 'i' && cmd[3] == 't') {
        desktop.shell_open = 0;
        /* Window will be closed by WM destroy */
        return;
    }
    /* unknown */
    else if (len > 0) {
        shell_puts(content, "Unknown: ");
        shell_puts(content, cmd);
        shell_putchar(content, '\n');
    }

    /* New prompt */
    shell_puts(content, "> ");
    desktop.shell_cursor_x = 2;
}

/* ================================================================
 * Shell: Handle Keyboard Input
 * ================================================================ */

static void shell_handle_key(int ascii)
{
    po_window_t *win;
    fb_surface_t *content;

    if (!desktop.shell_open) return;

    win = wm_find(desktop.shell_win_id);
    if (!win) { desktop.shell_open = 0; return; }

    content = wm_get_content(win);
    if (!content) return;

    /* Enter */
    if (ascii == 10 || ascii == 13) {
        desktop.shell_cmd[desktop.shell_cmd_len] = 0;
        shell_exec(content, desktop.shell_cmd, desktop.shell_cmd_len);
        desktop.shell_cmd_len = 0;
        return;
    }

    /* Backspace */
    if (ascii == 8) {
        if (desktop.shell_cmd_len > 0) {
            desktop.shell_cmd_len = desktop.shell_cmd_len - 1;
            desktop.shell_cursor_x = desktop.shell_cursor_x - 1;
            /* Erase character on screen */
            font_draw_char(content,
                           (uint32_t)(desktop.shell_cursor_x * 8),
                           (uint32_t)(desktop.shell_cursor_y * 16),
                           ' ', 0xFF0A0A1E, 0xFF0A0A1E);
        }
        return;
    }

    /* Printable character */
    if (ascii >= 32 && ascii < 127 && desktop.shell_cmd_len < 60) {
        desktop.shell_cmd[desktop.shell_cmd_len] = (char)ascii;
        desktop.shell_cmd_len = desktop.shell_cmd_len + 1;
        shell_putchar(content, (char)ascii);
    }
}

/* ================================================================
 * Desktop: Handle Icon Click
 * ================================================================ */

static void desktop_handle_icon_click(int32_t mx, int32_t my)
{
    int i;
    desktop_icon_t *di;

    /* Deselect all first */
    i = 0;
    while (i < desktop.icon_count) {
        desktop.icons[i].selected = 0;
        i = i + 1;
    }
    desktop.selected_icon = -1;

    /* Check each icon for hit */
    i = 0;
    while (i < desktop.icon_count) {
        di = &desktop.icons[i];
        if (!di->active) { i = i + 1; continue; }

        if (mx >= di->x && mx < di->x + DESKTOP_ICON_SIZE &&
            my >= di->y && my < di->y + DESKTOP_ICON_SIZE + DESKTOP_ICON_LABEL_H) {
            di->selected = 1;
            desktop.selected_icon = i;

            /* Action: open corresponding app */
            if (di->action[0] == 't') {
                /* "terminal" */
                desktop_open_shell();
            }
            /* Other actions can be added here */

            return;
        }
        i = i + 1;
    }
}

/* ================================================================
 * Desktop: Main Compose Frame
 *
 * Called every iteration of the event loop.
 * Draws the complete desktop to the back buffer, then flips.
 * ================================================================ */

static void desktop_compose(void)
{
    fb_surface_t *target;
    int32_t mx, my;

    target = &fb.back;

    /* 1. Desktop background gradient */
    fb_gradient_v(target, 0, DESKTOP_TITLEBAR_H,
                  (uint32_t)desktop.screen_w,
                  (uint32_t)(desktop.screen_h - DESKTOP_TITLEBAR_H - DESKTOP_TASKBAR_H),
                  DESKTOP_BG_TOP, DESKTOP_BG_BOT);

    /* 2. Title bar */
    desktop_draw_titlebar(target);

    /* 3. Desktop icons */
    desktop_draw_icons(target);

    /* 4. Windows (composited by WM in z-order) */
    wm_compose(target);

    /* 5. Taskbar */
    desktop_draw_taskbar(target);

    /* 6. Mouse cursor (always on top) */
    mouse_get_pos(&mx, &my);
    fb_cursor_draw(target, mx, my);

    /* 7. Present frame */
    fb_flip();
}

/* ================================================================
 * Desktop: Main Event Loop
 *
 * This is called from kernel_main() and runs forever.
 * Replaces the VGA text mode shell loop.
 *
 * Integration point: kernel_main() calls desktop_run() after
 * all hardware init (PIC, PIT, CPU detect, E820).
 * ================================================================ */

static void desktop_run(void)
{
    int32_t mx, my;
    int sc, key, ascii;
    int mouse_moved;
    int wm_consumed;

    while (1) {
        /* ---- Poll mouse ---- */
        mouse_moved = mouse_poll();

        /* ---- Poll keyboard ---- */
        sc = __inb(0x64);
        key = 0;
        ascii = 0;
        if (sc & 1) {
            key = __inb(0x60);
            if (key > 0 && key < 128) {
                /* Scancode → ASCII (same table as kernel v2.2) */
                if(key==0x10){ascii=113;} if(key==0x11){ascii=119;}
                if(key==0x12){ascii=101;} if(key==0x13){ascii=114;}
                if(key==0x14){ascii=116;} if(key==0x15){ascii=121;}
                if(key==0x16){ascii=117;} if(key==0x17){ascii=105;}
                if(key==0x18){ascii=111;} if(key==0x19){ascii=112;}
                if(key==0x1E){ascii=97;}  if(key==0x1F){ascii=115;}
                if(key==0x20){ascii=100;} if(key==0x21){ascii=102;}
                if(key==0x22){ascii=103;} if(key==0x23){ascii=104;}
                if(key==0x24){ascii=106;} if(key==0x25){ascii=107;}
                if(key==0x26){ascii=108;} if(key==0x2C){ascii=122;}
                if(key==0x2D){ascii=120;} if(key==0x2E){ascii=99;}
                if(key==0x2F){ascii=118;} if(key==0x30){ascii=98;}
                if(key==0x31){ascii=110;} if(key==0x32){ascii=109;}
                if(key==0x39){ascii=32;}  if(key==0x0C){ascii=45;}
                if(key==0x34){ascii=46;}
                if(key==0x02){ascii=49;} if(key==0x03){ascii=50;}
                if(key==0x04){ascii=51;} if(key==0x05){ascii=52;}
                if(key==0x06){ascii=53;} if(key==0x07){ascii=54;}
                if(key==0x08){ascii=55;} if(key==0x09){ascii=56;}
                if(key==0x0A){ascii=57;} if(key==0x0B){ascii=48;}
                if(key==0x1C){ascii=13;}  /* Enter */
                if(key==0x0E){ascii=8;}   /* Backspace */

                /* Dispatch keyboard to shell if open */
                if (ascii > 0 && desktop.shell_open) {
                    shell_handle_key(ascii);
                }
            }
        }

        /* ---- Mouse events ---- */
        mouse_get_pos(&mx, &my);

        if (mouse_left_clicked()) {
            /* Try WM first (windows consume clicks) */
            wm_consumed = wm_handle_mouse(mx, my,
                                           mouse_left_down(),
                                           mouse_left_clicked(),
                                           mouse_left_released());

            /* If WM didn't consume, check desktop icons */
            if (!wm_consumed) {
                desktop_handle_icon_click(mx, my);
            }
        }

        /* Handle ongoing drag */
        if (mouse_left_down()) {
            wm_handle_mouse(mx, my, 1, 0, 0);
        }
        if (mouse_left_released()) {
            wm_handle_mouse(mx, my, 0, 0, 1);
        }

        /* ---- Compose and present ---- */
        desktop_compose();
    }
}
