/*
 * kernel/panic.c — Kernel Panic Handler
 * FastOS v2.0
 *
 * Cuando algo que NUNCA DEBERÍA fallar falla, FastOS muestra diagnóstico
 * completo y detiene el CPU limpiamente.
 *
 * Sin silencio. Sin reinicio silencioso. Sin "mystery reboot at 3am".
 * Si el kernel entra en pánico → el usuario SABE por qué.
 *
 * Compilar:  adb cc kernel/panic.c --target fastos
 * Ver steps: adb step kernel/panic.c
 */

#include <kernel.h>
#include <types.h>
#include <fastos.h>

/* ─── Códigos de Pánico ─── */
typedef enum {
    PANIC_UNKNOWN           = 0,
    PANIC_NULL_DEREF        = 1,  /* nullptr desreferenciado */
    PANIC_STACK_OVERFLOW    = 2,  /* pila del kernel agotada */
    PANIC_DOUBLE_FAULT      = 3,  /* double fault en IDT */
    PANIC_PAGE_FAULT        = 4,  /* page fault no manejable */
    PANIC_INIT_DIED         = 5,  /* PID 1 (init) terminó */
    PANIC_BG_COMPROMISED    = 6,  /* Binary Guardian comprometido */
    PANIC_MEMORY_CORRUPT    = 7,  /* heap corruption detectada */
    PANIC_ASSERTION_FAILED  = 8,  /* assert() del kernel falló */
    PANIC_INVALID_INTERRUPT = 9,  /* interrupción no registrada */
} panic_code_t;

static const char *panic_names[] = {
    [PANIC_UNKNOWN]           = "Unknown panic",
    [PANIC_NULL_DEREF]        = "Null pointer dereference",
    [PANIC_STACK_OVERFLOW]    = "Kernel stack overflow",
    [PANIC_DOUBLE_FAULT]      = "CPU double fault",
    [PANIC_PAGE_FAULT]        = "Unrecoverable page fault",
    [PANIC_INIT_DIED]         = "Init process died (PID 1)",
    [PANIC_BG_COMPROMISED]    = "Binary Guardian compromised",
    [PANIC_MEMORY_CORRUPT]    = "Memory corruption detected",
    [PANIC_ASSERTION_FAILED]  = "Kernel assertion failed",
    [PANIC_INVALID_INTERRUPT] = "Invalid interrupt vector",
};

/* ─── Registro del pánico (para análisis post-mortem) ─── */
typedef struct {
    uint32_t    code;
    const char *message;
    const char *file;
    int         line;
    uint64_t    rip;    /* Instruction pointer al momento del pánico */
    uint64_t    rsp;    /* Stack pointer */
    uint64_t    cr3;    /* Page table base */
    uint64_t    timestamp;
} panic_record_t;

static panic_record_t last_panic;
static int panic_active = 0;

/* ─── Capturar registros del CPU ─── */
static void panic_capture_registers(panic_record_t *rec) {
    /* En producción: leer registros via inline ASM */
    /* Aquí el ABI de ADead-BIB garantiza que podemos leer la pila */
    rec->rip = 0; /* lea rip_val, [rip] — requiere ASM */
    rec->rsp = 0; /* mov rsp_val, rsp */
    rec->cr3 = 0; /* mov cr3_val, cr3 */
}

/* ─── Imprimir backfire de la pila ─── */
static void panic_print_stack(void) {
    printf("\n  Stack trace:\n");
    printf("    [frame 0] kernel/panic.c → panic()\n");
    printf("    [frame 1] (caller frame — %s)\n",
           last_panic.file ? last_panic.file : "unknown");
    /* En producción: unwinder de stack x86-64 via frame pointer chain */
}

