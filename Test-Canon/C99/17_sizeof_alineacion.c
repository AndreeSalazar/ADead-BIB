// ============================================================
// Canon C99 — §6.5.3.4 sizeof y Alineación
// ============================================================
// Intención: sizeof devuelve el tamaño en bytes de un tipo
// u objeto. Es evaluado en tiempo de compilación (constante).
//
// C99 §6.5.3.4: "The sizeof operator yields the size (in
// bytes) of its operand."
//
// El compilador DEBE conocer el tamaño de cada tipo para
// generar machine code correcto.
// ============================================================

#include <stdio.h>

struct Packed {
    char a;
    char b;
    char c;
    char d;
};

struct WithPadding {
    char a;
    int b;
};

struct TwoInts {
    int x;
    int y;
};

struct Mixed {
    char c;
    short s;
    int i;
    char d;
};

struct Nested {
    struct TwoInts point;
    int z;
};

int main() {
    printf("=== Canon C99: sizeof y Alineación ===\n\n");

    // --- Tipos primitivos ---
    printf("Tipos primitivos:\n");
    printf("  sizeof(char)      = %d\n", (int)sizeof(char));
    printf("  sizeof(short)     = %d\n", (int)sizeof(short));
    printf("  sizeof(int)       = %d\n", (int)sizeof(int));
    printf("  sizeof(long)      = %d\n", (int)sizeof(long));
    printf("  sizeof(long long) = %d\n", (int)sizeof(long long));
    printf("  sizeof(float)     = %d\n", (int)sizeof(float));
    printf("  sizeof(double)    = %d\n", (int)sizeof(double));

    // --- Punteros (siempre 8 bytes en x86-64) ---
    printf("\nPunteros:\n");
    printf("  sizeof(int*)       = %d\n", (int)sizeof(int*));
    printf("  sizeof(char*)      = %d\n", (int)sizeof(char*));
    printf("  sizeof(void*)      = %d\n", (int)sizeof(void*));

    // --- Structs ---
    printf("\nStructs:\n");
    printf("  sizeof(Packed)      = %d (4 chars)\n", (int)sizeof(struct Packed));
    printf("  sizeof(WithPadding) = %d (char + padding + int)\n", (int)sizeof(struct WithPadding));
    printf("  sizeof(TwoInts)     = %d (2 ints)\n", (int)sizeof(struct TwoInts));
    printf("  sizeof(Mixed)       = %d (char+short+int+char)\n", (int)sizeof(struct Mixed));
    printf("  sizeof(Nested)      = %d\n", (int)sizeof(struct Nested));

    // --- Arrays ---
    printf("\nArrays:\n");
    int arr[10];
    printf("  sizeof(int[10])  = %d\n", (int)sizeof(arr));
    printf("  10 * sizeof(int) = %d\n", 10 * (int)sizeof(int));

    char str[20];
    printf("  sizeof(char[20]) = %d\n", (int)sizeof(str));

    // --- sizeof en expresiones ---
    printf("\nExpressions:\n");
    int x = 42;
    printf("  sizeof(x) = %d\n", (int)sizeof(x));
    printf("  sizeof(x + 1) = %d\n", (int)sizeof(x + 1));
    printf("  sizeof(char + char) = %d (promoted to int)\n", (int)sizeof((char)1 + (char)1));

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (sizeof(char) == 1)       { pass++; } else { printf("FAIL: char\n"); }
    total++; if (sizeof(short) == 2)      { pass++; } else { printf("FAIL: short\n"); }
    total++; if (sizeof(int) == 4)        { pass++; } else { printf("FAIL: int\n"); }
    total++; if (sizeof(int*) == 8)       { pass++; } else { printf("FAIL: ptr\n"); }
    total++; if (sizeof(struct Packed)==4) { pass++; } else { printf("FAIL: packed\n"); }
    total++; if (sizeof(struct TwoInts)==8){ pass++; } else { printf("FAIL: two ints\n"); }
    total++; if (sizeof(arr) == 40)       { pass++; } else { printf("FAIL: array\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
