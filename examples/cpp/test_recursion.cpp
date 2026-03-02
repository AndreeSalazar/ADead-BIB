#include <cstdio>

int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

int main() {
    printf("fact(0) = %d\n", factorial(0));
    printf("fact(1) = %d\n", factorial(1));
    printf("fact(5) = %d\n", factorial(5));
    printf("fact(7) = %d\n", factorial(7));
    return 0;
}
