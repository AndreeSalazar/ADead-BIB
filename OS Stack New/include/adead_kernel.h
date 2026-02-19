/* ============================================================
 * ADead-OS — Kernel Interface Header (C)
 * ============================================================
 * ABI contract between ADead-BIB (base) and Rust (kernel).
 * 
 * ADead-BIB exports these symbols → Rust calls them.
 * Rust exports these symbols → ADead-BIB calls them.
 *
 * Calling convention: System V AMD64
 *   Args: RDI, RSI, RDX, RCX, R8, R9
 *   Return: RAX
 *   Callee-saved: RBX, RBP, R12-R15
 *
 * NO ASM anywhere. ADead-BIB is the assembler.
 * ============================================================ */

#ifndef ADEAD_KERNEL_H
#define ADEAD_KERNEL_H

#include "adead_types.h"

/* ============================================================
 * Functions EXPORTED by ADead-BIB (called by Rust)
 * These are implemented in ADead-BIB and generate raw x86-64.
 * ============================================================ */

/* Hardware I/O — ADead-BIB generates IN/OUT instructions */
extern void adead_outb(u16 port, u8 value);
extern u8   adead_inb(u16 port);
extern void adead_outw(u16 port, u16 value);
extern u16  adead_inw(u16 port);

/* Privileged instructions — ADead-BIB generates directly */
extern void adead_cli(void);      /* CLI — disable interrupts */
extern void adead_sti(void);      /* STI — enable interrupts */
extern void adead_hlt(void);      /* HLT — halt CPU */
extern void adead_invlpg(u64 addr); /* INVLPG — invalidate TLB entry */

/* Control registers — ADead-BIB generates MOV CR */
extern void adead_write_cr0(u64 value);
extern void adead_write_cr3(u64 value);
extern void adead_write_cr4(u64 value);
extern u64  adead_read_cr0(void);
extern u64  adead_read_cr2(void);  /* Page fault address */
extern u64  adead_read_cr3(void);

/* GDT/IDT — ADead-BIB generates LGDT/LIDT */
extern void adead_load_gdt(u64 gdt_ptr);
extern void adead_load_idt(u64 idt_ptr);

/* MSR — ADead-BIB generates RDMSR/WRMSR */
extern u64  adead_read_msr(u32 msr);
extern void adead_write_msr(u32 msr, u64 value);

/* CPUID — ADead-BIB generates CPUID */
extern void adead_cpuid(u32 leaf, u32 *eax, u32 *ebx, u32 *ecx, u32 *edx);

/* ============================================================
 * Functions EXPORTED by Rust (called by ADead-BIB)
 * ADead-BIB's stage2 jumps to kernel_main after mode switch.
 * ============================================================ */

/* Kernel entry point — Rust implements this */
extern void kernel_main(void);

/* Interrupt handlers — Rust implements the logic,
 * ADead-BIB wraps them with push/pop + iretq */
extern void rust_timer_handler(void);
extern void rust_keyboard_handler(void);
extern void rust_page_fault_handler(u64 error_code);
extern void rust_gpf_handler(u64 error_code);
extern void rust_double_fault_handler(void);

/* Panic — Rust implements */
extern void rust_panic(const char *message, u64 len);

/* ============================================================
 * Structures — shared layout (packed, no padding)
 * Both ADead-BIB and Rust must agree on these layouts.
 * ============================================================ */

/* GDT Descriptor Pointer (for LGDT/LIDT) */
typedef struct __attribute__((packed)) {
    u16 limit;
    u64 base;
} descriptor_ptr_t;

/* GDT Entry (8 bytes) */
typedef struct __attribute__((packed)) {
    u16 limit_low;
    u16 base_low;
    u8  base_mid;
    u8  access;
    u8  flags_limit_high;
    u8  base_high;
} gdt_entry_t;

/* IDT Entry for Long Mode (16 bytes) */
typedef struct __attribute__((packed)) {
    u16 offset_low;
    u16 selector;
    u8  ist;
    u8  type_attr;
    u16 offset_mid;
    u32 offset_high;
    u32 reserved;
} idt_entry_t;

/* TSS (Task State Segment) */
typedef struct __attribute__((packed)) {
    u32 reserved0;
    u64 rsp0;       /* Stack for Ring 0 */
    u64 rsp1;
    u64 rsp2;
    u64 reserved1;
    u64 ist1;       /* Interrupt Stack Table */
    u64 ist2;
    u64 ist3;
    u64 ist4;
    u64 ist5;
    u64 ist6;
    u64 ist7;
    u64 reserved2;
    u16 reserved3;
    u16 iomap_base;
} tss_t;

#endif /* ADEAD_KERNEL_H */
