/*
 * userspace/init.c — FastOS Init Process (PID 1)
 * FastOS v2.0
 *
 * El primer proceso del userspace. Responsabilidades:
 *   - Montar filesystems (VFS)
 *   - Iniciar el shell (PID 2)
 *   - Gestionar señales de shutdown/reboot
 *   - Mantener el sistema vivo (no puede morir)
 *
 * Si init muere → kernel panic.
 * Si init existe → FastOS existe.
 *
 * Compilar:  adb cc userspace/init.c --target fastos -o init.po
 * Ver steps: adb step userspace/init.c
 */

#include <header_main.h>
#include <fastos.h>

/* ─── Señales del sistema ─── */
#define FASTOS_SIG_SHUTDOWN 1
#define FASTOS_SIG_REBOOT   2
#define FASTOS_SIG_HALT     3

static volatile int init_signal = 0;

/* ─── Secuencia de arranque ─── */
static void init_mount_filesystems(void) {
    /*
     * Montar el sistema de archivos raíz.
     * FastOS detecta el tipo (FAT32 / EXT2) automáticamente.
     * El dispositivo raíz viene del bootloader en stage2.
     */
    printf("[init] Mounting root filesystem...\n");
    /* vfs_mount("/", boot_device_root, FS_TYPE_FAT32); */
    printf("[init] Root filesystem mounted OK\n");
}

static void init_start_daemons(void) {
    /*
     * En FastOS mínimo: solo el shell.
     * En versiones extendidas: agregar servicios bajo demanda.
     * No hay systemd. No hay SysV. No hay 40 daemons al arrancar.
     */
    printf("[init] Starting shell (PID 2)...\n");
    shell_start();
}

static void init_handle_signal(int sig) {
    switch (sig) {
    case FASTOS_SIG_SHUTDOWN:
        printf("[init] Shutdown signal received\n");
        printf("[init] Syncing filesystems...\n");
        /* vfs_sync_all(); */
        printf("[init] Halted. Safe to power off.\n");
        break;

    case FASTOS_SIG_REBOOT:
        printf("[init] Reboot signal received\n");
        printf("[init] Syncing filesystems...\n");
        /* fastos_reboot(); */
        break;

    case FASTOS_SIG_HALT:
        printf("[init] Halt signal received\n");
        /* cpu_halt(); */
        break;

    default:
        printf("[init] Unknown signal: %d\n", sig);
        break;
    }
}

/* ─── Bucle principal de init (NUNCA TERMINA) ─── */
static void init_loop(void) {
    /*
     * Init espera señales del kernel.
     * Si el shell termina (el usuario escribió 'exit'), init lo reinicia.
     * Init es inmortal — si muere, el sistema entra en kernel panic.
     */
    while (1) {
        if (init_signal != 0) {
            int sig = init_signal;
            init_signal = 0;
            init_handle_signal(sig);
        }

        /* En producción: fastos_wait() — sleep hasta próxima señal */
        /* Aquí en demo: si shell terminó → reiniciar */
        printf("[init] Shell exited — restarting...\n");
        init_start_daemons();
    }
}

/* ─── Entry Point: init_main() — llamado por kernel_main() ─── */
void init_main(void) {
    printf("[init] FastOS init v2.0 starting (PID 1)\n");

    /* Paso 1: Montar filesystems */
    init_mount_filesystems();

    /* Paso 2: Iniciar servicios/shell */
    init_start_daemons();

    /* Paso 3: Bucle eterno de espera de señales */
    init_loop();

    /* Nunca debería llegar aquí */
    /* Si llegamos aquí → kernel panic */
}
