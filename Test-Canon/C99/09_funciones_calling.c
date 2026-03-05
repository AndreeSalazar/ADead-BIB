// ============================================================
// Canon C99 — §6.9.1 Funciones y Calling Convention
// ============================================================
// Intención: Una función es una secuencia de instrucciones
// con su propio stack frame. Los argumentos se pasan por
// valor (copia). Retorno en RAX (int) o XMM0 (float).
//
// C99 §6.5.2.2: "Each argument shall have a type such that
// its value may be assigned to an object with the
// unqualified version of the type of its corresponding
// parameter."
//
// Calling convention (Windows x64):
//   args: RCX, RDX, R8, R9, stack
//   return: RAX
//   shadow space: 32 bytes
// ============================================================

#include <stdio.h>

// --- Prototipos (declaración forward) ---
int add(int a, int b);
int multiply(int a, int b);
int max(int a, int b);
int min(int a, int b);
int clamp(int value, int lo, int hi);
int gcd(int a, int b);
int power(int base, int exp);
int fibonacci_rec(int n);
int ackermann(int m, int n);

// --- Definiciones ---
int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}

int max(int a, int b) {
    if (a > b) return a;
    return b;
}

int min(int a, int b) {
    if (a < b) return a;
    return b;
}

int clamp(int value, int lo, int hi) {
    if (value < lo) return lo;
    if (value > hi) return hi;
    return value;
}

// --- Recursión directa ---
int gcd(int a, int b) {
    if (b == 0) return a;
    return gcd(b, a % b);
}

int power(int base, int exp) {
    if (exp == 0) return 1;
    return base * power(base, exp - 1);
}

int fibonacci_rec(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    return fibonacci_rec(n - 1) + fibonacci_rec(n - 2);
}

// --- Recursión mutua ---
int is_even(int n);
int is_odd(int n);

int is_even(int n) {
    if (n == 0) return 1;
    return is_odd(n - 1);
}

int is_odd(int n) {
    if (n == 0) return 0;
    return is_even(n - 1);
}

// --- Void function ---
void print_separator() {
    printf("  --------\n");
}

// --- Funciones con muchos parámetros ---
int sum4(int a, int b, int c, int d) {
    return a + b + c + d;
}

int main() {
    printf("=== Canon C99: Funciones y Calling ===\n\n");

    // --- Funciones básicas ---
    printf("add(3, 4) = %d\n", add(3, 4));
    printf("multiply(6, 7) = %d\n", multiply(6, 7));
    printf("max(10, 20) = %d\n", max(10, 20));
    printf("min(10, 20) = %d\n", min(10, 20));
    printf("clamp(150, 0, 100) = %d\n", clamp(150, 0, 100));
    printf("clamp(-5, 0, 100) = %d\n", clamp(-5, 0, 100));
    printf("clamp(50, 0, 100) = %d\n", clamp(50, 0, 100));

    // --- Recursión ---
    print_separator();
    printf("gcd(48, 18) = %d\n", gcd(48, 18));
    printf("gcd(100, 75) = %d\n", gcd(100, 75));
    printf("power(2, 10) = %d\n", power(2, 10));
    printf("power(3, 5) = %d\n", power(3, 5));
    printf("fib(10) = %d\n", fibonacci_rec(10));

    // --- Recursión mutua ---
    print_separator();
    printf("is_even(4) = %d\n", is_even(4));
    printf("is_even(7) = %d\n", is_even(7));
    printf("is_odd(3) = %d\n", is_odd(3));
    printf("is_odd(8) = %d\n", is_odd(8));

    // --- 4+ parámetros (usa stack en Windows x64) ---
    print_separator();
    printf("sum4(10, 20, 30, 40) = %d\n", sum4(10, 20, 30, 40));

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (add(3, 4) == 7)         { pass++; } else { printf("FAIL: add\n"); }
    total++; if (multiply(6, 7) == 42)   { pass++; } else { printf("FAIL: mul\n"); }
    total++; if (gcd(48, 18) == 6)       { pass++; } else { printf("FAIL: gcd\n"); }
    total++; if (power(2, 10) == 1024)   { pass++; } else { printf("FAIL: power\n"); }
    total++; if (fibonacci_rec(10) == 55){ pass++; } else { printf("FAIL: fib\n"); }
    total++; if (is_even(4) == 1)        { pass++; } else { printf("FAIL: even\n"); }
    total++; if (is_odd(3) == 1)         { pass++; } else { printf("FAIL: odd\n"); }
    total++; if (clamp(150, 0, 100)==100){ pass++; } else { printf("FAIL: clamp\n"); }
    total++; if (sum4(10,20,30,40)==100) { pass++; } else { printf("FAIL: sum4\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
