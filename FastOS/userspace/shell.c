/*
 * userspace/shell.c — FastOS Native Shell
 * FastOS v2.0
 *
 * Shell mínimo de FastOS. Corre como PID 2 (init = PID 1).
 * Sin bash. Sin scanf. Solo kprintf + keyboard_getchar().
 *
 * Comandos: help, uname, mem, sched, bg, clear, halt
 *
 * Compilar:  adb step userspace/shell.c
 */

#include "../include/kernel.h"

/* ─── Declaraciones de subsistemas ─────────────────────── */
extern char keyboard_getchar(void);
extern void keyboard_init(void);

/* ─── Constantes del shell ──────────────────────────────── */
#define SHELL_LINE_MAX  256
#define SHELL_ARGS_MAX  16
#define SHELL_PROMPT    "fastos> "

/* Colores VGA */
#define COLOR_PROMPT    VGA_COLOR(VGA_LIGHT_CYAN,   VGA_BLUE)
#define COLOR_OK        VGA_COLOR(VGA_LIGHT_GREEN,  VGA_BLUE)
#define COLOR_ERROR     VGA_COLOR(VGA_LIGHT_RED,    VGA_BLUE)
#define COLOR_INFO      VGA_COLOR(VGA_YELLOW,       VGA_BLUE)
#define COLOR_NORMAL    VGA_COLOR(VGA_WHITE,        VGA_BLUE)
#define COLOR_ACCENT    VGA_COLOR(VGA_LIGHT_MAGENTA,VGA_BLUE)

/* ─── Tipo comando ──────────────────────────────────────── */
typedef struct {
    char  line[SHELL_LINE_MAX];
    char *argv[SHELL_ARGS_MAX];
    int   argc;
} shell_cmd_t;

static int shell_running = 1;

/* ─── Comparación de strings mínima ────────────────────── */
static int str_eq(const char *a, const char *b) {
    while (*a && *b) {
        if (*a != *b) return 0;
        a++; b++;
    }
    return *a == 0 && *b == 0;
}

/* ─── Leer línea desde teclado PS/2 ────────────────────── */
static int shell_readline(char *buf, int max) {
    int i = 0;
    while (i < max - 1) {
        char c = keyboard_getchar();

        if (c == '\n' || c == '\r') {
            /* Enter: terminar línea */
            term_putchar('\n');
            break;
        } else if (c == 8 || c == 127) {
            /* Backspace */
            if (i > 0) {
                i--;
                /* Borrar último carácter visible: retroceder, espacio, retroceder */
                term_putchar(8);
                term_putchar(' ');
                term_putchar(8);
            }
        } else if (c >= 32) {
            /* Carácter imprimible */
            buf[i++] = c;
            term_putchar(c);   /* echo */
        }
        /* Ignorar caracteres de control desconocidos */
    }
    buf[i] = '\0';
    return i;
}

/* ─── Tokenizar línea en argc/argv ─────────────────────── */
static void shell_parse(shell_cmd_t *cmd, const char *line) {
    cmd->argc = 0;
    /* Copiar al buffer interno */
    int i = 0;
    while (line[i] && i < SHELL_LINE_MAX - 1) {
        cmd->line[i] = line[i];
        i++;
    }
    cmd->line[i] = '\0';

    /* Tokenizar por espacios */
    i = 0;
    while (cmd->line[i] && cmd->argc < SHELL_ARGS_MAX) {
        /* Saltar espacios */
        while (cmd->line[i] == ' ' || cmd->line[i] == '\t') i++;
        if (!cmd->line[i]) break;
        /* Token start */
        cmd->argv[cmd->argc++] = &cmd->line[i];
        /* Avanzar hasta próximo espacio */
        while (cmd->line[i] && cmd->line[i] != ' ' && cmd->line[i] != '\t') i++;
        if (cmd->line[i]) { cmd->line[i++] = '\0'; }
    }
}

/* ═══════════════════════════════════════════════
 * Comandos Builtin
 * ═══════════════════════════════════════════════ */

static void cmd_help(void) {
    term_write_color("\nFastOS Shell v2.0 — Comandos disponibles:\n\n", COLOR_INFO);
    term_write_color("  help   ", COLOR_ACCENT);  term_write("— este mensaje\n");
    term_write_color("  uname  ", COLOR_ACCENT);  term_write("— informacion del sistema\n");
    term_write_color("  mem    ", COLOR_ACCENT);  term_write("— estado de la memoria\n");
    term_write_color("  sched  ", COLOR_ACCENT);  term_write("— lista de procesos\n");
    term_write_color("  bg     ", COLOR_ACCENT);  term_write("— Binary Guardian status\n");
    term_write_color("  clear  ", COLOR_ACCENT);  term_write("— limpiar pantalla\n");
    term_write_color("  halt   ", COLOR_ACCENT);  term_write("— detener el sistema\n");
    term_write_color("  <path> ", COLOR_ACCENT);  term_write("— ejecutar binario .Po\n\n");
}

