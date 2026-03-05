// ============================================================
// Canon C99 — §6.8 Control de Flujo
// ============================================================
// Intención: Cada estructura de control tiene una traducción
// directa a jumps condicionales en x86-64.
//
// if/else     → cmp + jcc
// for         → cmp + jcc + jmp (loop)
// while       → cmp + jcc + jmp (loop)
// do-while    → cmp + jcc (loop back)
// switch/case → cmp + je (cadena de comparaciones)
// break       → jmp (salir del loop/switch)
// continue    → jmp (ir al inicio del loop)
// ============================================================

#include <stdio.h>

// --- Fibonacci iterativo (for loop canónico) ---
int fibonacci(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;

    int a = 0;
    int b = 1;
    int i;
    for (i = 2; i <= n; i++) {
        int temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

// --- Factorial iterativo (while) ---
int factorial(int n) {
    int result = 1;
    while (n > 1) {
        result = result * n;
        n = n - 1;
    }
    return result;
}

// --- Contar dígitos (do-while) ---
int count_digits(int n) {
    if (n < 0) n = -n;
    if (n == 0) return 1;
    int count = 0;
    do {
        count++;
        n = n / 10;
    } while (n > 0);
    return count;
}

// --- Grade switch ---
char grade(int score) {
    switch (score / 10) {
        case 10:
        case 9:
            return 'A';
        case 8:
            return 'B';
        case 7:
            return 'C';
        case 6:
            return 'D';
        default:
            return 'F';
    }
}

// --- Primos con break/continue ---
int is_prime(int n) {
    if (n < 2) return 0;
    int i;
    for (i = 2; i * i <= n; i++) {
        if (n % i == 0) return 0;
    }
    return 1;
}

int count_primes(int limit) {
    int count = 0;
    int i;
    for (i = 2; i <= limit; i++) {
        if (!is_prime(i)) continue;
        count++;
    }
    return count;
}

// --- Nested if/else ---
int classify(int x) {
    if (x > 0) {
        if (x > 100) {
            return 3;
        } else if (x > 10) {
            return 2;
        } else {
            return 1;
        }
    } else if (x == 0) {
        return 0;
    } else {
        return -1;
    }
}

int main() {
    printf("=== Canon C99: Control de Flujo ===\n\n");

    // --- for (fibonacci) ---
    printf("Fibonacci:\n");
    int i;
    for (i = 0; i <= 10; i++) {
        printf("  fib(%d) = %d\n", i, fibonacci(i));
    }

    // --- while (factorial) ---
    printf("\nFactorial:\n");
    printf("  5! = %d\n", factorial(5));
    printf("  7! = %d\n", factorial(7));
    printf("  10! = %d\n", factorial(10));

    // --- do-while (count digits) ---
    printf("\nCount digits:\n");
    printf("  digits(0) = %d\n", count_digits(0));
    printf("  digits(42) = %d\n", count_digits(42));
    printf("  digits(12345) = %d\n", count_digits(12345));

    // --- switch ---
    printf("\nGrades:\n");
    printf("  95 → %c\n", grade(95));
    printf("  85 → %c\n", grade(85));
    printf("  75 → %c\n", grade(75));
    printf("  65 → %c\n", grade(65));
    printf("  55 → %c\n", grade(55));

    // --- Primes (continue) ---
    printf("\nPrimes <= 20: ");
    for (i = 2; i <= 20; i++) {
        if (is_prime(i)) printf("%d ", i);
    }
    printf("\nCount primes <= 100: %d\n", count_primes(100));

    // --- Nested if ---
    printf("\nClassify:\n");
    printf("  classify(500) = %d\n", classify(500));
    printf("  classify(50) = %d\n", classify(50));
    printf("  classify(5) = %d\n", classify(5));
    printf("  classify(0) = %d\n", classify(0));
    printf("  classify(-5) = %d\n", classify(-5));

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (fibonacci(10) == 55)      { pass++; } else { printf("FAIL: fib(10)\n"); }
    total++; if (fibonacci(0) == 0)        { pass++; } else { printf("FAIL: fib(0)\n"); }
    total++; if (factorial(5) == 120)      { pass++; } else { printf("FAIL: 5!\n"); }
    total++; if (factorial(7) == 5040)     { pass++; } else { printf("FAIL: 7!\n"); }
    total++; if (count_digits(12345) == 5) { pass++; } else { printf("FAIL: digits\n"); }
    total++; if (grade(95) == 'A')         { pass++; } else { printf("FAIL: grade A\n"); }
    total++; if (grade(55) == 'F')         { pass++; } else { printf("FAIL: grade F\n"); }
    total++; if (count_primes(100) == 25)  { pass++; } else { printf("FAIL: primes\n"); }
    total++; if (classify(500) == 3)       { pass++; } else { printf("FAIL: classify\n"); }
    total++; if (classify(-5) == -1)       { pass++; } else { printf("FAIL: negative\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
