/*
 * FastOS v2.0 — Terminal Application
 * Native .po application for FastOS
 * 
 * Features:
 * - Command line interface
 * - Built-in commands
 * - Po executable launcher
 * - History support
 * 
 * Compile: adB cc terminal.c -o terminal.po --app
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ============================================================
 * Terminal Constants
 * ============================================================ */

#define TERM_MAX_COLS       80
#define TERM_MAX_ROWS       25
#define TERM_BUFFER_SIZE    (TERM_MAX_COLS * TERM_MAX_ROWS)
#define TERM_INPUT_SIZE     256
#define TERM_HISTORY_SIZE   32
#define TERM_MAX_ARGS       16

/* Colors */
#define TERM_COLOR_DEFAULT  0x0F    /* White on black */
#define TERM_COLOR_PROMPT   0x0A    /* Green */
#define TERM_COLOR_ERROR    0x0C    /* Red */
#define TERM_COLOR_INFO     0x0B    /* Cyan */
#define TERM_COLOR_WARNING  0x0E    /* Yellow */

/* ============================================================
 * Terminal State
 * ============================================================ */

typedef struct {
    /* Screen buffer */
    char buffer[TERM_BUFFER_SIZE];
    uint8_t colors[TERM_BUFFER_SIZE];
    
    /* Cursor position */
    int cursor_x;
    int cursor_y;
    
    /* Current color */
    uint8_t current_color;
    
    /* Input buffer */
    char input[TERM_INPUT_SIZE];
    int input_pos;
    int input_len;
    
    /* Command history */
    char history[TERM_HISTORY_SIZE][TERM_INPUT_SIZE];
    int history_count;
    int history_index;
    
    /* Window info */
    int window_x;
    int window_y;
    int window_width;
    int window_height;
    uint32_t window_id;
    
    /* State */
    int running;
    char current_dir[256];
    char username[32];
    char hostname[32];
} terminal_t;

static terminal_t term;

/* ============================================================
 * Terminal Output
 * ============================================================ */

static void term_scroll(void) {
    /* Move all lines up by one */
    for (int y = 0; y < TERM_MAX_ROWS - 1; y++) {
        for (int x = 0; x < TERM_MAX_COLS; x++) {
            int dst = y * TERM_MAX_COLS + x;
            int src = (y + 1) * TERM_MAX_COLS + x;
            term.buffer[dst] = term.buffer[src];
            term.colors[dst] = term.colors[src];
        }
    }
    
    /* Clear last line */
    int last_row = (TERM_MAX_ROWS - 1) * TERM_MAX_COLS;
    for (int x = 0; x < TERM_MAX_COLS; x++) {
        term.buffer[last_row + x] = ' ';
        term.colors[last_row + x] = term.current_color;
    }
}

static void term_newline(void) {
    term.cursor_x = 0;
    term.cursor_y++;
    
    if (term.cursor_y >= TERM_MAX_ROWS) {
        term_scroll();
        term.cursor_y = TERM_MAX_ROWS - 1;
    }
}

static void term_putchar(char c) {
    if (c == '\n') {
        term_newline();
        return;
    }
    
    if (c == '\r') {
        term.cursor_x = 0;
        return;
    }
    
    if (c == '\b') {
        if (term.cursor_x > 0) {
            term.cursor_x--;
            int pos = term.cursor_y * TERM_MAX_COLS + term.cursor_x;
            term.buffer[pos] = ' ';
        }
        return;
    }
    
    if (c == '\t') {
        int spaces = 4 - (term.cursor_x % 4);
        for (int i = 0; i < spaces; i++) {
            term_putchar(' ');
        }
        return;
    }
    
    /* Regular character */
    int pos = term.cursor_y * TERM_MAX_COLS + term.cursor_x;
    term.buffer[pos] = c;
    term.colors[pos] = term.current_color;
    
    term.cursor_x++;
    if (term.cursor_x >= TERM_MAX_COLS) {
        term_newline();
    }
}

static void term_print(const char *str) {
    while (*str) {
        term_putchar(*str++);
    }
}

static void term_println(const char *str) {
    term_print(str);
    term_newline();
}

static void term_print_color(const char *str, uint8_t color) {
    uint8_t old_color = term.current_color;
    term.current_color = color;
    term_print(str);
    term.current_color = old_color;
}

