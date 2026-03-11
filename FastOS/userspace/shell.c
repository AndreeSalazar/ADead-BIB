/*
 * userspace/shell.c — FastOS Native Shell
 * FastOS v2.0
 *
 * Shell mínimo de FastOS. Corre como PID 2 (init = PID 1).
 * Sin bash. Sin dash. Sin historia de Unix que no necesitamos.
 * Solo lo que FastOS necesita.
 *
 * Compilar:  adb cc userspace/shell.c --target fastos -o shell.po
 * Ver steps: adb step userspace/shell.c
 */

#include <header_main.h>
#include <fastos.h>

/* ─── Buffer de línea de comandos ─── */
#define SHELL_LINE_MAX  256
#define SHELL_ARGS_MAX  16

typedef struct {
    char  line[SHELL_LINE_MAX];
    char *argv[SHELL_ARGS_MAX];
    int   argc;
} shell_cmd_t;

/* ─── Comandos builtin ─── */
static void cmd_help(void);
static void cmd_uname(void);
static void cmd_mem(void);
static int  cmd_run(const char *path);

/* ─── Estado del shell ─── */
static int shell_running = 1;

/* ─── Parsear línea en argc/argv ─── */
static void shell_parse(shell_cmd_t *cmd, const char *line) {
    cmd->argc = 0;
    int i = 0, j = 0;
    char ch;

    /* Copiar línea */
    while ((ch = line[i]) && i < SHELL_LINE_MAX - 1) {
        cmd->line[i] = ch;
        i++;
    }
    cmd->line[i] = '\0';

    /* Tokenizar por espacios */
    i = 0;
    while (cmd->line[i] && cmd->argc < SHELL_ARGS_MAX) {
        /* Saltar espacios */
        while (cmd->line[i] == ' ' || cmd->line[i] == '\t') i++;
        if (!cmd->line[i]) break;

        /* Inicio de token */
        cmd->argv[cmd->argc++] = &cmd->line[i];

        /* Avanzar hasta próximo espacio */
        while (cmd->line[i] && cmd->line[i] != ' ' && cmd->line[i] != '\t') i++;
        if (cmd->line[i]) {
            cmd->line[i] = '\0'; /* terminar token */
            i++;
        }
    }
    (void)j;
}

/* ─── Ejecutar comando ─── */
static void shell_exec(const shell_cmd_t *cmd) {
    if (cmd->argc == 0) return;

    const char *name = cmd->argv[0];

    if (name[0]=='h' && name[1]=='e' && name[2]=='l' &&
        name[3]=='p' && name[4]=='\0') {
        cmd_help();
    }
    else if (name[0]=='u' && name[1]=='n' && name[2]=='a' &&
             name[3]=='m' && name[4]=='e' && name[5]=='\0') {
        cmd_uname();
    }
    else if (name[0]=='m' && name[1]=='e' && name[2]=='m' &&
             name[3]=='\0') {
        cmd_mem();
    }
    else if (name[0]=='e' && name[1]=='x' && name[2]=='i' &&
             name[3]=='t' && name[4]=='\0') {
        printf("[shell] Exiting FastOS...\n");
        shell_running = 0;
    }
    else {
        /* Intentar ejecutar como binario .Po */
        int result = cmd_run(name);
        if (result != 0) {
            printf("[shell] Command not found: %s\n", name);
        }
    }
}

/* ─── Builtins ─── */
static void cmd_help(void) {
    printf("\nFastOS Shell v2.0 💀🦈\n");
    printf("Commands:\n");
    printf("  help   — this message\n");
    printf("  uname  — system info\n");
    printf("  mem    — memory status\n");
    printf("  exit   — halt system\n");
    printf("  <path> — run a .Po binary\n\n");
}

static void cmd_uname(void) {
    printf("FastOS v2.0 — ~150KB — x86-64\n");
    printf("Compiler: ADead-BIB (GPLv2)\n");
    printf("Format: .Po (24 bytes header)\n");
    printf("Security: Binary Guardian (math, no heuristics)\n");
}

static void cmd_mem(void) {
    /* En producción: leer de kernel/memory */
    printf("[mem] Memory status: (query kernel/memory)\n");
}

static int cmd_run(const char *path) {
    /* En producción: vfs_open(path) → bg_preexec_gate() → exec */
    (void)path;
    return -1; /* "not found" en demo */
}

/* ─── Leer línea desde teclado ─── */
static int shell_readline(char *buf, int max) {
    int i = 0;
    char c;
    while (i < max - 1) {
        /* En producción: fastos_read_key() */
        /* Demo: scanf para host testing */
        if (scanf("%c", &c) != 1) break;
        if (c == '\n') break;
        buf[i++] = c;
    }
    buf[i] = '\0';
    return i;
}

/* ─── Entry point del shell ─── */
void shell_start(void) {
    printf("\nFastOS v2.0 💀🦈🇵🇪\n");
    printf("Type 'help' for commands.\n\n");

    char line[SHELL_LINE_MAX];
    shell_cmd_t cmd;

    while (shell_running) {
        printf("fastos> ");
        shell_readline(line, SHELL_LINE_MAX);
        shell_parse(&cmd, line);
        shell_exec(&cmd);
    }
}
