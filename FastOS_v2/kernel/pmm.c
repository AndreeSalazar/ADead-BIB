/*
 * FastOS v2.0 — Physical Memory Manager (PMM)
 * Bitmap allocator: 1 bit per 4KB page
 * Reads E820 map left by stage2 at 0x8000
 *
 * Memory layout:
 *   0x000000 - 0x0FFFFF: Low memory (BIOS, VGA, boot code) — reserved
 *   0x100000 - kernel_end: Kernel code/data — reserved
 *   After kernel: Available for allocation
 *
 * The bitmap itself is placed right after __kernel_end
 */

#include "include/kernel.h"

#define PAGE_SIZE      4096
#define PAGES_PER_BYTE 8

/* Max supported: 4GB = 1048576 pages = 128KB bitmap */
#define MAX_PAGES      (4ULL * 1024 * 1024 * 1024 / PAGE_SIZE)
#define BITMAP_SIZE    (MAX_PAGES / PAGES_PER_BYTE)

/* Bitmap: 0 = free, 1 = used/reserved */
static uint8_t *pmm_bitmap;
static uint64_t pmm_total_pages;
static uint64_t pmm_used_pages;

static inline void pmm_set_bit(uint64_t page) {
    pmm_bitmap[page / 8] |= (1 << (page % 8));
}

static inline void pmm_clear_bit(uint64_t page) {
    pmm_bitmap[page / 8] &= ~(1 << (page % 8));
}

static inline int pmm_test_bit(uint64_t page) {
    return (pmm_bitmap[page / 8] >> (page % 8)) & 1;
}

void pmm_init(e820_entry_t *map, uint32_t count) {
    /* Place bitmap right after kernel in memory */
    pmm_bitmap = (uint8_t *)ALIGN_UP((uint64_t)__kernel_end, PAGE_SIZE);
    pmm_total_pages = MAX_PAGES;
    pmm_used_pages = MAX_PAGES;  /* Mark all as used initially */

    /* Set all bits to 1 (all pages reserved) */
    memset(pmm_bitmap, 0xFF, BITMAP_SIZE);

    /* Walk E820 map: free usable regions */
    for (uint32_t i = 0; i < count; i++) {
        if (map[i].type != E820_USABLE) continue;

        uint64_t base = ALIGN_UP(map[i].base, PAGE_SIZE);
        uint64_t end  = ALIGN_DOWN(map[i].base + map[i].length, PAGE_SIZE);

        for (uint64_t addr = base; addr < end && addr / PAGE_SIZE < MAX_PAGES; addr += PAGE_SIZE) {
            pmm_clear_bit(addr / PAGE_SIZE);
            pmm_used_pages--;
        }
    }

    /* Re-reserve critical regions */

    /* Low memory 0-1MB */
    for (uint64_t p = 0; p < 256; p++) {
        if (!pmm_test_bit(p)) { pmm_set_bit(p); pmm_used_pages++; }
    }

    /* Kernel + bitmap region */
    uint64_t kern_start_page = (uint64_t)__kernel_start / PAGE_SIZE;
    uint64_t bitmap_end = (uint64_t)pmm_bitmap + BITMAP_SIZE;
    uint64_t kern_end_page = ALIGN_UP(bitmap_end, PAGE_SIZE) / PAGE_SIZE;
    for (uint64_t p = kern_start_page; p < kern_end_page && p < MAX_PAGES; p++) {
        if (!pmm_test_bit(p)) { pmm_set_bit(p); pmm_used_pages++; }
    }
}

/* Allocate a single 4KB page — returns physical address or NULL */
void *pmm_alloc_page(void) {
    /* Start search after 1MB to avoid low memory */
    for (uint64_t i = 256; i < pmm_total_pages; i++) {
        if (!pmm_test_bit(i)) {
            pmm_set_bit(i);
            pmm_used_pages++;
            void *addr = (void *)(i * PAGE_SIZE);
            memset(addr, 0, PAGE_SIZE);  /* Zero the page */
            return addr;
        }
    }
    return NULL;
}

/* Free a single 4KB page */
void pmm_free_page(void *addr) {
    uint64_t page = (uint64_t)addr / PAGE_SIZE;
    if (page < MAX_PAGES && pmm_test_bit(page)) {
        pmm_clear_bit(page);
        pmm_used_pages--;
    }
}

uint64_t pmm_get_free_pages(void) {
    return pmm_total_pages - pmm_used_pages;
}

uint64_t pmm_get_total_pages(void) {
    return pmm_total_pages;
}
