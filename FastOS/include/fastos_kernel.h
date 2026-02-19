/* ============================================================
 * FastOS — Kernel Interface Header (C)
 * ============================================================
 * ABI contract between ADead-BIB (base) and Rust (kernel).
 * ADead-BIB handles hardware. Rust handles logic. C glues them.
 *
 * Calling convention: System V AMD64
 * ============================================================ */

#ifndef FASTOS_KERNEL_H
#define FASTOS_KERNEL_H

#include "fastos_types.h"

/* ============================================================
 * ADead-BIB exports (hardware layer)
 * ============================================================ */
extern void fastos_outb(u16 port, u8 value);
extern u8   fastos_inb(u16 port);
extern void fastos_cli(void);
extern void fastos_sti(void);
extern void fastos_hlt(void);
extern void fastos_load_gdt(u64 gdt_ptr);
extern void fastos_load_idt(u64 idt_ptr);
extern void fastos_write_cr3(u64 value);
extern u64  fastos_read_cr2(void);

/* ============================================================
 * Rust kernel exports
 * ============================================================ */
extern void kernel_main(void);
extern void fastos_vga_init(void);
extern void fastos_vga_clear(void);
extern void fastos_vga_putchar(u8 ch);
extern void fastos_vga_print(const char *str, u64 len);
extern void fastos_vga_set_color(u8 fg, u8 bg);
extern void fastos_vga_set_cursor(u16 row, u16 col);

extern void fastos_keyboard_init(void);
extern u8   fastos_keyboard_read(void);

extern void fastos_shell_init(void);
extern void fastos_shell_run(void);

extern void fastos_installer_run(void);

/* ============================================================
 * Interrupt handlers (Rust logic, ADead-BIB wraps)
 * ============================================================ */
extern void fastos_timer_handler(void);
extern void fastos_keyboard_handler(void);
extern void fastos_page_fault_handler(u64 error_code);
extern void fastos_gpf_handler(u64 error_code);
extern void fastos_panic(const char *msg, u64 len);

/* ============================================================
 * Packed structures (shared layout)
 * ============================================================ */

typedef struct __attribute__((packed)) {
    u16 limit;
    u64 base;
} fastos_descriptor_ptr_t;

typedef struct __attribute__((packed)) {
    u16 limit_low;
    u16 base_low;
    u8  base_mid;
    u8  access;
    u8  flags_limit_high;
    u8  base_high;
} fastos_gdt_entry_t;

typedef struct __attribute__((packed)) {
    u16 offset_low;
    u16 selector;
    u8  ist;
    u8  type_attr;
    u16 offset_mid;
    u32 offset_high;
    u32 reserved;
} fastos_idt_entry_t;

#endif /* FASTOS_KERNEL_H */
