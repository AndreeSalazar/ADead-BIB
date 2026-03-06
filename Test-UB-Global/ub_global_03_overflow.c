// ============================================================
// ADead-BIB — Ejemplos: UB vs LIMPIO
// 03 - Integer Overflow, Division By Zero y Bitwise Shifts
// ============================================================
// Compilar con UB:    adeadc cc ub_global_03_overflow.c --warn-ub
// ============================================================

#include <stdio.h>
#include <limits.h>

void signed_overflow() {
    int max = 2147483647;
    int res = max + 1; // UB: Signed integer overflow [C99 6.5.5]
    printf("%d\n", res);
}

void signed_underflow() {
    int min = -2147483648;
    int res = min - 1; // UB: Signed integer underflow 
    printf("%d\n", res);
}

void division_by_zero() {
    int x = 10;
    int res = x / 0; // UB: Division by zero
    printf("%d\n", res);
}

void shift_negative_op() {
    int x = 1;
    int res = x << -1; // UB: Shift by negative
    printf("%d\n", res);
}

void shift_too_large() {
    int x = 1;
    int res = x << 32; // UB: Shift >= width of type
    printf("%d\n", res);
}

void shift_negative_val() {
    int x = -1;
    int res = x << 1; // UB in C99/C++98: Left shift of negative value
    printf("%d\n", res);
}

int main() {
    // signed_overflow();
    // signed_underflow();
    division_by_zero();
    shift_negative_op();
    shift_too_large();
    shift_negative_val();
    return 0;
}
