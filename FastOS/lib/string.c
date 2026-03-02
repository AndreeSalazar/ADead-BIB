/*
 * FastOS v2.0 — String Library
 * ADead-BIB Native Operating System
 */

#include "../include/types.h"

/* String length */
size_t strlen(const char *s) {
    size_t len = 0;
    while (s[len]) len++;
    return len;
}

/* String copy */
char* strcpy(char *dest, const char *src) {
    char *d = dest;
    while ((*d++ = *src++));
    return dest;
}

/* String copy with limit */
char* strncpy(char *dest, const char *src, size_t n) {
    size_t i;
    for (i = 0; i < n && src[i]; i++) {
        dest[i] = src[i];
    }
    for (; i < n; i++) {
        dest[i] = '\0';
    }
    return dest;
}

/* String compare */
int strcmp(const char *s1, const char *s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

/* String compare with limit */
int strncmp(const char *s1, const char *s2, size_t n) {
    while (n && *s1 && (*s1 == *s2)) {
        s1++;
        s2++;
        n--;
    }
    if (n == 0) return 0;
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

/* String concatenate */
char* strcat(char *dest, const char *src) {
    char *d = dest;
    while (*d) d++;
    while ((*d++ = *src++));
    return dest;
}

/* Find character in string */
char* strchr(const char *s, int c) {
    while (*s) {
        if (*s == (char)c) return (char*)s;
        s++;
    }
    return (c == '\0') ? (char*)s : NULL;
}

/* Find last occurrence of character */
char* strrchr(const char *s, int c) {
    const char *last = NULL;
    while (*s) {
        if (*s == (char)c) last = s;
        s++;
    }
    return (c == '\0') ? (char*)s : (char*)last;
}

/* Find substring */
char* strstr(const char *haystack, const char *needle) {
    if (!*needle) return (char*)haystack;
    
    while (*haystack) {
        const char *h = haystack;
        const char *n = needle;
        
        while (*h && *n && (*h == *n)) {
            h++;
            n++;
        }
        
        if (!*n) return (char*)haystack;
        haystack++;
    }
    return NULL;
}

/* Memory copy */
void* memcpy(void *dest, const void *src, size_t n) {
    uint8_t *d = (uint8_t*)dest;
    const uint8_t *s = (const uint8_t*)src;
    
    /* Fast copy for aligned data */
    while (n >= 8 && ((uintptr_t)d & 7) == 0 && ((uintptr_t)s & 7) == 0) {
        *(uint64_t*)d = *(const uint64_t*)s;
        d += 8;
        s += 8;
        n -= 8;
    }
    
    while (n--) {
        *d++ = *s++;
    }
    return dest;
}

/* Memory move (handles overlapping) */
void* memmove(void *dest, const void *src, size_t n) {
    uint8_t *d = (uint8_t*)dest;
    const uint8_t *s = (const uint8_t*)src;
    
    if (d < s) {
        while (n--) *d++ = *s++;
    } else {
        d += n;
        s += n;
        while (n--) *--d = *--s;
    }
    return dest;
}

/* Memory set */
void* memset(void *s, int c, size_t n) {
    uint8_t *p = (uint8_t*)s;
    uint8_t val = (uint8_t)c;
    
    /* Fast set for aligned data */
    if (n >= 8 && ((uintptr_t)p & 7) == 0) {
        uint64_t val64 = val;
        val64 |= val64 << 8;
        val64 |= val64 << 16;
        val64 |= val64 << 32;
        
        while (n >= 8) {
            *(uint64_t*)p = val64;
            p += 8;
            n -= 8;
        }
    }
    
    while (n--) {
        *p++ = val;
    }
    return s;
}

/* Memory compare */
int memcmp(const void *s1, const void *s2, size_t n) {
    const uint8_t *p1 = (const uint8_t*)s1;
    const uint8_t *p2 = (const uint8_t*)s2;
    
    while (n--) {
        if (*p1 != *p2) {
            return *p1 - *p2;
        }
        p1++;
        p2++;
    }
    return 0;
}

/* Find byte in memory */
void* memchr(const void *s, int c, size_t n) {
    const uint8_t *p = (const uint8_t*)s;
    uint8_t val = (uint8_t)c;
    
    while (n--) {
        if (*p == val) return (void*)p;
        p++;
    }
    return NULL;
}

/* Integer to string */
char* itoa(int64_t value, char *str, int base) {
    char *p = str;
    char *p1, *p2;
    uint64_t uvalue;
    
    if (base < 2 || base > 36) {
        *str = '\0';
        return str;
    }
    
    if (value < 0 && base == 10) {
        *p++ = '-';
        uvalue = -value;
    } else {
        uvalue = (uint64_t)value;
    }
    
    p1 = p;
    
    do {
        int digit = uvalue % base;
        *p++ = (digit < 10) ? ('0' + digit) : ('a' + digit - 10);
        uvalue /= base;
    } while (uvalue);
    
    *p = '\0';
    
    /* Reverse */
    p2 = p - 1;
    while (p1 < p2) {
        char tmp = *p1;
        *p1++ = *p2;
        *p2-- = tmp;
    }
    
    return str;
}

/* String to integer */
int64_t atoi(const char *str) {
    int64_t result = 0;
    int sign = 1;
    
    while (*str == ' ' || *str == '\t') str++;
    
    if (*str == '-') {
        sign = -1;
        str++;
    } else if (*str == '+') {
        str++;
    }
    
    while (*str >= '0' && *str <= '9') {
        result = result * 10 + (*str - '0');
        str++;
    }
    
    return sign * result;
}
