/*
 * FastOS v2.0 — Type Definitions
 * ADead-BIB Native Operating System
 */

#ifndef _FASTOS_TYPES_H
#define _FASTOS_TYPES_H

/* Fixed-width integer types */
typedef signed char        int8_t;
typedef unsigned char      uint8_t;
typedef signed short       int16_t;
typedef unsigned short     uint16_t;
typedef signed int         int32_t;
typedef unsigned int       uint32_t;
typedef signed long long   int64_t;
typedef unsigned long long uint64_t;

/* Size types */
typedef uint64_t size_t;
typedef int64_t  ssize_t;
typedef int64_t  ptrdiff_t;

/* Pointer types */
typedef uint64_t uintptr_t;
typedef int64_t  intptr_t;

/* Boolean */
typedef _Bool bool;
#define true  1
#define false 0

/* NULL */
#ifndef NULL
#define NULL ((void*)0)
#endif

/* Physical/Virtual address types */
typedef uint64_t phys_addr_t;
typedef uint64_t virt_addr_t;

/* PCI types */
typedef uint16_t pci_vendor_t;
typedef uint16_t pci_device_t;

/* Attribute macros */
#define __packed    __attribute__((packed))
#define __aligned(x) __attribute__((aligned(x)))
#define __noreturn  __attribute__((noreturn))
#define __unused    __attribute__((unused))
#define __weak      __attribute__((weak))

/* Inline assembly helpers */
#define asm __asm__
#define volatile __volatile__

/* Memory barriers */
#define barrier() asm volatile("" ::: "memory")
#define mb()      asm volatile("mfence" ::: "memory")
#define rmb()     asm volatile("lfence" ::: "memory")
#define wmb()     asm volatile("sfence" ::: "memory")

/* Min/Max */
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

/* Alignment */
#define ALIGN_UP(x, align)   (((x) + (align) - 1) & ~((align) - 1))
#define ALIGN_DOWN(x, align) ((x) & ~((align) - 1))
#define IS_ALIGNED(x, align) (((x) & ((align) - 1)) == 0)

/* Bit manipulation */
#define BIT(n)           (1ULL << (n))
#define BITS(hi, lo)     ((BIT((hi) - (lo) + 1) - 1) << (lo))
#define GET_BITS(x, hi, lo) (((x) >> (lo)) & (BIT((hi) - (lo) + 1) - 1))

/* Array size */
#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof((arr)[0]))

/* Stringify */
#define __stringify(x) #x
#define stringify(x)   __stringify(x)

/* Container of */
#define container_of(ptr, type, member) \
    ((type *)((char *)(ptr) - offsetof(type, member)))

/* Offset of */
#define offsetof(type, member) ((size_t)&((type *)0)->member)

#endif /* _FASTOS_TYPES_H */
