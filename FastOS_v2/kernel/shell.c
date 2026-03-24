/*
 * FastOS v2.0 — Interactive Shell
 * VGA text mode terminal with command parsing
 * Built-in commands: help, ver, mem, proc, cpuinfo, clear, uptime
 */

#include "include/kernel.h"

#define CMD_BUF_SIZE   256
#define HISTORY_SIZE   8

static char cmd_buf[CMD_BUF_SIZE];
static int  cmd_len = 0;

/* Command history */
static char history[HISTORY_SIZE][CMD_BUF_SIZE];
static int  hist_count = 0;
static int  hist_pos   = 0;

static void shell_prompt(void) {
    vga_puts_color("C:\\FastOS", VGA_COLOR(VGA_WHITE, VGA_BLACK));
    vga_puts_color("> ", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
}

/* ── Built-in command handlers ── */

static void cmd_help(void) {
    vga_puts_color("FastOS v2.0 Commands:\n", VGA_COLOR(VGA_YELLOW, VGA_BLACK));
    kprintf("  help     - Show this help\n");
    kprintf("  ver      - Version info\n");
    kprintf("  clear    - Clear screen\n");
    kprintf("  mem      - Memory statistics\n");
    kprintf("  proc     - List processes\n");
    kprintf("  cpuinfo  - CPU information\n");
    kprintf("  uptime   - System uptime\n");
    kprintf("  reboot   - Reboot system\n");
}

static void cmd_ver(void) {
    vga_puts_color("FastOS v2.0", VGA_COLOR(VGA_LGREEN, VGA_BLACK));
    kprintf(" - ADead-BIB Kernel\n");
    kprintf("  Target: AMD Ryzen 5 5600X (Zen3)\n");
    kprintf("  Mode:   64-bit Long Mode, AVX2\n");
    kprintf("  Boot:   CSM Legacy (MBR)\n");
    kprintf("  Author: Eddi Andree Salazar Matos\n");
}

static void cmd_mem(void) {
    vga_puts_color("Memory Info:\n", VGA_COLOR(VGA_LCYAN, VGA_BLACK));
    kprintf("  PMM: %llu/%llu pages free (%llu MB free)\n",
            pmm_get_free_pages(), pmm_get_total_pages(),
            (pmm_get_free_pages() * 4096) / (1024 * 1024));
    kprintf("  Heap: %llu bytes used, %llu bytes free\n",
            (uint64_t)kheap_used(), (uint64_t)kheap_free());
}

static void cmd_cpuinfo(void) {
    uint32_t eax, ebx, ecx, edx;

    /* Vendor string */
    cpuid(0, &eax, &ebx, &ecx, &edx);
    char vendor[13];
    *(uint32_t *)&vendor[0] = ebx;
    *(uint32_t *)&vendor[4] = edx;
    *(uint32_t *)&vendor[8] = ecx;
    vendor[12] = '\0';

    vga_puts_color("CPU Info:\n", VGA_COLOR(VGA_LCYAN, VGA_BLACK));
    kprintf("  Vendor: %s\n", vendor);

    /* Brand string (leaves 0x80000002-0x80000004) */
    char brand[49];
    cpuid(0x80000000, &eax, &ebx, &ecx, &edx);
    if (eax >= 0x80000004) {
        uint32_t *b = (uint32_t *)brand;
        cpuid(0x80000002, &b[0], &b[1], &b[2], &b[3]);
        cpuid(0x80000003, &b[4], &b[5], &b[6], &b[7]);
        cpuid(0x80000004, &b[8], &b[9], &b[10], &b[11]);
        brand[48] = '\0';
        kprintf("  Brand:  %s\n", brand);
    }

    /* Family/Model */
    cpuid(1, &eax, &ebx, &ecx, &edx);
    uint32_t family = ((eax >> 8) & 0xF) + ((eax >> 20) & 0xFF);
    uint32_t model  = (((eax >> 16) & 0xF) << 4) | ((eax >> 4) & 0xF);
    kprintf("  Family: 0x%x  Model: 0x%x\n", family, model);

    /* Features */
    kprintf("  SSE4.2: %s  AVX: %s  AVX2: ",
            (ecx & BIT(20)) ? "Yes" : "No",
            (ecx & BIT(28)) ? "Yes" : "No");
    cpuid(7, &eax, &ebx, &ecx, &edx);
    kprintf("%s\n", (ebx & BIT(5)) ? "Yes" : "No");
}

static void cmd_uptime(void) {
    uint64_t secs = timer_get_seconds();
    uint64_t mins = secs / 60;
    uint64_t hours = mins / 60;
    kprintf("Uptime: %llu:%02llu:%02llu (%llu ticks)\n",
            hours, mins % 60, secs % 60, timer_get_ticks());
}

static void cmd_reboot(void) {
    kprintf("Rebooting...\n");
    /* Triple fault: load null IDT and trigger interrupt */
    desc_ptr_t null_idt = { 0, 0 };
    lidt(&null_idt);
    hal_int3();
}

/* ── Command dispatcher ── */

static void shell_execute(const char *cmd) {
    /* Skip leading spaces */
    while (*cmd == ' ') cmd++;
    if (*cmd == '\0') return;

    if (strcmp(cmd, "help") == 0)         cmd_help();
    else if (strcmp(cmd, "ver") == 0)     cmd_ver();
    else if (strcmp(cmd, "clear") == 0)   { vga_clear(); return; }
    else if (strcmp(cmd, "mem") == 0)     cmd_mem();
    else if (strcmp(cmd, "proc") == 0)    scheduler_list();
    else if (strcmp(cmd, "cpuinfo") == 0) cmd_cpuinfo();
    else if (strcmp(cmd, "uptime") == 0)  cmd_uptime();
    else if (strcmp(cmd, "reboot") == 0)  cmd_reboot();
    else {
        vga_puts_color("Unknown command: ", VGA_COLOR(VGA_LRED, VGA_BLACK));
        kprintf("%s\n", cmd);
        kprintf("Type 'help' for available commands.\n");
    }
}

/* ── Shell main loop ── */

void shell_init(void) {
    cmd_len = 0;
    hist_count = 0;
    hist_pos = 0;
}

void shell_run(void) {
    vga_puts_color("\n=== FastOS v2.0 Shell ===\n", VGA_COLOR(VGA_YELLOW, VGA_BLACK));
    kprintf("Type 'help' for commands.\n\n");
    shell_prompt();

    for (;;) {
        char c = keyboard_getchar();

        if (c == '\n') {
            vga_putchar('\n');
            cmd_buf[cmd_len] = '\0';

            /* Save to history */
            if (cmd_len > 0 && hist_count < HISTORY_SIZE) {
                strcpy(history[hist_count++], cmd_buf);
            }

            shell_execute(cmd_buf);
            cmd_len = 0;
            shell_prompt();

        } else if (c == '\b') {
            if (cmd_len > 0) {
                cmd_len--;
                vga_putchar('\b');
            }
        } else if (c == '\t') {
            /* Tab completion: match partial command */
            static const char *commands[] = {
                "help","ver","clear","mem","proc","cpuinfo","uptime","reboot"
            };
            cmd_buf[cmd_len] = '\0';
            for (int i = 0; i < 8; i++) {
                if (strncmp(cmd_buf, commands[i], cmd_len) == 0 && cmd_len > 0) {
                    /* Complete the command */
                    const char *rest = commands[i] + cmd_len;
                    while (*rest) {
                        cmd_buf[cmd_len++] = *rest;
                        vga_putchar(*rest);
                        rest++;
                    }
                    break;
                }
            }
        } else {
            if (cmd_len < CMD_BUF_SIZE - 1) {
                cmd_buf[cmd_len++] = c;
                vga_putchar(c);
            }
        }
    }
}
