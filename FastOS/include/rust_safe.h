/*
 * FastOS — Rust Safety Layer Interface
 * 
 * Header C para llamar funciones Rust seguras desde el kernel.
 * Rust protege los puntos críticos de memoria:
 * - Memory Manager (VMM)
 * - Heap Allocator
 * - Page Tables
 * - Buffer Management
 * 
 * C domina el kernel, Rust protege la memoria.
 */

#ifndef _FASTOS_RUST_SAFE_H
#define _FASTOS_RUST_SAFE_H

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================
 * Memory — Direcciones seguras
 * ============================================================ */

/* Crear PhysAddr validada (retorna 0 si inválida) */
uint64_t rust_phys_addr_new(uint64_t addr);

/* Crear VirtAddr validada (canonical form) */
uint64_t rust_virt_addr_new(uint64_t addr);

/* Alinear dirección hacia abajo a página */
uint64_t rust_align_down(uint64_t addr);

/* Alinear dirección hacia arriba a página */
uint64_t rust_align_up(uint64_t addr);

/* Obtener índices de page table [PML4, PDPT, PD, PT] */
void rust_get_page_indices(uint64_t addr, size_t *out);

/* ============================================================
 * Allocator — Heap seguro
 * ============================================================ */

/* Inicializar heap */
void rust_heap_init(size_t heap_start, size_t heap_size);

/* Allocar memoria (bounds checked) */
void* rust_malloc(size_t size);

/* Allocar memoria alineada */
void* rust_malloc_aligned(size_t size, size_t align);

/* Liberar memoria (double-free safe) */
void rust_free(void *ptr);

/* Allocar y limpiar */
void* rust_calloc(size_t count, size_t size);

/* Realocar memoria */
void* rust_realloc(void *ptr, size_t old_size, size_t new_size);

/* Obtener memoria libre */
size_t rust_heap_free(void);

/* ============================================================
 * Page Table — Paginación segura
 * ============================================================ */

/* Crear entrada de page table */
uint64_t rust_pte_new(uint64_t addr, uint64_t flags);

/* Verificar si entrada está presente */
bool rust_pte_is_present(uint64_t entry);

/* Obtener dirección de entrada */
uint64_t rust_pte_addr(uint64_t entry);

/* Traducir dirección virtual a física */
uint64_t rust_translate(uint64_t pml4, uint64_t virt);

/* ============================================================
 * Buffer — Buffers seguros
 * ============================================================ */

/* Opaque handle para SafeBuffer */
typedef struct SafeBuffer SafeBuffer;

/* Crear buffer seguro */
SafeBuffer* rust_buffer_create(void *ptr, size_t len);

/* Destruir buffer */
void rust_buffer_destroy(SafeBuffer *buffer);

/* Leer byte (bounds checked, retorna -1 si fuera de rango) */
int rust_buffer_get(const SafeBuffer *buffer, size_t index);

/* Escribir byte (bounds checked) */
bool rust_buffer_set(SafeBuffer *buffer, size_t index, uint8_t value);

/* Copiar datos al buffer (bounds checked) */
bool rust_buffer_copy_from(SafeBuffer *buffer, const void *src, 
                           size_t src_len, size_t offset);

/* Copiar datos desde buffer (bounds checked) */
bool rust_buffer_copy_to(const SafeBuffer *buffer, void *dst,
                         size_t dst_len, size_t offset);

/* Llenar buffer con valor */
void rust_buffer_fill(SafeBuffer *buffer, uint8_t value);

/* ============================================================
 * Safe Memory Operations — Previenen buffer overflow
 * ============================================================ */

/* memcpy seguro (previene buffer overflow) */
bool rust_memcpy_safe(void *dst, size_t dst_size, 
                      const void *src, size_t count);

/* memmove seguro (previene buffer overflow) */
bool rust_memmove_safe(void *dst, size_t dst_size,
                       const void *src, size_t count);

/* memset seguro (previene buffer overflow) */
bool rust_memset_safe(void *dst, size_t dst_size,
                      uint8_t value, size_t count);

/* ============================================================
 * Macros de conveniencia
 * ============================================================ */

/* Allocar con tipo */
#define RUST_NEW(type) \
    ((type*)rust_malloc(sizeof(type)))

/* Allocar array con tipo */
#define RUST_NEW_ARRAY(type, count) \
    ((type*)rust_calloc(count, sizeof(type)))

/* Liberar con tipo (para claridad) */
#define RUST_DELETE(ptr) \
    rust_free(ptr)

/* Copiar seguro con verificación de tamaño */
#define RUST_MEMCPY(dst, src, count) \
    rust_memcpy_safe(dst, sizeof(dst), src, count)

/* Set seguro con verificación de tamaño */
#define RUST_MEMSET(dst, value, count) \
    rust_memset_safe(dst, sizeof(dst), value, count)

#ifdef __cplusplus
}
#endif

#endif /* _FASTOS_RUST_SAFE_H */
