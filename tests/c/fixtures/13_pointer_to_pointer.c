// ============================================================
// Test 13: Puntero a Puntero — int**, char**, void**, niveles múltiples
// ============================================================
// ADead-BIB Test Canon — C99 §6.5.3.2
// Verifica: doble/triple indirección, modificar via **, argv pattern
// ============================================================

#include <stdio.h>
#include <stdlib.h>

void set_value(int **pp, int val) {
    **pp = val;
}

void redirect(int **pp, int *new_target) {
    *pp = new_target;
}

int **alloc_int_ptr(int value) {
    int *p = (int *)malloc(sizeof(int));
    *p = value;
    int **pp = (int **)malloc(sizeof(int *));
    *pp = p;
    return pp;
}

void free_int_ptr(int **pp) {
    free(*pp);
    free(pp);
}

int main() {
    // --- Doble puntero básico ---
    int x = 10;
    int *p = &x;
    int **pp = &p;

    printf("x=%d *p=%d **pp=%d\n", x, *p, **pp);
    printf("&x=%p p=%p *pp=%p\n", (void *)&x, (void *)p, (void *)*pp);

    // --- Modificar via ** ---
    **pp = 42;
    printf("after **pp=42: x=%d\n", x);

    // --- set_value ---
    set_value(&p, 99);
    printf("after set_value: x=%d\n", x);

    // --- Redirect ---
    int y = 200;
    redirect(&p, &y);
    printf("after redirect: *p=%d (should be y=200)\n", *p);
    printf("x still=%d\n", x);

    // --- Triple puntero ---
    int a = 777;
    int *pa = &a;
    int **ppa = &pa;
    int ***pppa = &ppa;
    printf("***pppa=%d\n", ***pppa);
    ***pppa = 888;
    printf("a=%d\n", a);

    // --- Array de punteros (simula char **argv) ---
    const char *args[4];
    args[0] = "program";
    args[1] = "--verbose";
    args[2] = "--output";
    args[3] = "file.txt";

    int argc = 4;
    int i;
    printf("args:\n");
    for (i = 0; i < argc; i++) {
        printf("  [%d]=%s\n", i, args[i]);
    }

    // --- Puntero a array de punteros ---
    const char **argv = args;
    printf("argv[0]=%s argv[1]=%s\n", argv[0], argv[1]);
    argv++;
    printf("after argv++: argv[0]=%s\n", argv[0]);

    // --- Alloc via ** ---
    int **allocated = alloc_int_ptr(12345);
    printf("allocated=%d\n", **allocated);
    free_int_ptr(allocated);

    // --- Swap de punteros ---
    int v1 = 111, v2 = 222;
    int *p1 = &v1, *p2 = &v2;
    printf("before: *p1=%d *p2=%d\n", *p1, *p2);
    int *tmp = p1;
    p1 = p2;
    p2 = tmp;
    printf("after swap ptrs: *p1=%d *p2=%d\n", *p1, *p2);

    return 0;
}
// Expected:
// x=10 *p=10 **pp=10
// after **pp=42: x=42
// after set_value: x=99
// after redirect: *p=200 (should be y=200)
// x still=99
// ***pppa=777
// a=888
// args:
//   [0]=program
//   [1]=--verbose
//   [2]=--output
//   [3]=file.txt
// argv[0]=program argv[1]=--verbose
// after argv++: argv[0]=--verbose
// allocated=12345
// before: *p1=111 *p2=222
// after swap ptrs: *p1=222 *p2=111
