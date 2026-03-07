// Canon C++14 -- Return type deduction, relaxed constexpr
int printf(const char *format, ...);

// C++14: auto return type deduction
template<typename T>
auto double_val(T x) -> T {
    return x + x;
}

// constexpr function (relaxed in C++14 -- multiple statements)
constexpr int factorial(int n) {
    int result = 1;
    for (int i = 2; i <= n; i = i + 1) {
        result = result * i;
    }
    return result;
}

constexpr int fib(int n) {
    if (n <= 1) return n;
    int a = 0;
    int b = 1;
    for (int i = 2; i <= n; i = i + 1) {
        int tmp = a + b;
        a = b;
        b = tmp;
    }
    return b;
}

int main() {
    printf("=== Canon C++14: Return Deduction + constexpr ===\n\n");
    int pass = 0;
    int total = 0;

    // auto return type
    int d1 = double_val(21);
    printf("double_val(21) = %d\n", d1);
    total++; if (d1 == 42) { pass++; } else { printf("FAIL: double_val\n"); }

    // constexpr factorial
    int f5 = factorial(5);
    int f7 = factorial(7);
    printf("factorial(5) = %d\n", f5);
    printf("factorial(7) = %d\n", f7);
    total++; if (f5 == 120) { pass++; } else { printf("FAIL: fact(5)\n"); }
    total++; if (f7 == 5040) { pass++; } else { printf("FAIL: fact(7)\n"); }

    // constexpr fibonacci
    int fib10 = fib(10);
    printf("fib(10) = %d\n", fib10);
    total++; if (fib10 == 55) { pass++; } else { printf("FAIL: fib(10)\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}