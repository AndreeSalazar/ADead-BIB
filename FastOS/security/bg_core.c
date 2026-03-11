/*
 * security/bg_core.c — Binary Guardian Core Integration
 * FastOS v2.0
 *
 * Este archivo conecta el kernel C con el crate Rust "BG — Binary Guardian".
 * Funciona como wrapper para exponer `bg_rust_can_execute()`.
 *
 * Compilar con ADead-BIB:
 *   adb cc security/bg_core.c -o bg_core.bin --target fastos
 *   adb step security/bg_core.c   ← ver pipeline completo
 */

#include "../include/kernel.h"

/* ─── FFI Rust ─── */
extern int bg_rust_can_execute(const uint8_t *bytes, size_t len, int level);

/* Map levels para FFI */
#define BG_RUST_KERNEL  0
#define BG_RUST_DRIVER  1
#define BG_RUST_SERVICE 2
#define BG_RUST_USER    3

/* ─── Estado Global ─── */
static bg_state_t bg_global_state = {
    .initialized = 0,
    .level        = BG_LEVEL_MAX,
    .violations   = 0,
    .verified     = 0
};

/* ─── API PÚBLICA ─── */

void bg_init(void) {
    bg_global_state.initialized = 1;
    bg_global_state.violations  = 0;
    bg_global_state.verified    = 0;
    bg_global_state.level       = BG_LEVEL_MAX;
}

bg_result_t bg_verify_binary(const uint8_t *binary, size_t size, bg_capability_t caps) {
    if (!bg_global_state.initialized) return BG_RESULT_NOT_INITIALIZED;
    if (!binary || size < 24)         return BG_RESULT_NULL_INPUT;

    /* Extraer puntero al código saltando .Po header (24 bytes) */
    const uint8_t *code_start = binary + 24;
    size_t code_size = size - 24;
    
    if (code_size == 0) return BG_RESULT_NULL_INPUT;

    int rust_level;
    if (caps & BG_CAP_ALL) {
        rust_level = BG_RUST_KERNEL;
    } else if (caps & BG_CAP_DRIVER) {
        rust_level = BG_RUST_DRIVER;
    } else if (caps & BG_CAP_SYSCALL) {
        rust_level = BG_RUST_SERVICE;
    } else {
        rust_level = BG_RUST_USER;
    }

    /* Llamar a la verificacion ISA-level determinista en Rust */
    int is_safe = bg_rust_can_execute(code_start, code_size, rust_level);

    if (is_safe) {
        bg_global_state.verified++;
        return BG_RESULT_OK;
    } else {
        bg_global_state.violations++;
        return BG_RESULT_UNAUTHORIZED_SYSCALL; /* Denegado por policy */
    }
}

uint32_t bg_get_violations(void) {
    return bg_global_state.violations;
}

uint32_t bg_get_verified(void) {
    return bg_global_state.verified;
}

bg_level_t bg_get_level(void) {
    return bg_global_state.level;
}
