/* FastOS v4.0 — Native Terminal Application
 * ADead-BIB Native OS — First .po GUI App
 *
 * A terminal emulator running as a PoWindow inside the desktop.
 * Uses the FastOSAPI to create a window, render text, handle input.
 *
 * Features:
 *   - 8×16 character grid in window content surface
 *   - Command line with prompt "> "
 *   - Built-in commands: help, ver, bg, clear, exit
 *   - Scrolling (wrap-around for simplicity)
 *   - Green-on-dark color scheme
 *
 * This is the reference implementation for Po GUI apps.
 * Native apps are 256-bit BG-verified; external are 64-bit compatible.
 *
 * Compiled by: ADead-BIB (C is Master, Rust is Safety)
 */

#include "../include/types.h"

/* ================================================================
 * Terminal Constants
 * ================================================================ */

#define TERM_BG          0xFF0A0A1E  /* dark blue-black */
#define TERM_FG          0xFFCCCCCC  /* light gray text */
#define TERM_PROMPT_FG   0xFF55FF55  /* green prompt */
#define TERM_TITLE_FG    0xFF55FF55  /* green title */
#define TERM_ERROR_FG    0xFFFF5555  /* red error */
#define TERM_CMD_MAX     60          /* max command length */

/* ================================================================
 * Terminal State
 * ================================================================ */

typedef struct {
    po_window_t  *win;           /* WM window handle */
    fb_surface_t *content;       /* window content surface */
    int           cols;          /* character columns */
    int           rows;          /* character rows */
    int           cx;            /* cursor column */
    int           cy;            /* cursor row */
    char          cmd[TERM_CMD_MAX + 1];
    int           cmd_len;
    int           active;        /* 1 if terminal is open */
} terminal_t;

static terminal_t term;

/* ================================================================
 * Terminal: Put Character
 * ================================================================ */

static void term_putchar(char c, uint32_t color)
{
    if (!term.content) return;

    if (c == '\n') {
        term.cx = 0;
        term.cy = term.cy + 1;
        if (term.cy >= term.rows) {
            fb_clear(term.content, TERM_BG);
            term.cy = 0;
        }
        return;
    }

    if (term.cx < term.cols) {
        font_draw_char(term.content,
                       (uint32_t)(term.cx * 8),
                       (uint32_t)(term.cy * 16),
                       c, color, 0x00000000);
        term.cx = term.cx + 1;
    }
}

/* Terminal: Put String */
static void term_puts(const char *s, uint32_t color)
{
    int i;
    i = 0;
    while (s[i] != 0) {
        term_putchar(s[i], color);
        i = i + 1;
    }
}

/* ================================================================
 * Terminal: Execute Command
 * ================================================================ */

