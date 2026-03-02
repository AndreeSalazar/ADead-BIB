/*
 * FastOS v2.0 — 64-bit Boot Transition
 * Handles transition from real mode to long mode (64-bit)
 * Pure C implementation for ADead-BIB
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* GDT Entries for 64-bit mode */
typedef struct {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t  base_mid;
    uint8_t  access;
    uint8_t  granularity;
    uint8_t  base_high;
} __packed gdt_entry_t;

typedef struct {
    uint16_t limit;
    uint64_t base;
} __packed gdt_ptr_t;

/* GDT with 64-bit segments */
static gdt_entry_t gdt64[7] __attribute__((aligned(16)));
static gdt_ptr_t gdt64_ptr;

/* Page tables for identity mapping (4-level paging) */
static uint64_t pml4[512] __attribute__((aligned(4096)));
static uint64_t pdpt[512] __attribute__((aligned(4096)));
static uint64_t pd[512]   __attribute__((aligned(4096)));
static uint64_t pt[512]   __attribute__((aligned(4096)));

/* GDT segment selectors */
#define GDT_NULL        0x00
#define GDT_KERNEL_CODE 0x08
#define GDT_KERNEL_DATA 0x10
#define GDT_USER_CODE   0x18
#define GDT_USER_DATA   0x20
#define GDT_TSS         0x28

/* Set GDT entry */
static void gdt_set_entry(int num, uint32_t base, uint32_t limit,
                          uint8_t access, uint8_t gran) {
    gdt64[num].base_low = base & 0xFFFF;
    gdt64[num].base_mid = (base >> 16) & 0xFF;
    gdt64[num].base_high = (base >> 24) & 0xFF;
    gdt64[num].limit_low = limit & 0xFFFF;
    gdt64[num].granularity = ((limit >> 16) & 0x0F) | (gran & 0xF0);
    gdt64[num].access = access;
}

/* Setup GDT for 64-bit mode */
void setup_gdt64(void) {
    /* Null descriptor */
    gdt_set_entry(0, 0, 0, 0, 0);
    
    /* Kernel code segment (64-bit) */
    /* Access: Present, Ring 0, Code, Execute/Read */
    /* Granularity: Long mode, 4KB granularity */
    gdt_set_entry(1, 0, 0xFFFFF, 0x9A, 0xAF);
    
    /* Kernel data segment */
    gdt_set_entry(2, 0, 0xFFFFF, 0x92, 0xCF);
    
    /* User code segment (64-bit) */
    gdt_set_entry(3, 0, 0xFFFFF, 0xFA, 0xAF);
    
    /* User data segment */
    gdt_set_entry(4, 0, 0xFFFFF, 0xF2, 0xCF);
    
    /* TSS (will be set up later) */
    gdt_set_entry(5, 0, 0, 0, 0);
    gdt_set_entry(6, 0, 0, 0, 0);  /* TSS high */
    
    gdt64_ptr.limit = sizeof(gdt64) - 1;
    gdt64_ptr.base = (uint64_t)&gdt64;
}

/* Setup 4-level page tables for identity mapping */
void setup_paging64(void) {
    /* Clear all tables */
    for (int i = 0; i < 512; i++) {
        pml4[i] = 0;
        pdpt[i] = 0;
        pd[i] = 0;
        pt[i] = 0;
    }
    
    /* PML4[0] -> PDPT */
    pml4[0] = ((uint64_t)&pdpt) | 0x03;  /* Present + Writable */
    
    /* PDPT[0] -> PD */
    pdpt[0] = ((uint64_t)&pd) | 0x03;
    
    /* PD: Use 2MB huge pages for first 1GB */
    for (int i = 0; i < 512; i++) {
        /* 2MB pages: Present + Writable + Huge (PS bit) */
        pd[i] = (i * 0x200000ULL) | 0x83;
    }
    
    /* Alternative: 4KB pages for first 2MB (more control) */
    /* pd[0] = ((uint64_t)&pt) | 0x03; */
    /* for (int i = 0; i < 512; i++) { */
    /*     pt[i] = (i * 0x1000ULL) | 0x03; */
    /* } */
}

