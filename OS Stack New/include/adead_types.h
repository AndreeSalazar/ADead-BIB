/* ============================================================
 * ADead-OS — Shared Type Definitions (C Header)
 * ============================================================
 * Standard types shared between ADead-BIB, Rust, and C.
 * This is the ABI contract — all 3 languages agree on these sizes.
 *
 * NO ASM. ADead-BIB is the base. C provides compatibility.
 * ============================================================ */

#ifndef ADEAD_TYPES_H
#define ADEAD_TYPES_H

/* ---- Fixed-width integer types ---- */
typedef signed char        i8;
typedef unsigned char      u8;
typedef signed short       i16;
typedef unsigned short     u16;
typedef signed int         i32;
typedef unsigned int       u32;
typedef signed long long   i64;
typedef unsigned long long u64;

/* ---- Size types ---- */
typedef u64 usize;
typedef i64 isize;

/* ---- Boolean ---- */
typedef u8 bool_t;
#define TRUE  1
#define FALSE 0

/* ---- Null pointer ---- */
#define NULL_PTR ((void*)0)

/* ---- VGA types ---- */
typedef u16 vga_entry_t;       /* Character + attribute */
typedef u8  vga_color_t;       /* 4-bit fg + 4-bit bg */

/* VGA colors */
#define VGA_BLACK         0x0
#define VGA_BLUE          0x1
#define VGA_GREEN         0x2
#define VGA_CYAN          0x3
#define VGA_RED           0x4
#define VGA_MAGENTA       0x5
#define VGA_BROWN         0x6
#define VGA_LIGHT_GREY    0x7
#define VGA_DARK_GREY     0x8
#define VGA_LIGHT_BLUE    0x9
#define VGA_LIGHT_GREEN   0xA
#define VGA_LIGHT_CYAN    0xB
#define VGA_LIGHT_RED     0xC
#define VGA_LIGHT_MAGENTA 0xD
#define VGA_YELLOW        0xE
#define VGA_WHITE         0xF

#define VGA_COLOR(fg, bg) ((vga_color_t)((bg) << 4 | (fg)))
#define VGA_ENTRY(ch, color) ((vga_entry_t)((u16)(color) << 8 | (u16)(ch)))

/* ---- Memory addresses ---- */
#define VGA_BUFFER_ADDR   0xB8000
#define VGA_WIDTH         80
#define VGA_HEIGHT        25

/* ---- Port I/O ---- */
#define COM1_PORT         0x3F8
#define COM2_PORT         0x2F8
#define PIC1_CMD          0x20
#define PIC1_DATA         0x21
#define PIC2_CMD          0xA0
#define PIC2_DATA         0xA1
#define PIT_CHANNEL0      0x40
#define PIT_CMD           0x43
#define PS2_DATA          0x60
#define PS2_STATUS        0x64

/* ---- GDT Selectors ---- */
#define GDT_NULL          0x00
#define GDT_KERNEL_CODE   0x08
#define GDT_KERNEL_DATA   0x10
#define GDT_KERNEL_CODE32 0x18
#define GDT_USER_CODE     0x20
#define GDT_USER_DATA     0x28

/* ---- Page flags ---- */
#define PAGE_PRESENT      (1ULL << 0)
#define PAGE_WRITABLE     (1ULL << 1)
#define PAGE_USER         (1ULL << 2)
#define PAGE_WRITE_THROUGH (1ULL << 3)
#define PAGE_CACHE_DISABLE (1ULL << 4)
#define PAGE_HUGE         (1ULL << 7)
#define PAGE_NO_EXECUTE   (1ULL << 63)

/* ---- Interrupt vectors ---- */
#define INT_DIVIDE_ERROR    0
#define INT_DEBUG           1
#define INT_NMI             2
#define INT_BREAKPOINT      3
#define INT_OVERFLOW        4
#define INT_BOUND_RANGE     5
#define INT_INVALID_OPCODE  6
#define INT_DEVICE_NA       7
#define INT_DOUBLE_FAULT    8
#define INT_INVALID_TSS     10
#define INT_SEGMENT_NP      11
#define INT_STACK_FAULT     12
#define INT_GPF             13
#define INT_PAGE_FAULT      14
#define INT_X87_FP          16
#define INT_ALIGNMENT       17
#define INT_MACHINE_CHECK   18
#define INT_SIMD_FP         19
#define INT_TIMER           32
#define INT_KEYBOARD        33

#endif /* ADEAD_TYPES_H */
