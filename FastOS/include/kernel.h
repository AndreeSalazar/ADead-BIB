/*
 * FastOS v2.0 — Kernel Header
 * ADead-BIB Native Operating System
 */

#ifndef _FASTOS_KERNEL_H
#define _FASTOS_KERNEL_H

#include "types.h"

/* ============================================================
 * CPU Control
 * ============================================================ */

static inline void cli(void) {
    asm volatile("cli");
}

static inline void sti(void) {
    asm volatile("sti");
}

static inline void hlt(void) {
    asm volatile("hlt");
}

static inline void pause(void) {
    asm volatile("pause");
}

/* ============================================================
 * Port I/O
 * ============================================================ */

static inline void outb(uint16_t port, uint8_t val) {
    asm volatile("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    asm volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void outw(uint16_t port, uint16_t val) {
    asm volatile("outw %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint16_t inw(uint16_t port) {
    uint16_t ret;
    asm volatile("inw %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void outl(uint16_t port, uint32_t val) {
    asm volatile("outl %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint32_t inl(uint16_t port) {
    uint32_t ret;
    asm volatile("inl %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void io_wait(void) {
    outb(0x80, 0);
}

/* ============================================================
 * Control Registers
 * ============================================================ */

static inline uint64_t read_cr0(void) {
    uint64_t val;
    asm volatile("mov %%cr0, %0" : "=r"(val));
    return val;
}

static inline void write_cr0(uint64_t val) {
    asm volatile("mov %0, %%cr0" : : "r"(val));
}

static inline uint64_t read_cr2(void) {
    uint64_t val;
    asm volatile("mov %%cr2, %0" : "=r"(val));
    return val;
}

static inline uint64_t read_cr3(void) {
    uint64_t val;
    asm volatile("mov %%cr3, %0" : "=r"(val));
    return val;
}

static inline void write_cr3(uint64_t val) {
    asm volatile("mov %0, %%cr3" : : "r"(val));
}

static inline uint64_t read_cr4(void) {
    uint64_t val;
    asm volatile("mov %%cr4, %0" : "=r"(val));
    return val;
}

static inline void write_cr4(uint64_t val) {
    asm volatile("mov %0, %%cr4" : : "r"(val));
}

/* ============================================================
 * MSR Access
 * ============================================================ */

static inline uint64_t rdmsr(uint32_t msr) {
    uint32_t lo, hi;
    asm volatile("rdmsr" : "=a"(lo), "=d"(hi) : "c"(msr));
    return ((uint64_t)hi << 32) | lo;
}

static inline void wrmsr(uint32_t msr, uint64_t val) {
    asm volatile("wrmsr" : : "a"((uint32_t)val), "d"((uint32_t)(val >> 32)), "c"(msr));
}

/* ============================================================
 * GDT/IDT
 * ============================================================ */

typedef struct {
    uint16_t limit;
    uint64_t base;
} __packed gdt_ptr_t;

typedef struct {
    uint16_t limit;
    uint64_t base;
} __packed idt_ptr_t;

static inline void lgdt(gdt_ptr_t *gdtr) {
    asm volatile("lgdt (%0)" : : "r"(gdtr));
}

static inline void lidt(idt_ptr_t *idtr) {
    asm volatile("lidt (%0)" : : "r"(idtr));
}

static inline void ltr(uint16_t selector) {
    asm volatile("ltr %0" : : "r"(selector));
}

/* ============================================================
 * TLB
 * ============================================================ */

static inline void invlpg(void *addr) {
    asm volatile("invlpg (%0)" : : "r"(addr) : "memory");
}

static inline void flush_tlb(void) {
    write_cr3(read_cr3());
}

/* ============================================================
 * Timestamp Counter
 * ============================================================ */

static inline uint64_t rdtsc(void) {
    uint32_t lo, hi;
    asm volatile("rdtsc" : "=a"(lo), "=d"(hi));
    return ((uint64_t)hi << 32) | lo;
}

/* ============================================================
 * CPUID
 * ============================================================ */

static inline void cpuid(uint32_t leaf, uint32_t *eax, uint32_t *ebx, 
                         uint32_t *ecx, uint32_t *edx) {
    asm volatile("cpuid"
        : "=a"(*eax), "=b"(*ebx), "=c"(*ecx), "=d"(*edx)
        : "a"(leaf));
}

/* ============================================================
 * Kernel Panic
 * ============================================================ */

void __noreturn kernel_panic(const char *msg);

/* ============================================================
 * Logging
 * ============================================================ */

void kprintf(const char *fmt, ...);
void kputs(const char *s);

/* ============================================================
 * Memory
 * ============================================================ */

void *kmalloc(size_t size);
void kfree(void *ptr);
void *kzalloc(size_t size);

#endif /* _FASTOS_KERNEL_H */
