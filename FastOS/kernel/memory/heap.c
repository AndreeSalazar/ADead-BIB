/* ============================================================
 * FastOS Flat Heap — Physical Memory Allocator Implementation
 * ============================================================
 * Bitmap-based flat physical allocator. No MMU, no paging.
 * Direct physical addresses returned to caller.
 *
 * Design decisions:
 *   - 32KB block granularity (balance fragmentation vs overhead)
 *   - First-fit allocation (simple, O(n) worst case)
 *   - Bitmap: 1 byte per block (0=free, 1=used)
 *   - Free is O(1): just clear bitmap entries
 *   - No coalescing needed — bitmap handles it naturally
 *   - Multi-block allocations: find N contiguous free blocks
 *
 * Memory safety:
 *   - heap_init() zeros bitmap → all blocks free
 *   - kmalloc() returns 0 on failure (never crashes)
 *   - kfree() validates pointer is within heap range
 *   - No double-free protection (kernel is trusted, Ring 0)
 *
 * Author: Eddi Andreé Salazar Matos — Lima, Perú
 * FastOS v3.1
 * ============================================================ */

/* ============================================================
 * Global heap state
 * ============================================================ */
static FlatHeap kernel_heap;

/* ============================================================
 * heap_init() — Initialize the flat heap
 *
 * Zeros the bitmap, sets base/size.
 * Must be called once during kernel boot, after memory
 * detection (E820) confirms at least 16MB available.
 * ============================================================ */
static void heap_init(void) {
    unsigned int i;

    kernel_heap.base = HEAP_BASE;
    kernel_heap.size = HEAP_SIZE;
    kernel_heap.used = 0;
    kernel_heap.alloc_count = 0;
    kernel_heap.total_allocs = 0;
    kernel_heap.total_frees = 0;

    /* Zero bitmap — all blocks free */
    i = 0;
    while (i < HEAP_BITMAP_SZ) {
        kernel_heap.bitmap[i] = 0;
        i++;
    }

    /* Serial report */
    __outb(0x3F8, 'H'); __outb(0x3F8, 'E'); __outb(0x3F8, 'A');
    __outb(0x3F8, 'P'); __outb(0x3F8, ':'); __outb(0x3F8, ' ');
    __outb(0x3F8, 'O'); __outb(0x3F8, 'K');
    __outb(0x3F8, ' ');
    /* "14MB @ 0x200000" */
    __outb(0x3F8, '1'); __outb(0x3F8, '4'); __outb(0x3F8, 'M');
    __outb(0x3F8, 'B'); __outb(0x3F8, ' '); __outb(0x3F8, '@');
    __outb(0x3F8, ' '); __outb(0x3F8, '0'); __outb(0x3F8, 'x');
    __outb(0x3F8, '2'); __outb(0x3F8, '0'); __outb(0x3F8, '0');
    __outb(0x3F8, '0'); __outb(0x3F8, '0'); __outb(0x3F8, '0');
    __outb(0x3F8, 13); __outb(0x3F8, 10);
}

/* ============================================================
 * kmalloc() — Allocate contiguous physical memory
 *
 * Finds the first run of `blocks_needed` contiguous free blocks.
 * Marks them as used in bitmap. Returns physical address.
 *
 * size: requested bytes (rounded up to 32KB blocks)
 * Returns: physical address, or 0 (NULL) on failure
 *
 * Note: minimum allocation is 1 block (32KB).
 * For small allocations, caller should sub-allocate from
 * a larger kmalloc'd region (slab pattern).
 * ============================================================ */
static void *kmalloc(unsigned int size) {
    unsigned int blocks_needed;
    unsigned int start;
    unsigned int run;
    unsigned int i;

    if (size == 0) return (void *)0;

    /* Round up to block granularity */
    blocks_needed = (size + HEAP_BLOCK_SIZE - 1) / HEAP_BLOCK_SIZE;
    if (blocks_needed > HEAP_BLOCKS) return (void *)0;

    /* First-fit scan: find contiguous free blocks */
    start = 0;
    while (start + blocks_needed <= HEAP_BLOCKS) {
        /* Check if this run of blocks is all free */
        run = 0;
        while (run < blocks_needed) {
            if (kernel_heap.bitmap[start + run] != 0) break;
            run++;
        }

        if (run == blocks_needed) {
            /* Found! Mark blocks as used */
            i = 0;
            while (i < blocks_needed) {
                kernel_heap.bitmap[start + i] = 1;
                i++;
            }

            kernel_heap.used = kernel_heap.used + (blocks_needed * HEAP_BLOCK_SIZE);
            kernel_heap.alloc_count++;
            kernel_heap.total_allocs++;

            return (void *)((unsigned long long)(kernel_heap.base + start * HEAP_BLOCK_SIZE));
        }

        /* Skip past the blocking block */
        start = start + run + 1;
    }

    /* No contiguous space found */
    return (void *)0;
}

/* ============================================================
 * kfree() — Free previously allocated memory
 *
 * Calculates which block(s) the pointer belongs to and
 * clears the bitmap entry. Caller must pass the original
 * pointer from kmalloc() — not an offset within the block.
 *
 * Currently frees only 1 block (32KB). For multi-block
 * frees, caller must track original size and call kfree_sized().
 * ============================================================ */
static void kfree(void *ptr) {
    unsigned int addr;
    unsigned int block_idx;

    if (ptr == (void *)0) return;

    addr = (unsigned int)((unsigned long long)ptr);

    /* Validate pointer is within heap range */
    if (addr < kernel_heap.base) return;
    if (addr >= kernel_heap.base + kernel_heap.size) return;

    /* Calculate block index */
    block_idx = (addr - kernel_heap.base) / HEAP_BLOCK_SIZE;
    if (block_idx >= HEAP_BLOCKS) return;

    /* Free the block */
    if (kernel_heap.bitmap[block_idx] != 0) {
        kernel_heap.bitmap[block_idx] = 0;
        if (kernel_heap.used >= HEAP_BLOCK_SIZE) {
            kernel_heap.used = kernel_heap.used - HEAP_BLOCK_SIZE;
        }
        if (kernel_heap.alloc_count > 0) {
            kernel_heap.alloc_count--;
        }
        kernel_heap.total_frees++;
    }
}

/* ============================================================
 * Query functions
 * ============================================================ */

/* Returns total bytes currently allocated */
static unsigned int kmem_used(void) {
    return kernel_heap.used;
}

/* Returns total bytes available for allocation */
static unsigned int kmem_free(void) {
    return kernel_heap.size - kernel_heap.used;
}

/* Returns number of blocks currently in use */
static unsigned int kmem_blocks_used(void) {
    unsigned int i;
    unsigned int count;
    count = 0;
    i = 0;
    while (i < HEAP_BLOCKS) {
        if (kernel_heap.bitmap[i] != 0) count++;
        i++;
    }
    return count;
}

/* Returns number of blocks currently free */
static unsigned int kmem_blocks_free(void) {
    return HEAP_BLOCKS - kmem_blocks_used();
}
