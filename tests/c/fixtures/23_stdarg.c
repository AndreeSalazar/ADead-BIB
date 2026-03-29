// Test: <stdarg.h> — Variable arguments
// Expected: Compile OK — va_list, va_start, va_arg, va_end

#include <stdarg.h>
#include <stdio.h>

int sum_ints(int count, ...) {
    va_list args;
    va_start(args, count);

    int total = 0;
    for (int i = 0; i < count; i++) {
        total += va_arg(args, int);
    }

    va_end(args);
    return total;
}

void print_fmt(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);
    vprintf(fmt, args);
    va_end(args);
}

int main() {
    printf("=== stdarg.h test ===\n");

    int s1 = sum_ints(3, 10, 20, 30);
    printf("sum_ints(3, 10, 20, 30) = %d\n", s1);

    int s2 = sum_ints(5, 1, 2, 3, 4, 5);
    printf("sum_ints(5, 1..5) = %d\n", s2);

    print_fmt("formatted: %d + %d = %d\n", 10, 20, 30);

    printf("=== stdarg.h OK ===\n");
    return 0;
}
