/*
 * kernel/main.c — FastOS v2.0 Kernel Entry Point
 *
 * Compilar: adb cc kernel/main.c --target fastos --flat --org=0x100000
 * Ver las 7 fases del pipeline ADead-BIB: adb step kernel/main.c
 *
 * El CPU llegó aquí GRADUALMENTE desde:
 *   boot/mbr64.asm    (stage1 — 16-bit real mode)
 *     -> boot/loader64.asm (stage2 — 16 -> 32 -> 64-bit)
 *       -> kernel/main.c   (64-bit long mode, CPU completamente orientado)
 *
 * C solo dirige lo que ya existe.
 * El CPU ya sabe donde esta.
 *
 * ARQUITECTURA (exacta segun README.md):
 *
 *   void kernel_main() {
 *       memory_init();
 *       interrupts_init();
 *       scheduler_init();
 *       hotplug_init();    // detecta hardware, descarga drivers
 *       bg_init();         // Binary Guardian activo
 *       init_main();       // PID 1 -> PID 2 (shell)
 *   }
 */

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/fastos.h"

/* VGA terminal functions now in kernel/vga.c */

/* ═══════════════════════════════════════════════════════
 * Subsystem Declarations
 * Implementados en sus archivos correspondientes.
 * kernel_main() orquesta — no implementa nada.
 * ═══════════════════════════════════════════════════════ */

/* kernel/cpuid.c        */ extern void cpuid_init(void);
/* kernel/memory/e820.c  */ extern void memory_init(void);
/* kernel/interrupts.c   */ extern void interrupts_init(void);
/* kernel/keyboard.c     */ extern void keyboard_init(void);
/* kernel/scheduler.c    */ extern void scheduler_init(void);
/* kernel/hotplug.c      */ extern void hotplug_init(void);
/* security/bg_core.c    */ extern void bg_init(void);
/* userspace/init.c      */ extern void init_main(void);

/* ═══════════════════════════════════════════════════════
 * kernel_main() — Orquestador del Boot
 *
 * Llamado por boot/loader64.asm tras activar long mode.
 * El CPU llega aqui DESPIERTO, sin estado fantasma,
 * con GDT/paginacion ya configurados.
 * ═══════════════════════════════════════════════════════ */
void kernel_main(void) {

    /* VGA online — primer output visual */
    term_init();

    /* Banner */
    term_write_color("  +================================================================+\n", VGA_COLOR(VGA_LIGHT_CYAN, VGA_BLUE));
    term_write_color("  |                    FastOS v2.0 — 256-bit                       |\n", VGA_COLOR(VGA_WHITE, VGA_BLUE));
    term_write_color("  |       16->32->64->SSE(128)->AVX2(256) Gradual Boot             |\n", VGA_COLOR(VGA_LIGHT_GREEN, VGA_BLUE));
    term_write_color("  |         ADead-BIB Native OS   *   Made in Peru                 |\n", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write_color("  +================================================================+\n\n", VGA_COLOR(VGA_LIGHT_CYAN, VGA_BLUE));

    term_write_color("[BOOT] ", VGA_COLOR(VGA_LIGHT_MAGENTA, VGA_BLUE));
    term_write("16-bit -> 32-bit -> 64-bit -> SSE 128-bit -> AVX2 256-bit\n");
    term_write_color("[BOOT] ", VGA_COLOR(VGA_LIGHT_MAGENTA, VGA_BLUE));
    term_write("CPU awakened gradually. YMM regs active. Kernel prefetched L1.\n");
    term_write_color("[BOOT] ", VGA_COLOR(VGA_LIGHT_MAGENTA, VGA_BLUE));
    term_write("Kernel entry at 0x100000\n\n");

    /* ─── 1. cpuid_init() — detect CPU features ─── */
    term_write_color("[1/6] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("cpuid_init()       CPU feature detection\n");
    cpuid_init();
    term_write_color("      OK\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* ─── 2. memory_init() — paginacion, heap ─── */
    term_write_color("[2/6] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("memory_init()      E820 map, heap\n");
    memory_init();
    term_write_color("      OK\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* ─── 3. interrupts_init() — IDT, IRQ handlers ─── */
    term_write_color("[3/6] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("interrupts_init()  IDT + PIC + PIT + keyboard\n");
    interrupts_init();
    keyboard_init();
    term_write_color("      OK\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* ─── 4. scheduler_init() — round-robin preemptivo ─── */
    term_write_color("[4/6] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("scheduler_init()   round-robin + BG heartbeat\n");
    scheduler_init();
    term_write_color("      OK\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* ─── 5. hotplug_init() — PCI scan, 0 drivers pre-instalados ─── */
    term_write_color("[5/6] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("hotplug_init()     PCI scan, on-demand .Po drivers\n");
    hotplug_init();
    term_write_color("      OK\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* ─── 6. bg_init() — Binary Guardian 4 niveles ─── */
    term_write_color("[6/6] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("bg_init()          Binary Guardian (4 niveles, determinista)\n");
    bg_init();
    term_write_color("      OK\n\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* Sistema listo */
    term_write_color("[OK] FastOS v2.0 ready. 256-bit YMM. ~150KB. 0 drivers. 0 muletas.\n\n",
                     VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /*
     * Lanzar init_main() — PID 1.
     * init_main() lanza shell (PID 2) y no retorna jamas.
     * El scheduler toma control desde aqui.
     *
     * Si init_main() retorna => kernel panic (PANIC_INIT_DIED).
     * El sistema no puede existir sin PID 1.
     */
    init_main();

    /* NUNCA deberia llegar aqui */
    term_write_color(
        "\n[PANIC] init_main() returned — PANIC_INIT_DIED — system halted.\n",
        VGA_COLOR(VGA_WHITE, VGA_RED));
    while (1) {
        hlt();  /* CPU quieto, sin consumir ciclos */
    }
}
