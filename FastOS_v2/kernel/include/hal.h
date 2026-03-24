/*
 * FastOS v2.0 — Hardware Abstraction Layer
 * Provides extern function declarations for all hardware operations.
 * Implementations are in hal.asm (NASM, 64-bit).
 *
 * This replaces all inline assembly in kernel.h so the kernel
 * can be compiled with ADead-BIB (adb cc --flat64 --org=0x100000).
 */

#ifndef _FASTOS_HAL_H
#define _FASTOS_HAL_H

#include "kernel.h"

/* ============================================================
 * Port I/O
 * ============================================================ */

extern void    hal_outb(uint16_t port, uint8_t val);
extern uint8_t hal_inb(uint16_t port);
extern void    hal_outw(uint16_t port, uint16_t val);
extern uint16_t hal_inw(uint16_t port);
extern void    hal_outl(uint16_t port, uint32_t val);
extern uint32_t hal_inl(uint16_t port);
extern void    hal_io_wait(void);

/* ============================================================
 * CPU Control
 * ============================================================ */

extern void hal_cli(void);
extern void hal_sti(void);
extern void hal_hlt(void);
extern void hal_pause(void);
extern void hal_int3(void);

/* ============================================================
 * Memory Barriers
 * ============================================================ */

extern void hal_mfence(void);
extern void hal_lfence(void);
extern void hal_sfence(void);

/* ============================================================
 * Timestamp Counter
 * ============================================================ */

extern uint64_t hal_rdtsc(void);

/* ============================================================
 * Model-Specific Registers
 * ============================================================ */

extern uint64_t hal_rdmsr(uint32_t msr);
extern void     hal_wrmsr(uint32_t msr, uint64_t val);

/* ============================================================
 * CPUID
 * ============================================================ */

extern void hal_cpuid(uint32_t leaf, uint32_t *eax, uint32_t *ebx,
                      uint32_t *ecx, uint32_t *edx);

/* ============================================================
 * Control Registers
 * ============================================================ */

extern uint64_t hal_read_cr0(void);
extern uint64_t hal_read_cr2(void);
extern uint64_t hal_read_cr3(void);
extern void     hal_write_cr3(uint64_t val);
extern uint64_t hal_read_cr4(void);
extern void     hal_write_cr4(uint64_t val);

/* ============================================================
 * TLB Management
 * ============================================================ */

extern void hal_invlpg(void *addr);
extern void hal_flush_tlb(void);

/* ============================================================
 * Descriptor Tables
 * ============================================================ */

extern void hal_lgdt(void *gdtr);
extern void hal_lidt(void *idtr);
extern void hal_ltr(uint16_t sel);

/* ============================================================
 * Segment Reload
 * ============================================================ */

extern void hal_reload_segments(void);

#endif /* _FASTOS_HAL_H */
