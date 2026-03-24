/*
 * FastOS v2.0 — Kernel Heap (kmalloc/kfree)
 * First-fit allocator with block headers
 * Heap starts at 0x200000 (2MB) — safe above kernel at 1MB
 * Size: 8MB initial (expandable with PMM)
 *
 * Block header:
 *   [size:8][magic:4][free:4] = 16 bytes, aligned to 16
 * Allocation returns pointer after header.
 * Free blocks are coalesced on kfree.
 */

#include "include/kernel.h"

#define HEAP_START  0x200000ULL
#define HEAP_SIZE   (8ULL * 1024 * 1024)
#define HEAP_END    (HEAP_START + HEAP_SIZE)
#define HEAP_ALIGN  16
#define HEAP_MAGIC  0xDEADC0DEUL

typedef struct block_header {
    size_t   size;          /* Usable bytes (excluding header) */
    uint32_t magic;
    uint32_t free;          /* 1 = free, 0 = allocated */
    struct block_header *next;
} __packed block_header_t;

static block_header_t *heap_head = NULL;
static size_t heap_used = 0;

void heap_init(void) {
    heap_head = (block_header_t *)HEAP_START;
    heap_head->size  = HEAP_SIZE - sizeof(block_header_t);
    heap_head->magic = HEAP_MAGIC;
    heap_head->free  = 1;
    heap_head->next  = NULL;
    heap_used = 0;
}

/* Split a free block if it's big enough to hold the allocation + a new header */
static void heap_split(block_header_t *block, size_t size) {
    size_t remaining = block->size - size - sizeof(block_header_t);
    if (remaining < HEAP_ALIGN + sizeof(block_header_t)) return;

    block_header_t *new_block = (block_header_t *)((uint8_t *)(block + 1) + size);
    new_block->size  = remaining;
    new_block->magic = HEAP_MAGIC;
    new_block->free  = 1;
    new_block->next  = block->next;

    block->size = size;
    block->next = new_block;
}

void *kmalloc(size_t size) {
    if (size == 0) return NULL;
    size = ALIGN_UP(size, HEAP_ALIGN);

    block_header_t *block = heap_head;
    while (block) {
        if (block->magic != HEAP_MAGIC) {
            kernel_panic("kmalloc: heap corruption");
        }
        if (block->free && block->size >= size) {
            heap_split(block, size);
            block->free = 0;
            heap_used += block->size;
            return (void *)(block + 1);
        }
        block = block->next;
    }
    return NULL;  /* Out of memory */
}

void *kzalloc(size_t size) {
    void *ptr = kmalloc(size);
    if (ptr) memset(ptr, 0, size);
    return ptr;
}

/* Coalesce adjacent free blocks */
static void heap_coalesce(void) {
    block_header_t *block = heap_head;
    while (block && block->next) {
        if (block->free && block->next->free) {
            block->size += sizeof(block_header_t) + block->next->size;
            block->next = block->next->next;
        } else {
            block = block->next;
        }
    }
}

void kfree(void *ptr) {
    if (!ptr) return;

    block_header_t *block = (block_header_t *)ptr - 1;
    if (block->magic != HEAP_MAGIC) {
        kernel_panic("kfree: invalid pointer or heap corruption");
    }
    if (block->free) {
        kernel_panic("kfree: double free detected");
    }

    heap_used -= block->size;
    block->free = 1;
    heap_coalesce();
}

size_t kheap_used(void) {
    return heap_used;
}

size_t kheap_free(void) {
    return HEAP_SIZE - heap_used;
}
