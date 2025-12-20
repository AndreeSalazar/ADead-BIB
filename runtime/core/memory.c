/**
 * ADead-BIB Universal Runtime - Memory Manager Implementation
 * ============================================================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with love in Peru
 */

#include "memory.h"
#include <stdlib.h>
#include <string.h>

/* ============================================================
 * Arena Allocator Implementation
 * ============================================================ */

ADeadError adead_arena_init(ADeadArena* arena, usize capacity) {
    if (!arena || capacity == 0) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    arena->base = (u8*)malloc(capacity);
    if (!arena->base) {
        return ADEAD_ERROR_OUT_OF_MEMORY;
    }
    
    arena->offset = 0;
    arena->capacity = capacity;
    arena->peak = 0;
    
    return ADEAD_OK;
}

void adead_arena_destroy(ADeadArena* arena) {
    if (arena && arena->base) {
        free(arena->base);
        arena->base = NULL;
        arena->offset = 0;
        arena->capacity = 0;
        arena->peak = 0;
    }
}

void* adead_arena_alloc(ADeadArena* arena, usize size, usize alignment) {
    if (!arena || !arena->base || size == 0) {
        return NULL;
    }
    
    /* Alinear offset */
    usize aligned_offset = adead_align_up(arena->offset, alignment);
    
    /* Verificar espacio */
    if (aligned_offset + size > arena->capacity) {
        return NULL;  /* Sin espacio */
    }
    
    void* ptr = arena->base + aligned_offset;
    arena->offset = aligned_offset + size;
    
    /* Actualizar peak */
    if (arena->offset > arena->peak) {
        arena->peak = arena->offset;
    }
    
    return ptr;
}

void adead_arena_reset(ADeadArena* arena) {
    if (arena) {
        arena->offset = 0;
    }
}

usize adead_arena_used(const ADeadArena* arena) {
    return arena ? arena->offset : 0;
}

usize adead_arena_available(const ADeadArena* arena) {
    return arena ? (arena->capacity - arena->offset) : 0;
}

/* ============================================================
 * Pool Allocator Implementation
 * ============================================================ */

ADeadError adead_pool_init(ADeadPool* pool, usize block_size, usize num_blocks) {
    if (!pool || block_size == 0 || num_blocks == 0) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    /* Asegurar que block_size sea al menos sizeof(ADeadPoolBlock) */
    if (block_size < sizeof(ADeadPoolBlock)) {
        block_size = sizeof(ADeadPoolBlock);
    }
    
    /* Alinear block_size */
    block_size = adead_align_up(block_size, ADEAD_ALIGNMENT);
    
    usize total_size = block_size * num_blocks;
    pool->base = (u8*)malloc(total_size);
    if (!pool->base) {
        return ADEAD_ERROR_OUT_OF_MEMORY;
    }
    
    pool->block_size = block_size;
    pool->capacity = num_blocks;
    pool->used = 0;
    
    /* Construir free list */
    pool->free_list = NULL;
    for (usize i = 0; i < num_blocks; i++) {
        ADeadPoolBlock* block = (ADeadPoolBlock*)(pool->base + i * block_size);
        block->next = pool->free_list;
        pool->free_list = block;
    }
    
    return ADEAD_OK;
}

void adead_pool_destroy(ADeadPool* pool) {
    if (pool && pool->base) {
        free(pool->base);
        pool->base = NULL;
        pool->free_list = NULL;
        pool->block_size = 0;
        pool->capacity = 0;
        pool->used = 0;
    }
}

void* adead_pool_alloc(ADeadPool* pool) {
    if (!pool || !pool->free_list) {
        return NULL;
    }
    
    ADeadPoolBlock* block = pool->free_list;
    pool->free_list = block->next;
    pool->used++;
    
    return (void*)block;
}

void adead_pool_free(ADeadPool* pool, void* ptr) {
    if (!pool || !ptr) {
        return;
    }
    
    /* Verificar que ptr está dentro del pool */
    u8* p = (u8*)ptr;
    if (p < pool->base || p >= pool->base + pool->block_size * pool->capacity) {
        return;  /* Puntero inválido */
    }
    
    ADeadPoolBlock* block = (ADeadPoolBlock*)ptr;
    block->next = pool->free_list;
    pool->free_list = block;
    pool->used--;
}

void adead_pool_reset(ADeadPool* pool) {
    if (!pool || !pool->base) {
        return;
    }
    
    /* Reconstruir free list */
    pool->free_list = NULL;
    pool->used = 0;
    
    for (usize i = 0; i < pool->capacity; i++) {
        ADeadPoolBlock* block = (ADeadPoolBlock*)(pool->base + i * pool->block_size);
        block->next = pool->free_list;
        pool->free_list = block;
    }
}

/* ============================================================
 * Memory Manager Implementation
 * ============================================================ */

ADeadError adead_memory_init(ADeadMemoryManager* mm, usize scratch_size, usize pool_size) {
    if (!mm) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    memset(mm, 0, sizeof(ADeadMemoryManager));
    
    /* Inicializar scratch arena */
    ADeadError err = adead_arena_init(&mm->scratch, scratch_size);
    if (err != ADEAD_OK) {
        return err;
    }
    
    /* Inicializar tensor pool (bloques de 256 bytes) */
    err = adead_pool_init(&mm->tensor_pool, 256, pool_size / 256);
    if (err != ADEAD_OK) {
        adead_arena_destroy(&mm->scratch);
        return err;
    }
    
    return ADEAD_OK;
}

void adead_memory_destroy(ADeadMemoryManager* mm) {
    if (mm) {
        adead_arena_destroy(&mm->scratch);
        adead_pool_destroy(&mm->tensor_pool);
        memset(mm, 0, sizeof(ADeadMemoryManager));
    }
}

void* adead_memory_scratch(ADeadMemoryManager* mm, usize size) {
    if (!mm) {
        return NULL;
    }
    
    void* ptr = adead_arena_alloc(&mm->scratch, size, ADEAD_ALIGNMENT);
    if (ptr) {
        mm->total_alloc += size;
        mm->alloc_count++;
    }
    
    return ptr;
}

void adead_memory_scratch_reset(ADeadMemoryManager* mm) {
    if (mm) {
        mm->total_freed += mm->scratch.offset;
        mm->free_count++;
        adead_arena_reset(&mm->scratch);
    }
}

void adead_memory_stats(const ADeadMemoryManager* mm,
                        usize* total_alloc, usize* total_freed,
                        u32* alloc_count, u32* free_count) {
    if (!mm) {
        return;
    }
    
    if (total_alloc) *total_alloc = mm->total_alloc;
    if (total_freed) *total_freed = mm->total_freed;
    if (alloc_count) *alloc_count = mm->alloc_count;
    if (free_count) *free_count = mm->free_count;
}
