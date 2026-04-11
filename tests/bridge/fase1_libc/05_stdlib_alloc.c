#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    int pass = 0, fail = 0;

    // malloc + free
    int *p = (int*)malloc(10 * sizeof(int));
    if (p) {
        p[0] = 42;
        p[9] = 99;
        if (p[0] == 42 && p[9] == 99) { pass++; printf("PASS: malloc read/write\n"); }
        else { fail++; printf("FAIL: malloc read/write\n"); }
        free(p);
        pass++;
        printf("PASS: free\n");
    } else { fail += 2; printf("FAIL: malloc\n"); }

    // calloc
    int *c = (int*)calloc(10, sizeof(int));
    if (c) {
        int all_zero = 1;
        int i;
        for (i = 0; i < 10; i++) {
            if (c[i] != 0) all_zero = 0;
        }
        if (all_zero) { pass++; printf("PASS: calloc zeroed\n"); }
        else { fail++; printf("FAIL: calloc not zeroed\n"); }
        free(c);
    } else { fail++; printf("FAIL: calloc\n"); }

    // realloc
    char *s = (char*)malloc(16);
    if (s) {
        strcpy(s, "hello");
        s = (char*)realloc(s, 64);
        if (s && strcmp(s, "hello") == 0) { pass++; printf("PASS: realloc preserves data\n"); }
        else { fail++; printf("FAIL: realloc\n"); }
        if (s) free(s);
    } else { fail++; printf("FAIL: malloc for realloc\n"); }

    printf("\n=== stdlib_alloc: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