static void cmd_uname(void) {
    term_write_color("FastOS v2.0\n", COLOR_OK);
    term_write("  Arch:     x86-64 Long Mode\n");
    term_write("  Compiler: ADead-BIB (Native, 7-phase pipeline)\n");
    term_write("  Format:   .Po (FastOS Portable Object, 24B header)\n");
    term_write("  Size:     ~150KB loaded\n");
    term_write("  Security: Binary Guardian (4 niveles, matematica pura)\n");
    term_write("  Origin:   Made in Peru\n\n");
}

static void cmd_mem(void) {
    term_write_color("[MEM] Estado de memoria:\n", COLOR_INFO);
    term_write("  Heap inicio: 0x200000 (2MB)\n");
    term_write("  Heap fin:    0xA00000 (8MB tope)\n");
    term_write("  Mapa E820:   0x20000 (128KB)\n");
    /* En producción: llamar a heap_dump() */
    heap_dump();
}

extern void scheduler_list(void);
static void cmd_sched(void) {
    term_write_color("[SCHED] Procesos activos:\n", COLOR_INFO);
    scheduler_list();
}

extern uint32_t bg_get_violations(void);
extern uint32_t bg_get_verified(void);

static void cmd_bg(void) {
    term_write_color("[BG] Binary Guardian Status:\n", COLOR_INFO);
    uint32_t violations = bg_get_violations();
    uint32_t verified   = bg_get_verified();
    kprintf("  Verificados:  %u\n", verified);
    kprintf("  Violaciones:  %u\n", violations);
    if (violations == 0) {
        term_write_color("  Estado: CLEAN — ningun intento de evasion detectado\n\n", COLOR_OK);
    } else {
        term_write_color("  Estado: ALERTA — violaciones detectadas!\n\n", COLOR_ERROR);
    }
}

static void cmd_clear(void) {
    term_init();   /* Limpiar pantalla y resetear cursor */
}

static void cmd_halt(void) {
    term_write_color("\n[HALT] Deteniendo FastOS...\n", COLOR_ERROR);
    term_write_color("       Es seguro apagar el sistema.\n\n", COLOR_NORMAL);
    shell_running = 0;
    /* La CPU se detiene cuando kernel_main() sale del loop de init */
}

static int cmd_run(const char *path) {
    /*
     * En producción:
     *   1. vfs_open(path) → leer binario .Po
     *   2. bg_preexec_gate() → verificar con BG niveles 1-4
     *   3. process_create() → nuevo proceso con security_level
     *   4. scheduler (yield) → ejecutar
     *
     * Por ahora: no tenemos VFS inicializado en la demo.
     */
    (void)path;
    return -1;
}

/* ─── Dispatcher de comandos ────────────────────────────── */
static void shell_exec(const shell_cmd_t *cmd) {
    if (cmd->argc == 0) return;
    const char *name = cmd->argv[0];

    if      (str_eq(name, "help"))  cmd_help();
    else if (str_eq(name, "uname")) cmd_uname();
    else if (str_eq(name, "mem"))   cmd_mem();
    else if (str_eq(name, "sched")) cmd_sched();
    else if (str_eq(name, "bg"))    cmd_bg();
    else if (str_eq(name, "clear")) cmd_clear();
    else if (str_eq(name, "halt"))  cmd_halt();
    else if (str_eq(name, "exit"))  cmd_halt();  /* alias */
    else {
        /* Intentar como binario .Po */
        int r = cmd_run(name);
        if (r != 0) {
            term_write_color("Error: comando no encontrado: ", COLOR_ERROR);
            term_write(name);
            term_write("\n       Escribe 'help' para ver los comandos.\n\n");
        }
    }
}

/* ═══════════════════════════════════════════════
 * shell_start() — Entry point (PID 2)
 * Llamado por init_main(). No retorna nunca.
 * ═══════════════════════════════════════════════ */
void shell_start(void) {
    /* Banner del shell */
    term_write_color("\n", COLOR_NORMAL);
    term_write_color("  +-------------------------------------------------+\n", COLOR_ACCENT);
    term_write_color("  |  FastOS Shell v2.0  --  Made in Peru  💀🦈🇵🇪  |\n", COLOR_ACCENT);
    term_write_color("  |  Escribe 'help' para ver los comandos.          |\n", COLOR_NORMAL);
    term_write_color("  +-------------------------------------------------+\n\n", COLOR_ACCENT);

    char line[SHELL_LINE_MAX];
    shell_cmd_t cmd;

    while (shell_running) {
        /* Prompt en color cyan */
        term_write_color(SHELL_PROMPT, COLOR_PROMPT);
        /* Leer línea con echo y backspace */
        shell_readline(line, SHELL_LINE_MAX);
        /* Parsear y ejecutar */
        shell_parse(&cmd, line);
        shell_exec(&cmd);
    }

    /* Si salimos del loop → halt */
    asm volatile("cli; hlt");
}
