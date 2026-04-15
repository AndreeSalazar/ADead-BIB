// ============================================================
// Test 06: Funciones — declaración, definición, recursión, prototipos
// ============================================================
// ADead-BIB Test Canon — C99 §6.7.6, §6.9.1
// Verifica: parámetros, retorno, recursión, forward decls
// ============================================================

#include <stdio.h>

// --- Forward declarations ---
int add(int a, int b);
int multiply(int a, int b);
void print_int(int x);

// --- Funciones simples ---
int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}

int max(int a, int b) {
    return (a > b) ? a : b;
}

int min(int a, int b) {
    return (a < b) ? a : b;
}

int clamp(int val, int lo, int hi) {
    if (val < lo) return lo;
    if (val > hi) return hi;
    return val;
}

// --- Void functions ---
void print_int(int x) {
    printf("%d\n", x);
}

// --- Recursión ---
int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

int fibonacci(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

int gcd(int a, int b) {
    if (b == 0) return a;
    return gcd(b, a % b);
}

int power(int base, int exp) {
    if (exp == 0) return 1;
    return base * power(base, exp - 1);
}

// --- Múltiples parámetros ---
int sum4(int a, int b, int c, int d) {
    return a + b + c + d;
}

int sum6(int a, int b, int c, int d, int e, int f) {
    return a + b + c + d + e + f;
}

// --- Funciones que llaman funciones ---
int compute(int x, int y) {
    int s = add(x, y);
    int m = multiply(x, y);
    return add(s, m);
}

int main() {
    printf("add(3,4)=%d\n", add(3, 4));
    printf("multiply(6,7)=%d\n", multiply(6, 7));
    printf("max(3,7)=%d min(3,7)=%d\n", max(3, 7), min(3, 7));
    printf("clamp(15,0,10)=%d\n", clamp(15, 0, 10));
    printf("factorial(10)=%d\n", factorial(10));
    printf("fibonacci(10)=%d\n", fibonacci(10));
    printf("gcd(48,18)=%d\n", gcd(48, 18));
    printf("power(2,10)=%d\n", power(2, 10));
    printf("sum4=%d sum6=%d\n", sum4(1, 2, 3, 4), sum6(1, 2, 3, 4, 5, 6));
    printf("compute(3,4)=%d\n", compute(3, 4));
    return 0;
}
// Expected:
// add(3,4)=7
// multiply(6,7)=42
// max(3,7)=7 min(3,7)=3
// clamp(15,0,10)=10
// factorial(10)=3628800
// fibonacci(10)=55
// gcd(48,18)=6
// power(2,10)=1024
// sum4=10 sum6=21
// compute(3,4)=19
