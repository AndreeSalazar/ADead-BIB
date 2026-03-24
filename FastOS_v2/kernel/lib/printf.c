/*
 * FastOS v2.0 — kprintf() implementation
 * Minimal printf for kernel space, outputs via VGA driver
 * Supports: %s %c %d %u %x %X %p %% with width and zero-padding
 *
 * Uses a manual va_args implementation for ADead-BIB compiler
 * (no __builtin_va_list support). On System V AMD64 ABI,
 * kprintf(const char *fmt, ...) receives fmt in RDI, then
 * variadic args in RSI, RDX, RCX, R8, R9, then stack.
 *
 * The hal_kprintf_va_start() function (in hal.asm) captures
 * the register arguments into a buffer. Since we cannot do
 * that purely in C without inline asm, we use a simpler approach:
 * treat all variadic arguments as uint64_t-sized slots accessed
 * via a pointer that walks through a saved argument area.
 *
 * APPROACH: We define kprintf as a wrapper that takes a fixed
 * number of arguments (up to 8 variadic), and the real formatter
 * consumes them from an array. This avoids va_list entirely.
 */

#include "../include/kernel.h"

/* ── Internal: unsigned int to string ── */

static void uint_to_str(uint64_t val, char *buf, int base, int upper) {
    const char *digits = upper ? "0123456789ABCDEF" : "0123456789abcdef";
    char tmp[20];
    int i = 0;

    if (val == 0) { buf[0] = '0'; buf[1] = '\0'; return; }

    while (val > 0) {
        tmp[i++] = digits[val % base];
        val /= base;
    }
    int j;
    for (j = 0; i > 0; j++, i--) buf[j] = tmp[i - 1];
    buf[j] = '\0';
}

/* ── Internal: write padded string ── */

static void write_padded(const char *s, int width, char pad) {
    int len = 0;
    const char *p = s;
    while (*p++) len++;
    for (int i = len; i < width; i++) vga_putchar(pad);
    vga_puts(s);
}

/* ── Internal formatter: consumes args from uint64_t array ── */

static void kprintf_internal(const char *fmt, uint64_t *args, int nargs) {
    char buf[32];
    int is_ll, width;
    char pad;
    int ai = 0;

    while (*fmt) {
        if (*fmt != '%') { vga_putchar(*fmt++); continue; }
        fmt++;

        is_ll = 0; width = 0; pad = ' ';

        if (*fmt == '0') { pad = '0'; fmt++; }
        while (*fmt >= '0' && *fmt <= '9') {
            width = width * 10 + (*fmt - '0');
            fmt++;
        }
        if (*fmt == 'l') { fmt++; if (*fmt == 'l') { is_ll = 1; fmt++; } }

        switch (*fmt) {
        case 's': {
            const char *s = (ai < nargs) ? (const char *)args[ai++] : "(null)";
            if (!s) s = "(null)";
            if (width) write_padded(s, width, pad); else vga_puts(s);
            break;
        }
        case 'c':
            vga_putchar((char)(ai < nargs ? args[ai++] : '?'));
            break;
        case 'd': {
            int64_t v = (ai < nargs) ? (int64_t)args[ai++] : 0;
            if (!is_ll) v = (int32_t)v;
            if (v < 0) { buf[0] = '-'; uint_to_str((uint64_t)(-v), buf + 1, 10, 0); }
            else uint_to_str((uint64_t)v, buf, 10, 0);
            if (width) write_padded(buf, width, pad); else vga_puts(buf);
            break;
        }
        case 'u': {
            uint64_t v = (ai < nargs) ? args[ai++] : 0;
            if (!is_ll) v = (uint32_t)v;
            uint_to_str(v, buf, 10, 0);
            if (width) write_padded(buf, width, pad); else vga_puts(buf);
            break;
        }
        case 'x': {
            uint64_t v = (ai < nargs) ? args[ai++] : 0;
            if (!is_ll) v = (uint32_t)v;
            uint_to_str(v, buf, 16, 0);
            if (width) write_padded(buf, width, pad); else vga_puts(buf);
            break;
        }
        case 'X': {
            uint64_t v = (ai < nargs) ? args[ai++] : 0;
            if (!is_ll) v = (uint32_t)v;
            uint_to_str(v, buf, 16, 1);
            if (width) write_padded(buf, width, pad); else vga_puts(buf);
            break;
        }
        case 'p': {
            uint64_t v = (ai < nargs) ? args[ai++] : 0;
            vga_puts("0x");
            uint_to_str(v, buf, 16, 0);
            write_padded(buf, 16, '0');
            break;
        }
        case '%':
            vga_putchar('%');
            break;
        default:
            vga_putchar('%'); vga_putchar(*fmt);
            break;
        }
        fmt++;
    }
}

/*
 * kprintf — public variadic interface
 *
 * On System V AMD64, kprintf(fmt, a, b, c, d, e, ...) gets:
 *   RDI=fmt, RSI=a, RDX=b, RCX=c, R8=d, R9=e, stack=rest
 *
 * Since ADead-BIB has no __builtin_va_*, we declare kprintf with
 * 8 explicit uint64_t arguments. Callers with fewer args will
 * pass garbage in the unused positions, but we only consume
 * what the format string requests. All integer/pointer types
 * fit in uint64_t on x86_64.
 *
 * The actual kprintf symbol is defined in hal.asm as a trampoline
 * that captures all register args into an array and calls
 * kprintf_internal. This way C callers just do kprintf(fmt, ...)
 * with normal calling convention.
 */

/*
 * Since ADead-BIB compiles C and we need true variadic support,
 * we implement kprintf as a C function that receives up to 8
 * extra uint64_t arguments. The C ABI will pass them in registers
 * and stack automatically. We just need to capture them.
 */
void kprintf(const char *fmt,
             uint64_t a1, uint64_t a2, uint64_t a3,
             uint64_t a4, uint64_t a5, uint64_t a6,
             uint64_t a7, uint64_t a8) {
    uint64_t args[8];
    args[0] = a1; args[1] = a2; args[2] = a3; args[3] = a4;
    args[4] = a5; args[5] = a6; args[6] = a7; args[7] = a8;
    kprintf_internal(fmt, args, 8);
}