/* Check if CPU supports long mode */
int cpu_supports_long_mode(void) {
    uint32_t eax, ebx, ecx, edx;
    
    /* Check for extended CPUID */
    __asm__ volatile(
        "mov $0x80000000, %%eax\n"
        "cpuid\n"
        : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx)
    );
    
    if (eax < 0x80000001) {
        return 0;  /* Extended CPUID not supported */
    }
    
    /* Check for long mode support */
    __asm__ volatile(
        "mov $0x80000001, %%eax\n"
        "cpuid\n"
        : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx)
    );
    
    return (edx & (1 << 29)) != 0;  /* LM bit */
}

/* Enable PAE (Physical Address Extension) */
void enable_pae(void) {
    uint64_t cr4;
    __asm__ volatile("mov %%cr4, %0" : "=r"(cr4));
    cr4 |= (1 << 5);  /* PAE bit */
    __asm__ volatile("mov %0, %%cr4" : : "r"(cr4));
}

/* Load CR3 with PML4 address */
void load_cr3(void) {
    __asm__ volatile("mov %0, %%cr3" : : "r"((uint64_t)&pml4));
}

/* Enable long mode in EFER MSR */
void enable_long_mode_efer(void) {
    uint32_t eax, edx;
    
    /* Read EFER */
    __asm__ volatile(
        "mov $0xC0000080, %%ecx\n"
        "rdmsr\n"
        : "=a"(eax), "=d"(edx)
        : : "ecx"
    );
    
    /* Set LME (Long Mode Enable) bit */
    eax |= (1 << 8);
    
    /* Write EFER */
    __asm__ volatile(
        "mov $0xC0000080, %%ecx\n"
        "wrmsr\n"
        : : "a"(eax), "d"(edx) : "ecx"
    );
}

/* Enable paging (activates long mode if LME is set) */
void enable_paging(void) {
    uint64_t cr0;
    __asm__ volatile("mov %%cr0, %0" : "=r"(cr0));
    cr0 |= (1 << 31);  /* PG bit */
    cr0 |= (1 << 0);   /* PE bit (should already be set) */
    __asm__ volatile("mov %0, %%cr0" : : "r"(cr0));
}

/* Load GDT */
void load_gdt64(void) {
    __asm__ volatile("lgdt %0" : : "m"(gdt64_ptr));
}

/* Jump to 64-bit code segment */
void jump_to_kernel64(uint64_t entry) {
    /* Far jump to 64-bit code segment */
    __asm__ volatile(
        "push %0\n"       /* Code segment selector */
        "push %1\n"       /* Entry point */
        "retfq\n"         /* Far return = far jump */
        : : "r"((uint64_t)GDT_KERNEL_CODE), "r"(entry)
    );
}

/* Full transition to 64-bit mode */
int transition_to_long_mode(uint64_t kernel_entry) {
    kprintf("[BOOT64] Checking CPU capabilities...\n");
    
    if (!cpu_supports_long_mode()) {
        kprintf("[BOOT64] ERROR: CPU does not support 64-bit mode!\n");
        return -1;
    }
    kprintf("[BOOT64] Long mode supported\n");
    
    kprintf("[BOOT64] Setting up GDT...\n");
    setup_gdt64();
    
    kprintf("[BOOT64] Setting up page tables...\n");
    setup_paging64();
    
    kprintf("[BOOT64] Enabling PAE...\n");
    enable_pae();
    
    kprintf("[BOOT64] Loading CR3...\n");
    load_cr3();
    
    kprintf("[BOOT64] Enabling long mode in EFER...\n");
    enable_long_mode_efer();
    
    kprintf("[BOOT64] Loading GDT...\n");
    load_gdt64();
    
    kprintf("[BOOT64] Enabling paging...\n");
    enable_paging();
    
    kprintf("[BOOT64] Jumping to 64-bit kernel at 0x%016llX\n", kernel_entry);
    
    /* Disable interrupts before jump */
    cli();
    
    /* Jump to 64-bit kernel */
    jump_to_kernel64(kernel_entry);
    
    /* Should never reach here */
    return -1;
}

/* 64-bit kernel entry point */
void kernel64_main(void) {
    /* We're now in 64-bit mode! */
    
    /* Reload segment registers */
    __asm__ volatile(
        "mov %0, %%ds\n"
        "mov %0, %%es\n"
        "mov %0, %%fs\n"
        "mov %0, %%gs\n"
        "mov %0, %%ss\n"
        : : "r"((uint16_t)GDT_KERNEL_DATA)
    );
    
    /* Initialize 64-bit kernel components */
    /* ... */
    
    /* Halt */
    while (1) {
        hlt();
    }
}
