// ADead-BIB Bridge Test 03 — Math & Logic (ASM-BIB bridge)
// Level: BASIC
// Tests: abs, min, max, clamp, swap, popcount, bsr, bsf, bswap

#include <stdio.h>
#include <stdlib.h>

int main() {
    printf("=== ADead-BIB Bridge Test 03: Math & Logic ===\n");
    int pass = 0, fail = 0;

    // abs
    if (abs(-42) == 42)  { pass++; } else { fail++; printf("FAIL: abs(-42)\n"); }
    if (abs(42) == 42)   { pass++; } else { fail++; printf("FAIL: abs(42)\n"); }
    if (abs(0) == 0)     { pass++; } else { fail++; printf("FAIL: abs(0)\n"); }

    // min/max via ternary (C doesn't have min/max builtins)
    int a = 10, b = 20;
    int mn = (a < b) ? a : b;
    int mx = (a > b) ? a : b;
    if (mn == 10) { pass++; } else { fail++; printf("FAIL: min\n"); }
    if (mx == 20) { pass++; } else { fail++; printf("FAIL: max\n"); }

    // clamp pattern
    int val = 150;
    int lo = 0, hi = 100;
    int clamped = val;
    if (clamped < lo) clamped = lo;
    if (clamped > hi) clamped = hi;
    if (clamped == 100) { pass++; } else { fail++; printf("FAIL: clamp high\n"); }

    val = -50;
    clamped = val;
    if (clamped < lo) clamped = lo;
    if (clamped > hi) clamped = hi;
    if (clamped == 0) { pass++; } else { fail++; printf("FAIL: clamp low\n"); }

    // swap pattern
    int x = 111, y = 222;
    int tmp = x; x = y; y = tmp;
    if (x == 222 && y == 111) { pass++; } else { fail++; printf("FAIL: swap\n"); }

    // bit operations
    unsigned int v = 0xFF00FF00;
    // count set bits manually
    int bits = 0;
    unsigned int t = v;
    while (t) { bits += t & 1; t >>= 1; }
    if (bits == 16) { pass++; } else { fail++; printf("FAIL: popcount %d\n", bits); }

    // highest bit of 0x80 = bit 7
    unsigned int hb = 0x80;
    int highest = -1;
    for (int i = 31; i >= 0; i--) {
        if (hb & (1u << i)) { highest = i; break; }
    }
    if (highest == 7) { pass++; } else { fail++; printf("FAIL: bsr %d\n", highest); }

    // lowest bit of 0x80 = bit 7
    int lowest = -1;
    for (int i = 0; i < 32; i++) {
        if (hb & (1u << i)) { lowest = i; break; }
    }
    if (lowest == 7) { pass++; } else { fail++; printf("FAIL: bsf %d\n", lowest); }

    // byte swap: 0x12345678 -> 0x78563412
    unsigned int bsw = 0x12345678;
    unsigned int swapped = ((bsw >> 24) & 0xFF)
                         | ((bsw >> 8) & 0xFF00)
                         | ((bsw << 8) & 0xFF0000)
                         | ((bsw << 24) & 0xFF000000);
    if (swapped == 0x78563412) { pass++; } else { fail++; printf("FAIL: bswap32 0x%08X\n", swapped); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 03: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
