// ============================================================
// Test 26: C99 Features — mixed decls, _Bool, inline, designated init
// ============================================================
// ADead-BIB Test Canon — C99 §6.7, §6.7.9
// Verifica: features específicos de C99
// ============================================================

#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>

// --- inline function ---
static inline int square(int x) {
    return x * x;
}

static inline int cube(int x) {
    return x * x * x;
}

// --- Struct con designated initializers ---
struct Config {
    int width;
    int height;
    int depth;
    int fullscreen;
};

int main() {
    // --- Mixed declarations (C99) ---
    int a = 10;
    printf("a=%d\n", a);
    int b = 20;
    printf("b=%d\n", b);
    int c = a + b;
    printf("c=%d\n", c);

    // --- _Bool / bool ---
    bool t = true;
    bool f = false;
    printf("true=%d false=%d\n", t, f);
    printf("sizeof(bool)=%d\n", (int)sizeof(bool));

    // --- Bool logic ---
    bool and_r = t && f;
    bool or_r = t || f;
    bool not_r = !t;
    printf("&&=%d ||=%d !=%d\n", and_r, or_r, not_r);

    // --- For-loop variable declaration (C99) ---
    int sum = 0;
    for (int i = 0; i < 10; i++) {
        sum += i;
    }
    printf("for_decl sum=%d\n", sum);

    // --- inline ---
    printf("square(5)=%d cube(3)=%d\n", square(5), cube(3));

    // --- Designated initializers ---
    struct Config cfg = {
        .width = 1920,
        .height = 1080,
        .depth = 32,
        .fullscreen = 1
    };
    printf("config: %dx%d depth=%d fs=%d\n",
           cfg.width, cfg.height, cfg.depth, cfg.fullscreen);

    // --- Array designated init ---
    int arr[10] = {
        [0] = 100,
        [5] = 500,
        [9] = 900
    };
    printf("designated: [0]=%d [5]=%d [9]=%d [3]=%d\n",
           arr[0], arr[5], arr[9], arr[3]);

    // --- Fixed-width types (C99 stdint.h) ---
    int8_t i8 = INT8_MAX;
    int16_t i16 = INT16_MAX;
    int32_t i32 = INT32_MAX;
    int64_t i64 = INT64_MAX;
    printf("int8_max=%d int16_max=%d\n", i8, i16);
    printf("int32_max=%d\n", i32);
    printf("int64_max=%lld\n", (long long)i64);

    // --- Compound literal ---
    int *p = (int []){10, 20, 30};
    printf("compound: %d %d %d\n", p[0], p[1], p[2]);

    return 0;
}