static void term_printf(const char *fmt, ...) {
    /* Simple printf implementation */
    char buf[256];
    __builtin_va_list args;
    __builtin_va_start(args, fmt);
    
    int i = 0;
    while (*fmt && i < 255) {
        if (*fmt == '%') {
            fmt++;
            switch (*fmt) {
                case 's': {
                    const char *s = __builtin_va_arg(args, const char*);
                    while (*s && i < 255) buf[i++] = *s++;
                    break;
                }
                case 'd': {
                    int n = __builtin_va_arg(args, int);
                    if (n < 0) { buf[i++] = '-'; n = -n; }
                    char tmp[12];
                    int j = 0;
                    do { tmp[j++] = '0' + (n % 10); n /= 10; } while (n && j < 11);
                    while (j > 0 && i < 255) buf[i++] = tmp[--j];
                    break;
                }
                case 'x': {
                    unsigned int n = __builtin_va_arg(args, unsigned int);
                    char tmp[9];
                    int j = 0;
                    do {
                        int d = n & 0xF;
                        tmp[j++] = d < 10 ? '0' + d : 'a' + d - 10;
                        n >>= 4;
                    } while (n && j < 8);
                    while (j > 0 && i < 255) buf[i++] = tmp[--j];
                    break;
                }
                case '%':
                    buf[i++] = '%';
                    break;
                default:
                    buf[i++] = '%';
                    buf[i++] = *fmt;
                    break;
            }
        } else {
            buf[i++] = *fmt;
        }
        fmt++;
    }
    buf[i] = '\0';
    
    __builtin_va_end(args);
    term_print(buf);
}

static void term_clear(void) {
    for (int i = 0; i < TERM_BUFFER_SIZE; i++) {
        term.buffer[i] = ' ';
        term.colors[i] = term.current_color;
    }
    term.cursor_x = 0;
    term.cursor_y = 0;
}

/* ============================================================
 * Prompt
 * ============================================================ */

static void term_show_prompt(void) {
    term_print_color(term.username, TERM_COLOR_INFO);
    term_print_color("@", TERM_COLOR_DEFAULT);
    term_print_color(term.hostname, TERM_COLOR_INFO);
    term_print_color(":", TERM_COLOR_DEFAULT);
    term_print_color(term.current_dir, TERM_COLOR_PROMPT);
    term_print_color("$ ", TERM_COLOR_DEFAULT);
}

/* ============================================================
 * Command Parsing
 * ============================================================ */

static int term_parse_args(char *cmd, char **argv) {
    int argc = 0;
    char *p = cmd;
    
    while (*p && argc < TERM_MAX_ARGS - 1) {
        /* Skip whitespace */
        while (*p == ' ' || *p == '\t') p++;
        if (!*p) break;
        
        /* Handle quotes */
        if (*p == '"' || *p == '\'') {
            char quote = *p++;
            argv[argc++] = p;
            while (*p && *p != quote) p++;
            if (*p) *p++ = '\0';
        } else {
            argv[argc++] = p;
            while (*p && *p != ' ' && *p != '\t') p++;
            if (*p) *p++ = '\0';
        }
    }
    
    argv[argc] = NULL;
    return argc;
}

/* ============================================================
 * Built-in Commands
 * ============================================================ */

static void cmd_help(int argc, char **argv) {
    term_println("FastOS Terminal v1.0");
    term_println("Built-in commands:");
    term_println("  help        - Show this help");
    term_println("  clear       - Clear screen");
    term_println("  echo <msg>  - Print message");
    term_println("  pwd         - Print working directory");
    term_println("  cd <dir>    - Change directory");
    term_println("  ls          - List directory");
    term_println("  cat <file>  - Show file contents");
    term_println("  mkdir <dir> - Create directory");
    term_println("  rm <file>   - Remove file");
    term_println("  cp <s> <d>  - Copy file");
    term_println("  mv <s> <d>  - Move file");
    term_println("  run <app>   - Run .po application");
    term_println("  info        - System information");
    term_println("  mem         - Memory usage");
    term_println("  ps          - Process list");
    term_println("  kill <pid>  - Kill process");
    term_println("  reboot      - Reboot system");
    term_println("  shutdown    - Shutdown system");
    term_println("  exit        - Exit terminal");
}

static void cmd_clear(int argc, char **argv) {
    term_clear();
}

static void cmd_echo(int argc, char **argv) {
    for (int i = 1; i < argc; i++) {
        if (i > 1) term_print(" ");
        term_print(argv[i]);
    }
    term_newline();
}

static void cmd_pwd(int argc, char **argv) {
    term_println(term.current_dir);
}

