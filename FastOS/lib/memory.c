/*
 * lib/memory.c — Allocador de Heap para el Kernel FastOS
 *
 * Bump allocator inicial (fase 1 del boot).
 * Una vez que memory_init() determina las regiones E820 usables,
 * el heap empieza justo despues del kernel (0x200000 en adelante).
 *
 * Filosofia FastOS: minimal. Sin complejidad hasta que se necesite.
 * Fase 1: Bump allocator — O(1) alloc, kfree no libera nada.
 * Fase 2 (futuro): free list con coalescencia de bloques.
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ─── Heap Layout ─── */

/* El kernel se carga en 0x100000 y ocupa hasta ~150KB.
 * El heap empieza en 0x200000 (2MB) — bien por encima del kernel. */
#define HEAP_START  0x200000ULL
#define HEAP_SIZE   (8ULL * 1024 * 1024)   /* 8MB inicial */
#define HEAP_END    (HEAP_START + HEAP_SIZE)

/* Alineamiento de 16 bytes (requerido para SSE/AVX y estructuras packed) */
#define HEAP_ALIGN  16

/* ─── Estado del Bump Allocator ─── */
static uint64_t heap_next = HEAP_START;
static uint64_t heap_peak = HEAP_START;   /* Para diagnostico */

/* ─── Block header (para kfree eventual) ─── */
typedef struct {
    size_t   size;        /* Bytes solicitados */
    uint32_t magic;       /* 0xDEADC0DE — integridad */
    uint32_t reserved;
} __packed heap_block_t;

#define HEAP_MAGIC 0xDEADC0DEUL

/* ─── kmalloc ─── */

void *kmalloc(size_t size) {
    if (size == 0) return NULL;

    /* Alinear a 16 bytes */
    size = ALIGN_UP(size, HEAP_ALIGN);

    /* Espacio para el header */
    size_t total = size + sizeof(heap_block_t);

    if (heap_next + total > HEAP_END) {
        KERNEL_PANIC(7, "kmalloc: heap exhausted");
    }

    heap_block_t *hdr = (heap_block_t *)heap_next;
    hdr->size    = size;
    hdr->magic   = HEAP_MAGIC;
    hdr->reserved = 0;

    heap_next += total;
    if (heap_next > heap_peak) heap_peak = heap_next;

    return (void *)(hdr + 1);   /* Puntero despues del header */
}

/* ─── kzalloc — kmalloc + zero ─── */

void *kzalloc(size_t size) {
    void *ptr = kmalloc(size);
    if (ptr) {
        uint8_t *p = (uint8_t *)ptr;
        for (size_t i = 0; i < size; i++) p[i] = 0;
    }
    return ptr;
}

/* ─── krealloc (bump: alloc new + copy) ─── */

void *krealloc(void *ptr, size_t new_size) {
    if (!ptr) return kmalloc(new_size);
    if (new_size == 0) { kfree(ptr); return NULL; }

    /* Recuperar el header para saber el tamano original */
    heap_block_t *hdr = (heap_block_t *)ptr - 1;
    if (hdr->magic != HEAP_MAGIC) {
        KERNEL_PANIC(7, "krealloc: block magic corrupted");
    }

    void *new_ptr = kmalloc(new_size);
    if (!new_ptr) return NULL;

    /* Copiar hasta min(old_size, new_size) */
    size_t copy_size = hdr->size < new_size ? hdr->size : new_size;
    uint8_t *src = (uint8_t *)ptr;
    uint8_t *dst = (uint8_t *)new_ptr;
    for (size_t i = 0; i < copy_size; i++) dst[i] = src[i];

    /* Bump allocator: no podemos liberar el bloque viejo todavia */
    return new_ptr;
}

/* ─── kfree ─── */

void kfree(void *ptr) {
    if (!ptr) return;

    /* Verificar magic — si falla = corrupcjon de heap */
    heap_block_t *hdr = (heap_block_t *)ptr - 1;
    if (hdr->magic != HEAP_MAGIC) {
        KERNEL_PANIC(7, "kfree: heap corruption (bad magic)");
    }

    /* Marcamos el magic como invalido para detectar double-free.
     * 0xFEEDDEAD = "FEED DEAD" — bloque liberado */
    hdr->magic = 0xFEEDDEADUL;

    /* TODO Fase 2: insertar en free list y coalescer bloques adyacentes */
}

/* ─── Operaciones de memoria del kernel ─── */

void *kmemcpy(void *dest, const void *src, size_t n) {
    uint8_t *d = (uint8_t *)dest;
    const uint8_t *s = (const uint8_t *)src;
    while (n--) *d++ = *s++;
    return dest;
}

void *kmemset(void *s, int c, size_t n) {
    uint8_t *p = (uint8_t *)s;
    while (n--) *p++ = (uint8_t)c;
    return s;
}

int kmemcmp(const void *s1, const void *s2, size_t n) {
    const uint8_t *a = (const uint8_t *)s1;
    const uint8_t *b = (const uint8_t *)s2;
    while (n--) {
        if (*a != *b) return (int)*a - (int)*b;
        a++; b++;
    }
    return 0;
}

/* ─── String helpers del kernel ─── */

size_t kstrlen(const char *s) {
    size_t n = 0; while (s[n]) n++; return n;
}

char *kstrcpy(char *dest, const char *src) {
    char *d = dest; while ((*d++ = *src++)); return dest;
}

char *kstrncpy(char *dest, const char *src, size_t n) {
    size_t i;
    for (i = 0; i < n && src[i]; i++) dest[i] = src[i];
    for (; i < n; i++) dest[i] = '\0';
    return dest;
}

int kstrcmp(const char *s1, const char *s2) {
    while (*s1 && *s1 == *s2) { s1++; s2++; }
    return (unsigned char)*s1 - (unsigned char)*s2;
}

int kstrncmp(const char *s1, const char *s2, size_t n) {
    while (n && *s1 && *s1 == *s2) { s1++; s2++; n--; }
    if (!n) return 0;
    return (unsigned char)*s1 - (unsigned char)*s2;
}

char *kstrcat(char *dest, const char *src) {
    char *d = dest; while (*d) d++;
    while ((*d++ = *src++)); return dest;
}

/* ─── Diagnostico del heap ─── */
void heap_dump(void) {
    kprintf("[HEAP] Start=0x%016llX  Next=0x%016llX  Peak=0x%016llX\n",
            (unsigned long long)HEAP_START,
            (unsigned long long)heap_next,
            (unsigned long long)heap_peak);
    kprintf("[HEAP] Used=%llu bytes  Free=%llu bytes\n",
            (unsigned long long)(heap_next - HEAP_START),
            (unsigned long long)(HEAP_END - heap_next));
}
