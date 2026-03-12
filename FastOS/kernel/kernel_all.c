/*
 * kernel/kernel_all.c — FastOS v2.0 Unity Build
 *
 * ADead-BIB --flat mode compiles a single translation unit.
 * Cross-file calls (e.g. kernel_main → memory_init) require all
 * functions to be in ONE compilation unit so the compiler resolves
 * all call targets within the same binary.
 *
 * Order matters: dependencies first, kernel_main() last.
 *
 * Compilar: adb cc kernel/kernel_all.c -o build/kernel64.bin --flat --org=0x100000
 */

/* ─── 1. Library: string/memory (no dependencies) ─── */
#include "../lib/string.c"
#include "../lib/memory.c"

/* ─── 2. VGA terminal (term_putchar etc — needed by printf) ─── */
#include "vga.c"

/* ─── 3. Printf (uses term_putchar from vga.c) ─── */
#include "../lib/printf.c"

/* ─── 4. Kernel subsystems (use kprintf, called by kernel_main) ─── */
#include "cpuid.c"
#include "memory/e820.c"
#include "interrupts.c"
#include "keyboard.c"
#include "scheduler.c"
#include "panic.c"
#include "syscall.c"
#include "hotplug.c"

/* ─── 5. Security: Binary Guardian ─── */
#include "../security/bg_core.c"
#include "../security/bg_fastos.c"
#include "../security/bg_levels.c"
#include "../security/bg_preexec.c"

/* ─── 6. Filesystem ─── */
#include "../fs/vfs.c"

/* ─── 7. Userspace ─── */
#include "../userspace/init.c"
#include "../userspace/shell.c"

/* ─── 8. Kernel entry — MUST BE LAST (calls everything above) ─── */
#include "main.c"
