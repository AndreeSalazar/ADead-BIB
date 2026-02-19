/* ============================================================
 * FastOS — Type Definitions (C Header)
 * ============================================================
 * ABI contract for FastOS. All 3 languages agree on these.
 * ADead-BIB (base) + Rust (security) + C (compatibility)
 *
 * Format: FsOS (magic: 0x46734F53)
 * ============================================================ */

#ifndef FASTOS_TYPES_H
#define FASTOS_TYPES_H

/* ---- Fixed-width integers ---- */
typedef signed char        i8;
typedef unsigned char      u8;
typedef signed short       i16;
typedef unsigned short     u16;
typedef signed int         i32;
typedef unsigned int       u32;
typedef signed long long   i64;
typedef unsigned long long u64;
typedef u64 usize;
typedef i64 isize;

/* ---- Boolean ---- */
typedef u8 bool_t;
#define FASTOS_TRUE  1
#define FASTOS_FALSE 0
#define NULL_PTR ((void*)0)

/* ---- FastOS Magic ---- */
#define FASTOS_MAGIC_0 0x46  /* 'F' */
#define FASTOS_MAGIC_1 0x73  /* 's' */
#define FASTOS_MAGIC_2 0x4F  /* 'O' */
#define FASTOS_MAGIC_3 0x53  /* 'S' */
#define FASTOS_MAGIC   0x46734F53

/* ---- FastOS Version ---- */
#define FASTOS_VERSION_MAJOR 1
#define FASTOS_VERSION_MINOR 0
#define FASTOS_VERSION_PATCH 0

/* ---- VGA Text Mode ---- */
#define VGA_BUFFER    0xB8000
#define VGA_WIDTH     80
#define VGA_HEIGHT    25
#define VGA_SIZE      (VGA_WIDTH * VGA_HEIGHT)

typedef u16 vga_entry_t;
typedef u8  vga_color_t;

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

#define VGA_COLOR(fg, bg)    ((vga_color_t)((bg) << 4 | (fg)))
#define VGA_ENTRY(ch, color) ((vga_entry_t)((u16)(color) << 8 | (u16)(ch)))

/* ---- FastOS Theme Colors ---- */
#define FASTOS_FG         VGA_LIGHT_GREEN
#define FASTOS_BG         VGA_BLACK
#define FASTOS_ACCENT     VGA_LIGHT_CYAN
#define FASTOS_ERROR      VGA_LIGHT_RED
#define FASTOS_WARN       VGA_YELLOW
#define FASTOS_TITLE      VGA_WHITE
#define FASTOS_PROMPT     VGA_GREEN

/* ---- Port I/O ---- */
#define PS2_DATA          0x60
#define PS2_STATUS        0x64
#define COM1_PORT         0x3F8
#define PIC1_CMD          0x20
#define PIC1_DATA         0x21
#define PIC2_CMD          0xA0
#define PIC2_DATA         0xA1
#define PIT_CHANNEL0      0x40
#define PIT_CMD           0x43

/* ---- GDT Selectors ---- */
#define GDT_NULL          0x00
#define GDT_KERNEL_CODE   0x08
#define GDT_KERNEL_DATA   0x10
#define GDT_USER_CODE     0x18
#define GDT_USER_DATA     0x20

/* ---- Memory Layout ---- */
#define FASTOS_KERNEL_BASE  0x100000   /* 1MB */
#define FASTOS_STACK_TOP    0x90000
#define FASTOS_PAGE_TABLES  0x1000

/* ---- Keyboard Scancodes (Set 1) ---- */
#define KEY_ESC       0x01
#define KEY_1         0x02
#define KEY_2         0x03
#define KEY_ENTER     0x1C
#define KEY_A         0x1E
#define KEY_S         0x1F
#define KEY_D         0x20
#define KEY_Y         0x15
#define KEY_N         0x31
#define KEY_BACKSPACE 0x0E
#define KEY_SPACE     0x39
#define KEY_LSHIFT    0x2A
#define KEY_RSHIFT    0x36
#define KEY_CAPSLOCK  0x3A
#define KEY_F1        0x3B
#define KEY_F2        0x3C
#define KEY_UP        0x48
#define KEY_DOWN      0x50
#define KEY_LEFT      0x4B
#define KEY_RIGHT     0x4D

/* ---- FastOS Installer States ---- */
#define INSTALLER_WELCOME   0
#define INSTALLER_LANGUAGE  1
#define INSTALLER_CONFIRM   2
#define INSTALLER_PROGRESS  3
#define INSTALLER_COMPLETE  4
#define INSTALLER_SHELL     5

#endif /* FASTOS_TYPES_H */
