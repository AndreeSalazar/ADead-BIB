// Test: Production memory — malloc, calloc, realloc, free patterns
// Expected: Compile + Run — heap operations must work correctly
// Strict: Verifies allocation, write, read-back, realloc growth

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    printf("=== PRODUCTION: Memory ===\n");
    int pass = 0, fail = 0;

    // malloc + write + read
    int *arr = (int*)malloc(sizeof(int) * 10);
    if (arr != 0) { pass++; } else { fail++; printf("FAIL: malloc returned NULL\n"); return 1; }
    for (int i = 0; i < 10; i++) arr[i] = i * i;
    if (arr[0] == 0 && arr[3] == 9 && arr[9] == 81) { pass++; } else { fail++; printf("FAIL: arr values\n"); }
    free(arr);

    // calloc — zero initialized
    int *zarr = (int*)calloc(5, sizeof(int));
    if (zarr != 0) { pass++; } else { fail++; printf("FAIL: calloc NULL\n"); return 1; }
    int all_zero = 1;
    for (int i = 0; i < 5; i++) { if (zarr[i] != 0) all_zero = 0; }
    if (all_zero) { pass++; } else { fail++; printf("FAIL: calloc not zeroed\n"); }
    free(zarr);

    // realloc — grow
    char *buf = (char*)malloc(8);
    if (buf != 0) { pass++; } else { fail++; return 1; }
    strcpy(buf, "hello");
    buf = (char*)realloc(buf, 32);
    if (buf != 0) { pass++; } else { fail++; printf("FAIL: realloc NULL\n"); return 1; }
    if (strcmp(buf, "hello") == 0) { pass++; } else { fail++; printf("FAIL: realloc lost data\n"); }
    strcat(buf, " world of production!");
    printf("realloc result: \"%s\"\n", buf);
    free(buf);

    // Multiple alloc/free cycles
    for (int cycle = 0; cycle < 100; cycle++) {
        void *p = malloc(64);
        if (p == 0) { fail++; printf("FAIL: cycle %d\n", cycle); break; }
        memset(p, 0xAB, 64);
        free(p);
    }
    pass++;

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== PRODUCTION: Memory %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