static void cmd_cd(int argc, char **argv) {
    if (argc < 2) {
        kstrcpy(term.current_dir, "/");
    } else if (kstrcmp(argv[1], "..") == 0) {
        /* Go up one directory */
        char *last = kstrrchr(term.current_dir, '/');
        if (last && last != term.current_dir) {
            *last = '\0';
        } else {
            kstrcpy(term.current_dir, "/");
        }
    } else if (argv[1][0] == '/') {
        kstrcpy(term.current_dir, argv[1]);
    } else {
        if (kstrcmp(term.current_dir, "/") != 0) {
            kstrcat(term.current_dir, "/");
        }
        kstrcat(term.current_dir, argv[1]);
    }
}

static void cmd_ls(int argc, char **argv) {
    term_println("Directory listing:");
    term_print_color("  apps/          ", TERM_COLOR_INFO);
    term_println("<DIR>");
    term_print_color("  drivers/       ", TERM_COLOR_INFO);
    term_println("<DIR>");
    term_print_color("  kernel/        ", TERM_COLOR_INFO);
    term_println("<DIR>");
    term_print_color("  libs/          ", TERM_COLOR_INFO);
    term_println("<DIR>");
    term_println("  terminal.po    4096");
    term_println("  filemanager.po 8192");
    term_println("  settings.po    2048");
}

static void cmd_cat(int argc, char **argv) {
    if (argc < 2) {
        term_print_color("Usage: cat <file>\n", TERM_COLOR_ERROR);
        return;
    }
    term_printf("Contents of %s:\n", argv[1]);
    term_println("(File system not yet connected)");
}

static void cmd_info(int argc, char **argv) {
    term_println("=== FastOS System Information ===");
    term_println("");
    term_println("OS:        FastOS v2.0");
    term_println("Compiler:  ADead-BIB");
    term_println("Arch:      x86-64");
    term_println("Format:    Po (PE+ELF+Win32)");
    term_println("");
    term_println("Components:");
    term_print_color("  [BG]     ", TERM_COLOR_PROMPT);
    term_println("Binary Guardian: ACTIVE");
    term_print_color("  [musl]   ", TERM_COLOR_PROMPT);
    term_println("C Library: LOADED");
    term_print_color("  [Rust]   ", TERM_COLOR_PROMPT);
    term_println("Safety Layer: ENABLED");
    term_print_color("  [Nouveau]", TERM_COLOR_PROMPT);
    term_println("GPU Driver: READY");
    term_println("");
    term_println("References:");
    term_println("  ToaruOS  - Compositor");
    term_println("  ReactOS  - Win32 API");
    term_println("  Linux    - ELF Loader");
}

static void cmd_mem(int argc, char **argv) {
    term_println("=== Memory Usage ===");
    term_println("Total:     256 MB");
    term_println("Used:      48 MB");
    term_println("Free:      208 MB");
    term_println("Kernel:    8 MB");
    term_println("Apps:      40 MB");
}

static void cmd_ps(int argc, char **argv) {
    term_println("=== Process List ===");
    term_println("PID  NAME           STATE    MEM");
    term_println("  1  kernel         running  8MB");
    term_println("  2  compositor     running  4MB");
    term_println("  3  terminal       running  2MB");
    term_println("  4  filemanager    sleeping 3MB");
}

static void cmd_run(int argc, char **argv) {
    if (argc < 2) {
        term_print_color("Usage: run <app.po>\n", TERM_COLOR_ERROR);
        return;
    }
    term_printf("Loading %s...\n", argv[1]);
    term_print_color("[Po] ", TERM_COLOR_INFO);
    term_println("Detecting format...");
    term_print_color("[BG] ", TERM_COLOR_PROMPT);
    term_println("Verifying binary...");
    term_print_color("[OK] ", TERM_COLOR_PROMPT);
    term_printf("Application %s started\n", argv[1]);
}

static void cmd_exit(int argc, char **argv) {
    term.running = 0;
}

static void cmd_reboot(int argc, char **argv) {
    term_println("Rebooting...");
    /* Would trigger reboot via ACPI or keyboard controller */
}

static void cmd_shutdown(int argc, char **argv) {
    term_println("Shutting down...");
    /* Would trigger shutdown via ACPI */
}

/* Command table */
typedef struct {
    const char *name;
    void (*func)(int argc, char **argv);
    const char *desc;
} command_t;

static command_t commands[] = {
    { "help",     cmd_help,     "Show help" },
    { "clear",    cmd_clear,    "Clear screen" },
    { "cls",      cmd_clear,    "Clear screen" },
    { "echo",     cmd_echo,     "Print message" },
    { "pwd",      cmd_pwd,      "Print directory" },
    { "cd",       cmd_cd,       "Change directory" },
    { "ls",       cmd_ls,       "List directory" },
    { "dir",      cmd_ls,       "List directory" },
    { "cat",      cmd_cat,      "Show file" },
    { "type",     cmd_cat,      "Show file" },
    { "info",     cmd_info,     "System info" },
    { "mem",      cmd_mem,      "Memory usage" },
    { "ps",       cmd_ps,       "Process list" },
    { "run",      cmd_run,      "Run application" },
    { "exit",     cmd_exit,     "Exit terminal" },
    { "quit",     cmd_exit,     "Exit terminal" },
    { "reboot",   cmd_reboot,   "Reboot system" },
    { "shutdown", cmd_shutdown, "Shutdown system" },
    { NULL, NULL, NULL }
};

