/*
 * include/fastos.h — FastOS Native API v2.2
 *
 * El header nativo de FastOS. Un solo include da acceso
 * a toda la API del OS. Sin linker externo. Sin flags misteriosos.
 *
 * Uso:
 *   #include <fastos.h>
 *
 * Compilar con ADead-BIB:
 *   adb cc miapp.c --target fastos -o miapp.po
 *   adb step miapp.c   ← ver pipeline completo de 7 fases
 */

#ifndef _FASTOS_H
#define _FASTOS_H

#include <types.h>

/* ══════════════════════════════════════════════════════
 * § 1. Formato .Po — Ejecutable Nativo FastOS
 * ══════════════════════════════════════════════════════ */

/*
 * Header .Po: 24 bytes exactos. Solo lo que existe.
 *
 * Offset  Size  Campo
 * ------  ----  --------------
 *    0      6   magic: "FASTOS"
 *    6      2   version
 *    8      4   code_offset
 *   12      4   code_size
 *   16      4   data_offset
 *   20      4   data_size
 * ────────────────────────────
 * Total:   24 bytes
 *
 * PE de Windows: ~1KB mínimo
 * ELF de Linux:  64 bytes + program headers + section headers
 * .Po de FastOS: 24 bytes. Solo lo que existe.
 */
typedef struct __attribute__((packed)) {
    char     magic[6];       /* "FASTOS" */
    uint16_t version;        /* versión del formato */
    uint32_t code_offset;    /* offset al código desde el inicio */
    uint32_t code_size;      /* tamaño del código en bytes */
    uint32_t data_offset;    /* offset a los datos */
    uint32_t data_size;      /* tamaño de los datos */
} fastos_po_header_t;

#define FASTOS_PO_MAGIC    "FASTOS"
#define FASTOS_PO_VERSION  2
#define FASTOS_PO_HDRSIZE  24

/* ══════════════════════════════════════════════════════
 * § 2. Syscalls FastOS
 * ══════════════════════════════════════════════════════ */

/* Números de syscall (POSIX-like + extensiones FastOS) */
#define SYS_EXIT         1
#define SYS_READ         3
#define SYS_WRITE        4
#define SYS_OPEN         5
#define SYS_CLOSE        6
#define SYS_MMAP        90
#define SYS_MUNMAP      91
/* Extensiones FastOS */
#define SYS_HOTPLUG_QUERY  200  /* consultar hardware detectado */
#define SYS_BG_VERIFY      201  /* verificar binario con Binary Guardian */
#define SYS_PO_EXEC        202  /* ejecutar binario .Po */
#define SYS_DRIVER_LOAD    203  /* cargar driver desde disco */

/* ══════════════════════════════════════════════════════
 * § 3. Binary Guardian — Capabilities
 * ══════════════════════════════════════════════════════ */

typedef uint32_t bg_capability_t;
#define BG_CAP_NONE        0x00000000
#define BG_CAP_SYSCALL     0x00000001  /* puede hacer syscalls */
#define BG_CAP_IO_DIRECT   0x00000002  /* acceso directo I/O (solo drivers) */
#define BG_CAP_DRIVER      0x00000004  /* es un driver */
#define BG_CAP_NETWORK     0x00000008  /* acceso de red */
#define BG_CAP_FS_WRITE    0x00000010  /* escritura al filesystem */
#define BG_CAP_ALL         0xFFFFFFFF  /* solo para el kernel */

#define BG_MAX_FORMAT_VERSION 2

typedef enum {
    BG_RESULT_OK                  = 0,
    BG_RESULT_CORRUPT             = 1,
    BG_RESULT_INVALID_MAGIC       = 2,
    BG_RESULT_VERSION_MISMATCH    = 3,
    BG_RESULT_OVERFLOW            = 4,
    BG_RESULT_UNAUTHORIZED_SYSCALL = 5,
    BG_RESULT_UNAUTHORIZED_IO     = 6,
    BG_RESULT_NULL_INPUT          = 7,
    BG_RESULT_NOT_INITIALIZED     = 8,
    BG_RESULT_INTEGRITY_FAILURE   = 9,
} bg_result_t;

typedef enum {
    BG_LEVEL_1 = 1,  /* Re-build automático */
    BG_LEVEL_2 = 2,  /* + Firewall Humano */
    BG_LEVEL_3 = 3,  /* + BG Pre-execution */
    BG_LEVEL_MAX = 4 /* + Dead Man's Switch */
} bg_level_t;

typedef struct {
    int        initialized;
    bg_level_t level;
    uint32_t   violations;
    uint32_t   verified;
} bg_state_t;

/* ══════════════════════════════════════════════════════
 * § 4. Process API
 * ══════════════════════════════════════════════════════ */

typedef uint32_t fastos_pid_t;

