// Test: <stdio.h> — Standard I/O
// Expected: Compile + Run + Print output

#include <stdio.h>

int main() {
    printf("=== stdio.h test ===\n");
    printf("printf: int=%d float=%.2f char=%c str=%s\n", 42, 3.14, 'A', "hello");
    printf("hex=%x oct=%o unsigned=%u\n", 255, 255, 4000000000u);

    // sprintf
    char buf[64];
    sprintf(buf, "formatted: %d", 123);
    printf("sprintf: %s\n", buf);

    // puts
    puts("puts: simple line");

    // putchar
    putchar('Z');
    putchar('\n');

    printf("=== stdio.h OK ===\n");
    return 0;
}
