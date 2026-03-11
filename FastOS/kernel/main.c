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

/* ═══════════════════════════════════════════════════════
 * VGA Text Mode — 80x25 colores
 * VGA_BUFFER, VGA_WIDTH, VGA_HEIGHT, VGA_COLOR, vga_color_t
 * definidos en kernel.h. Aqui solo el helper VGA_ENTRY.
 * ═══════════════════════════════════════════════════════ */

/* Alias locales de colores para legibilidad del banner */
#define VGA_LIGHT_GREY    VGA_LGRAY
#define VGA_DARK_GREY     VGA_DGRAY
#define VGA_LIGHT_BLUE    VGA_LBLUE
#define VGA_LIGHT_GREEN   VGA_LGREEN
#define VGA_LIGHT_CYAN    VGA_LCYAN
#define VGA_LIGHT_RED     VGA_LRED
#define VGA_LIGHT_MAGENTA VGA_LMAGENTA

#define VGA_ENTRY(ch, attr) ((uint16_t)((uint8_t)(ch)) | ((uint16_t)(attr) << 8))

/* Terminal estado global */
static int     term_row   = 0;
static int     term_col   = 0;
static uint8_t term_color = 0; /* inicializado en term_init */

void term_init(void) {
    term_color = VGA_COLOR(VGA_WHITE, VGA_BLUE);
    term_row   = 0;
    term_col   = 0;
    /* Limpiar pantalla */
    for (int y = 0; y < VGA_HEIGHT; y++)
        for (int x = 0; x < VGA_WIDTH; x++)
            VGA_BUFFER[y * VGA_WIDTH + x] = VGA_ENTRY(' ', term_color);
}

static void term_scroll(void) {
    for (int y = 0; y < VGA_HEIGHT - 1; y++)
        for (int x = 0; x < VGA_WIDTH; x++)
            VGA_BUFFER[y * VGA_WIDTH + x] = VGA_BUFFER[(y+1) * VGA_WIDTH + x];
    uint8_t blank_color = VGA_COLOR(VGA_WHITE, VGA_BLUE);
    for (int x = 0; x < VGA_WIDTH; x++)
        VGA_BUFFER[(VGA_HEIGHT-1) * VGA_WIDTH + x] = VGA_ENTRY(' ', blank_color);
    term_row = VGA_HEIGHT - 1;
}

void term_putchar(char c) {
    if (c == '\n') { term_col = 0; term_row++; }
    else if (c == '\r') { term_col = 0; }
    else if (c == '\t') { term_col = (term_col + 8) & ~7; }
    else {
        VGA_BUFFER[term_row * VGA_WIDTH + term_col] = VGA_ENTRY(c, term_color);
        term_col++;
    }
    if (term_col >= VGA_WIDTH)  { term_col = 0; term_row++; }
    if (term_row >= VGA_HEIGHT) { term_scroll(); }
}

void term_write(const char *str) {
    while (*str) term_putchar(*str++);
}

void term_write_color(const char *str, uint8_t color) {
    uint8_t old = term_color;
    term_color = color;
    term_write(str);
    term_color = old;
}

void kputs(const char *s) {
    term_write(s);
    term_putchar('\n');
}

/* ═══════════════════════════════════════════════════════
 * Subsystem Declarations
 * Implementados en sus archivos correspondientes.
 * kernel_main() orquesta — no implementa nada.
 * ═══════════════════════════════════════════════════════ */

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
    term_write_color("  |                         FastOS  v2.0                           |\n", VGA_COLOR(VGA_WHITE, VGA_BLUE));
    term_write_color("  |        The Best of Windows (UX/Drivers) & Linux (Core)         |\n", VGA_COLOR(VGA_LIGHT_GREEN, VGA_BLUE));
    term_write_color("  |              ADead-BIB Native OS   *   Made in Peru            |\n", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write_color("  +================================================================+\n\n", VGA_COLOR(VGA_LIGHT_CYAN, VGA_BLUE));

    term_write_color("[BOOT] ", VGA_COLOR(VGA_LIGHT_MAGENTA, VGA_BLUE));
    term_write("16-bit real mode -> 32-bit protected -> 64-bit long mode\n");
    term_write_color("[BOOT] ", VGA_COLOR(VGA_LIGHT_MAGENTA, VGA_BLUE));
    term_write("CPU awakened gradually. No ghost state. No lost context.\n");
    term_write_color("[BOOT] ", VGA_COLOR(VGA_LIGHT_MAGENTA, VGA_BLUE));
    term_write("Kernel entry at 0x100000\n\n");

    /* ─── 1. memory_init() — paginacion, heap ─── */
    term_write_color("[1/5] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("memory_init()      paginacion, heap\n");
    memory_init();
    term_write_color("      OK\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* ─── 2. interrupts_init() — IDT, IRQ handlers ─── */
    term_write_color("[2/5] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("interrupts_init()  IDT, IRQ handlers + keyboard\n");
    interrupts_init();
    keyboard_init();
    term_write_color("      OK\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* ─── 3. scheduler_init() — round-robin preemptivo ─── */
    term_write_color("[3/5] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("scheduler_init()   round-robin preemptivo\n");
    scheduler_init();
    term_write_color("      OK\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* ─── 4. hotplug_init() — 0 drivers pre-instalados ─── */
    term_write_color("[4/5] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("hotplug_init()     hardware detection, on-demand drivers\n");
    hotplug_init();
    term_write_color("      OK\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* ─── 5. bg_init() — Binary Guardian 4 niveles ─── */
    term_write_color("[5/5] ", VGA_COLOR(VGA_YELLOW, VGA_BLUE));
    term_write("bg_init()          Binary Guardian activo (4 niveles, mat. pura)\n");
    bg_init();
    term_write_color("      OK\n\n", VGA_COLOR(VGA_GREEN, VGA_BLUE));

    /* Sistema listo */
    term_write_color("[OK] FastOS kernel ready. ~150KB. 0 drivers. 0 muletas.\n\n",
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
