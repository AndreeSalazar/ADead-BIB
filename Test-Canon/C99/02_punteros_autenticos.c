// ============================================================
// Canon C99 — §6.5.3.2 Punteros Auténticos
// ============================================================
// Intención: Un puntero es una dirección de memoria real.
// No es una referencia abstracta — es un número que apunta
// a una ubicación concreta en la memoria del proceso.
//
// C99 §6.5.3.2: "The unary & operator yields the address
// of its operand."
// C99 §6.5.3.2: "The unary * operator denotes indirection."
//
// Aritmética de punteros:
//   p + n  avanza  n * sizeof(*p)  bytes
//   p - q  da la distancia en elementos, no bytes
// ============================================================

#include <stdio.h>

// --- Modificación via puntero ---
void increment(int *p) {
    *p = *p + 1;
}

void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

void zero_array(int *arr, int len) {
    int i;
    for (i = 0; i < len; i++) {
        arr[i] = 0;
    }
}

void fill_sequence(int *arr, int len) {
    int i;
    for (i = 0; i < len; i++) {
        *(arr + i) = i * 10;
    }
}

int sum_array(int *arr, int len) {
    int total = 0;
    int i;
    for (i = 0; i < len; i++) {
        total = total + arr[i];
    }
    return total;
}

int main() {
    printf("=== Canon C99: Punteros Auténticos ===\n\n");

    // --- Puntero básico: & y * ---
    int x = 42;
    int *p = &x;
    printf("x = %d\n", x);
    printf("*p = %d (mismo valor, puntero a x)\n", *p);

    // --- Modificación a través de puntero ---
    *p = 100;
    printf("Después de *p = 100: x = %d\n", x);

    increment(&x);
    printf("Después de increment(&x): x = %d\n", x);

    // --- Swap via punteros ---
    int a = 10;
    int b = 20;
    printf("\nAntes swap: a=%d b=%d\n", a, b);
    swap(&a, &b);
    printf("Después swap: a=%d b=%d\n", a, b);

    // --- Puntero a array ---
    int arr[5];
    arr[0] = 100;
    arr[1] = 200;
    arr[2] = 300;
    arr[3] = 400;
    arr[4] = 500;

    int *q = arr;
    printf("\nArray via puntero:\n");
    printf("  arr[0] = %d, *q = %d\n", arr[0], *q);
    printf("  arr[1] = %d, *(q+1) = %d\n", arr[1], *(q + 1));
    printf("  arr[2] = %d, *(q+2) = %d\n", arr[2], *(q + 2));

    // --- Aritmética de punteros ---
    printf("\nAritmética de punteros:\n");
    int *first = &arr[0];
    int *third = &arr[2];
    printf("  arr[0] a arr[2]: distancia = %d elementos\n", (int)(third - first));

    // --- Función con puntero a array ---
    fill_sequence(arr, 5);
    printf("\nDespués fill_sequence:\n");
    int i;
    for (i = 0; i < 5; i++) {
        printf("  arr[%d] = %d\n", i, arr[i]);
    }

    int total = sum_array(arr, 5);
    printf("  sum = %d\n", total);

    // --- Puntero a puntero ---
    int val = 999;
    int *pval = &val;
    int **ppval = &pval;
    printf("\nPuntero a puntero:\n");
    printf("  val = %d\n", val);
    printf("  *pval = %d\n", *pval);
    printf("  **ppval = %d\n", **ppval);

    **ppval = 777;
    printf("  Después **ppval = 777: val = %d\n", val);

    // --- NULL pointer ---
    int *null_ptr = 0;
    printf("\nNULL pointer: %d\n", null_ptr == 0);

    // --- Verificación ---
    int pass = 0;
    int tests = 0;

    tests++; if (x == 101)    { pass++; } else { printf("FAIL: increment\n"); }
    tests++; if (a == 20)     { pass++; } else { printf("FAIL: swap a\n"); }
    tests++; if (b == 10)     { pass++; } else { printf("FAIL: swap b\n"); }
    tests++; if (total == 100){ pass++; } else { printf("FAIL: sum\n"); }
    tests++; if (val == 777)  { pass++; } else { printf("FAIL: double ptr\n"); }

    printf("\n%d/%d passed\n", pass, tests);
    return 0;
}
