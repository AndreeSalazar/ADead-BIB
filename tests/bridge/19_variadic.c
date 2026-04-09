// ADead-BIB Bridge Test 19 — Variadic Functions
// Level: INTERMEDIATE
// Tests: variadic functions using va_list, va_start, va_arg, va_end

#include <stdio.h>
#include <stdarg.h>
#include <string.h>

int sum(int count, ...) {
    va_list args;
    va_start(args, count);
    int total = 0;
    for (int i = 0; i < count; i++) {
        total += va_arg(args, int);
    }
    va_end(args);
    return total;
}

int max_of(int count, ...) {
    va_list args;
    va_start(args, count);
    int m = va_arg(args, int);
    for (int i = 1; i < count; i++) {
        int v = va_arg(args, int);
        if (v > m) m = v;
    }
    va_end(args);
    return m;
}

int min_of(int count, ...) {
    va_list args;
    va_start(args, count);
    int m = va_arg(args, int);
    for (int i = 1; i < count; i++) {
        int v = va_arg(args, int);
        if (v < m) m = v;
    }
    va_end(args);
    return m;
}

// Simple sprintf-like: supports %d and %s only, writes to buf
void simple_format(char *buf, int bufsize, const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);
    int pos = 0;
    for (const char *p = fmt; *p && pos < bufsize - 1; p++) {
        if (*p == '%' && *(p+1) == 'd') {
            int val = va_arg(args, int);
            pos += snprintf(buf + pos, bufsize - pos, "%d", val);
            p++;
        } else if (*p == '%' && *(p+1) == 's') {
            const char *s = va_arg(args, const char*);
            pos += snprintf(buf + pos, bufsize - pos, "%s", s);
            p++;
        } else {
            buf[pos++] = *p;
        }
    }
    buf[pos] = '\0';
    va_end(args);
}

int main() {
    printf("=== ADead-BIB Bridge Test 19: Variadic ===\n");
    int pass = 0, fail = 0;

    // sum(3, 10, 20, 30) == 60
    if (sum(3, 10, 20, 30) == 60) { pass++; } else { fail++; printf("FAIL: sum(3,10,20,30)\n"); }

    // sum(5, 1, 2, 3, 4, 5) == 15
    if (sum(5, 1, 2, 3, 4, 5) == 15) { pass++; } else { fail++; printf("FAIL: sum(5,1..5)\n"); }

    // sum(1, 42) == 42
    if (sum(1, 42) == 42) { pass++; } else { fail++; printf("FAIL: sum(1,42)\n"); }

    // max_of(4, 10, 42, 7, 35) == 42
    if (max_of(4, 10, 42, 7, 35) == 42) { pass++; } else { fail++; printf("FAIL: max_of\n"); }

    // min_of(4, 10, 42, 7, 35) == 7
    if (min_of(4, 10, 42, 7, 35) == 7) { pass++; } else { fail++; printf("FAIL: min_of\n"); }

    // max_of with negative numbers
    if (max_of(3, -5, -1, -10) == -1) { pass++; } else { fail++; printf("FAIL: max_of neg\n"); }

    // simple_format with %d
    char buf[128];
    simple_format(buf, sizeof(buf), "value=%d!", 99);
    if (strcmp(buf, "value=99!") == 0) { pass++; } else { fail++; printf("FAIL: simple_format %%d got '%s'\n", buf); }

    // simple_format with %s and %d
    simple_format(buf, sizeof(buf), "%s has %d items", "list", 5);
    if (strcmp(buf, "list has 5 items") == 0) { pass++; } else { fail++; printf("FAIL: simple_format %%s%%d got '%s'\n", buf); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 19: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
