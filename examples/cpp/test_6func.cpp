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
    printf("  %d/%d passed\n", pass, total);
    return 0;
}
