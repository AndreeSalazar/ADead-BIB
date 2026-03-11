/*
 * security/bg_preexec.c — Binary Guardian Pre-Execution Hook
 * FastOS v2.0
 *
 * Cada binario .Po pasa por aquí ANTES de su primera instrucción.
 * Si la verificación falla → el proceso nunca empieza.
 * Sin excepciones. Sin bypass. Sin "modo debug sin verificación".
 */

#include <kernel.h>
<parameter name="CodeContent">/*
 * security/bg_preexec.c — Binary Guardian Pre-Execution Hook
 * FastOS v2.0
 *
 * Cada binario .Po pasa por aquí ANTES de su primera instrucción.
 * Si la verificación falla → el proceso nunca empieza.
 * Sin excepciones. Sin bypass. Sin "modo debug sin verificación".
 */

#include <kernel.h>
#include <types.h>
#include <bg_guardian.h>

/* ─── Registro de Binarios Verificados ─── */
/* Hash tabla simple (FNV-1a key → resultado cacheado) */
#define BG_CACHE_SIZE 256

typedef struct {
    uint64_t    hash;       /* FNV-1a del binario completo */
    bg_result_t result;     /* Resultado de la última verificación */
    uint32_t    exec_count; /* Cuántas veces se ejecutó */
} bg_cache_entry_t;

static bg_cache_entry_t bg_cache[BG_CACHE_SIZE];

static uint32_t bg_cache_index(uint64_t hash) {
    return (uint32_t)(hash % BG_CACHE_SIZE);
}

/* ─── Función Principal: Pre-Execution Gate ─── */
/*
 * Llamado por el scheduler ANTES de hacer el primer context-switch
 * al nuevo proceso. Si retorna != BG_RESULT_OK, el proceso es cancelado.
 *
 * Flujo:
 *   kernel carga .Po en memoria
 *        ↓
 *   bg_preexec_gate() ← AQUÍ
 *        ↓
 *   BG_RESULT_OK → CPU ejecuta primera instrucción
 *   BG_RESULT_* (error) → proceso cancelado, nunca ejecuta
 */
bg_result_t bg_preexec_gate(const uint8_t  *binary,
                              size_t          size,
                              bg_capability_t caps,
                              uint32_t        pid) {
    if (!binary || size < 24) {
        return BG_RESULT_NULL_INPUT;
    }

    /* FNV-1a del binario completo para cache lookup */
    uint64_t hash = 0;
    {
        uint64_t h = 14695981039346656037ULL;
        for (size_t i = 0; i < size; i++) {
            h ^= (uint64_t)binary[i];
            h *= 1099511628211ULL;
        }
        hash = h;
    }

    /* Buscar en cache */
    uint32_t idx = bg_cache_index(hash);
    if (bg_cache[idx].hash == hash) {
        /* Cache hit — el binario fue verificado antes */
        bg_cache[idx].exec_count++;
        return bg_cache[idx].result;
    }

    /* Cache miss — verificar ahora */
    bg_result_t result = bg_level3_preexec(binary, size, caps);

    /* Registrar en cache */
    bg_cache[idx].hash       = hash;
    bg_cache[idx].result     = result;
    bg_cache[idx].exec_count = 1;

    if (result != BG_RESULT_OK) {
        /* Log del intento fallido con PID */
        (void)pid;
        /* En producción: escribir al log de seguridad de FastOS */
    }

    return result;
}

/* ─── Invalidar Cache (tras actualización de binario) ─── */
void bg_preexec_invalidate(uint64_t hash) {
    uint32_t idx = bg_cache_index(hash);
    if (bg_cache[idx].hash == hash) {
        bg_cache[idx].hash       = 0;
        bg_cache[idx].result     = BG_RESULT_OK;
        bg_cache[idx].exec_count = 0;
    }
}

/* ─── Estadísticas ─── */
uint32_t bg_preexec_cache_hits(void) {
    uint32_t hits = 0;
    for (int i = 0; i < BG_CACHE_SIZE; i++) {
        if (bg_cache[i].hash != 0) hits++;
    }
    return hits;
}
