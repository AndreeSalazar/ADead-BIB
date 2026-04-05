// ADead-BIB Bridge Test 07 — Pointers & Arrays
// Level: INTERMEDIATE
// Tests: pointer arithmetic, multi-dim arrays, function pointers, void*

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void fill_array(int *arr, int n, int val) {
    for (int i = 0; i < n; i++) arr[i] = val + i;
}

int sum_array(const int *arr, int n) {
    int s = 0;
    for (int i = 0; i < n; i++) s += arr[i];
    return s;
}

typedef int (*BinaryOp)(int, int);
int add(int a, int b) { return a + b; }
int mul(int a, int b) { return a * b; }

int apply(BinaryOp op, int a, int b) {
    return op(a, b);
}

int main() {
    printf("=== ADead-BIB Bridge Test 07: Pointers ===\n");
    int pass = 0, fail = 0;

    // Pointer arithmetic
    int arr[10];
    fill_array(arr, 10, 0);
    if (arr[0] == 0 && arr[9] == 9) { pass++; } else { fail++; printf("FAIL: fill_array\n"); }

    int *p = arr + 5;
    if (*p == 5) { pass++; } else { fail++; printf("FAIL: ptr+5\n"); }
    if (p - arr == 5) { pass++; } else { fail++; printf("FAIL: ptr diff\n"); }

    // Array sum
    if (sum_array(arr, 10) == 45) { pass++; } else { fail++; printf("FAIL: sum_array\n"); }

    // 2D array
    int mat[3][3] = {{1,2,3},{4,5,6},{7,8,9}};
    int diag = mat[0][0] + mat[1][1] + mat[2][2];
    if (diag == 15) { pass++; } else { fail++; printf("FAIL: 2d array diag=%d\n", diag); }

    // Function pointers
    if (apply(add, 10, 20) == 30) { pass++; } else { fail++; printf("FAIL: fn ptr add\n"); }
    if (apply(mul, 6, 7) == 42) { pass++; } else { fail++; printf("FAIL: fn ptr mul\n"); }

    // void* generic swap
    int a = 111, b = 222;
    void *va = &a, *vb = &b;
    int tmp;
    memcpy(&tmp, va, sizeof(int));
    memcpy(va, vb, sizeof(int));
    memcpy(vb, &tmp, sizeof(int));
    if (a == 222 && b == 111) { pass++; } else { fail++; printf("FAIL: void* swap\n"); }

    // Dynamic array
    int n = 100;
    int *dyn = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) dyn[i] = i;
    if (sum_array(dyn, n) == 4950) { pass++; } else { fail++; printf("FAIL: dynamic array\n"); }
    free(dyn);

    // Pointer to pointer
    int val = 42;
    int *pp = &val;
    int **ppp = &pp;
    if (**ppp == 42) { pass++; } else { fail++; printf("FAIL: ptr to ptr\n"); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 07: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
