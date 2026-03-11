/*
 * security/bg_fastos.c — BG Binary Guardian: Wrapper C para FastOS kernel
 * FastOS v2.0
 *
 * El kernel de FastOS esta escrito en C (via ADead-BIB).
 * El Binary Guardian real esta en Rust (BG — Binary Guardian crate).
 * Este archivo expone la API Rust de BG al kernel C.
 *
 * Arquitectura de integracion:
 *
 *   kernel/main.c
 *       bg_init()  ← este archivo
 *           ↓
 *   bg_fastos_can_execute()  ← llama al Rust via ABI C
 *       ↓
 *   BinaryGuardian::can_execute()  ← BG Rust crate
 *       ↓
 *   ISA Decoder → Architecture Map → Policy Engine → APPROVE/DENY
 *
 * Compilar el crate Rust:
 *   cd FastOS/security
 *   cargo build --release
 *   → genera security/target/release/libbg_fastos.a
 *
 * Compilar con ADead-BIB:
 *   adb cc security/bg_fastos.c -L security/target/release -l bg_fastos
 *   adb step security/bg_fastos.c   ← ver las 7 fases del pipeline
 *
 * La funcion bg_fastos_can_execute() es declarada extern "C" en Rust.
 */

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/bg_guardian.h"
#include "../include/fastos.h"

/* ─── Declaraciones de funciones Rust (extern "C") ─── */
/*
 * Estas funciones estan implementadas en security/bg/lib.rs
 * con #[no_mangle] y extern "C", exportadas al linker de ADead-BIB.
 */

/* Verificacion rapida: ¿puede el binario ejecutarse al nivel dado? */
extern int bg_rust_can_execute(const uint8_t *bytes, size_t len, int level);

/* Analisis completo: retorna codigo de resultado BG */
extern int bg_rust_analyze(const uint8_t *bytes, size_t len, int policy_level,
                            char *report_buf, size_t report_size);

/* Version del crate BG */
extern const char *bg_rust_version(void);

/* ─── Niveles de Seguridad (espejo de SecurityLevel Rust) ─── */
#define BG_LEVEL_KERNEL  0   /* Ring 0 — todo permitido */
#define BG_LEVEL_DRIVER  1   /* Ring 1 — IO + restringidas, sin CR/MSR */
#define BG_LEVEL_SERVICE 2   /* Ring 2 — solo syscalls */
#define BG_LEVEL_USER    3   /* Ring 3 — instrucciones safe + syscalls */
#define BG_LEVEL_SANDBOX 4   /* Ring 3 sandbox — casi nada */

/* ─── Hardware: la `bg_state_t` global ─── */
static int bg_fastos_initialized = 0;

/*
 * bg_fastos_init() — llamado desde kernel_main() via bg_init()
 *
 * Inicializa el backend Rust del Binary Guardian.
 * Desde este momento, NINGUN binario puede ejecutarse
 * sin pasar por bg_fastos_can_execute().
 */
void bg_fastos_init(void) {
    bg_fastos_initialized = 1;
    /* En produccion: bg_rust_init() para inicializar estructuras Rust */
    /* Por ahora: el crate es zero-init (sin estado global mutable) */
}

/*
 * bg_fastos_can_execute() — gate pre-ejecucion
 *
 * Retorna 1 (APPROVED) o 0 (DENIED).
 * Llamado por bg_preexec.c antes de ejecutar cualquier binario .Po.
 *
 * Mapeamos bg_capability_t de FastOS a SecurityLevel de BG:
 *   BG_CAP_ALL      → BG_LEVEL_KERNEL  (solo para modulos del kernel)
 *   BG_CAP_DRIVER   → BG_LEVEL_DRIVER
 *   BG_CAP_SYSCALL  → BG_LEVEL_SERVICE
 *   (ninguno)       → BG_LEVEL_USER
 */
int bg_fastos_can_execute(const uint8_t *binary, size_t size,
                           bg_capability_t caps) {
    if (!bg_fastos_initialized) return 0; /* Fail-safe */
    if (!binary || size < 24)   return 0; /* .Po header minimo */

    /* Mapear capabilities FastOS → nivel BG */
    int level;
    if (caps & BG_CAP_ALL) {
        level = BG_LEVEL_KERNEL;
    } else if (caps & BG_CAP_DRIVER) {
        level = BG_LEVEL_DRIVER;
    } else if (caps & BG_CAP_SYSCALL) {
        level = BG_LEVEL_SERVICE;
    } else {
        level = BG_LEVEL_USER;
    }

    /* Saltar header .Po (24 bytes) para analizar solo el codigo */
    const uint8_t *code_start = binary + 24;
    size_t code_size = size - 24;

    if (code_size == 0) return 0;

    /* Llamar al BG Rust crate */
    return bg_rust_can_execute(code_start, code_size, level);
}

/*
 * bg_fastos_analyze_report() — analisis completo con reporte
 *
 * Genera un reporte de texto del analisis BG.
 * Util para el "Firewall Humano" (Nivel 2):
 *   "Este driver accede a PCI, DMA y puertos de red. ¿Permitir? [S/N]"
 */
int bg_fastos_analyze_report(const uint8_t *binary, size_t size,
                              bg_capability_t caps,
                              char *report, size_t report_size) {
    if (!bg_fastos_initialized || !binary || size < 24) return 0;

    int level;
    if      (caps & BG_CAP_ALL)    level = BG_LEVEL_KERNEL;
    else if (caps & BG_CAP_DRIVER) level = BG_LEVEL_DRIVER;
    else if (caps & BG_CAP_SYSCALL)level = BG_LEVEL_SERVICE;
    else                            level = BG_LEVEL_USER;

    const uint8_t *code = binary + 24;
    size_t code_len     = size - 24;

    return bg_rust_analyze(code, code_len, level, report, report_size);
}

/*
 * bg_fastos_version() — version del crate BG
 */
const char *bg_fastos_version(void) {
    return bg_rust_version();
}
