// Test: <setjmp.h> — Non-local jumps
// Expected: Compile OK — setjmp/longjmp declarations

#include <setjmp.h>
#include <stdio.h>

jmp_buf env;

int main() {
    printf("=== setjmp.h test ===\n");

    int val = setjmp(env);
    if (val == 0) {
        printf("setjmp returned 0 (initial)\n");
        longjmp(env, 42);
    } else {
        printf("longjmp returned %d (expected 42)\n", val);
    }

    printf("=== setjmp.h OK ===\n");
    return 0;
}
