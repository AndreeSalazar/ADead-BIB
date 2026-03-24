/*
 * FastOS v2.0 — Kernel Main
 * Orchestrates all subsystem initialization in order
 * Entry point called by kernel_entry.asm
 *
 * Boot sequence:
 *   1. VGA init (for debug output from the start)
 *   2. GDT reload (kernel-owned GDT with TSS)
 *   3. IDT setup (256 entries, exception + IRQ handlers)
 *   4. PIC remap (IRQs to vectors 32-47)
 *   5. PMM init (physical memory from E820 map)
 *   6. VMM init (read current page tables from CR3)
 *   7. Heap init (kmalloc/kfree at 0x200000)
 *   8. Timer init (PIT 100Hz, enables preemption)
 *   9. Keyboard init (PS/2, IRQ1)
 *  10. Scheduler init
 *  11. Enable interrupts
 *  12. Shell (interactive terminal)
 */

#include "include/kernel.h"

/* Serial output for debug (COM1 = 0x3F8) */
static void serial_init(void) {
    outb(0x3F9, 0x00);   /* Disable interrupts */
    outb(0x3FB, 0x80);   /* Enable DLAB */
    outb(0x3F8, 0x01);   /* Baud divisor low (115200) */
    outb(0x3F9, 0x00);   /* Baud divisor high */
    outb(0x3FB, 0x03);   /* 8N1 */
    outb(0x3FA, 0xC7);   /* FIFO enable, clear, 14-byte threshold */
    outb(0x3FC, 0x0B);   /* IRQs, RTS/DSR set */
}

static void serial_putchar(char c) {
    while (!(inb(0x3FD) & 0x20));   /* Wait for transmit buffer empty */
    outb(0x3F8, c);
}

static void serial_puts(const char *s) {
    while (*s) serial_putchar(*s++);
}

/* Read E820 count from the map area at 0x8000.
 * stage2.asm stores entries at 0x8000, each 24 bytes.
 * We count entries by scanning until we hit a zero-length entry
 * or reach a maximum. */
static uint32_t e820_count_entries(e820_entry_t *map) {
    uint32_t count = 0;
    for (uint32_t i = 0; i < 64; i++) {
        if (map[i].length == 0) break;
        count++;
    }
    return count;
}

/* Kernel panic: print message and halt */
__noreturn void kernel_panic(const char *msg) {
    cli();
    vga_set_color(VGA_COLOR(VGA_WHITE, VGA_RED));
    kprintf("\n\n !!! KERNEL PANIC !!!\n %s\n", msg);
    serial_puts("\n!!! KERNEL PANIC: ");
    serial_puts(msg);
    serial_puts("\n");
    for (;;) hlt();
}

/* ════════════════════════════════════════════
 * kernel_main — Called by kernel_entry.asm
 * RDI = boot info address (0x500)
 * ════════════════════════════════════════════ */

void kernel_main(uint64_t boot_info_addr) {
    (void)boot_info_addr;

    /* ── Phase 0: Serial + VGA ── */
    serial_init();
    serial_puts("[FastOS] Kernel starting...\r\n");

    vga_init();
    vga_puts_color("FastOS v2.0", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    vga_puts_color(" — ADead-BIB Kernel for Ryzen 5 5600X\n\n",
                   VGA_COLOR(VGA_LGRAY, VGA_BLACK));

    /* ── Phase 1: GDT ── */
    kprintf("[INIT] GDT...");
    gdt_init();
    vga_puts_color(" OK\n", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    serial_puts("[INIT] GDT OK\r\n");

    /* ── Phase 2: IDT ── */
    kprintf("[INIT] IDT...");
    idt_init();
    vga_puts_color(" OK\n", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    serial_puts("[INIT] IDT OK\r\n");

    /* ── Phase 3: PIC ── */
    kprintf("[INIT] PIC 8259...");
    pic_init();
    vga_puts_color(" OK\n", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    serial_puts("[INIT] PIC OK\r\n");

    /* ── Phase 4: Physical Memory ── */
    kprintf("[INIT] PMM (E820)...");
    e820_entry_t *e820_map = (e820_entry_t *)0x8000;
    uint32_t e820_count = e820_count_entries(e820_map);
    pmm_init(e820_map, e820_count);
    kprintf(" OK (%llu pages free, %llu MB)\n",
            pmm_get_free_pages(),
            (pmm_get_free_pages() * 4096) / (1024 * 1024));
    serial_puts("[INIT] PMM OK\r\n");

    /* ── Phase 5: Virtual Memory ── */
    kprintf("[INIT] VMM...");
    vmm_init();
    vga_puts_color(" OK\n", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    serial_puts("[INIT] VMM OK\r\n");

    /* ── Phase 6: Kernel Heap ── */
    kprintf("[INIT] Heap (8MB @ 0x200000)...");
    heap_init();
    vga_puts_color(" OK\n", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    serial_puts("[INIT] Heap OK\r\n");

    /* ── Phase 7: Scheduler ── */
    kprintf("[INIT] Scheduler...");
    scheduler_init();
    vga_puts_color(" OK\n", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    serial_puts("[INIT] Scheduler OK\r\n");

    /* ── Phase 8: Timer (100 Hz) ── */
    kprintf("[INIT] PIT Timer (100 Hz)...");
    timer_init(100);
    vga_puts_color(" OK\n", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    serial_puts("[INIT] Timer OK\r\n");

    /* ── Phase 9: Keyboard ── */
    kprintf("[INIT] PS/2 Keyboard...");
    keyboard_init();
    vga_puts_color(" OK\n", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    serial_puts("[INIT] Keyboard OK\r\n");

    /* ── Phase 10: Enable interrupts ── */
    kprintf("[INIT] Enabling interrupts...");
    sti();
    vga_puts_color(" OK\n", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    serial_puts("[INIT] STI OK\r\n");

    /* ── CPU Info Banner ── */
    vga_putchar('\n');
    uint32_t eax, ebx, ecx, edx;
    cpuid(0, &eax, &ebx, &ecx, &edx);
    char vendor[13];
    *(uint32_t *)&vendor[0] = ebx;
    *(uint32_t *)&vendor[4] = edx;
    *(uint32_t *)&vendor[8] = ecx;
    vendor[12] = '\0';
    kprintf("CPU: %s", vendor);

    cpuid(0x80000000, &eax, &ebx, &ecx, &edx);
    if (eax >= 0x80000004) {
        char brand[49];
        uint32_t *b = (uint32_t *)brand;
        cpuid(0x80000002, &b[0], &b[1], &b[2], &b[3]);
        cpuid(0x80000003, &b[4], &b[5], &b[6], &b[7]);
        cpuid(0x80000004, &b[8], &b[9], &b[10], &b[11]);
        brand[48] = '\0';
        kprintf(" — %s", brand);
    }
    vga_putchar('\n');

    /* Check AVX2 */
    cpuid(7, &eax, &ebx, &ecx, &edx);
    if (ebx & BIT(5)) {
        vga_puts_color("AVX2: Active (256-bit YMM)\n",
                       VGA_COLOR(VGA_YELLOW, VGA_BLACK));
    }

    kprintf("Kernel: 0x%p - 0x%p (%llu KB)\n",
            __kernel_start, __kernel_end,
            ((uint64_t)__kernel_end - (uint64_t)__kernel_start) / 1024);

    /* ── Phase 11: Shell ── */
    serial_puts("[INIT] Starting shell...\r\n");
    shell_init();
    shell_run();   /* Never returns */

    /* Should never reach here */
    kernel_panic("kernel_main: shell_run() returned");
}
