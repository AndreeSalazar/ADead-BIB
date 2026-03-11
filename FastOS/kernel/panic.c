/*
 * kernel/panic.c — Kernel Panic Handler
 * FastOS v2.0
 *
 * Sin silencio. Sin reinicio misterioso.
 * Si el kernel entra en panico → el usuario SABE por que.
 *
 * Compilar: adb cc kernel/panic.c --target fastos
 */

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/fastos.h"

/* ─── Codigos de Panico ─── */
typedef enum {
    PANIC_UNKNOWN           = 0,
    PANIC_NULL_DEREF        = 1,
    PANIC_STACK_OVERFLOW    = 2,
    PANIC_DOUBLE_FAULT      = 3,
    PANIC_PAGE_FAULT        = 4,
    PANIC_INIT_DIED         = 5,
    PANIC_BG_COMPROMISED    = 6,
    PANIC_MEMORY_CORRUPT    = 7,
    PANIC_ASSERTION_FAILED  = 8,
    PANIC_INVALID_INTERRUPT = 9,
} panic_code_t;

static const char *panic_names[] = {
    "Unknown panic",
    "Null pointer dereference",
    "Kernel stack overflow",
    "CPU double fault",
    "Unrecoverable page fault",
    "Init process died (PID 1)",
    "Binary Guardian compromised",
    "Memory corruption detected",
    "Kernel assertion failed",
    "Invalid interrupt vector",
};

/* ─── Registro del panico ─── */
typedef struct {
    uint32_t    code;
    const char *message;
    const char *file;
    int         line;
} panic_record_t;

static panic_record_t last_panic;
static volatile int   panic_active = 0;

/* ─── Detener CPU limpiamente ─── */
static void __attribute__((noreturn)) panic_halt_cpu(void) {
    cli();
    while (1) {
        asm volatile("hlt");  /* CPU duerme, sin consumir ciclos */
    }
}

/* ─── Pantalla de Panic en VGA ─── */
static void panic_display(const panic_record_t *rec) {
    /* Borde rojo para el panic — color 0x4F (blanco sobre rojo) */
    uint8_t red = VGA_COLOR(VGA_WHITE, VGA_RED);

    term_write_color("\n", red);
    term_write_color("╔══════════════════════════════════════════╗\n", red);
    term_write_color("║  *** FASTOS KERNEL PANIC ***             ║\n", red);
    term_write_color("╠══════════════════════════════════════════╣\n", red);

    /* Determinar nombre del panico */
    const char *name = (rec->code < 10) ? panic_names[rec->code] : "UNKNOWN";

    /* Imprimir codigo y nombre */
    term_write_color("║  Code: ", red);
    /* Imprimir numero del codigo */
    char num[8];
    int n = (int)rec->code, i = 0;
    if (n == 0) { num[i++] = '0'; }
    else {
        int tmp = n;
        while (tmp > 0) { num[i++] = '0' + (tmp % 10); tmp /= 10; }
        /* Invertir */
        for (int a = 0, b = i-1; a < b; a++, b--) {
            char t = num[a]; num[a] = num[b]; num[b] = t;
        }
    }
    num[i] = '\0';
    term_write_color(num, red);
    term_write_color(" — ", red);
    term_write_color(name, red);
    term_write_color("\n║  Msg:  ", red);
    term_write_color(rec->message ? rec->message : "(none)", red);
    term_write_color("\n", red);

    if (rec->file) {
        term_write_color("║  File: ", red);
        term_write_color(rec->file, red);
        term_write_color("\n", red);
    }

    term_write_color("╠══════════════════════════════════════════╣\n", red);
    term_write_color("║  CPU detenido. Reinicia con reset.       ║\n", red);
    term_write_color("╚══════════════════════════════════════════╝\n", red);
}

/* ─── API PUBLICA ─── */

void __attribute__((noreturn))
kernel_panic(uint32_t code, const char *message,
             const char *file, int line) {
    /* Prevenir panico recursivo */
    if (panic_active) {
        panic_halt_cpu();
    }
    panic_active = 1;

    last_panic.code    = code;
    last_panic.message = message;
    last_panic.file    = file;
    last_panic.line    = line;

    panic_display(&last_panic);
    panic_halt_cpu();
}

/* ─── Kernel Assert ─── */
void kernel_assert_fail(const char *expr, const char *file, int line) {
    /* Construir mensaje: "assert(EXPR) failed" en stack */
    char msg[128];
    int i = 0;
    const char *p;

    p = "assert(";
    while (*p && i < 120) msg[i++] = *p++;
    p = expr;
    while (*p && i < 120) msg[i++] = *p++;
    p = ") failed";
    while (*p && i < 120) msg[i++] = *p++;
    msg[i] = '\0';

    kernel_panic(PANIC_ASSERTION_FAILED, msg, file, line);
}