#define FASTOS_PID_KERNEL 0
#define FASTOS_PID_INIT   1
#define FASTOS_PID_SHELL  2

typedef enum {
    PROCESS_RUNNING   = 1,
    PROCESS_SLEEPING  = 2,
    PROCESS_ZOMBIE    = 3,
    PROCESS_DEAD      = 4
} process_state_t;

/* ══════════════════════════════════════════════════════
 * § 5. Prototipos de la API del Kernel
 * ══════════════════════════════════════════════════════ */

/* kernel/main.c */
void kernel_main(void);

/* kernel/hotplug.c */
void hotplug_init(void);
void hotplug_tick(void);
void hotplug_on_pci_device(uint16_t vendor, uint16_t device,
                             uint8_t bus, uint8_t slot, uint8_t func);

/* kernel/panic.c */
__attribute__((noreturn))
void kernel_panic(uint32_t code, const char *message,
                  const char *file, int line);
void kernel_assert_fail(const char *expr, const char *file, int line);

#define KERNEL_PANIC(code, msg) \
    kernel_panic((code), (msg), __FILE__, __LINE__)

#define KERNEL_ASSERT(expr) \
    ((expr) ? (void)0 : kernel_assert_fail(#expr, __FILE__, __LINE__))

/* security/bg_core.c */
void       bg_init(void);
bg_result_t bg_verify_binary(const uint8_t *binary, size_t size,
                               bg_capability_t caps);
uint32_t   bg_get_violations(void);
uint32_t   bg_get_verified(void);
bg_level_t bg_get_level(void);

/* security/bg_levels.c */
bg_result_t bg_level1_rebuild_check(const char *path, uint64_t expected_hash);
bg_result_t bg_level2_capability_check(uint32_t pid, bg_capability_t req,
                                         bg_capability_t allowed);
bg_result_t bg_level3_preexec(const uint8_t *binary, size_t size,
                                bg_capability_t caps);
void        bg_level4_heartbeat(void);
bg_result_t bg_level4_integrity_check(void);

/* security/bg_preexec.c */
bg_result_t bg_preexec_gate(const uint8_t *binary, size_t size,
                              bg_capability_t caps, uint32_t pid);
void        bg_preexec_invalidate(uint64_t hash);
uint32_t    bg_preexec_cache_hits(void);

/* fs/vfs.c */
void vfs_init(void);

/* userspace/shell.c */
void shell_start(void);

/* userspace/init.c */
void init_main(void);

/* ══════════════════════════════════════════════════════
 * § 6. Compatibility Layer (v2.2)
 *
 * FastOS no hereda Windows ni Linux. TRADUCE sus llamadas.
 * ADead-BIB actúa como intérprete automático.
 *
 * App Win32/POSIX → ADead-BIB traduce → FastOS nativo → CPU libre
 *
 * Archivos en compat/:
 *   fastos_syscall.h  — syscalls nativas FastOS (todo se traduce a estas)
 *   fastos_stdlib.h   — stdlib mínima propia (mem, str, I/O, math AVX2)
 *   fastos_win32.h    — Win32 subset → macros que llaman fastos_syscall.h
 *   fastos_posix.h    — POSIX subset → macros que llaman fastos_syscall.h
 *   compat_test.c     — test suite de traducción
 *
 * Win32 traducido: CreateFile, ReadFile, WriteFile, CloseHandle,
 *   VirtualAlloc, VirtualFree, GetSystemInfo, CreateThread, Sleep
 * POSIX traducido: open, read, write, close, malloc, free, mmap,
 *   pthread_create, gettimeofday, printf, exit
 * IGNORADO: registry, COM, DCOM, WMI, systemd, dbus, X11, telemetría
 * ══════════════════════════════════════════════════════ */

/* ══════════════════════════════════════════════════════
 * § 7. Filosofía FastOS — Compilado en el Header
 * ══════════════════════════════════════════════════════ */

/*
 * "Un OS es un OS. No una muleta."
 * "El CPU ya sabe todo — solo hay que dejarlo recordar gradualmente."
 * "Los drivers van en el disco, no en el OS."
 * "No heredar, TRADUCIR."
 *
 * Si compilas con ADead-BIB:
 *   adb cc miapp.c --target fastos   → binario .Po 24-byte header
 *   adb cc miapp.c --target windows  → PE compatible
 *   adb cc miapp.c --target linux    → ELF compatible
 *   adb cc miapp.c --target all      → los 3 simultáneamente
 *   adb step miapp.c                 → ver las 7 fases del compilador
 *
 * SIN linker. SIN flags. SIN Stack Overflow obligatorio.
 * UN comando. UN binario. Cero dolor.
 * Binary Is Binary.
 */

#endif /* _FASTOS_H */
