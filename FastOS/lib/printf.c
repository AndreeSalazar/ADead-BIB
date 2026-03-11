/*
 * lib/printf.c — kprintf() y kputs() para el kernel FastOS
 *
 * Implementacion minima de printf para kernel space.
 * Sin libc externa. Sin heap. Buffer en stack.
 *
 * Formatos soportados:
 *   %s  string
 *   %c  char
 *   %d  int decimal (signed)
 *   %u  uint decimal
 *   %x  hex lowercase
 *   %X  hex uppercase
 *   %016llX / %016llx  — 64-bit hex, 16 digitos, zero-padded
 *   %04X / %02X        — hex con ancho minimo
 *   %p  puntero (hex 64-bit)
 *
 * kputs(s)      : escribe s + newline
 * kprintf(fmt)  : formato completo
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ─── Conversion de enteros a string ─── */

static void uint_to_str(uint64_t val, char *buf, int base, int uppercase) {
    const char *digits_lo = "0123456789abcdef";
    const char *digits_hi = "0123456789ABCDEF";
    const char *digits = uppercase ? digits_hi : digits_lo;

    char tmp[20];
    int  i = 0;

    if (val == 0) { buf[0] = '0'; buf[1] = '\0'; return; }

    while (val > 0) {
        tmp[i++] = digits[val % base];
        val /= base;
    }
    /* Invertir */
    int j;
    for (j = 0; i > 0; j++, i--) buf[j] = tmp[i-1];
    buf[j] = '\0';
}

static void int_to_str(int64_t val, char *buf) {
    if (val < 0) {
        buf[0] = '-';
        uint_to_str((uint64_t)(-val), buf+1, 10, 0);
    } else {
        uint_to_str((uint64_t)val, buf, 10, 0);
    }
}

/* ─── Escribir string con padding ─── */

static void write_padded(const char *s, int width, char pad) {
    int len = 0;
    const char *p = s;
    while (*p++) len++;

    /* Zero/space padding a la izquierda */
    for (int i = len; i < width; i++) term_putchar(pad);
    term_write(s);
}

/* ─── kprintf ─── */

void kprintf(const char *fmt, ...) {
    /* Acceso manual a argumentos mediante __builtin_va_list
     * (compatible con ADead-BIB y GCC freestanding) */
    __builtin_va_list args;
    __builtin_va_start(args, fmt);

    char buf[32];
    int  is_long_long = 0;
    int  width        = 0;
    char pad_char     = ' ';

    while (*fmt) {
        if (*fmt != '%') {
            term_putchar(*fmt++);
            continue;
        }
        fmt++;  /* Salta '%' */

        /* Reiniciar flags */
        is_long_long = 0;
        width        = 0;
        pad_char     = ' ';

        /* Zero padding */
        if (*fmt == '0') { pad_char = '0'; fmt++; }

        /* Ancho minimo */
        while (*fmt >= '1' && *fmt <= '9') {
            width = width * 10 + (*fmt - '0');
            fmt++;
        }

        /* Modificador de longitud: ll o l */
        if (*fmt == 'l') {
            fmt++;
            if (*fmt == 'l') { is_long_long = 1; fmt++; }
        }

        switch (*fmt) {
        case 's': {
            const char *s = __builtin_va_arg(args, const char*);
            if (!s) s = "(null)";
            if (width > 0) write_padded(s, width, pad_char);
            else           term_write(s);
            break;
        }
        case 'c':
            term_putchar((char)__builtin_va_arg(args, int));
            break;

        case 'd': {
            int64_t v = is_long_long
                ? __builtin_va_arg(args, int64_t)
                : (int64_t)__builtin_va_arg(args, int);
            int_to_str(v, buf);
            if (width > 0) write_padded(buf, width, pad_char);
            else           term_write(buf);
            break;
        }
        case 'u': {
            uint64_t v = is_long_long
                ? __builtin_va_arg(args, uint64_t)
                : (uint64_t)__builtin_va_arg(args, unsigned int);
            uint_to_str(v, buf, 10, 0);
            if (width > 0) write_padded(buf, width, pad_char);
            else           term_write(buf);
            break;
        }
        case 'x': {
            uint64_t v = is_long_long
                ? __builtin_va_arg(args, uint64_t)
                : (uint64_t)__builtin_va_arg(args, unsigned int);
            uint_to_str(v, buf, 16, 0);
            if (width > 0) write_padded(buf, width, pad_char);
            else           term_write(buf);
            break;
        }
        case 'X': {
            uint64_t v = is_long_long
                ? __builtin_va_arg(args, uint64_t)
                : (uint64_t)__builtin_va_arg(args, unsigned int);
            uint_to_str(v, buf, 16, 1);
            if (width > 0) write_padded(buf, width, pad_char);
            else           term_write(buf);
            break;
        }
        case 'p': {
            uint64_t v = (uint64_t)__builtin_va_arg(args, void*);
            term_write("0x");
            uint_to_str(v, buf, 16, 0);
            write_padded(buf, 16, '0');
            break;
        }
        case '%':
            term_putchar('%');
            break;
        default:
            term_putchar('%');
            term_putchar(*fmt);
            break;
        }
        fmt++;
    }

    __builtin_va_end(args);
}

/* kputs: definido en kernel/main.c (sin newline automatico) */
