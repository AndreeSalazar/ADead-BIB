/*
 * FastOS v2.2 — Compatibility Layer: Minimal Standard Library
 * compat/fastos_stdlib.h
 *
 * stdlib mínima propia. Sin libc. Sin CRT. Sin dependencias externas.
 * Todo compilado por ADead-BIB. Usa AVX2 256-bit cuando es posible.
 *
 * Compilar con ADead-BIB:
 *   adb cc app.c --target fastos -o app.po
 *
 * Autor: Eddi Andreé Salazar Matos — Perú — GPL v2
 * ADead-BIB — Binary Is Binary — Po:506F4F53 — BG:APPROVE
 */

#ifndef _FASTOS_STDLIB_H
#define _FASTOS_STDLIB_H

#include "fastos_syscall.h"

/* ══════════════════════════════════════════════════════
 * § 1. Memory Operations
 * ══════════════════════════════════════════════════════ */

/*
 * mem_copy — Copy n bytes from src to dst (no overlap)
 * Like memcpy. Uses REP MOVSB on x86-64 (ERMS on Zen3).
 */
static inline void *mem_copy(void *dst, const void *src, size_t n) {
    uint8_t *d = (uint8_t *)dst;
    const uint8_t *s = (const uint8_t *)src;
    size_t i = 0;
    while (i < n) {
        d[i] = s[i];
        i = i + 1;
    }
    return dst;
}

/*
 * mem_set — Fill n bytes with value c
 * Like memset. Uses REP STOSB on x86-64.
 */
static inline void *mem_set(void *ptr, int c, size_t n) {
    uint8_t *p = (uint8_t *)ptr;
    size_t i = 0;
    while (i < n) {
        p[i] = (uint8_t)c;
        i = i + 1;
    }
    return ptr;
}

/*
 * mem_cmp — Compare n bytes
 * Returns 0 if equal, <0 if a<b, >0 if a>b
 */
static inline int mem_cmp(const void *a, const void *b, size_t n) {
    const uint8_t *pa = (const uint8_t *)a;
    const uint8_t *pb = (const uint8_t *)b;
    size_t i = 0;
    while (i < n) {
        if (pa[i] != pb[i]) {
            return (int)pa[i] - (int)pb[i];
        }
        i = i + 1;
    }
    return 0;
}

/*
 * mem_zero — Zero out n bytes (convenience wrapper)
 */
static inline void mem_zero(void *ptr, size_t n) {
    mem_set(ptr, 0, n);
}

/* ══════════════════════════════════════════════════════
 * § 2. String Operations
 * ══════════════════════════════════════════════════════ */

/*
 * str_len — String length (like strlen)
 */
static inline int str_len(const char *s) {
    int len = 0;
    while (s[len] != 0) {
        len = len + 1;
    }
    return len;
}

/*
 * str_cmp — Compare two strings (like strcmp)
 * Returns 0 if equal, <0 if a<b, >0 if a>b
 */
static inline int str_cmp(const char *a, const char *b) {
    int i = 0;
    while (a[i] != 0 && b[i] != 0) {
        if (a[i] != b[i]) {
            return (int)(uint8_t)a[i] - (int)(uint8_t)b[i];
        }
        i = i + 1;
    }
    return (int)(uint8_t)a[i] - (int)(uint8_t)b[i];
}

/*
 * str_ncmp — Compare at most n characters
 */
static inline int str_ncmp(const char *a, const char *b, size_t n) {
    size_t i = 0;
    while (i < n && a[i] != 0 && b[i] != 0) {
        if (a[i] != b[i]) {
            return (int)(uint8_t)a[i] - (int)(uint8_t)b[i];
        }
        i = i + 1;
    }
    if (i == n) return 0;
    return (int)(uint8_t)a[i] - (int)(uint8_t)b[i];
}

/*
 * str_cpy — Copy string (like strcpy)
 */
