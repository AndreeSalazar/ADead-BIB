// ============================================================
// ADead-BIB — Ejemplos: UB vs LIMPIO
// Punteros — Los casos más comunes
// ============================================================
// Compilar con UB:    adeadc cc este_archivo.c
//                     → SE DETIENE con reporte
// Compilar limpio:    adeadc cc limpio.c
//                     → "No undefined behavior detected"
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// ============================================================
// CASO 1 — Null Pointer Dereference
// ============================================================

// ❌ CON UB:
void caso1_ub() {
    int *ptr = 0;        // ptr = NULL
    *ptr = 42;           // BOOM — deref de NULL
    // ADead-BIB: [ERROR] NullPointerDereference línea X
}

// ✅ LIMPIO:
void caso1_limpio() {
    int valor = 42;
    int *ptr = &valor;   // ptr apunta a algo real ✅
    *ptr = 42;           // seguro ✅
    printf("caso1: %d\n", *ptr);
}

// ✅ LIMPIO con malloc:
void caso1_limpio_malloc() {
    int *ptr = (int*)malloc(sizeof(int));
    if (ptr == 0) {   // check NULL siempre ✅
        return;
    }
    *ptr = 42;           // seguro ✅
    printf("caso1 malloc: %d\n", *ptr);
    free(ptr);
    ptr = 0;
}

// ============================================================
// CASO 2 — Use After Free
// ============================================================

// ❌ CON UB:
void caso2_ub() {
    int *ptr = (int*)malloc(sizeof(int));
    *ptr = 100;
    free(ptr);
    printf("%d\n", *ptr);  // BOOM — uso después de free
    // ADead-BIB: [ERROR] UseAfterFree línea X
}

// ✅ LIMPIO:
void caso2_limpio() {
    int *ptr = (int*)malloc(sizeof(int));
    if (ptr == 0) return;

    *ptr = 100;
    printf("caso2: %d\n", *ptr);  // uso ANTES de free ✅

    free(ptr);
    ptr = 0;                    // NULL después de free ✅

    // ptr == NULL → no puede ser dereferenciado
    if (ptr != 0) {             // esta rama nunca entra ✅
        printf("%d\n", *ptr);
    }
}

// ============================================================
// CASO 3 — Double Free
// ============================================================

// ❌ CON UB:
void caso3_ub() {
    int *ptr = (int*)malloc(sizeof(int));
    free(ptr);
    free(ptr);    // BOOM — double free
    // ADead-BIB: [ERROR] DoubleFree línea X
}

// ✅ LIMPIO:
void caso3_limpio() {
    int *ptr = (int*)malloc(sizeof(int));
    if (ptr == 0) return;

    free(ptr);
    ptr = 0;    // NULL después de free ✅

    // free(NULL) es seguro en C99 ✅
    // pero con ptr = NULL ya no hay riesgo
    if (ptr != 0) {
        free(ptr);  // nunca llega aquí ✅
    }
    printf("caso3: double free imposible\n");
}

// ============================================================
// CASO 4 — Array Out of Bounds
// ============================================================

// ❌ CON UB:
void caso4_ub() {
    int arr[5]; // = {1, 2, 3, 4, 5};
    arr[10] = 99;   // BOOM — fuera de bounds
    // ADead-BIB: [ERROR] ArrayOutOfBounds línea X
}

// ✅ LIMPIO:
void caso4_limpio() {
    int arr[5]; // = {1, 2, 3, 4, 5};
    int size = 5;

    // acceso con check ✅
    int index = 3;
    if (index >= 0 && index < size) {
        arr[index] = 99;
        printf("caso4: arr[%d] = %d\n", index, arr[index]);
    }

    // loop dentro de bounds ✅
    for (int i = 0; i < size; i++) {
        printf("  arr[%d] = %d\n", i, arr[i]);
    }
}

// ============================================================
// CASO 5 — Dangling Pointer (puntero colgante)
// ============================================================