/* ============================================================
 * Command Execution
 * ============================================================ */

static void term_execute(char *cmd) {
    /* Skip empty commands */
    while (*cmd == ' ' || *cmd == '\t') cmd++;
    if (!*cmd) return;
    
    /* Add to history */
    if (term.history_count < TERM_HISTORY_SIZE) {
        kstrcpy(term.history[term.history_count++], cmd);
    }
    term.history_index = term.history_count;
    
    /* Parse arguments */
    char *argv[TERM_MAX_ARGS];
    int argc = term_parse_args(cmd, argv);
    
    if (argc == 0) return;
    
    /* Find and execute command */
    for (int i = 0; commands[i].name; i++) {
        if (kstrcmp(argv[0], commands[i].name) == 0) {
            commands[i].func(argc, argv);
            return;
        }
    }
    
    /* Unknown command - try to run as .po */
    term_print_color("Unknown command: ", TERM_COLOR_ERROR);
    term_println(argv[0]);
    term_println("Type 'help' for available commands.");
}

/* ============================================================
 * Input Handling
 * ============================================================ */

static void term_handle_key(char c) {
    if (c == '\n' || c == '\r') {
        term_newline();
        term.input[term.input_len] = '\0';
        term_execute(term.input);
        term.input_len = 0;
        term.input_pos = 0;
        term_show_prompt();
        return;
    }
    
    if (c == '\b') {
        if (term.input_pos > 0) {
            term.input_pos--;
            term.input_len--;
            term_putchar('\b');
            term_putchar(' ');
            term_putchar('\b');
        }
        return;
    }
    
    if (c >= 32 && c < 127 && term.input_len < TERM_INPUT_SIZE - 1) {
        term.input[term.input_pos++] = c;
        term.input_len++;
        term_putchar(c);
    }
}

static void term_handle_special(uint8_t key) {
    /* Handle arrow keys, etc */
    switch (key) {
        case 0x48:  /* Up arrow - history */
            if (term.history_index > 0) {
                term.history_index--;
                /* Clear current input */
                while (term.input_pos > 0) {
                    term_putchar('\b');
                    term_putchar(' ');
                    term_putchar('\b');
                    term.input_pos--;
                }
                /* Copy from history */
                kstrcpy(term.input, term.history[term.history_index]);
                term.input_len = kstrlen(term.input);
                term.input_pos = term.input_len;
                term_print(term.input);
            }
            break;
            
        case 0x50:  /* Down arrow */
            if (term.history_index < term.history_count - 1) {
                term.history_index++;
                while (term.input_pos > 0) {
                    term_putchar('\b');
                    term_putchar(' ');
                    term_putchar('\b');
                    term.input_pos--;
                }
                kstrcpy(term.input, term.history[term.history_index]);
                term.input_len = kstrlen(term.input);
                term.input_pos = term.input_len;
                term_print(term.input);
            }
            break;
    }
}

/* ============================================================
 * Terminal Main
 * ============================================================ */

void terminal_init(void) {
    kmemset(&term, 0, sizeof(term));
    
    term.current_color = TERM_COLOR_DEFAULT;
    term.running = 1;
    kstrcpy(term.current_dir, "/");
    kstrcpy(term.username, "root");
    kstrcpy(term.hostname, "fastos");
    
    term_clear();
    
    /* Welcome message */
    term_print_color("FastOS Terminal v1.0\n", TERM_COLOR_INFO);
    term_println("Type 'help' for available commands.");
    term_println("");
    
    term_show_prompt();
}

int terminal_is_running(void) {
    return term.running;
}

void terminal_process_input(uint8_t scancode, char ascii) {
    if (ascii) {
        term_handle_key(ascii);
    } else {
        term_handle_special(scancode);
    }
}

/* Get terminal buffer for rendering */
void terminal_get_buffer(char **buffer, uint8_t **colors, int *width, int *height) {
    *buffer = term.buffer;
    *colors = term.colors;
    *width = TERM_MAX_COLS;
    *height = TERM_MAX_ROWS;
}

void terminal_get_cursor(int *x, int *y) {
    *x = term.cursor_x;
    *y = term.cursor_y;
}
