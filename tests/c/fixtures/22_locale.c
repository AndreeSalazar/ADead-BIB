// Test: <locale.h> — Localization
// Expected: Compile OK — setlocale + localeconv declarations

#include <locale.h>
#include <stdio.h>

int main() {
    printf("=== locale.h test ===\n");

    // setlocale
    char *loc = setlocale(0, "C");
    if (loc) {
        printf("setlocale: %s\n", loc);
    }

    // localeconv
    struct lconv *lc = localeconv();
    if (lc) {
        printf("decimal_point: %s\n", lc->decimal_point);
    }

    printf("=== locale.h OK ===\n");
    return 0;
}