// ❌ CON UB:
int* caso5_ub() {
    int local = 42;
    return &local;   // BOOM — retorna dirección de stack
    // local desaparece al salir de función
    // ADead-BIB: [ERROR] DanglingPointer línea X
}

// ✅ LIMPIO opción 1 — malloc:
int* caso5_limpio_malloc() {
    int *ptr = (int*)malloc(sizeof(int));
    if (ptr == 0) return 0;
    *ptr = 42;       // vive en heap ✅
    return ptr;      // caller debe hacer free ✅
}

// ✅ LIMPIO opción 2 — retorna valor, no puntero:
int caso5_limpio_valor() {
    int local = 42;
    return local;    // retorna copia del valor ✅
}

// ============================================================
// CASO 6 — Uninitialized Variable
// ============================================================

// ❌ CON UB:
void caso6_ub() {
    int x;              // sin inicializar
    printf("%d\n", x);  // BOOM — valor basura
    // ADead-BIB: [ERROR] UninitializedVariable línea X
}

// ❌ CON UB (sutil — en array):
void caso6_ub_array() {
    int arr[5];         // sin inicializar
    arr[0] = 10;        // solo [0] inicializado
    int suma = 0;
    for (int i = 0; i < 5; i++) {
        suma += arr[i]; // [1]-[4] son basura
    }
    // ADead-BIB: [ERROR] UninitializedVariable
}

// ✅ LIMPIO:
void caso6_limpio() {
    int x = 0;          // inicializado ✅
    printf("caso6: %d\n", x);

    int arr[5];   // todos = 0 ✅
    arr[0] = 10;
    int suma = 0;
    for (int i = 0; i < 5; i++) {
        suma += arr[i]; // todos inicializados ✅
    }
    printf("caso6 suma: %d\n", suma);
}

// ============================================================
// CASO 7 — Division by Zero
// ============================================================

// ❌ CON UB:
void caso7_ub() {
    int a = 10;
    int b = 0;
    int c = a / b;  // BOOM — division por cero
    // ADead-BIB: [ERROR] DivisionByZero línea X
}

// ✅ LIMPIO:
void caso7_limpio() {
    int a = 10;
    int b = 0;

    if (b != 0) {           // check siempre ✅
        int c = a / b;
        printf("caso7: %d\n", c);
    } else {
        printf("caso7: division por cero evitada\n");
    }
}

// ============================================================
// CASO 8 — Integer Overflow
// ============================================================

// ❌ CON UB:
void caso8_ub() {
    int x = 2147483647;  // INT_MAX
    int y = x + 1;       // BOOM — overflow signed int
    // ADead-BIB: [ERROR] IntegerOverflow línea X
    printf("%d\n", y);
}

// ✅ LIMPIO:
void caso8_limpio() {
    int x = 2147483647 - 10;   // lejos del límite ✅
    int y = x + 1;          // seguro ✅
    printf("caso8: %d\n", y);

    // o usar tipo más grande:
    long lx = 2147483647;
    long ly = lx + 1;       // long no overflowa aquí ✅
    printf("caso8 long: %d\n", ly);
}

// ============================================================
// CASO 9 — Type Confusion / Invalid Cast
// ============================================================

// ❌ CON UB:
void caso9_ub() {
    long value = 42;
    int *ptr = (int*)value;  // BOOM — cast de valor a puntero
    *ptr = 10;               // dirección inválida
    // ADead-BIB: [WARN] InvalidCast línea X
}

// ✅ LIMPIO:
void caso9_limpio() {
    int value = 42;
    int *ptr = &value;       // puntero a variable real ✅
    *ptr = 10;               // seguro ✅
    printf("caso9: %d\n", value);

    // cast seguro: mismo tipo de vuelta
    void *generic = (void*)ptr;    // void* seguro ✅
    int *back = (int*)generic;     // mismo tipo original ✅
    printf("caso9 void*: %d\n", *back);
}

int main() {
    return 0;
}
