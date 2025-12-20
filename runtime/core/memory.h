/**
 * ADead-BIB Universal Runtime - Memory Manager
 * =============================================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with love in Peru
 * 
 * Gestor de memoria determinista y ultra ligero.
 * Sin fragmentación, comportamiento predecible.
 */

#ifndef ADEAD_MEMORY_H
#define ADEAD_MEMORY_H

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================
 * Arena Allocator - Memoria temporal
 * ============================================================ */

typedef struct {
    u8*   base;       /* Base del pool */
    usize offset;     /* Offset actual */
    usize capacity;   /* Capacidad total */
    usize peak;       /* Uso máximo alcanzado */
} ADeadArena;

/* Crear arena con capacidad especificada */
ADeadError adead_arena_init(ADeadArena* arena, usize capacity);

/* Destruir arena */
void adead_arena_destroy(ADeadArena* arena);

/* Asignar memoria de la arena */
void* adead_arena_alloc(ADeadArena* arena, usize size, usize alignment);

/* Resetear arena (libera todo de una vez) */
void adead_arena_reset(ADeadArena* arena);

/* Obtener uso actual */
usize adead_arena_used(const ADeadArena* arena);

/* Obtener espacio disponible */
usize adead_arena_available(const ADeadArena* arena);

/* ============================================================
 * Pool Allocator - Bloques de tamaño fijo
 * ============================================================ */

typedef struct ADeadPoolBlock {
    struct ADeadPoolBlock* next;
} ADeadPoolBlock;

typedef struct {
    u8*            base;        /* Base del pool */
    ADeadPoolBlock* free_list;  /* Lista de bloques libres */
    usize          block_size;  /* Tamaño de cada bloque */
    usize          capacity;    /* Número total de bloques */
    usize          used;        /* Bloques en uso */
} ADeadPool;

/* Crear pool con bloques de tamaño fijo */
ADeadError adead_pool_init(ADeadPool* pool, usize block_size, usize num_blocks);

/* Destruir pool */
void adead_pool_destroy(ADeadPool* pool);

/* Asignar un bloque */
void* adead_pool_alloc(ADeadPool* pool);

/* Liberar un bloque */
void adead_pool_free(ADeadPool* pool, void* ptr);

/* Resetear pool */
void adead_pool_reset(ADeadPool* pool);

/* ============================================================
 * Memory Manager Global
 * ============================================================ */

typedef struct {
    ADeadArena  scratch;      /* Arena para operaciones temporales */
    ADeadPool   tensor_pool;  /* Pool para tensores pequeños */
    usize       total_alloc;  /* Total asignado */
    usize       total_freed;  /* Total liberado */
    u32         alloc_count;  /* Número de asignaciones */
    u32         free_count;   /* Número de liberaciones */
} ADeadMemoryManager;

/* Inicializar memory manager global */
ADeadError adead_memory_init(ADeadMemoryManager* mm, usize scratch_size, usize pool_size);

/* Destruir memory manager */
void adead_memory_destroy(ADeadMemoryManager* mm);

/* Asignar memoria temporal (se libera con reset) */
void* adead_memory_scratch(ADeadMemoryManager* mm, usize size);

/* Resetear memoria temporal */
void adead_memory_scratch_reset(ADeadMemoryManager* mm);

/* Estadísticas */
void adead_memory_stats(const ADeadMemoryManager* mm, 
                        usize* total_alloc, usize* total_freed,
                        u32* alloc_count, u32* free_count);

/* ============================================================
 * Utilidades de alineación
 * ============================================================ */

/* Alinear tamaño hacia arriba */
static inline usize adead_align_up(usize size, usize alignment) {
    return (size + alignment - 1) & ~(alignment - 1);
}

/* Verificar si está alineado */
static inline int adead_is_aligned(const void* ptr, usize alignment) {
    return ((usize)ptr & (alignment - 1)) == 0;
}

#ifdef __cplusplus
}
#endif

#endif /* ADEAD_MEMORY_H */
