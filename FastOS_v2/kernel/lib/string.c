/*
 * FastOS v2.0 — String & Memory Library
 * Freestanding implementations — no libc dependency
 * ADead-BIB base layer for kernel operations
 */

#include "../include/kernel.h"

/* ── String functions ── */

size_t strlen(const char *s) {
    size_t n = 0;
    while (s[n]) n++;
    return n;
}

char *strcpy(char *dest, const char *src) {
    char *d = dest;
    while ((*d++ = *src++));
    return dest;
}

char *strncpy(char *dest, const char *src, size_t n) {
    size_t i;
    for (i = 0; i < n && src[i]; i++)
        dest[i] = src[i];
    for (; i < n; i++)
        dest[i] = '\0';
    return dest;
}

int strcmp(const char *s1, const char *s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(unsigned char *)s1 - *(unsigned char *)s2;
}

int strncmp(const char *s1, const char *s2, size_t n) {
    while (n && *s1 && (*s1 == *s2)) { s1++; s2++; n--; }
    if (n == 0) return 0;
    return *(unsigned char *)s1 - *(unsigned char *)s2;
}

char *strcat(char *dest, const char *src) {
    char *d = dest;
    while (*d) d++;
    while ((*d++ = *src++));
    return dest;
}

char *strchr(const char *s, int c) {
    while (*s) {
        if (*s == (char)c) return (char *)s;
        s++;
    }
    return (c == '\0') ? (char *)s : NULL;
}

/* ── Memory functions ── */

void *memcpy(void *dest, const void *src, size_t n) {
    uint8_t *d = (uint8_t *)dest;
    const uint8_t *s = (const uint8_t *)src;
    /* 64-bit aligned fast path */
    while (n >= 8 && ((uintptr_t)d & 7) == 0 && ((uintptr_t)s & 7) == 0) {
        *(uint64_t *)d = *(const uint64_t *)s;
        d += 8; s += 8; n -= 8;
    }
    while (n--) *d++ = *s++;
    return dest;
}

void *memset(void *s, int c, size_t n) {
    uint8_t *p = (uint8_t *)s;
    uint8_t val = (uint8_t)c;
    /* 64-bit aligned fast path */
    if (n >= 8 && ((uintptr_t)p & 7) == 0) {
        uint64_t v64 = val;
        v64 |= v64 << 8;  v64 |= v64 << 16; v64 |= v64 << 32;
        while (n >= 8) {
            *(uint64_t *)p = v64;
            p += 8; n -= 8;
        }
    }
    while (n--) *p++ = val;
    return s;
}

void *memmove(void *dest, const void *src, size_t n) {
    uint8_t *d = (uint8_t *)dest;
    const uint8_t *s = (const uint8_t *)src;
    if (d < s) {
        while (n--) *d++ = *s++;
    } else {
        d += n; s += n;
        while (n--) *--d = *--s;
    }
    return dest;
}

int memcmp(const void *s1, const void *s2, size_t n) {
    const uint8_t *a = (const uint8_t *)s1;
    const uint8_t *b = (const uint8_t *)s2;
    while (n--) {
        if (*a != *b) return *a - *b;
        a++; b++;
    }
    return 0;
}

/* ── Integer to ASCII conversion ── */

char *itoa(int64_t value, char *str, int base) {
    char *p = str;
    char *p1, *p2;
    uint64_t uval;

    if (base < 2 || base > 36) { *str = '\0'; return str; }

    if (value < 0 && base == 10) {
        *p++ = '-';
        uval = (uint64_t)(-value);
    } else {
        uval = (uint64_t)value;
    }

    p1 = p;
    do {
        int d = uval % base;
        *p++ = (d < 10) ? ('0' + d) : ('a' + d - 10);
        uval /= base;
    } while (uval);
    *p = '\0';

    /* Reverse digits */
    p2 = p - 1;
    while (p1 < p2) {
        char tmp = *p1; *p1++ = *p2; *p2-- = tmp;
    }
    return str;
}
