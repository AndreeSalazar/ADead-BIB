/*
 * ADead-BIB OS Development Library
 * kernel.h - Kernel primitives for OS development
 */

#ifndef _ADEAD_KERNEL_H
#define _ADEAD_KERNEL_H

#include "../c/stdint.h"
#include "../c/stddef.h"

/* CPU Control */
static inline void cli(void) { __asm__ volatile("cli"); }
static inline void sti(void) { __asm__ volatile("sti"); }
static inline void hlt(void) { __asm__ volatile("hlt"); }
static inline void nop(void) { __asm__ volatile("nop"); }

/* CPU Information */
static inline void cpuid(uint32_t leaf, uint32_t* eax, uint32_t* ebx, uint32_t* ecx, uint32_t* edx) {
    __asm__ volatile("cpuid"
        : "=a"(*eax), "=b"(*ebx), "=c"(*ecx), "=d"(*edx)
        : "a"(leaf));
}

/* Port I/O */
static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void outw(uint16_t port, uint16_t val) {
    __asm__ volatile("outw %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint16_t inw(uint16_t port) {
    uint16_t ret;
    __asm__ volatile("inw %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void outl(uint16_t port, uint32_t val) {
    __asm__ volatile("outl %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint32_t inl(uint16_t port) {
    uint32_t ret;
    __asm__ volatile("inl %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

/* IO Wait */
static inline void io_wait(void) {
    outb(0x80, 0);
}

/* Control Registers */
static inline uint64_t read_cr0(void) {
    uint64_t val;
    __asm__ volatile("mov %%cr0, %0" : "=r"(val));
    return val;
}

static inline void write_cr0(uint64_t val) {
    __asm__ volatile("mov %0, %%cr0" : : "r"(val));
}

static inline uint64_t read_cr2(void) {
    uint64_t val;
    __asm__ volatile("mov %%cr2, %0" : "=r"(val));
    return val;
}

static inline uint64_t read_cr3(void) {
    uint64_t val;
    __asm__ volatile("mov %%cr3, %0" : "=r"(val));
    return val;
}

static inline void write_cr3(uint64_t val) {
    __asm__ volatile("mov %0, %%cr3" : : "r"(val));
}

static inline uint64_t read_cr4(void) {
    uint64_t val;
    __asm__ volatile("mov %%cr4, %0" : "=r"(val));
    return val;
}

static inline void write_cr4(uint64_t val) {
    __asm__ volatile("mov %0, %%cr4" : : "r"(val));
}

/* MSR Access */
static inline uint64_t rdmsr(uint32_t msr) {
    uint32_t lo, hi;
    __asm__ volatile("rdmsr" : "=a"(lo), "=d"(hi) : "c"(msr));
    return ((uint64_t)hi << 32) | lo;
}

static inline void wrmsr(uint32_t msr, uint64_t val) {
    __asm__ volatile("wrmsr" : : "a"((uint32_t)val), "d"((uint32_t)(val >> 32)), "c"(msr));
}

/* TLB */
static inline void invlpg(void* addr) {
    __asm__ volatile("invlpg (%0)" : : "r"(addr) : "memory");
}

/* GDT/IDT */
typedef struct {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed)) descriptor_table_ptr;

static inline void lgdt(descriptor_table_ptr* gdtr) {
    __asm__ volatile("lgdt (%0)" : : "r"(gdtr));
}

static inline void lidt(descriptor_table_ptr* idtr) {
    __asm__ volatile("lidt (%0)" : : "r"(idtr));
}

static inline void ltr(uint16_t selector) {
    __asm__ volatile("ltr %0" : : "r"(selector));
}

/* Timestamp Counter */
static inline uint64_t rdtsc(void) {
    uint32_t lo, hi;
    __asm__ volatile("rdtsc" : "=a"(lo), "=d"(hi));
    return ((uint64_t)hi << 32) | lo;
}

/* Pause for spinlocks */
static inline void cpu_pause(void) {
    __asm__ volatile("pause");
}

/* Memory Barrier */
static inline void mfence(void) {
    __asm__ volatile("mfence" ::: "memory");
}

static inline void sfence(void) {
    __asm__ volatile("sfence" ::: "memory");
}

static inline void lfence(void) {
    __asm__ volatile("lfence" ::: "memory");
}

#endif /* _ADEAD_KERNEL_H */
