// ============================================================
// Test 03: Operaciones Bitwise — &, |, ^, ~, <<, >>
// ============================================================
// ADead-BIB Test Canon — C99 §6.5.10-§6.5.12
// Verifica: AND, OR, XOR, NOT, shifts, máscaras
// ============================================================

#include <stdio.h>

int main() {
    // --- AND ---
    int a = 0xFF & 0x0F;
    int b = 0xAA & 0x55;
    int c = 0xFF & 0xFF;
    int d = 0xFF & 0x00;

    // --- OR ---
    int e = 0xF0 | 0x0F;
    int f = 0xAA | 0x55;
    int g = 0x00 | 0x00;

    // --- XOR ---
    int h = 0xFF ^ 0xFF;
    int i = 0xFF ^ 0x00;
    int j = 0xAA ^ 0x55;

    // --- NOT ---
    unsigned char k = ~(unsigned char)0;
    unsigned char l = ~(unsigned char)0xFF;

    // --- Shift izquierdo ---
    int m = 1 << 0;
    int n = 1 << 4;
    int o = 1 << 31;

    // --- Shift derecho ---
    int p = 256 >> 4;
    int q = 0xFF >> 4;
    unsigned int r = 0x80000000U >> 16;

    // --- Compound assignment bitwise ---
    int x = 0xFF;
    x &= 0x0F;
    x |= 0xA0;
    x ^= 0x05;
    x <<= 2;
    x >>= 1;

    // --- Máscaras prácticas ---
    int val = 0xDEADBEEF;
    int low_byte = val & 0xFF;
    int high_nibble = (val >> 28) & 0xF;
    int set_bit3 = val | (1 << 3);
    int clear_bit0 = val & ~1;
    int toggle_bit7 = val ^ (1 << 7);
    int bit5_set = (val >> 5) & 1;

    printf("AND: %x %x %x %x\n", a, b, c, d);
    printf("OR:  %x %x %x\n", e, f, g);
    printf("XOR: %x %x %x\n", h, i, j);
    printf("NOT: %x %x\n", k, l);
    printf("SHL: %d %d %d\n", m, n, o);
    printf("SHR: %d %d %u\n", p, q, r);
    printf("compound: %x\n", x);
    printf("masks: low=%x high=%x bit5=%d\n", low_byte, high_nibble, bit5_set);

    return 0;
}
