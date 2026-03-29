// Test: <errno.h> + <assert.h> — Error handling and assertions
// Expected: Compile + Run OK

#include <errno.h>
#include <assert.h>
#include <stdio.h>

int safe_divide(int a, int b, int *result) {
    if (b == 0) {
        errno = 1; // EDOM-like
        return -1;
    }
    *result = a / b;
    return 0;
}

int main() {
    printf("=== errno/assert test ===\n");

    int r;
    int ok = safe_divide(10, 2, &r);
    printf("10/2=%d ok=%d\n", r, ok);

    int fail = safe_divide(10, 0, &r);
    printf("10/0 fail=%d errno=%d\n", fail, errno);

    // assert that should pass
    assert(1 == 1);
    assert(42 > 0);

    printf("=== errno/assert OK ===\n");
    return 0;
}
