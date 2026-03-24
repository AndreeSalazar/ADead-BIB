/*
 * FastOS v2.0 — Virtual Memory Manager (VMM)
 * 4-level page tables (PML4 → PDPT → PD → PT)
 * Currently uses identity mapping set up by stage2 (2MB pages)
 * Provides dynamic page mapping for future use (heap, user space)
 *
 * Page table flags:
 *   Bit 0: Present
 *   Bit 1: Read/Write
 *   Bit 2: User/Supervisor
 *   Bit 7: Page Size (1=2MB for PD entries, 1=1GB for PDPT entries)
 */

#include "include/kernel.h"

#define PTE_PRESENT  BIT(0)
#define PTE_WRITE    BIT(1)
#define PTE_USER     BIT(2)
#define PTE_PS       BIT(7)    /* Page Size: 2MB (PD) or 1GB (PDPT) */

#define PT_ENTRIES   512
#define PAGE_SIZE_4K 0x1000

/* PML4 is at 0x70000 (set by stage2) */
static uint64_t *pml4;

/* Extract page table indices from virtual address */
#define PML4_IDX(v) (((v) >> 39) & 0x1FF)
#define PDPT_IDX(v) (((v) >> 30) & 0x1FF)
#define PD_IDX(v)   (((v) >> 21) & 0x1FF)
#define PT_IDX(v)   (((v) >> 12) & 0x1FF)

void vmm_init(void) {
    /* Read current PML4 from CR3 (set by stage2 to 0x70000) */
    pml4 = (uint64_t *)read_cr3();
}

/* Map a 4KB page: virt → phys with given flags */
void vmm_map_page(uint64_t virt, uint64_t phys, uint64_t flags) {
    uint64_t *table;

    /* PML4 → PDPT */
    table = pml4;
    uint64_t pml4e = table[PML4_IDX(virt)];
    uint64_t *pdpt;
    if (pml4e & PTE_PRESENT) {
        pdpt = (uint64_t *)(pml4e & ~0xFFFULL);
    } else {
        pdpt = (uint64_t *)pmm_alloc_page();
        if (!pdpt) return;
        table[PML4_IDX(virt)] = (uint64_t)pdpt | PTE_PRESENT | PTE_WRITE | flags;
    }

    /* PDPT → PD */
    uint64_t pdpte = pdpt[PDPT_IDX(virt)];
    uint64_t *pd;
    if (pdpte & PTE_PRESENT) {
        if (pdpte & PTE_PS) return;  /* 1GB page, can't subdivide here */
        pd = (uint64_t *)(pdpte & ~0xFFFULL);
    } else {
        pd = (uint64_t *)pmm_alloc_page();
        if (!pd) return;
        pdpt[PDPT_IDX(virt)] = (uint64_t)pd | PTE_PRESENT | PTE_WRITE | flags;
    }

    /* PD → PT */
    uint64_t pde = pd[PD_IDX(virt)];
    uint64_t *pt;
    if (pde & PTE_PRESENT) {
        if (pde & PTE_PS) return;  /* 2MB page, can't subdivide here */
        pt = (uint64_t *)(pde & ~0xFFFULL);
    } else {
        pt = (uint64_t *)pmm_alloc_page();
        if (!pt) return;
        pd[PD_IDX(virt)] = (uint64_t)pt | PTE_PRESENT | PTE_WRITE | flags;
    }

    /* PT → Physical page */
    pt[PT_IDX(virt)] = (phys & ~0xFFFULL) | PTE_PRESENT | flags;

    invlpg((void *)virt);
}

/* Unmap a 4KB page */
void vmm_unmap_page(uint64_t virt) {
    uint64_t *table;

    table = pml4;
    if (!(table[PML4_IDX(virt)] & PTE_PRESENT)) return;
    uint64_t *pdpt = (uint64_t *)(table[PML4_IDX(virt)] & ~0xFFFULL);

    if (!(pdpt[PDPT_IDX(virt)] & PTE_PRESENT)) return;
    if (pdpt[PDPT_IDX(virt)] & PTE_PS) return;
    uint64_t *pd = (uint64_t *)(pdpt[PDPT_IDX(virt)] & ~0xFFFULL);

    if (!(pd[PD_IDX(virt)] & PTE_PRESENT)) return;
    if (pd[PD_IDX(virt)] & PTE_PS) return;
    uint64_t *pt = (uint64_t *)(pd[PD_IDX(virt)] & ~0xFFFULL);

    pt[PT_IDX(virt)] = 0;
    invlpg((void *)virt);
}
