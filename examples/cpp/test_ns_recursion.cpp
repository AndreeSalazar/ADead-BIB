#include <cstdio>

namespace math {
    int factorial(int n) {
        if (n <= 1) return 1;
        return n * factorial(n - 1);
    }
    int add(int a, int b) { return a + b; }
}

int main() {
    printf("math::add(3,7) = %d\n", math::add(3, 7));
    printf("math::factorial(0) = %d\n", math::factorial(0));
    printf("math::factorial(5) = %d\n", math::factorial(5));
    printf("math::factorial(7) = %d\n", math::factorial(7));

    int pass = 0;
    int total = 0;
    total = total + 1;
    if (math::add(3, 7) == 10) { pass = pass + 1; } else { printf("FAIL: add\n"); }
    total = total + 1;
    if (math::factorial(0) == 1) { pass = pass + 1; } else { printf("FAIL: fact(0)\n"); }
    total = total + 1;
    if (math::factorial(7) == 5040) { pass = pass + 1; } else { printf("FAIL: fact(7)\n"); }
    printf("%d/%d passed\n", pass, total);
    return 0;
}
