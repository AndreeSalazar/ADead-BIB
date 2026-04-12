// ADead-BIB Test: stdarg.h variadic functions
#include <stdio.h>
#include <stdarg.h>

int sum_ints(int count, ...) {
    va_list ap;
    va_start(ap, count);
    int total = 0;
    int i;
    for (i = 0; i < count; i++) {
        total += va_arg(ap, int);
    }
    va_end(ap);
    return total;
}

void print_ints(const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    vprintf(fmt, ap);
    va_end(ap);
}

int main() {
    int pass = 0, fail = 0;

    // va_arg sum
    int s = sum_ints(4, 10, 20, 30, 40);
    if (s == 100) { pass++; printf("PASS: sum_ints(4, 10,20,30,40)=%d\n", s); }
    else { fail++; printf("FAIL: sum_ints=%d expected 100\n", s); }

    // single arg
    s = sum_ints(1, 42);
    if (s == 42) { pass++; printf("PASS: sum_ints(1, 42)=%d\n", s); }
    else { fail++; printf("FAIL: sum_ints=%d expected 42\n", s); }

    // zero args
    s = sum_ints(0);
    if (s == 0) { pass++; printf("PASS: sum_ints(0)=%d\n", s); }
    else { fail++; printf("FAIL: sum_ints=%d expected 0\n", s); }

    // vprintf wrapper
    print_ints("PASS: vprintf wrapper %d %d %d\n", 1, 2, 3);
    pass++;

    printf("\n=== stdarg_basic: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
