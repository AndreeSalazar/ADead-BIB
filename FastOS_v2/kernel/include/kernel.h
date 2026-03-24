/*
 * FastOS v2.0 — Kernel Header
 * Core types, macros, and HAL (Hardware Abstraction Layer)
 * Target: AMD Ryzen 5 5600X (Zen3, x86_64)
 * No libc — all types defined from scratch
 */

#ifndef _FASTOS_KERNEL_H
#define _FASTOS_KERNEL_H

/* ============================================================
 * Fixed-Width Integer Types
 * ============================================================ */

typedef signed char        int8_t;
typedef unsigned char      uint8_t;
typedef signed short       int16_t;
typedef unsigned short     uint16_t;
typedef signed int         int32_t;
typedef unsigned int       uint32_t;
typedef signed long long   int64_t;
typedef unsigned long long uint64_t;

typedef uint64_t size_t;
typedef int64_t  ssize_t;
typedef uint64_t uintptr_t;
typedef int64_t  intptr_t;
typedef int64_t  ptrdiff_t;

typedef _Bool bool;
#define true  1
#define false 0

#ifndef NULL
#define NULL ((void*)0)
#endif

/* ============================================================
 * Compiler Attributes
 * ============================================================ */

#define __packed     __attribute__((packed))
#define __aligned(x) __attribute__((aligned(x)))
#define __noreturn   __attribute__((noreturn))
#define __unused     __attribute__((unused))

/* ============================================================
 * Utility Macros
 * ============================================================ */

#define ALIGN_UP(x, a)    (((x) + ((a) - 1)) & ~((a) - 1))
#define ALIGN_DOWN(x, a)  ((x) & ~((a) - 1))
#define IS_ALIGNED(x, a)  (((x) & ((a) - 1)) == 0)

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define BIT(n)       (1ULL << (n))
#define ARRAY_SIZE(a) (sizeof(a) / sizeof((a)[0]))
#define offsetof(type, member) ((size_t)&((type *)0)->member)

/* ============================================================
 * Memory Barriers — implemented in hal.asm
 * ============================================================ */

extern void hal_mfence(void);
extern void hal_lfence(void);
extern void hal_sfence(void);

#define barrier() hal_mfence()
#define mb()      hal_mfence()
#define rmb()     hal_lfence()
#define wmb()     hal_sfence()

/* ============================================================
 * Port I/O — implemented in hal.asm
 * ============================================================ */

extern void     hal_outb(uint16_t port, uint8_t val);
extern uint8_t  hal_inb(uint16_t port);
extern void     hal_outw(uint16_t port, uint16_t val);
extern uint16_t hal_inw(uint16_t port);
extern void     hal_outl(uint16_t port, uint32_t val);
extern uint32_t hal_inl(uint16_t port);
extern void     hal_io_wait(void);

#define outb     hal_outb
#define inb      hal_inb
#define outw     hal_outw
#define inw      hal_inw
#define outl     hal_outl
#define inl      hal_inl
#define io_wait  hal_io_wait

/* ============================================================
 * CPU Control — implemented in hal.asm
 * ============================================================ */

extern void     hal_cli(void);
extern void     hal_sti(void);
extern void     hal_hlt(void);
extern void     hal_pause(void);
extern void     hal_int3(void);
extern uint64_t hal_rdtsc(void);
extern uint64_t hal_rdmsr(uint32_t msr);
extern void     hal_wrmsr(uint32_t msr, uint64_t val);
extern void     hal_cpuid(uint32_t leaf, uint32_t *eax, uint32_t *ebx,
                           uint32_t *ecx, uint32_t *edx);

#define cli      hal_cli
#define sti      hal_sti
#define hlt      hal_hlt
#define rdtsc    hal_rdtsc
#define rdmsr    hal_rdmsr
#define wrmsr    hal_wrmsr
#define cpuid    hal_cpuid

/* ============================================================
 * Control Registers — implemented in hal.asm
 * ============================================================ */

extern uint64_t hal_read_cr0(void);
extern uint64_t hal_read_cr2(void);
extern uint64_t hal_read_cr3(void);
extern void     hal_write_cr3(uint64_t val);
extern uint64_t hal_read_cr4(void);
extern void     hal_write_cr4(uint64_t val);
extern void     hal_invlpg(void *addr);
extern void     hal_flush_tlb(void);

#define read_cr0  hal_read_cr0
#define read_cr2  hal_read_cr2
#define read_cr3  hal_read_cr3
#define write_cr3 hal_write_cr3
#define invlpg    hal_invlpg
#define flush_tlb hal_flush_tlb

/* ============================================================
 * GDT/IDT Descriptor Pointers — implemented in hal.asm
 * ============================================================ */

typedef struct {
    uint16_t limit;
    uint64_t base;
} __packed desc_ptr_t;

extern void hal_lgdt(void *gdtr);
extern void hal_lidt(void *idtr);
extern void hal_ltr(uint16_t sel);
extern void hal_reload_segments(void);

#define lgdt(p)  hal_lgdt((void*)(p))
#define lidt(p)  hal_lidt((void*)(p))
#define ltr      hal_ltr

/* ============================================================
 * Boot Info (passed from stage2 at 0x500)
 * ============================================================ */

