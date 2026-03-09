#include <stdarg.h>
#include <stdio.h>

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

double average(int count, ...) {
    va_list args;
    va_start(args, count);
    double total = 0.0;
    for (int i = 0; i < count; i++) {
        total += va_arg(args, double);
    }
    va_end(args);
    return total / count;
}

int main() {
    int s = sum(4, 10, 20, 30, 40);
    double a = average(3, 1.0, 2.0, 3.0);
    printf("sum = %d, avg = %f\n", s, a);
    return 0;
}
