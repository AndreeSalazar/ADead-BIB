// ============================================================
// Test 25: Expresiones Completas — todos los operadores, precedencia, comma
// ============================================================
// ADead-BIB Test Canon — C99 §6.5
// Verifica: precedencia, asociatividad, comma operator, compound literals
// ============================================================

#include <stdio.h>

int main() {
    // --- Precedencia: * antes que + ---
    int p1 = 2 + 3 * 4;
    printf("2+3*4=%d\n", p1);

    int p2 = (2 + 3) * 4;
    printf("(2+3)*4=%d\n", p2);

    // --- Precedencia: shift vs comparison ---
    int p3 = 1 << 4 | 1 << 2;
    printf("1<<4|1<<2=%d\n", p3);

    // --- Asociatividad izquierda ---
    int p4 = 100 - 50 - 25;
    printf("100-50-25=%d\n", p4);

    int p5 = 100 / 10 / 5;
    printf("100/10/5=%d\n", p5);

    // --- Asociatividad derecha (asignación) ---
    int a, b, c;
    a = b = c = 42;
    printf("chain assign: a=%d b=%d c=%d\n", a, b, c);

    // --- Comma operator ---
    int x = (1, 2, 3, 4, 5);
    printf("comma: x=%d\n", x);

    int y;
    int i;
    for (i = 0, y = 10; i < 5; i++, y += 10) {
    }
    printf("for comma: i=%d y=%d\n", i, y);

    // --- Conditional chains ---
    int val = 15;
    const char *label =
        (val > 20) ? "high" :
        (val > 10) ? "medium" :
        (val > 0) ? "low" : "zero";
    printf("label=%s\n", label);

    // --- Complex expressions ---
    int r1 = (3 + 4) * (5 - 2) / (1 + 1);
    printf("complex1=%d\n", r1);

    int r2 = ((1 << 3) | (1 << 1)) & 0xFF;
    printf("complex2=%d\n", r2);

    int r3 = (5 > 3) && (10 < 20) || (0 == 1);
    printf("complex3=%d\n", r3);

    // --- Operadores en punteros ---
    int arr[5] = {10, 20, 30, 40, 50};
    int *p = arr;
    int deref_add = *p + *(p + 2);
    printf("*p + *(p+2)=%d\n", deref_add);

    // --- sizeof en expresión ---
    int sz = sizeof(int) + sizeof(char) + sizeof(void *);
    printf("sizeof sum=%d\n", sz);

    // --- Nested ternary ---
    int n = 42;
    int class = (n % 2 == 0) ? ((n > 100) ? 3 : 2) : 1;
    printf("nested_ternary=%d\n", class);

    return 0;
}
// Expected:
// 2+3*4=14
// (2+3)*4=20
// 100-50-25=25
// 100/10/5=2
// chain assign: a=42 b=42 c=42
// comma: x=5
// label=medium
// complex3=1