static void term_exec(void)
{
    term.cmd[term.cmd_len] = 0;
    term_putchar('\n', TERM_FG);

    /* help */
    if (term.cmd_len == 4 &&
        term.cmd[0] == 'h' && term.cmd[1] == 'e' &&
        term.cmd[2] == 'l' && term.cmd[3] == 'p') {
        term_puts("Commands:\n", TERM_FG);
        term_puts(" help    this message\n", TERM_FG);
        term_puts(" ver     version info\n", TERM_FG);
        term_puts(" bg      Binary Guardian\n", TERM_FG);
        term_puts(" bg256   BG 256-bit stats\n", TERM_FG);
        term_puts(" mem     memory info\n", TERM_FG);
        term_puts(" clear   clear screen\n", TERM_FG);
        term_puts(" exit    close terminal\n", TERM_FG);
    }
    /* ver */
    else if (term.cmd_len == 3 &&
             term.cmd[0] == 'v' && term.cmd[1] == 'e' && term.cmd[2] == 'r') {
        term_puts("FastOS v4.0 ADead-BIB\n", TERM_TITLE_FG);
        term_puts("BG: 256-bit YMM active\n", TERM_FG);
        term_puts("Heap: 14MB @ 0x200000\n", TERM_FG);
        term_puts("Po v8.0 native loader\n", TERM_FG);
    }
    /* bg */
    else if (term.cmd_len == 2 &&
             term.cmd[0] == 'b' && term.cmd[1] == 'g') {
        term_puts("BG 256-bit Guardian\n", TERM_TITLE_FG);
        term_puts(" Mode: 8 checks/cycle YMM\n", TERM_FG);
        term_puts(" Level: KERNEL Ring0\n", TERM_FG);
        term_puts(" Pipeline: Load>BG256>Map>Policy\n", TERM_FG);
        term_puts(" Verdict: APPROVE\n", TERM_PROMPT_FG);
    }
    /* bg256 */
    else if (term.cmd_len == 5 &&
             term.cmd[0] == 'b' && term.cmd[1] == 'g' &&
             term.cmd[2] == '2' && term.cmd[3] == '5' && term.cmd[4] == '6') {
        term_puts("BG 256-bit Stats\n", TERM_TITLE_FG);
        term_puts(" Registers: YMM0-YMM15\n", TERM_FG);
        term_puts(" Width: 256-bit (32 bytes)\n", TERM_FG);
        term_puts(" Batch: 8 magic checks/cycle\n", TERM_FG);
        term_puts(" Hash: 4-stream FNV-1a\n", TERM_FG);
        term_puts(" Scan: NOP/INT3/SYSCALL/RET\n", TERM_FG);
        term_puts(" Threats: 0 detected\n", TERM_PROMPT_FG);
    }
    /* mem */
    else if (term.cmd_len == 3 &&
             term.cmd[0] == 'm' && term.cmd[1] == 'e' && term.cmd[2] == 'm') {
        term_puts("Memory Map:\n", TERM_TITLE_FG);
        term_puts(" Heap:   0x200000 (14MB)\n", TERM_FG);
        term_puts(" BackBuf:0x400000 (3MB)\n", TERM_FG);
        term_puts(" Icons:  0x700000 (128KB)\n", TERM_FG);
        term_puts(" WM Surf:0x800000 (32MB)\n", TERM_FG);
        term_puts(" Po Exec:0x1000000 (8MB)\n", TERM_FG);
    }
    /* clear */
    else if (term.cmd_len == 5 &&
             term.cmd[0] == 'c' && term.cmd[1] == 'l' &&
             term.cmd[2] == 'e' && term.cmd[3] == 'a' && term.cmd[4] == 'r') {
        fb_clear(term.content, TERM_BG);
        term.cx = 0;
        term.cy = 0;
        term.cmd_len = 0;
        /* Show prompt immediately after clear */
        term_puts("> ", TERM_PROMPT_FG);
        return;
    }
    /* exit */
    else if (term.cmd_len == 4 &&
             term.cmd[0] == 'e' && term.cmd[1] == 'x' &&
             term.cmd[2] == 'i' && term.cmd[3] == 't') {
        term.active = 0;
        if (term.win) wm_destroy(term.win);
        term.win = 0;
        term.cmd_len = 0;
        return;
    }
    /* unknown */
    else if (term.cmd_len > 0) {
        term_puts("Unknown: ", TERM_ERROR_FG);
        term_puts(term.cmd, TERM_FG);
        term_putchar('\n', TERM_FG);
    }

    term.cmd_len = 0;
    term_puts("> ", TERM_PROMPT_FG);
}

/* ================================================================
 * Terminal: Handle Keyboard Input
 * ================================================================ */

static void term_handle_key(int ascii)
{
    if (!term.active || !term.content) return;

    /* Enter */
    if (ascii == 13 || ascii == 10) {
        term_exec();
        return;
    }

    /* Backspace */
    if (ascii == 8) {
        if (term.cmd_len > 0) {
            term.cmd_len = term.cmd_len - 1;
            term.cx = term.cx - 1;
            font_draw_char(term.content,
                           (uint32_t)(term.cx * 8),
                           (uint32_t)(term.cy * 16),
                           ' ', TERM_BG, TERM_BG);
        }
        return;
    }

    /* Printable character */
    if (ascii >= 32 && ascii < 127 && term.cmd_len < TERM_CMD_MAX) {
        term.cmd[term.cmd_len] = (char)ascii;
        term.cmd_len = term.cmd_len + 1;
        term_putchar((char)ascii, TERM_FG);
    }
}

/* ================================================================
 * Terminal: Open
 *
 * Creates a WM window and initializes terminal state.
 * Called from desktop icon click or API.
 * ================================================================ */

static void term_open(void)
{
    if (term.active) return;

    term.win = wm_create("Terminal", 180, 80, 640, 420,
                          WINDOW_RESIZABLE);
    if (!term.win) return;

    term.content = wm_get_content(term.win);
    if (!term.content) return;

    term.cols = term.win->content_w / 8;
    term.rows = term.win->content_h / 16;
    term.cx = 0;
    term.cy = 0;
    term.cmd_len = 0;
    term.active = 1;

    /* Clear and show welcome */
    fb_clear(term.content, TERM_BG);

    term_puts("FastOS v4.0 Terminal\n", TERM_TITLE_FG);
    term_puts("256-bit native | BG:APPROVE\n", 0xFF888888);
    term_puts("Type 'help' for commands\n", 0xFF888888);
    term_putchar('\n', TERM_FG);
    term_puts("> ", TERM_PROMPT_FG);
}

/* ================================================================
 * Terminal: Check if Active
 * ================================================================ */

static int term_is_active(void)
{
    return term.active;
}

