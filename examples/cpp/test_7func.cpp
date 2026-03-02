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
}

int main() {
    int pass = 0;
    int total = 0;
    printf("[1] math::\n");
    total++; if (math::add(3, 7) == 10) { pass++; } else { printf("  FAIL: add\n"); }
    total++; if (math::factorial(7) == 5040) { pass++; } else { printf("  FAIL: fact(7)\n"); }
    total++; if (math::gcd(48, 18) == 6) { pass++; } else { printf("  FAIL: gcd\n"); }
    printf("  %d/%d passed\n", pass, total);
    return 0;
}
