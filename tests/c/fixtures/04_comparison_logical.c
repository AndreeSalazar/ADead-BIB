// ============================================================
// Test 04: Comparación y Lógica — ==, !=, <, >, <=, >=, &&, ||, !
// ============================================================
// ADead-BIB Test Canon — C99 §6.5.8-§6.5.9, §6.5.13-§6.5.14
// Verifica: operadores relacionales, lógicos, ternario
// ============================================================

#include <stdio.h>

int main() {
    int a = 5, b = 10, c = 5;

    // --- Comparaciones ---
    int eq = (a == c);
    int ne = (a != b);
    int lt = (a < b);
    int gt = (b > a);
    int le = (a <= c);
    int ge = (b >= a);

    printf("eq=%d ne=%d lt=%d gt=%d le=%d ge=%d\n", eq, ne, lt, gt, le, ge);

    // --- Lógicos ---
    int and1 = (1 && 1);
    int and2 = (1 && 0);
    int and3 = (0 && 1);
    int or1 = (1 || 0);
    int or2 = (0 || 0);
    int not1 = !0;
    int not2 = !1;
    int not3 = !42;

    printf("&&: %d %d %d  ||: %d %d  !: %d %d %d\n",
           and1, and2, and3, or1, or2, not1, not2, not3);

    // --- Short-circuit evaluation ---
    int x = 0;
    int sc1 = (0 && (x = 1));
    printf("short-circuit &&: x=%d (should be 0)\n", x);
    int sc2 = (1 || (x = 1));
    printf("short-circuit ||: x=%d (should be 0)\n", x);

    // --- Ternario ---
    int t1 = (a > b) ? 100 : 200;
    int t2 = (a == c) ? 100 : 200;
    int t3 = (a < b) ? (b < 20 ? 1 : 2) : 3;

    printf("ternary: %d %d %d\n", t1, t2, t3);

    // --- Cadenas de comparación ---
    int in_range = (a >= 0 && a <= 10);
    int out_range = (a < 0 || a > 10);
    printf("range: in=%d out=%d\n", in_range, out_range);

    return 0;
}
// Expected:
// eq=1 ne=1 lt=1 gt=1 le=1 ge=1
// &&: 1 0 0  ||: 1 0  !: 1 0 0
// short-circuit &&: x=0 (should be 0)
// short-circuit ||: x=0 (should be 0)
// ternary: 200 100 1
// range: in=1 out=0
