#include <cstdio>

namespace math {
    int add(int a, int b) { return a + b; }
    int sub(int a, int b) { return a - b; }
    int mul(int a, int b) { return a * b; }
    int divide(int a, int b) { return (b != 0) ? a / b : 0; }
}

int main() {
    int pass = 0;
    int fail = 0;
    int total = 0;
    int sp = pass, st = total;

    printf("[1] math::\n");
    total++; if (math::add(3, 7) == 10) { pass++; } else { fail++; printf("  FAIL: add\n"); }
    total++; if (math::sub(20, 8) == 12) { pass++; } else { fail++; printf("  FAIL: sub\n"); }
    total++; if (math::mul(6, 7) == 42) { pass++; } else { fail++; printf("  FAIL: mul\n"); }
    total++; if (math::divide(100, 4) == 25) { pass++; } else { fail++; printf("  FAIL: div\n"); }
    total++; if (math::divide(10, 0) == 0) { pass++; } else { fail++; printf("  FAIL: div0\n"); }

    printf("  %d/%d passed (section: %d/%d)\n", pass, total, pass - sp, total - st);
    return 0;
}