#define BOOT_INFO_ADDR 0x500

/* E820 Memory Map Entry */
typedef struct {
    uint64_t base;
    uint64_t length;
    uint32_t type;
    uint32_t acpi;
} __packed e820_entry_t;

#define E820_USABLE      1
#define E820_RESERVED    2
#define E820_ACPI_RECLAIM 3
#define E820_ACPI_NVS    4
#define E820_BAD         5

/* ============================================================
 * VGA Text Mode
 * ============================================================ */

#define VGA_BUFFER  ((volatile uint16_t *)0xB8000)
#define VGA_WIDTH   80
#define VGA_HEIGHT  25

typedef enum {
    VGA_BLACK   = 0,  VGA_BLUE      = 1,  VGA_GREEN  = 2,
    VGA_CYAN    = 3,  VGA_RED       = 4,  VGA_MAGENTA = 5,
    VGA_BROWN   = 6,  VGA_LGRAY     = 7,  VGA_DGRAY   = 8,
    VGA_LBLUE   = 9,  VGA_LGREEN   = 10,  VGA_LCYAN  = 11,
    VGA_LRED   = 12,  VGA_LMAGENTA = 13,  VGA_YELLOW = 14,
    VGA_WHITE  = 15,
} vga_color_t;

#define VGA_COLOR(fg, bg) ((uint8_t)((bg) << 4 | (fg)))

/* VGA driver (vga.c) */
void vga_init(void);
void vga_clear(void);
void vga_putchar(char c);
void vga_puts(const char *s);
void vga_puts_color(const char *s, uint8_t color);
void vga_set_color(uint8_t color);
void vga_set_cursor(int row, int col);
void vga_get_cursor(int *row, int *col);
void vga_scroll(void);
void vga_update_cursor(void);

/* kprintf (lib/printf.c)
 * Accepts up to 8 variadic uint64_t arguments.
 * Callers use normal kprintf(fmt, arg1, arg2, ...) syntax;
 * unused argument slots are harmless (only consumed per format). */
void kprintf(const char *fmt,
             uint64_t a1, uint64_t a2, uint64_t a3,
             uint64_t a4, uint64_t a5, uint64_t a6,
             uint64_t a7, uint64_t a8);

/* ============================================================
 * String/Memory functions (lib/string.c)
 * ============================================================ */

size_t strlen(const char *s);
char  *strcpy(char *dest, const char *src);
char  *strncpy(char *dest, const char *src, size_t n);
int    strcmp(const char *s1, const char *s2);
int    strncmp(const char *s1, const char *s2, size_t n);
char  *strcat(char *dest, const char *src);
char  *strchr(const char *s, int c);
void  *memcpy(void *dest, const void *src, size_t n);
void  *memset(void *s, int c, size_t n);
void  *memmove(void *dest, const void *src, size_t n);
int    memcmp(const void *s1, const void *s2, size_t n);
char  *itoa(int64_t value, char *str, int base);

/* ============================================================
 * Subsystem Init Prototypes
 * ============================================================ */

/* GDT (gdt.c) */
void gdt_init(void);

/* IDT (idt.c) */
void idt_init(void);

/* PIC (pic.c) */
void pic_init(void);
void pic_send_eoi(uint8_t irq);
void pic_set_mask(uint8_t irq);
void pic_clear_mask(uint8_t irq);

/* Timer (timer.c) */
void timer_init(uint32_t freq_hz);
uint64_t timer_get_ticks(void);
uint64_t timer_get_seconds(void);

/* Keyboard (keyboard.c) */
void keyboard_init(void);
char keyboard_getchar(void);
int  keyboard_has_key(void);

/* PMM — Physical Memory Manager (pmm.c) */
void     pmm_init(e820_entry_t *map, uint32_t count);
void    *pmm_alloc_page(void);
void     pmm_free_page(void *addr);
uint64_t pmm_get_free_pages(void);
uint64_t pmm_get_total_pages(void);

/* VMM — Virtual Memory Manager (vmm.c) */
void vmm_init(void);
void vmm_map_page(uint64_t virt, uint64_t phys, uint64_t flags);
void vmm_unmap_page(uint64_t virt);

/* Heap (heap.c) */
void  heap_init(void);
void *kmalloc(size_t size);
void *kzalloc(size_t size);
void  kfree(void *ptr);
size_t kheap_used(void);
size_t kheap_free(void);

/* Scheduler (scheduler.c) */
void scheduler_init(void);
int  process_create(const char *name, void (*entry)(void));
void process_exit(void);
void process_yield(void);
void scheduler_tick(void);
void scheduler_list(void);

/* Shell (shell.c) */
void shell_init(void);
void shell_run(void);

/* ============================================================
 * Kernel Panic
 * ============================================================ */

__noreturn void kernel_panic(const char *msg);

#define PANIC(msg) kernel_panic(msg)
#define ASSERT(expr) \
    do { if (!(expr)) kernel_panic("ASSERT: " #expr); } while(0)

/* Linker symbols */
extern uint8_t __kernel_start[];
extern uint8_t __kernel_end[];
extern uint8_t __bss_start[];
extern uint8_t __bss_end[];

#endif /* _FASTOS_KERNEL_H */
