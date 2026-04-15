// ============================================================
// Test 18: Casts y Sizeof — conversiones, truncamiento, sizeof
// ============================================================
// ADead-BIB Test Canon — C99 §6.5.4, §6.5.3.4
// Verifica: todos los casts válidos, sizeof en runtime
// ============================================================

#include <stdio.h>
#include <stdint.h>

struct SmallStruct { char a; int b; };
struct LargeStruct { int data[100]; };

int main() {
    // --- int ↔ char ---
    int i = 65;
    char c = (char)i;
    int back = (int)c;
    printf("int->char->int: %d -> '%c' -> %d\n", i, c, back);

    // --- Truncamiento ---
    int big = 0x12345678;
    short s = (short)big;
    char ch = (char)big;
    printf("truncate: 0x%x -> short=0x%x char=0x%x\n", big, s & 0xFFFF, ch & 0xFF);

    // --- Sign extension ---
    char neg = -1;
    int extended = (int)neg;
    unsigned int u_ext = (unsigned int)(unsigned char)neg;
    printf("sign_ext: char=%d int=%d uint=%u\n", neg, extended, u_ext);

    // --- int ↔ unsigned ---
    int signed_val = -1;
    unsigned int unsigned_val = (unsigned int)signed_val;
    printf("signed->unsigned: %d -> %u\n", signed_val, unsigned_val);

    unsigned int u = 4294967295U;
    int s2 = (int)u;
    printf("unsigned->signed: %u -> %d\n", u, s2);

    // --- Pointer casts ---
    int x = 0xDEADBEEF;
    void *vp = (void *)&x;
    int *ip = (int *)vp;
    printf("ptr cast: %d\n", *ip);

    // --- int ↔ pointer (uintptr_t) ---
    int y = 42;
    uintptr_t addr = (uintptr_t)&y;
    int *recovered = (int *)addr;
    printf("uintptr: addr=0x%llx val=%d\n", (unsigned long long)addr, *recovered);

    // --- sizeof básicos ---
    printf("sizeof char=%d\n", (int)sizeof(char));
    printf("sizeof short=%d\n", (int)sizeof(short));
    printf("sizeof int=%d\n", (int)sizeof(int));
    printf("sizeof long=%d\n", (int)sizeof(long));
    printf("sizeof long long=%d\n", (int)sizeof(long long));
    printf("sizeof float=%d\n", (int)sizeof(float));
    printf("sizeof double=%d\n", (int)sizeof(double));
    printf("sizeof void*=%d\n", (int)sizeof(void *));

    // --- sizeof struct ---
    printf("sizeof SmallStruct=%d\n", (int)sizeof(struct SmallStruct));
    printf("sizeof LargeStruct=%d\n", (int)sizeof(struct LargeStruct));

    // --- sizeof array ---
    int arr[10];
    printf("sizeof arr=%d\n", (int)sizeof(arr));
    printf("sizeof arr[0]=%d\n", (int)sizeof(arr[0]));
    printf("arr count=%d\n", (int)(sizeof(arr) / sizeof(arr[0])));

    // --- sizeof expression ---
    printf("sizeof(1+2)=%d\n", (int)sizeof(1 + 2));
    printf("sizeof(1.0)=%d\n", (int)sizeof(1.0));

    // --- Cast aritmético ---
    int numerator = 7;
    int denominator = 2;
    double result = (double)numerator / (double)denominator;
    printf("7/2 as double=%.1f\n", result);

    return 0;
}