/* ─── Pantalla de Kernel Panic ─── */
/*
 * ╔══════════════════════════════════════════╗
 * ║  💀 FASTOS KERNEL PANIC 💀              ║
 * ║  [código] [mensaje]                      ║
 * ║  File: kernel/memory.c  Line: 42         ║
 * ║  RIP: 0xFFFF800000001234                 ║
 * ║  RSP: 0xFFFF800001FF0000                 ║
 * ║  CR3: 0x0000000000100000                 ║
 * ║                                          ║
 * ║  El CPU está detenido. No es seguro      ║
 * ║  encender/apagar el hardware bruscamente.║
 * ║  Reinicia con el botón de reset o        ║
 * ║  Ctrl+Alt+Del.                           ║
 * ╚══════════════════════════════════════════╝
 */
static void panic_display(const panic_record_t *rec) {
    printf("\n");
    printf("╔══════════════════════════════════════════╗\n");
    printf("║  \xf0\x9f\x92\x80 FASTOS KERNEL PANIC \xf0\x9f\x92\x80              ║\n");
    printf("╠══════════════════════════════════════════╣\n");

    const char *name = (rec->code < 10)
                       ? panic_names[rec->code]
                       : "UNKNOWN";
    printf("║  Code: %u — %s\n",  rec->code, name);
    printf("║  Msg:  %s\n",  rec->message ? rec->message : "(none)");

    if (rec->file) {
        printf("║  File: %s  Line: %d\n", rec->file, rec->line);
    }
    printf("║\n");
    printf("║  RIP: 0x%016llx\n", (unsigned long long)rec->rip);
    printf("║  RSP: 0x%016llx\n", (unsigned long long)rec->rsp);
    printf("║  CR3: 0x%016llx\n", (unsigned long long)rec->cr3);
    printf("║\n");

    panic_print_stack();

    printf("╠══════════════════════════════════════════╣\n");
    printf("║  CPU detenido. Reinicia con reset/Ctrl+  ║\n");
    printf("║  Alt+Del. Reporta en issues.fastos.io    ║\n");
    printf("╚══════════════════════════════════════════╝\n");
}

/* ─── Detener el CPU definitivamente ─── */
static void panic_halt_cpu(void) {
    /* Deshabilitar interrupciones y detener el CPU */
    /* cli; hlt en loop — nunca retorna */
    /* En ADead-BIB via inline ASM o intrinsic */
    while (1) {
        /* hlt — el CPU no consume ciclos, espera interrupción NMI */
        /* pero interrupciones están deshabilitadas → CPU queda quieto */
    }
}

/* ─── API PÚBLICA ─── */

/* kernel_panic() — punto de entrada desde cualquier lugar del kernel */
void __attribute__((noreturn))
kernel_panic(uint32_t code, const char *message,
             const char *file, int line) {
    /* Prevenir pánico recursivo */
    if (panic_active) {
        /* Doble pánico → halt inmediato sin prints */
        panic_halt_cpu();
    }
    panic_active = 1;

    /* Registrar */
    last_panic.code    = code;
    last_panic.message = message;
    last_panic.file    = file;
    last_panic.line    = line;
    panic_capture_registers(&last_panic);

    /* Mostrar diagnóstico */
    panic_display(&last_panic);

    /* CPU se detiene aquí, para siempre */
    panic_halt_cpu();
}

/* Macro conveniente para usar en el kernel: */
/* KERNEL_PANIC(PANIC_INIT_DIED, "init returned unexpectedly"); */

/* ─── Kernel Assert ─── */
void kernel_assert_fail(const char *expr, const char *file, int line) {
    char msg[256];
    /* Construir mensaje: "assert(" + expr + ") failed" */
    int i = 0, j = 0;
    const char prefix[] = "assert(";
    while (prefix[j]) msg[i++] = prefix[j++];
    j = 0;
    while (expr[j] && i < 240) msg[i++] = expr[j++];
    const char suffix[] = ") failed";
    j = 0;
    while (suffix[j]) msg[i++] = suffix[j++];
    msg[i] = '\0';

    kernel_panic(PANIC_ASSERTION_FAILED, msg, file, line);
}
