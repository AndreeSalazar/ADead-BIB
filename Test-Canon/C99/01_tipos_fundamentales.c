// ============================================================
// Canon C99 — §6.2.5 Tipos Fundamentales
// ============================================================
// Intención: Cada tipo tiene un tamaño exacto en bytes.
// El compilador DEBE respetar estos tamaños para generar
// machine code correcto (mov al vs mov eax vs mov rax).
//
// C99 §6.2.5: "An object declared as type char is large
// enough to store any member of the basic execution
// character set."
//
// C99 §6.7.2: int = al menos 16 bits (típicamente 32)
//             long = al menos 32 bits
//             long long = al menos 64 bits
// ============================================================

#include <stdio.h>

int main() {
    printf("=== Canon C99: Tipos Fundamentales ===\n\n");

    // --- Enteros con signo ---
    char c = 'A';
    short s = 1000;
    int i = 100000;
    long l = 1000000;
    long long ll = 1000000000;

    printf("char    c = '%c' (valor: %d)\n", c, c);
    printf("short   s = %d\n", s);
    printf("int     i = %d\n", i);
    printf("long    l = %ld\n", l);
    printf("long long ll = %lld\n", ll);

    // --- Enteros sin signo ---
    unsigned char uc = 255;
    unsigned short us = 65535;
    unsigned int ui = 4000000000;

    printf("\nunsigned char  uc = %u\n", uc);
    printf("unsigned short us = %u\n", us);
    printf("unsigned int   ui = %u\n", ui);

    // --- Flotantes ---
    float f = 3.14;
    double d = 3.141592653589793;

    printf("\nfloat  f = %.2f\n", f);
    printf("double d = %.15f\n", d);

    // --- sizeof (tamaños canónicos) ---
    printf("\n--- sizeof ---\n");
    printf("sizeof(char)      = %d\n", (int)sizeof(char));
    printf("sizeof(short)     = %d\n", (int)sizeof(short));
    printf("sizeof(int)       = %d\n", (int)sizeof(int));
    printf("sizeof(long)      = %d\n", (int)sizeof(long));
    printf("sizeof(long long) = %d\n", (int)sizeof(long long));
    printf("sizeof(float)     = %d\n", (int)sizeof(float));
    printf("sizeof(double)    = %d\n", (int)sizeof(double));

    // --- Límites y overflow ---
    char max_char = 127;
    char min_char = -128;
    printf("\nchar range: %d to %d\n", min_char, max_char);

    int max_int = 2147483647;
    printf("int max: %d\n", max_int);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (c == 65)          { pass++; } else { printf("FAIL: char\n"); }
    total++; if (s == 1000)        { pass++; } else { printf("FAIL: short\n"); }
    total++; if (i == 100000)      { pass++; } else { printf("FAIL: int\n"); }
    total++; if (uc == 255)        { pass++; } else { printf("FAIL: unsigned char\n"); }
    total++; if (max_char == 127)  { pass++; } else { printf("FAIL: char max\n"); }
    total++; if (min_char == -128) { pass++; } else { printf("FAIL: char min\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
