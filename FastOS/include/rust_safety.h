/*
 * FastOS v2.0 - Rust Safety Layer C Interface
 * FFI header for calling Rust safety functions from C
 * 
 * Philosophy: C is Master, Rust provides Safety
 */

#ifndef _FASTOS_RUST_SAFETY_H
#define _FASTOS_RUST_SAFETY_H

#include "boot_types.h"

/* ============================================================
 * Address Operations
 * ============================================================ */

/* Create validated physical address */
u64 rust_phys_addr_new(u64 addr);

/* Create validated virtual address */
u64 rust_virt_addr_new(u64 addr);

/* Align address up to boundary */
u64 rust_align_up(u64 addr, u64 align);

/* Align address down to boundary */
u64 rust_align_down(u64 addr, u64 align);

/* ============================================================
 * Bounds Checking
 * ============================================================ */

/* Check if access is within bounds */
bool rust_check_bounds(u64 ptr, u64 len, u64 access_offset, u64 access_size);

/* Safe memcpy with bounds checking - returns false if would overflow */
bool rust_memcpy_safe(void* dest, size_t dest_size, const void* src, size_t count);

/* Safe memset with bounds checking - returns false if would overflow */
bool rust_memset_safe(void* dest, size_t dest_size, u8 value, size_t count);

/* ============================================================
 * Memory Allocator
 * ============================================================ */

/* Initialize heap allocator */
void rust_heap_init(size_t start, size_t size);

/* Allocate memory (8-byte aligned) */
void* rust_malloc(size_t size);

/* Note: This is a bump allocator - free is a no-op */
/* For real free, use a more complex allocator */

/* ============================================================
 * Binary Guardian
 * ============================================================ */

/* Check if instruction opcode is allowed at current security level */
bool rust_bg_check_instruction(u8 opcode);

/* Check if memory access is allowed */
bool rust_bg_check_memory(u64 addr, size_t size, bool write);

/* Get number of security violations */
size_t rust_bg_violation_count(void);

/* ============================================================
 * Convenience Macros
 * ============================================================ */

/* Safe array access */
#define SAFE_ARRAY_GET(arr, idx, len, default_val) \
    ((idx) < (len) ? (arr)[idx] : (default_val))

/* Safe array set */
#define SAFE_ARRAY_SET(arr, idx, len, val) \
    do { if ((idx) < (len)) (arr)[idx] = (val); } while(0)

/* Safe memcpy wrapper */
#define SAFE_MEMCPY(dest, src, count) \
    rust_memcpy_safe((dest), sizeof(dest), (src), (count))

/* Safe memset wrapper */
#define SAFE_MEMSET(dest, val, count) \
    rust_memset_safe((dest), sizeof(dest), (val), (count))

/* BG-checked memory write */
#define BG_WRITE(addr, val) \
    do { \
        if (rust_bg_check_memory((u64)(addr), sizeof(val), true)) { \
            *(typeof(val)*)(addr) = (val); \
        } \
    } while(0)

/* BG-checked memory read */
#define BG_READ(addr, type) \
    (rust_bg_check_memory((u64)(addr), sizeof(type), false) ? \
        *(type*)(addr) : (type)0)

/* ============================================================
 * Security Levels
 * ============================================================ */

#define SECURITY_KERNEL     0
#define SECURITY_DRIVER     1
#define SECURITY_SYSTEM     2
#define SECURITY_USER       3
#define SECURITY_SANDBOX    4

/* ============================================================
 * VGA Safe Operations
 * ============================================================ */

#define VGA_BASE    0xB8000
#define VGA_SIZE    (80 * 25 * 2)

/* Safe VGA write - checks bounds and BG */
static inline bool vga_safe_write(u32 offset, u16 value) {
    if (offset >= VGA_SIZE / 2) return false;
    if (!rust_bg_check_memory(VGA_BASE + offset * 2, 2, true)) return false;
    
    volatile u16* vga = (volatile u16*)VGA_BASE;
    vga[offset] = value;
    return true;
}

/* Safe VGA read */
static inline u16 vga_safe_read(u32 offset) {
    if (offset >= VGA_SIZE / 2) return 0;
    if (!rust_bg_check_memory(VGA_BASE + offset * 2, 2, false)) return 0;
    
    volatile u16* vga = (volatile u16*)VGA_BASE;
    return vga[offset];
}

/* ============================================================
 * Port I/O Safe Operations
 * ============================================================ */

/* Safe port output - checks BG */
static inline bool port_safe_outb(u16 port, u8 value) {
    /* Port I/O requires kernel level */
    if (!rust_bg_check_instruction(0xEE)) return false; /* OUT opcode */
    outb(port, value);
    return true;
}

/* Safe port input - checks BG */
static inline u8 port_safe_inb(u16 port) {
    if (!rust_bg_check_instruction(0xEC)) return 0; /* IN opcode */
    return inb(port);
}

#endif /* _FASTOS_RUST_SAFETY_H */
