/*
 * FastOS v2.0 — GDT (Global Descriptor Table) for 64-bit Long Mode
 * Reloads GDT in kernel space with proper TSS entry
 *
 * Segments:
 *   0x00: Null descriptor
 *   0x08: Kernel Code 64-bit (L=1, D=0)
 *   0x10: Kernel Data 64-bit
 *   0x18: User Code 64-bit (DPL=3)
 *   0x20: User Data 64-bit (DPL=3)
 *   0x28: TSS descriptor (16 bytes, spans 0x28-0x37)
 */

#include "include/kernel.h"

/* GDT entry: 8 bytes each */
typedef struct {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t  base_mid;
    uint8_t  access;
    uint8_t  granularity;
    uint8_t  base_high;
} __packed gdt_entry_t;

/* TSS for 64-bit Long Mode */
typedef struct {
    uint32_t reserved0;
    uint64_t rsp0;      /* Kernel stack for ring 0 */
    uint64_t rsp1;
    uint64_t rsp2;
    uint64_t reserved1;
    uint64_t ist1;      /* Interrupt Stack Table entries */
    uint64_t ist2;
    uint64_t ist3;
    uint64_t ist4;
    uint64_t ist5;
    uint64_t ist6;
    uint64_t ist7;
    uint64_t reserved2;
    uint16_t reserved3;
    uint16_t iomap_base;
} __packed tss_t;

/* TSS descriptor: 16 bytes (special in 64-bit mode) */
typedef struct {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t  base_mid;
    uint8_t  access;
    uint8_t  granularity;
    uint8_t  base_high;
    uint32_t base_upper;
    uint32_t reserved;
} __packed tss_desc_t;

/* GDT: 5 normal entries + 1 TSS (16 bytes) = 56 bytes */
static gdt_entry_t gdt[5] __aligned(16);
static tss_desc_t  gdt_tss __aligned(8);
static tss_t       tss __aligned(16);

static void gdt_set_entry(int idx, uint32_t base, uint32_t limit,
                           uint8_t access, uint8_t gran) {
    gdt[idx].limit_low   = limit & 0xFFFF;
    gdt[idx].base_low    = base & 0xFFFF;
    gdt[idx].base_mid    = (base >> 16) & 0xFF;
    gdt[idx].access      = access;
    gdt[idx].granularity  = ((limit >> 16) & 0x0F) | (gran & 0xF0);
    gdt[idx].base_high   = (base >> 24) & 0xFF;
}

static void gdt_set_tss(uint64_t base, uint32_t limit) {
    gdt_tss.limit_low   = limit & 0xFFFF;
    gdt_tss.base_low    = base & 0xFFFF;
    gdt_tss.base_mid    = (base >> 16) & 0xFF;
    gdt_tss.access      = 0x89;   /* Present, TSS available (64-bit) */
    gdt_tss.granularity  = ((limit >> 16) & 0x0F);
    gdt_tss.base_high   = (base >> 24) & 0xFF;
    gdt_tss.base_upper  = (uint32_t)(base >> 32);
    gdt_tss.reserved    = 0;
}

void gdt_init(void) {
    /* 0x00: Null */
    gdt_set_entry(0, 0, 0, 0, 0);

    /* 0x08: Kernel Code 64 — access=0x9A (Present,DPL0,Code,Exec,Read) gran=0x20 (Long mode) */
    gdt_set_entry(1, 0, 0, 0x9A, 0x20);

    /* 0x10: Kernel Data 64 — access=0x92 (Present,DPL0,Data,Write) */
    gdt_set_entry(2, 0, 0, 0x92, 0x00);

    /* 0x18: User Code 64 — access=0xFA (Present,DPL3,Code,Exec,Read) */
    gdt_set_entry(3, 0, 0, 0xFA, 0x20);

    /* 0x20: User Data 64 — access=0xF2 (Present,DPL3,Data,Write) */
    gdt_set_entry(4, 0, 0, 0xF2, 0x00);

    /* TSS: zero and set kernel stack */
    memset(&tss, 0, sizeof(tss));
    tss.rsp0 = 0x90000;    /* Kernel stack pointer */
    tss.iomap_base = sizeof(tss);

    /* 0x28: TSS descriptor */
    gdt_set_tss((uint64_t)&tss, sizeof(tss) - 1);

    /* Build GDTR — gdt is 5 entries (40 bytes) + tss_desc (16 bytes) = 56 bytes total
     * But they must be contiguous. We'll use a flat buffer approach. */
    static uint8_t gdt_flat[56] __aligned(16);
    memcpy(gdt_flat, gdt, sizeof(gdt));          /* 40 bytes */
    memcpy(gdt_flat + 40, &gdt_tss, 16);         /* 16 bytes */

    desc_ptr_t gdtr;
    gdtr.limit = sizeof(gdt_flat) - 1;
    gdtr.base  = (uint64_t)gdt_flat;
    lgdt(&gdtr);

    /* Reload data segments (CS is reloaded by retfq in asm, but we stay in ring 0) */
    hal_reload_segments();

    /* Load TSS (selector 0x28) */
    ltr(0x28);
}
