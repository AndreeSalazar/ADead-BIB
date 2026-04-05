// ADead-BIB Bridge Test 04 — Memory Allocation
// Level: INTERMEDIATE
// Tests: malloc, free, calloc, realloc, NULL checks

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    printf("=== ADead-BIB Bridge Test 04: Memory ===\n");
    int pass = 0, fail = 0;

    // malloc + free
    int *p = (int*)malloc(sizeof(int) * 10);
    if (p != NULL) { pass++; } else { fail++; printf("FAIL: malloc\n"); return 1; }

    for (int i = 0; i < 10; i++) p[i] = i * i;
    if (p[5] == 25 && p[9] == 81) { pass++; } else { fail++; printf("FAIL: malloc write\n"); }
    free(p);

    // calloc (zero-initialized)
    int *z = (int*)calloc(10, sizeof(int));
    if (z != NULL) { pass++; } else { fail++; printf("FAIL: calloc\n"); return 1; }
    int all_zero = 1;
    for (int i = 0; i < 10; i++) { if (z[i] != 0) all_zero = 0; }
    if (all_zero) { pass++; } else { fail++; printf("FAIL: calloc zero\n"); }
    free(z);

    // realloc grow
    char *buf = (char*)malloc(16);
    if (buf == NULL) { fail++; printf("FAIL: malloc for realloc\n"); return 1; }
    strcpy(buf, "Hello");
    buf = (char*)realloc(buf, 64);
    if (buf != NULL && strcmp(buf, "Hello") == 0) { pass++; } else { fail++; printf("FAIL: realloc\n"); }
    free(buf);

    // malloc(0) — implementation defined, but should not crash
    void *z2 = malloc(0);
    free(z2);
    pass++;

    // large allocation
    char *big = (char*)malloc(1024 * 1024); // 1MB
    if (big != NULL) {
        memset(big, 0xAA, 1024 * 1024);
        if ((unsigned char)big[0] == 0xAA && (unsigned char)big[1024*1024-1] == 0xAA) {
            pass++;
        } else { fail++; printf("FAIL: large alloc content\n"); }
        free(big);
    } else { fail++; printf("FAIL: large alloc\n"); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 04: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
