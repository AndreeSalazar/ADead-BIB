#include <stdio.h>

int fib(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    return fib(n - 1) + fib(n - 2);
}

int power(int base, int exp) {
    if (exp == 0) return 1;
    return base * power(base, exp - 1);
}

int main() {
    printf("fib(10)=%d\n", fib(10));
    printf("power(2,10)=%d\n", power(2, 10));
    printf("power(3,5)=%d\n", power(3, 5));
    return 0;
}
