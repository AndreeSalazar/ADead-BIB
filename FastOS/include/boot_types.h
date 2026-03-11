/*
 * FastOS v2.0 - Boot Types
 * Basic types for bootloader code
 * Compatible with ADead-BIB C Compiler
 */

#ifndef _FASTOS_BOOT_TYPES_H
#define _FASTOS_BOOT_TYPES_H

/* Fixed-width integer types */
typedef unsigned char       u8;
typedef signed char         i8;
typedef unsigned short      u16;
typedef signed short        i16;
typedef unsigned int        u32;
typedef signed int          i32;
typedef unsigned long long  u64;
typedef signed long long    i64;

/* Size types */
typedef u64 size_t;
typedef i64 ssize_t;
typedef u64 uintptr_t;
typedef i64 intptr_t;

/* Boolean */
typedef int bool;
#define true  1
#define false 0

/* NULL */
#ifndef NULL
#define NULL ((void*)0)
#endif

/* Compiler attributes */
#define __packed        __attribute__((packed))
#define __aligned(x)    __attribute__((aligned(x)))
#define __noreturn      __attribute__((noreturn))
#define __naked         __attribute__((naked))
#define __section(x)    __attribute__((section(x)))
#define __used          __attribute__((used))
#define __weak          __attribute__((weak))

/* Inline assembly helpers */
#define cli()   __asm__ volatile("cli")
#define sti()   __asm__ volatile("sti")
#define hlt()   __asm__ volatile("hlt")
#define nop()   __asm__ volatile("nop")

/* Memory barriers */
#define barrier()       __asm__ volatile("" ::: "memory")
#define mb()            __asm__ volatile("mfence" ::: "memory")
#define rmb()           __asm__ volatile("lfence" ::: "memory")
#define wmb()           __asm__ volatile("sfence" ::: "memory")

/* Port I/O */
static inline void outb(u16 port, u8 value) {
    __asm__ volatile("outb %0, %1" : : "a"(value), "Nd"(port));
}

static inline u8 inb(u16 port) {
    u8 ret;
    __asm__ volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void outw(u16 port, u16 value) {
    __asm__ volatile("outw %0, %1" : : "a"(value), "Nd"(port));
}

static inline u16 inw(u16 port) {
    u16 ret;
    __asm__ volatile("inw %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void outl(u16 port, u32 value) {
    __asm__ volatile("outl %0, %1" : : "a"(value), "Nd"(port));
}

static inline u32 inl(u16 port) {
    u32 ret;
    __asm__ volatile("inl %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

/* String operations */
static inline void* memset(void* dest, int c, size_t n) {
    u8* d = (u8*)dest;
    while (n--) *d++ = (u8)c;
    return dest;
}

static inline void* memcpy(void* dest, const void* src, size_t n) {
    u8* d = (u8*)dest;
    const u8* s = (const u8*)src;
    while (n--) *d++ = *s++;
    return dest;
}

static inline size_t strlen(const char* s) {
    size_t len = 0;
    while (*s++) len++;
    return len;
}

static inline int strcmp(const char* s1, const char* s2) {
    while (*s1 && *s1 == *s2) {
        s1++;
        s2++;
    }
    return *(u8*)s1 - *(u8*)s2;
}

static inline char* strcpy(char* dest, const char* src) {
    char* d = dest;
    while ((*d++ = *src++));
    return dest;
}

/* Bit operations */
#define BIT(n)          (1ULL << (n))
#define BITS(h, l)      ((BIT((h) - (l) + 1) - 1) << (l))
#define GET_BITS(v, h, l) (((v) >> (l)) & (BIT((h) - (l) + 1) - 1))
#define SET_BIT(v, n)   ((v) | BIT(n))
#define CLR_BIT(v, n)   ((v) & ~BIT(n))
#define TST_BIT(v, n)   (((v) >> (n)) & 1)

/* Min/Max */
#define MIN(a, b)       ((a) < (b) ? (a) : (b))
#define MAX(a, b)       ((a) > (b) ? (a) : (b))
#define CLAMP(v, lo, hi) MIN(MAX(v, lo), hi)

/* Alignment */
#define ALIGN_UP(x, a)      (((x) + (a) - 1) & ~((a) - 1))
#define ALIGN_DOWN(x, a)    ((x) & ~((a) - 1))
#define IS_ALIGNED(x, a)    (((x) & ((a) - 1)) == 0)

/* Array size */
#define ARRAY_SIZE(arr)     (sizeof(arr) / sizeof((arr)[0]))

/* Offset of field in struct */
#define offsetof(type, member) ((size_t)&((type*)0)->member)

/* Container of */
#define container_of(ptr, type, member) \
    ((type*)((char*)(ptr) - offsetof(type, member)))

/* Cross-compatibility with types.h naming */
#ifndef _FASTOS_TYPES_H
typedef u8  uint8_t;
typedef u16 uint16_t;
typedef u32 uint32_t;
typedef u64 uint64_t;
typedef i8  int8_t;
typedef i16 int16_t;
typedef i32 int32_t;
typedef i64 int64_t;
#endif

#endif /* _FASTOS_BOOT_TYPES_H */
