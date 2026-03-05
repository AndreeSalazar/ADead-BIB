// ============================================================
// Canon C99 — §6.5.4 Cast y Conversión de Tipos
// ============================================================
// Intención: Un cast es una instrucción directa al compilador
// para reinterpretar o convertir un valor.
//
// C99 §6.3: "Several operators convert operand values from
// one type to another automatically."
//
// Tipos de conversión:
//   Promoción:   char → int (automática, sign-extend)
//   Truncamiento: int → char (pierde bits altos)
//   Sign-extend: char → int (propaga bit de signo)
//   Zero-extend: unsigned char → unsigned int (rellena 0)
// ============================================================

#include <stdio.h>

int main() {
    printf("=== Canon C99: Casting de Tipos ===\n\n");

    // --- Promoción entera (automática) ---
    char c = 65;
    int promoted = c;
    printf("char 65 → int: %d (='%c')\n", promoted, c);

    short s = 1000;
    int s_promoted = s;
    printf("short 1000 → int: %d\n", s_promoted);

    // --- Truncamiento (cast explícito) ---
    int big = 0x41424344;
    char truncated = (char)big;
    printf("\nint 0x%X → char: 0x%02X ('%c')\n", big, (unsigned char)truncated, truncated);

    int large = 100000;
    short to_short = (short)large;
    printf("int %d → short: %d (truncado)\n", large, to_short);

    // --- Signed / Unsigned ---
    int negative = -1;
    unsigned int as_unsigned = (unsigned int)negative;
    printf("\n(signed) -1 → (unsigned): %u\n", as_unsigned);

    unsigned int max_u = 0xFFFFFFFF;
    int as_signed = (int)max_u;
    printf("(unsigned) 0xFFFFFFFF → (signed): %d\n", as_signed);

    // --- División entera vs float ---
    int a = 7;
    int b = 2;
    int int_div = a / b;
    printf("\n7 / 2 (int) = %d\n", int_div);

    // --- Char arithmetic ---
    char letter = 'A';
    int offset = letter - 'A';
    printf("\n'A' - 'A' = %d\n", offset);

    char next = letter + 1;
    printf("'A' + 1 = '%c'\n", next);

    char digit = '7';
    int digit_val = digit - '0';
    printf("'7' - '0' = %d\n", digit_val);

    // --- Boolean (int as bool) ---
    int truthy = 42;
    int falsy = 0;
    printf("\n42 as bool: %d\n", truthy != 0);
    printf("0 as bool: %d\n", falsy != 0);

    // --- Pointer cast ---
    int value = 0x12345678;
    int *ptr = &value;
    char *byte_ptr = (char *)ptr;
    printf("\nPointer cast:\n");
    printf("  int value = 0x%X\n", value);
    printf("  first byte via char*: 0x%02X\n", (unsigned char)*byte_ptr);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (promoted == 65)           { pass++; } else { printf("FAIL: promotion\n"); }
    total++; if (truncated == 'D')         { pass++; } else { printf("FAIL: truncation\n"); }
    total++; if (int_div == 3)             { pass++; } else { printf("FAIL: int div\n"); }
    total++; if (offset == 0)              { pass++; } else { printf("FAIL: char arith\n"); }
    total++; if (next == 'B')              { pass++; } else { printf("FAIL: next char\n"); }
    total++; if (digit_val == 7)           { pass++; } else { printf("FAIL: digit val\n"); }
    total++; if (as_signed == -1)          { pass++; } else { printf("FAIL: signed\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
