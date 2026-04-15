// ============================================================
// Test 17: Gestión de Memoria — malloc, calloc, realloc, free
// ============================================================
// ADead-BIB Test Canon — C99 §7.22.3
// Verifica: alloc/dealloc, patterns comunes, dynamic arrays
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// --- Dynamic array (vector) ---
struct DynArray {
    int *data;
    int size;
    int capacity;
};

struct DynArray *dynarray_create(int initial_cap) {
    struct DynArray *da = (struct DynArray *)malloc(sizeof(struct DynArray));
    da->data = (int *)malloc(sizeof(int) * initial_cap);
    da->size = 0;
    da->capacity = initial_cap;
    return da;
}

void dynarray_push(struct DynArray *da, int value) {
    if (da->size >= da->capacity) {
        da->capacity *= 2;
        da->data = (int *)realloc(da->data, sizeof(int) * da->capacity);
    }
    da->data[da->size] = value;
    da->size++;
}

int dynarray_get(struct DynArray *da, int index) {
    return da->data[index];
}

void dynarray_free(struct DynArray *da) {
    free(da->data);
    free(da);
}

// --- 2D array dinámico ---
int **alloc_matrix(int rows, int cols) {
    int **mat = (int **)malloc(sizeof(int *) * rows);
    int i;
    for (i = 0; i < rows; i++) {
        mat[i] = (int *)calloc(cols, sizeof(int));
    }
    return mat;
}

void free_matrix(int **mat, int rows) {
    int i;
    for (i = 0; i < rows; i++) {
        free(mat[i]);
    }
    free(mat);
}

// --- String duplicado ---
char *my_strdup(const char *s) {
    int len = (int)strlen(s);
    char *dup = (char *)malloc(len + 1);
    strcpy(dup, s);
    return dup;
}

int main() {
    // --- malloc básico ---
    int *p = (int *)malloc(sizeof(int));
    *p = 42;
    printf("malloc: %d\n", *p);
    free(p);

    // --- calloc (zero-initialized) ---
    int *zeros = (int *)calloc(5, sizeof(int));
    printf("calloc: %d %d %d %d %d\n",
           zeros[0], zeros[1], zeros[2], zeros[3], zeros[4]);

    // --- Llenar y verificar ---
    int i;
    for (i = 0; i < 5; i++) {
        zeros[i] = (i + 1) * 10;
    }
    printf("filled: %d %d %d %d %d\n",
           zeros[0], zeros[1], zeros[2], zeros[3], zeros[4]);

    // --- realloc ---
    int *grown = (int *)realloc(zeros, sizeof(int) * 10);
    for (i = 5; i < 10; i++) {
        grown[i] = (i + 1) * 10;
    }
    printf("realloc[9]=%d\n", grown[9]);
    free(grown);

    // --- malloc array de structs ---
    struct {
        int x;
        int y;
    } *points;
    points = malloc(sizeof(*points) * 3);
    points[0].x = 1; points[0].y = 2;
    points[1].x = 3; points[1].y = 4;
    points[2].x = 5; points[2].y = 6;
    printf("points: (%d,%d) (%d,%d) (%d,%d)\n",
           points[0].x, points[0].y,
           points[1].x, points[1].y,
           points[2].x, points[2].y);
    free(points);

    // --- DynArray ---
    struct DynArray *da = dynarray_create(4);
    for (i = 0; i < 20; i++) {
        dynarray_push(da, i * i);
    }
    printf("dynarray size=%d cap=%d\n", da->size, da->capacity);
    printf("da[0]=%d da[5]=%d da[19]=%d\n",
           dynarray_get(da, 0), dynarray_get(da, 5), dynarray_get(da, 19));
    dynarray_free(da);

    // --- 2D matrix ---
    int **mat = alloc_matrix(3, 4);
    int r, c;
    for (r = 0; r < 3; r++) {
        for (c = 0; c < 4; c++) {
            mat[r][c] = r * 4 + c;
        }
    }
    printf("mat[1][2]=%d mat[2][3]=%d\n", mat[1][2], mat[2][3]);
    free_matrix(mat, 3);

    // --- strdup ---
    char *s = my_strdup("ADead-BIB");
    printf("strdup=%s\n", s);
    free(s);

    // --- Large allocation ---
    int *big = (int *)malloc(sizeof(int) * 100000);
    big[0] = 1;
    big[99999] = 2;
    printf("big[0]=%d big[99999]=%d\n", big[0], big[99999]);
    free(big);

    return 0;
}
