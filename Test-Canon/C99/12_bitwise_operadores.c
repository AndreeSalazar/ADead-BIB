// ============================================================
// Canon C99 — §6.5.10-12 Operadores Bitwise
// ============================================================
// Intención: Los operadores bitwise operan directamente
// sobre los bits de la representación en memoria.
// Se traducen a instrucciones x86-64 de 1 ciclo.
//
// &  (AND)  → and rax, rbx
// |  (OR)   → or  rax, rbx
// ^  (XOR)  → xor rax, rbx
// ~  (NOT)  → not rax
// << (SHL)  → shl rax, cl
// >> (SHR)  → shr rax, cl (unsigned) / sar rax, cl (signed)
// ============================================================

#include <stdio.h>

// --- Funciones bitwise ---

unsigned int set_bit(unsigned int value, int bit) {
    return value | (1 << bit);
}

unsigned int clear_bit(unsigned int value, int bit) {
    return value & ~(1 << bit);
}

int test_bit(unsigned int value, int bit) {
    return (value >> bit) & 1;
}

unsigned int toggle_bit(unsigned int value, int bit) {
    return value ^ (1 << bit);
}

int count_set_bits(unsigned int n) {
    int count = 0;
    while (n > 0) {
        count = count + (n & 1);
        n = n >> 1;
    }
    return count;
}

int is_power_of_two(unsigned int n) {
    if (n == 0) return 0;
    return (n & (n - 1)) == 0;
}

unsigned int next_power_of_two(unsigned int n) {
    if (n == 0) return 1;
    n = n - 1;
    n = n | (n >> 1);
    n = n | (n >> 2);
    n = n | (n >> 4);
    n = n | (n >> 8);
    n = n | (n >> 16);
    return n + 1;
}

void swap_xor(int *a, int *b) {
    *a = *a ^ *b;
    *b = *a ^ *b;
    *a = *a ^ *b;
}

unsigned int extract_byte(unsigned int value, int byte_index) {
    return (value >> (byte_index * 8)) & 0xFF;
}

int main() {
    printf("=== Canon C99: Bitwise Operadores ===\n\n");

    // --- Operadores básicos ---
    unsigned int a = 0xAA;
    unsigned int b = 0x55;

    printf("a = 0x%02X, b = 0x%02X\n", a, b);
    printf("a & b = 0x%02X\n", a & b);
    printf("a | b = 0x%02X\n", a | b);
    printf("a ^ b = 0x%02X\n", a ^ b);
    printf("~a    = 0x%02X\n", (~a) & 0xFF);

    // --- Shifts ---
    printf("\nShifts:\n");
    unsigned int x = 1;
    printf("  1 << 0 = %u\n", x << 0);
    printf("  1 << 1 = %u\n", x << 1);
    printf("  1 << 4 = %u\n", x << 4);
    printf("  1 << 8 = %u\n", x << 8);

    unsigned int y = 256;
    printf("  256 >> 1 = %u\n", y >> 1);
    printf("  256 >> 4 = %u\n", y >> 4);
    printf("  256 >> 8 = %u\n", y >> 8);

    // --- Manipulación de bits ---
    printf("\nBit manipulation:\n");
    unsigned int flags = 0;
    flags = set_bit(flags, 0);
    flags = set_bit(flags, 3);
    flags = set_bit(flags, 7);
    printf("  Set bits 0,3,7: 0x%02X\n", flags);

    printf("  bit 0 = %d\n", test_bit(flags, 0));
    printf("  bit 1 = %d\n", test_bit(flags, 1));
    printf("  bit 3 = %d\n", test_bit(flags, 3));

    flags = clear_bit(flags, 3);
    printf("  Clear bit 3: 0x%02X\n", flags);

    flags = toggle_bit(flags, 4);
    printf("  Toggle bit 4: 0x%02X\n", flags);

    // --- Popcount ---
    printf("\nPopcount:\n");
    printf("  bits(0xFF) = %d\n", count_set_bits(0xFF));
    printf("  bits(0x0F) = %d\n", count_set_bits(0x0F));
    printf("  bits(0x01) = %d\n", count_set_bits(0x01));

    // --- Power of 2 ---
    printf("\nPower of 2:\n");
    printf("  is_pow2(16) = %d\n", is_power_of_two(16));
    printf("  is_pow2(17) = %d\n", is_power_of_two(17));
    printf("  next_pow2(5) = %u\n", next_power_of_two(5));
    printf("  next_pow2(16) = %u\n", next_power_of_two(16));

    // --- XOR swap ---
    int p = 42;
    int q = 99;
    printf("\nXOR swap: before p=%d q=%d\n", p, q);
    swap_xor(&p, &q);
    printf("  after p=%d q=%d\n", p, q);

    // --- Byte extraction ---
    unsigned int word = 0x41424344;
    printf("\nExtract bytes from 0x%08X:\n", word);
    printf("  byte[0] = 0x%02X\n", extract_byte(word, 0));
    printf("  byte[1] = 0x%02X\n", extract_byte(word, 1));
    printf("  byte[2] = 0x%02X\n", extract_byte(word, 2));
    printf("  byte[3] = 0x%02X\n", extract_byte(word, 3));

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if ((a & b) == 0x00)        { pass++; } else { printf("FAIL: AND\n"); }
    total++; if ((a | b) == 0xFF)        { pass++; } else { printf("FAIL: OR\n"); }
    total++; if ((a ^ b) == 0xFF)        { pass++; } else { printf("FAIL: XOR\n"); }
    total++; if (count_set_bits(0xFF)==8) { pass++; } else { printf("FAIL: popcount\n"); }
    total++; if (is_power_of_two(16))    { pass++; } else { printf("FAIL: pow2\n"); }
    total++; if (!is_power_of_two(17))   { pass++; } else { printf("FAIL: !pow2\n"); }
    total++; if (next_power_of_two(5)==8){ pass++; } else { printf("FAIL: next_pow2\n"); }
    total++; if (p == 99 && q == 42)     { pass++; } else { printf("FAIL: xor swap\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
