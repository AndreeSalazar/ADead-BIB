int printf(const char *format, ...);

constexpr int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

constexpr int fib(int n) {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

int square(int x) { return x * x; }
int cube(int x) { return x * x * x; }

int main() {
    constexpr int f5 = factorial(5);
    constexpr int f10 = fib(10);
    printf("5! = %d\n", f5);
    printf("fib(10) = %d\n", f10);
    printf("square(7) = %d\n", square(7));
    printf("cube(3) = %d\n", cube(3));
    return 0;
}
