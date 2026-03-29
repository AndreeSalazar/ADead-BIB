// Test: <stdlib.h> — Standard library utilities
// Expected: Compile + Run OK

#include <stdlib.h>
#include <stdio.h>

int main() {
    printf("=== stdlib.h test ===\n");

    // malloc/free
    int *p = (int *)malloc(sizeof(int) * 4);
    p[0] = 10; p[1] = 20; p[2] = 30; p[3] = 40;
    printf("malloc: %d %d %d %d\n", p[0], p[1], p[2], p[3]);
    free(p);

    // abs
    int a = abs(-42);
    printf("abs(-42)=%d\n", a);

    // atoi
    int n = atoi("12345");
    printf("atoi(\"12345\")=%d\n", n);

    printf("=== stdlib.h OK ===\n");
    return 0;
}
