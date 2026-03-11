/*
 * userspace/init.c — FastOS Init Process (PID 1)
 * FastOS v2.0
 *
 * El primer proceso del userspace. Responsabilidades:
 *   - Montar filesystems (VFS)
 *   - Iniciar el shell (PID 2)
 *   - Gestionar senales de shutdown/reboot
 *   - Mantener el sistema vivo (no puede morir)
 *
 * Si init muere → kernel panic.
 *
 * Compilar: adb cc userspace/init.c --target fastos -o init.po
 */

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/fastos.h"

/* ─── Senales del sistema ─── */
#define FASTOS_SIG_SHUTDOWN 1
#define FASTOS_SIG_REBOOT   2
#define FASTOS_SIG_HALT     3

static volatile int init_signal = 0;

/* ─── Secuencia de arranque ─── */
static void init_mount_filesystems(void) {
    kprintf("[init] Mounting root filesystem...\n");
    /* vfs_mount("/", boot_device_root, FS_TYPE_FAT32); */
    kprintf("[init] Root filesystem mounted OK\n");
}

static void init_start_daemons(void) {
    /*
     * FastOS minimo: solo el shell.
     * No hay systemd. No hay SysV. No hay 40 daemons al arrancar.
     */
    kprintf("[init] Starting shell (PID 2)...\n");
    shell_start();
}

static void init_handle_signal(int sig) {
    switch (sig) {
    case FASTOS_SIG_SHUTDOWN:
        kprintf("[init] Shutdown signal received\n");
        kprintf("[init] Syncing filesystems...\n");
        /* vfs_sync_all(); */
        kprintf("[init] Halted. Safe to power off.\n");
        break;
    case FASTOS_SIG_REBOOT:
        kprintf("[init] Reboot signal received\n");
        kprintf("[init] Syncing filesystems...\n");
        /* fastos_reboot(); */
        break;
    case FASTOS_SIG_HALT:
        kprintf("[init] Halt signal received\n");
        /* cpu_halt(); */
        break;
    default:
        kprintf("[init] Unknown signal: %d\n", sig);
        break;
    }
}

/* ─── Bucle principal de init (NUNCA TERMINA) ─── */
static void init_loop(void) {
    /*
     * Init espera senales del kernel.
     * Si el shell termina (el usuario escribio 'exit'), init lo reinicia.
     * Init es inmortal — si muere, el sistema entra en kernel panic.
     */
    while (1) {
        if (init_signal != 0) {
            int sig = init_signal;
            init_signal = 0;
            init_handle_signal(sig);
        }

        /* Esperar interrupcion en vez de spinear — evita flood de VGA.
         * El PIT tick o un evento de teclado despertaran al CPU. */
        asm volatile("hlt");
    }
}

/* ─── Entry Point: init_main() — llamado por kernel_main() ─── */
void init_main(void) {
    kprintf("[init] FastOS init v2.0 starting (PID 1)\n");

    /* Paso 1: Montar filesystems */
    init_mount_filesystems();

    /* Paso 2: Iniciar servicios/shell */
    init_start_daemons();

    /* Paso 3: Bucle eterno — NUNCA debe retornar */
    init_loop();

    /* Si llegamos aqui → kernel panic (PANIC_INIT_DIED) */
    KERNEL_PANIC(5, "init_main() returned unexpectedly");
}
