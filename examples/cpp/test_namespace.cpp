#include <cstdio>

namespace math {
    int add(int a, int b) { return a + b; }
    int sub(int a, int b) { return a - b; }
    int mul(int a, int b) { return a * b; }
}

int main() {
    printf("Namespace test\n");
    int x = math::add(10, 20);
    printf("math::add(10,20) = %d\n", x);
    int y = math::sub(50, 8);
    printf("math::sub(50,8) = %d\n", y);
    int z = math::mul(6, 7);
    printf("math::mul(6,7) = %d\n", z);

    int pass = 0;
    int total = 0;
    total = total + 1;
    if (x == 30) { pass = pass + 1; } else { printf("  FAIL: add\n"); }
    total = total + 1;
    if (y == 42) { pass = pass + 1; } else { printf("  FAIL: sub\n"); }
    total = total + 1;
    if (z == 42) { pass = pass + 1; } else { printf("  FAIL: mul\n"); }
    printf("%d/%d passed\n", pass, total);
    return 0;
}
