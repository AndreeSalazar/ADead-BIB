// ============================================================
// Test 28: Punteros Avanzados — arrays de ptrs, ptr a arrays, const ptrs
// ============================================================
// ADead-BIB Test Canon — C99 §6.7.6
// Verifica: patrones avanzados de punteros, matrices via ptrs
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// --- Array de strings (char** pattern) ---
int find_string(const char **strings, int count, const char *target) {
    int i;
    for (i = 0; i < count; i++) {
        const char *a = strings[i];
        const char *b = target;
        while (*a && *b && *a == *b) { a++; b++; }
        if (*a == '\0' && *b == '\0') return i;
    }
    return -1;
}

// --- Pointer a array fijo ---
void process_row(int (*row)[4]) {
    int i;
    for (i = 0; i < 4; i++) {
        (*row)[i] *= 2;
    }
}

// --- Matrix via pointer to array ---
void print_matrix(int (*mat)[4], int rows) {
    int i, j;
    for (i = 0; i < rows; i++) {
        for (j = 0; j < 4; j++) {
            printf("%3d ", mat[i][j]);
        }
        printf("\n");
    }
}

// --- Pointer a const vs const pointer ---
void demonstrate_const() {
    int a = 10, b = 20;

    const int *p1 = &a;
    int *const p2 = &a;
    const int *const p3 = &a;

    p1 = &b;
    *p2 = 30;

    printf("const demo: *p1=%d *p2=%d *p3=%d\n", *p1, *p2, *p3);
}

// --- Array de function pointers ---
typedef int (*MathFunc)(int);

int double_it(int x) { return x * 2; }
int negate_it(int x) { return -x; }
int square_it(int x) { return x * x; }
int identity(int x) { return x; }

void apply_pipeline(int *val, MathFunc *funcs, int n) {
    int i;
    for (i = 0; i < n; i++) {
        *val = funcs[i](*val);
    }
}

// --- Restrict-like pattern (pointer aliasing) ---
void add_arrays(int * dst, const int * src1, const int * src2, int n) {
    int i;
    for (i = 0; i < n; i++) {
        dst[i] = src1[i] + src2[i];
    }
}

int main() {
    // --- Array de strings ---
    const char *fruits[] = {"apple", "banana", "cherry", "date", "elderberry"};
    int idx = find_string(fruits, 5, "cherry");
    printf("find 'cherry'=%d\n", idx);
    idx = find_string(fruits, 5, "grape");
    printf("find 'grape'=%d\n", idx);

    // --- Pointer to array ---
    int mat[3][4] = {
        {1, 2, 3, 4},
        {5, 6, 7, 8},
        {9, 10, 11, 12}
    };
    printf("before:\n");
    print_matrix(mat, 3);

    process_row(&mat[1]);
    printf("after process_row[1]:\n");
    print_matrix(mat, 3);

    // --- const pointers ---
    demonstrate_const();

    // --- Function pointer pipeline ---
    MathFunc pipeline[3] = {double_it, square_it, negate_it};
    int val = 3;
    apply_pipeline(&val, pipeline, 3);
    printf("pipeline(3)=%d\n", val);

    // --- Add arrays ---
    int a[4] = {1, 2, 3, 4};
    int b[4] = {10, 20, 30, 40};
    int c[4];
    add_arrays(c, a, b, 4);
    printf("add_arrays: %d %d %d %d\n", c[0], c[1], c[2], c[3]);

    // --- Pointer to pointer to array (dynamic 2D) ---
    int rows = 3, cols = 4;
    int **dyn = (int **)malloc(sizeof(int *) * rows);
    int i, j;
    for (i = 0; i < rows; i++) {
        dyn[i] = (int *)malloc(sizeof(int) * cols);
        for (j = 0; j < cols; j++) {
            dyn[i][j] = i * cols + j;
        }
    }
    printf("dyn[1][2]=%d dyn[2][3]=%d\n", dyn[1][2], dyn[2][3]);
    for (i = 0; i < rows; i++) free(dyn[i]);
    free(dyn);

    return 0;
}
// Expected:
// find 'cherry'=2
// find 'grape'=-1
// pipeline(3)=-36
// add_arrays: 11 22 33 44
// dyn[1][2]=6 dyn[2][3]=11