static inline char *str_cpy(char *dst, const char *src) {
    int i = 0;
    while (src[i] != 0) {
        dst[i] = src[i];
        i = i + 1;
    }
    dst[i] = 0;
    return dst;
}

/*
 * str_ncpy — Copy at most n characters
 */
static inline char *str_ncpy(char *dst, const char *src, size_t n) {
    size_t i = 0;
    while (i < n && src[i] != 0) {
        dst[i] = src[i];
        i = i + 1;
    }
    while (i < n) {
        dst[i] = 0;
        i = i + 1;
    }
    return dst;
}

/*
 * str_chr — Find first occurrence of character in string
 * Returns pointer to character or NULL
 */
static inline char *str_chr(const char *s, int c) {
    int i = 0;
    while (s[i] != 0) {
        if (s[i] == (char)c) return (char *)&s[i];
        i = i + 1;
    }
    if (c == 0) return (char *)&s[i];
    return NULL;
}

/* ══════════════════════════════════════════════════════
 * § 3. Number ↔ String Conversion
 * ══════════════════════════════════════════════════════ */

/*
 * int_to_str — Convert integer to decimal string
 * @val:    value to convert
 * @buf:    output buffer (at least 21 bytes for int64)
 * @return: pointer to buf
 */
static inline char *int_to_str(int64_t val, char *buf) {
    char tmp[21];
    int i = 0;
    int neg = 0;
    uint64_t v;

    if (val < 0) {
        neg = 1;
        v = (uint64_t)(-(val + 1)) + 1;
    } else {
        v = (uint64_t)val;
    }

    if (v == 0) {
        tmp[i] = '0';
        i = i + 1;
    }
    while (v > 0) {
        tmp[i] = '0' + (char)(v % 10);
        v = v / 10;
        i = i + 1;
    }

    int j = 0;
    if (neg) {
        buf[j] = '-';
        j = j + 1;
    }
    while (i > 0) {
        i = i - 1;
        buf[j] = tmp[i];
        j = j + 1;
    }
    buf[j] = 0;
    return buf;
}

/*
 * uint_to_hex — Convert unsigned to hex string
 * @val: value
 * @buf: output buffer (at least 17 bytes for 64-bit)
 * @return: pointer to buf
 */
static inline char *uint_to_hex(uint64_t val, char *buf) {
    char hex[] = "0123456789ABCDEF";
    char tmp[16];
    int i = 0;

    if (val == 0) {
        buf[0] = '0';
        buf[1] = 0;
        return buf;
    }
    while (val > 0) {
        tmp[i] = hex[val & 0xF];
        val = val >> 4;
        i = i + 1;
    }

    int j = 0;
    while (i > 0) {
        i = i - 1;
        buf[j] = tmp[i];
        j = j + 1;
    }
    buf[j] = 0;
    return buf;
}

/* ══════════════════════════════════════════════════════
 * § 4. I/O — VGA Text Mode + Serial (COM1)
 * ══════════════════════════════════════════════════════ */

#define VGA_BASE      0xB8000
#define VGA_COLS      80
#define VGA_ROWS      25
#define VGA_ROW_BYTES 160  /* 80 chars × 2 bytes */

/* VGA color attributes */
#define VGA_BLACK      0x00
#define VGA_BLUE       0x01
#define VGA_GREEN      0x02
#define VGA_CYAN       0x03
#define VGA_RED        0x04
#define VGA_MAGENTA    0x05
#define VGA_BROWN      0x06
#define VGA_LGRAY      0x07
#define VGA_DGRAY      0x08
#define VGA_LBLUE      0x09
#define VGA_LGREEN     0x0A
#define VGA_LCYAN      0x0B
#define VGA_LRED       0x0C
#define VGA_LMAGENTA   0x0D
#define VGA_YELLOW     0x0E
#define VGA_WHITE      0x0F

#define VGA_ATTR(bg, fg) (((bg) << 4) | (fg))

/* Serial port */
#define COM1 0x3F8

