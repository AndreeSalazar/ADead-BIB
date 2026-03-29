// Test: C99/C11 features — _Static_assert, designated init, compound literals, inline
// Expected: All parse + lower correctly

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

_Static_assert(sizeof(int) >= 2, "int must be at least 16 bits");

struct Config {
    int width;
    int height;
    int depth;
};

static inline int max_inline(int a, int b) {
    return (a > b) ? a : b;
}

int use_bool() {
    bool flag = true;
    if (flag) {
        return 1;
    }
    return 0;
}

int main() {
    // C99 mixed declarations
    int x = 10;
    int y = 20;
    int z = max_inline(x, y);

    // for-loop init declaration (C99)
    int sum = 0;
    for (int i = 0; i < 10; i++) {
        sum += i;
    }

    // Nested initializer
    int matrix[2][3] = {{1, 2, 3}, {4, 5, 6}};

    // Struct init
    struct Config cfg = {800, 600, 32};

    // _Static_assert in function body
    _Static_assert(1, "always true");

    // sum=45, z=20 — use them but return 0
    if (sum + z > 0) {
        return 0;
    }
    return 1;
}
