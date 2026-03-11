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

/* kernel_panic: 4-arg version (aligned with panic.c + fastos.h) */
__attribute__((noreturn))
void kernel_panic(uint32_t code, const char *message,
                  const char *file, int line);
void kernel_assert_fail(const char *expr, const char *file, int line);

#define KERNEL_PANIC(code, msg) \
    kernel_panic((code), (msg), __FILE__, __LINE__)

#define KERNEL_ASSERT(expr) \
    ((expr) ? (void)0 : kernel_assert_fail(#expr, __FILE__, __LINE__))

/* ============================================================
 * VGA Colors
 * ============================================================ */

typedef enum {
    VGA_BLACK   = 0,  VGA_BLUE      = 1,  VGA_GREEN  = 2,
    VGA_CYAN    = 3,  VGA_RED       = 4,  VGA_MAGENTA = 5,
    VGA_BROWN   = 6,  VGA_LGRAY     = 7,  VGA_DGRAY   = 8,
    VGA_LBLUE   = 9,  VGA_LGREEN   = 10,  VGA_LCYAN  = 11,
    VGA_LRED   = 12,  VGA_LMAGENTA = 13,  VGA_YELLOW = 14,
    VGA_WHITE  = 15,
} vga_color_t;

/* VGA_COLOR(fg, bg): atributo 8-bit para VGA text mode */
#define VGA_COLOR(fg, bg)  ((uint8_t)((uint8_t)(bg) << 4 | (uint8_t)(fg)))

#define VGA_BUFFER  ((volatile uint16_t *)0xB8000)
#define VGA_WIDTH   80
#define VGA_HEIGHT  25

/* ============================================================
 * Alignment Utilities
 * ============================================================ */

#define ALIGN_UP(x, a)    (((x) + ((a) - 1)) & ~((a) - 1))
#define ALIGN_DOWN(x, a)  ((x) & ~((a) - 1))

/* ============================================================
 * VGA Output (implemented in kernel/main.c)
 * ============================================================ */

void term_init(void);
void term_putchar(char c);
void term_write(const char *str);
void term_write_color(const char *str, uint8_t color);

/* vga_putchar: alias de term_putchar para módulos que lo usan directamente */
static inline void vga_putchar(char c) { term_putchar(c); }

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
void *krealloc(void *ptr, size_t size);
void *kmemcpy(void *dest, const void *src, size_t n);
void *kmemset(void *s, int c, size_t n);
int kmemcmp(const void *s1, const void *s2, size_t n);

/* ============================================================
 * String Functions
 * ============================================================ */

size_t kstrlen(const char *s);
char *kstrcpy(char *dest, const char *src);
char *kstrncpy(char *dest, const char *src, size_t n);
int kstrcmp(const char *s1, const char *s2);
int kstrncmp(const char *s1, const char *s2, size_t n);
char *kstrcat(char *dest, const char *src);

/* ============================================================
 * VBE / Graphics
 * ============================================================ */

/* VBE Mode Info (from BIOS) */
typedef struct {
    uint16_t attributes;
    uint8_t  window_a, window_b;
    uint16_t granularity, window_size;
    uint16_t segment_a, segment_b;
    uint32_t win_func_ptr;
    uint16_t pitch;
    uint16_t width, height;
    uint8_t  char_width, char_height;
    uint8_t  planes, bpp, banks;
    uint8_t  memory_model, bank_size, image_pages;
    uint8_t  reserved0;
    uint8_t  red_mask, red_position;
    uint8_t  green_mask, green_position;
    uint8_t  blue_mask, blue_position;
    uint8_t  rsv_mask, rsv_position;
    uint8_t  direct_color;
    uint32_t framebuffer;
    uint32_t off_screen;
    uint16_t off_screen_size;
    uint8_t  reserved1[206];
} __packed vbe_mode_info_t;

/* Framebuffer structure */
typedef struct {
    uint32_t *buffer;
    uint32_t width;
    uint32_t height;
    uint32_t pitch;
    uint32_t bpp;
    uint32_t size;
} framebuffer_t;

/* Graphics functions */
void fb_init(framebuffer_t *fb, uint32_t *buffer, uint32_t w, uint32_t h, uint32_t pitch, uint32_t bpp);
void fb_clear(framebuffer_t *fb, uint32_t color);
void fb_pixel(framebuffer_t *fb, uint32_t x, uint32_t y, uint32_t color);
void fb_rect(framebuffer_t *fb, uint32_t x, uint32_t y, uint32_t w, uint32_t h, uint32_t color);
void fb_line(framebuffer_t *fb, int x0, int y0, int x1, int y1, uint32_t color);
void fb_char(framebuffer_t *fb, uint32_t x, uint32_t y, char c, uint32_t fg, uint32_t bg);
void fb_string(framebuffer_t *fb, uint32_t x, uint32_t y, const char *s, uint32_t fg, uint32_t bg);

/* ============================================================
 * Drivers
 * ============================================================ */

/* Keyboard */
void keyboard_init(void);
void keyboard_handler(void);
char kb_getchar(void);
int kb_getchar_nonblock(char *c);
int kb_has_key(void);

/* Mouse */
void mouse_init(void);
void mouse_handler(void);
void mouse_get_position(int32_t *x, int32_t *y);
int mouse_left_pressed(void);
int mouse_right_pressed(void);
void mouse_set_bounds(int32_t w, int32_t h);

/* Timer */
void timer_init(uint32_t freq);
void timer_handler(void);
uint64_t timer_get_ticks(void);
void timer_sleep(uint32_t ms);

/* ============================================================
 * Process Management
 * ============================================================ */

#define MAX_PROCESSES 64
#define KERNEL_STACK_SIZE 4096
#define USER_STACK_SIZE   (64 * 1024)

typedef enum {
    PROC_UNUSED = 0,
    PROC_READY,
    PROC_RUNNING,
    PROC_BLOCKED,
    PROC_ZOMBIE
} proc_state_t;

/* Contexto CPU completo para context-switch */
typedef struct {
    uint64_t rax, rbx, rcx, rdx;
    uint64_t rsi, rdi, rbp, rsp;
    uint64_t r8,  r9,  r10, r11;
    uint64_t r12, r13, r14, r15;
    uint64_t rip, rflags;
    uint64_t cs, ss;
    uint64_t cr3;
} __packed cpu_context_t;

/* Process Control Block (PCB) — definicion canonica unica.
 * scheduler.c y kernel.h usan ESTA estructura. Sin redefiniciones. */
typedef struct {
    uint32_t      pid;
    uint32_t      ppid;
    proc_state_t  state;
    uint8_t       priority;
    uint8_t       security_level;  /* BG security level */

    cpu_context_t context;         /* Registros CPU completos */

    uint64_t      kernel_stack;    /* Pila del kernel de este proceso */
    uint64_t      user_stack;      /* Pila del userspace */
    uint64_t      page_table;      /* CR3 de este proceso */

    uint64_t      time_slice;      /* Ticks restantes en este turno */
    uint64_t      total_time;      /* Tiempo total de CPU */

    char          name[32];
} process_t;

void interrupts_init(void);   /* idt_init() + interrupts_enable() */
void scheduler_init(void);
int  process_create(const char *name, void (*entry)(void), uint8_t security_level);
void process_exit(int code);
void process_yield(void);
process_t *process_current(void);
void process_block(void);
void process_unblock(uint32_t pid);
void scheduler_tick(void);
void scheduler_list(void);

/* ============================================================
 * Syscall Interface (Hybrid Windows/Linux)
 * ============================================================ */

/* Linux-style syscalls (0x000-0x0FF) */
#define SYS_READ        0x000
#define SYS_WRITE       0x001
#define SYS_OPEN        0x002
#define SYS_CLOSE       0x003
#define SYS_FORK        0x039
#define SYS_EXEC        0x03B
#define SYS_EXIT        0x03C
#define SYS_GETPID      0x027

/* FastOS extensions (0xF00-0xFFF) */
#define SYS_FB_INIT     0xF00
#define SYS_FB_PIXEL    0xF01
#define SYS_FB_RECT     0xF02
#define SYS_GPU_DETECT  0xF10
#define SYS_GPU_MODE    0xF11

int64_t syscall(uint64_t num, uint64_t a1, uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5);

/* ============================================================
 * Po Executable Format
 * ============================================================ */

#define PO_MAGIC 0x506F4F53  /* "PoOS" */

typedef struct {
    uint32_t magic;
    uint16_t type;
    uint16_t machine;
    uint32_t version;
    uint64_t entry;
    uint64_t phoff;
    uint64_t shoff;
    uint32_t flags;
    uint16_t ehsize;
    uint16_t phentsize;
    uint16_t phnum;
    uint16_t shentsize;
    uint16_t shnum;
    uint16_t shstrndx;
    uint8_t  security_level;
    uint8_t  reserved[7];
} __packed po_header_t;

int po_load(const char *path, uint64_t *entry);
int po_verify(po_header_t *hdr);

/* ============================================================
 * Rust Safety Layer (FFI)
 * ============================================================ */

#ifdef __cplusplus
extern "C" {
#endif

/* Safe memory operations from Rust */
void *rust_malloc(size_t size);
void rust_free(void *ptr);
void *rust_calloc(size_t nmemb, size_t size);
int rust_memcpy_safe(void *dest, size_t dest_size, const void *src, size_t count);
int rust_memset_safe(void *dest, size_t dest_size, int value, size_t count);

/* Page table operations */
uint64_t rust_translate(uint64_t cr3, uint64_t virt);
int rust_map_page(uint64_t cr3, uint64_t virt, uint64_t phys, uint64_t flags);

/* Buffer operations */
void *rust_buffer_create(size_t size);
void rust_buffer_destroy(void *buf);
int rust_buffer_get(void *buf, size_t index, uint8_t *value);
int rust_buffer_set(void *buf, size_t index, uint8_t value);

#ifdef __cplusplus
}
#endif

/* ============================================================
 * GPU / Nouveau
 * ============================================================ */

/* Forward declarations */
struct nv_device;
struct nv_framebuffer;

int gpu_init(void);
int gpu_detect_nvidia(struct nv_device *dev);
int gpu_set_mode(uint32_t width, uint32_t height, uint32_t bpp);
uint32_t *gpu_get_framebuffer(void);

#endif /* _FASTOS_KERNEL_H */
