// ============================================================
// Test 01: Tipos Básicos — int, char, short, long, unsigned
// ============================================================
// ADead-BIB Test Canon — C99 §6.2.5
// Verifica: declaración, inicialización, sizeof, límites
// ============================================================

#include <stdio.h>
#include <limits.h>
#include <stdint.h>

int main() {
    // --- Tipos enteros con signo ---
    char c = 'A';
    short s = -32000;
    int i = 42;
    long l = 100000L;
    long long ll = 9223372036854775807LL;

    // --- Tipos enteros sin signo ---
    unsigned char uc = 255;
    unsigned short us = 65535;
    unsigned int ui = 4294967295U;
    unsigned long ul = 4294967295UL;
    unsigned long long ull = 18446744073709551615ULL;

    // --- Tipos de ancho fijo (stdint.h) ---
    int8_t i8 = -128;
    int16_t i16 = -32768;
    int32_t i32 = -2147483647 - 1;
    int64_t i64 = -9223372036854775807LL - 1;
    uint8_t u8 = 255;
    uint16_t u16 = 65535;
    uint32_t u32 = 4294967295U;
    uint64_t u64 = 18446744073709551615ULL;

    // --- sizeof ---
    int sz_char = sizeof(char);
    int sz_short = sizeof(short);
    int sz_int = sizeof(int);
    int sz_long = sizeof(long);
    int sz_ll = sizeof(long long);
    int sz_ptr = sizeof(void *);

    printf("char=%d short=%d int=%d long=%d ll=%d ptr=%d\n",
           sz_char, sz_short, sz_int, sz_long, sz_ll, sz_ptr);

    // --- Límites ---
    printf("CHAR_MIN=%d CHAR_MAX=%d\n", CHAR_MIN, CHAR_MAX);
    printf("SHRT_MIN=%d SHRT_MAX=%d\n", SHRT_MIN, SHRT_MAX);
    printf("INT_MIN=%d INT_MAX=%d\n", INT_MIN, INT_MAX);
    printf("UINT_MAX=%u\n", UINT_MAX);

    // --- Conversiones implícitas ---
    int from_char = c;
    long from_int = i;
    double from_long = (double)l;

    printf("char='%c'(%d) int=%d long=%ld\n", c, from_char, i, l);

    return 0;
}
// Expected output:
// char=1 short=2 int=4 long=4 ll=8 ptr=8
// CHAR_MIN=-128 CHAR_MAX=127
// SHRT_MIN=-32768 SHRT_MAX=32767
// INT_MIN=-2147483648 INT_MAX=2147483647
// UINT_MAX=4294967295
// char='A'(65) int=42 long=100000
