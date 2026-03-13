/* ============================================================
 * FastOS Flat Heap — Physical Memory Allocator Header
 * ============================================================
 * Flat physical heap for FastOS kernel. No MMU, no paging —
 * direct physical address allocation via bitmap.
 *
 * Memory layout:
 *   0x000000 - 0x0FFFFF  → Reserved (BIOS, VGA, boot)
 *   0x100000 - 0x1FFFFF  → Kernel code/data (1MB)
 *   0x200000 - 0xFFFFFF  → Heap region (14MB available)
 *                           512 blocks × 32KB = 16MB addressable
 *   0x400000 - 0x3FFFFF  → (overlap: GUI framebuffer back buffer)
 *   0x700000 - 0x7FFFFF  → (overlap: GUI icon cache)
 *   0x800000 - 0xFFFFFF  → (overlap: GUI window surfaces)
 *
 * Block size: 32KB (0x8000 bytes)
 *   — Good balance: not too small (fragmentation), not too large
 *   — 512 blocks covers full 16MB addressable range
 *   — Each bit in bitmap[512] = 1 byte = 8 sub-blocks (future)
 *     Currently: 1 byte = 1 block (0=free, 1=used)
 *
 * Allocation strategy: first-fit sequential scan.
 * No coalescing needed — bitmap makes free() O(1).
 *
 * Author: Eddi Andreé Salazar Matos — Lima, Perú
 * FastOS v3.1
 * ============================================================ */

#ifndef HEAP_H
#define HEAP_H

/* Heap configuration */
#define HEAP_BASE       0x200000    /* 2MB — after kernel */
#define HEAP_SIZE       0xE00000    /* 14MB available */
#define HEAP_BLOCK_SIZE 0x8000      /* 32KB per block */
#define HEAP_BLOCKS     448         /* 14MB / 32KB = 448 blocks */
#define HEAP_BITMAP_SZ  448         /* 1 byte per block */

/* Heap state */
typedef struct {
    unsigned int base;              /* HEAP_BASE */
    unsigned int size;              /* HEAP_SIZE */
    unsigned int used;              /* bytes currently allocated */
    unsigned int alloc_count;       /* number of active allocations */
    unsigned int total_allocs;      /* lifetime allocation count */
    unsigned int total_frees;       /* lifetime free count */
    unsigned char bitmap[HEAP_BITMAP_SZ]; /* 0=free, 1=used per block */
} FlatHeap;

/* ============================================================
 * Function Prototypes
 * ============================================================ */

/* Initialize heap — call once at boot */
static void  heap_init(void);

/* Allocate contiguous physical memory (32KB aligned)
 * Returns physical address, or 0 on failure */
static void *kmalloc(unsigned int size);

/* Free previously allocated memory */
static void  kfree(void *ptr);

/* Query functions */
static unsigned int kmem_used(void);
static unsigned int kmem_free(void);
static unsigned int kmem_blocks_used(void);
static unsigned int kmem_blocks_free(void);

#endif /* HEAP_H */