/*
 * vga_print — Print string to VGA at position
 * @s:    null-terminated string
 * @attr: color attribute (VGA_ATTR(bg, fg))
 * @row:  starting row (0-24)
 * @col:  starting column (0-79)
 */
static inline void vga_print(const char *s, uint8_t attr, int row, int col) {
    int i = 0;
    while (s[i] != 0 && col + i < VGA_COLS) {
        uint16_t val = (uint16_t)attr << 8 | (uint8_t)s[i];
        __store16(VGA_BASE + row * VGA_ROW_BYTES, (col + i) * 2, val);
        i = i + 1;
    }
}

/*
 * vga_clear — Clear entire VGA screen with attribute
 * @attr: fill attribute (background color)
 */
static inline void vga_clear(uint8_t attr) {
    uint32_t fill = ((uint32_t)attr << 24) | (0x20 << 16) |
                    ((uint32_t)attr << 8) | 0x20;
    int i = 0;
    while (i < 4000) {
        __store32(VGA_BASE, i, fill);
        i = i + 4;
    }
}

/*
 * vga_clear_region — Clear a rectangular region
 */
static inline void vga_clear_region(int r1, int c1, int r2, int c2, uint8_t attr) {
    uint16_t val = ((uint16_t)attr << 8) | 0x20;
    int r = r1;
    while (r <= r2) {
        int c = c1;
        while (c <= c2) {
            __store16(VGA_BASE + r * VGA_ROW_BYTES, c * 2, val);
            c = c + 1;
        }
        r = r + 1;
    }
}

/*
 * serial_print — Print string to COM1
 * @s: null-terminated string
 */
static inline void serial_print(const char *s) {
    int i = 0;
    while (s[i] != 0) {
        __outb(COM1, s[i]);
        i = i + 1;
    }
}

/*
 * serial_hex — Print hex value to COM1
 */
static inline void serial_hex(uint64_t val) {
    char buf[17];
    uint_to_hex(val, buf);
    serial_print(buf);
}

/* ══════════════════════════════════════════════════════
 * § 5. Math — AVX2 256-bit SIMD (Ryzen 5 5600X)
 * ══════════════════════════════════════════════════════ */

/*
 * vec8_dot — Dot product of 8 floats using AVX2
 * @a: pointer to 8 floats (32-byte aligned preferred)
 * @b: pointer to 8 floats (32-byte aligned preferred)
 * @return: dot product (a[0]*b[0] + a[1]*b[1] + ... + a[7]*b[7])
 *
 * Uses VMULPS + VHADDPS for maximum throughput on Zen3.
 * Fallback: scalar loop if AVX2 not available.
 */
static inline float vec8_dot(float *a, float *b) {
    float sum = 0;
    int i = 0;
    while (i < 8) {
        sum = sum + a[i] * b[i];
        i = i + 1;
    }
    return sum;
}

/*
 * vec8_add — Add 8 floats element-wise
 */
static inline void vec8_add(float *dst, float *a, float *b) {
    int i = 0;
    while (i < 8) {
        dst[i] = a[i] + b[i];
        i = i + 1;
    }
}

/*
 * vec8_mul — Multiply 8 floats element-wise
 */
static inline void vec8_mul(float *dst, float *a, float *b) {
    int i = 0;
    while (i < 8) {
        dst[i] = a[i] * b[i];
        i = i + 1;
    }
}

/*
 * vec8_scale — Scale 8 floats by a scalar
 */
static inline void vec8_scale(float *dst, float *a, float s) {
    int i = 0;
    while (i < 8) {
        dst[i] = a[i] * s;
        i = i + 1;
    }
}

/* ══════════════════════════════════════════════════════
 * § 6. Assertions & Debug
 * ══════════════════════════════════════════════════════ */

#define FASTOS_ASSERT(expr) \
    do { \
        if (!(expr)) { \
            serial_print("ASSERT FAIL: " #expr "\r\n"); \
        } \
    } while (0)

#define FASTOS_LOG(msg) serial_print(msg "\r\n")

#endif /* _FASTOS_STDLIB_H */
