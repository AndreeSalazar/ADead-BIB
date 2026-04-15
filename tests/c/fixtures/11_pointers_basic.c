// ============================================================
// Test 11: Punteros Básicos — declaración, deref, address-of
// ============================================================
// ADead-BIB Test Canon — C99 §6.5.3.2
// Verifica: *, &, NULL, asignación, swap
// ============================================================

#include <stdio.h>

void swap(int *a, int *b) {
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

void triple(int *p) {
    *p = *p * 3;
}

void set_to_zero(int *p) {
    *p = 0;
}

int add_via_ptr(const int *a, const int *b) {
    return *a + *b;
}

int main() {
    // --- Declaración y deref ---
    int x = 42;
    int *p = &x;
    printf("x=%d *p=%d\n", x, *p);
    printf("&x=%p p=%p\n", (void *)&x, (void *)p);

    // --- Modificar via puntero ---
    *p = 100;
    printf("after *p=100: x=%d\n", x);

    // --- Swap ---
    int a = 10, b = 20;
    printf("before swap: a=%d b=%d\n", a, b);
    swap(&a, &b);
    printf("after swap:  a=%d b=%d\n", a, b);

    // --- Triple ---
    int val = 5;
    triple(&val);
    printf("triple(5)=%d\n", val);

    // --- Set to zero ---
    int z = 999;
    set_to_zero(&z);
    printf("set_to_zero: z=%d\n", z);

    // --- Puntero NULL ---
    int *null_ptr = (int *)0;
    printf("null_ptr=%p\n", (void *)null_ptr);

    // --- const pointer vs pointer to const ---
    int c = 30;
    const int *ptr_to_const = &c;
    int d = 40;
    int *const const_ptr = &d;
    *const_ptr = 50;
    printf("ptr_to_const=%d const_ptr=%d\n", *ptr_to_const, *const_ptr);

    // --- add_via_ptr ---
    int v1 = 100, v2 = 200;
    int sum = add_via_ptr(&v1, &v2);
    printf("add_via_ptr=%d\n", sum);

    // --- Puntero a puntero básico ---
    int **pp = &p;
    printf("**pp=%d\n", **pp);

    // --- Re-apuntar ---
    int y = 77;
    p = &y;
    printf("re-pointed: *p=%d\n", *p);

    return 0;
}
// Expected:
// x=42 *p=42
// after *p=100: x=100
// before swap: a=10 b=20
// after swap:  a=20 b=10
// triple(5)=15
// set_to_zero: z=0
// ptr_to_const=30 const_ptr=50
// add_via_ptr=300
// **pp=100
// re-pointed: *p=77
