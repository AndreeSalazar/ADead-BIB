#include <cstdio>

namespace math {
    int add(int a, int b) { return a + b; }
    int sub(int a, int b) { return a - b; }
    int mul(int a, int b) { return a * b; }
    int divide(int a, int b) { return (b != 0) ? a / b : 0; }
    int factorial(int n) {
        if (n <= 1) return 1;
        return n * factorial(n - 1);
    }
    int gcd(int a, int b) {
        while (b != 0) { int t = b; b = a % b; a = t; }
        return a;
    }
    int is_prime(int n) {
        if (n < 2) return 0;
        for (int i = 2; i * i <= n; i++) { if (n % i == 0) return 0; }
        return 1;
    }
    int power(int base, int exp) {
        int result = 1;
        for (int i = 0; i < exp; i++) { result = result * base; }
        return result;
    }
}

int main() {
    int pass = 0;
    int total = 0;

    printf("[1] math::\n");
    total++; if (math::add(3, 7) == 10) { pass++; } else { printf("  FAIL: add\n"); }
    total++; if (math::sub(20, 8) == 12) { pass++; } else { printf("  FAIL: sub\n"); }
    total++; if (math::mul(6, 7) == 42) { pass++; } else { printf("  FAIL: mul\n"); }
    total++; if (math::divide(100, 4) == 25) { pass++; } else { printf("  FAIL: div\n"); }
    total++; if (math::factorial(7) == 5040) { pass++; } else { printf("  FAIL: fact(7)\n"); }
    total++; if (math::gcd(48, 18) == 6) { pass++; } else { printf("  FAIL: gcd\n"); }
    total++; if (math::is_prime(17) == 1) { pass++; } else { printf("  FAIL: prime(17)\n"); }
    total++; if (math::power(2, 10) == 1024) { pass++; } else { printf("  FAIL: pow(2,10)\n"); }

    printf("  %d/%d passed\n", pass, total);
    return 0;
}
