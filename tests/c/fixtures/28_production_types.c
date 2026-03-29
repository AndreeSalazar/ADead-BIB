// Test: Production types — sizeof, alignment, casting, type limits
// Expected: Compile + Run — ALL assertions must pass for production use
// Strict: Every type size verified, every cast validated

#include <stdio.h>
#include <stdint.h>
#include <limits.h>
#include <float.h>
#include <stddef.h>

int main() {
    printf("=== PRODUCTION: Type System ===\n");
    int pass = 0, fail = 0;

    // sizeof verification
    if (sizeof(char) == 1)        { pass++; } else { fail++; printf("FAIL: sizeof(char)=%d\n", (int)sizeof(char)); }
    if (sizeof(short) == 2)       { pass++; } else { fail++; printf("FAIL: sizeof(short)=%d\n", (int)sizeof(short)); }
    if (sizeof(int) == 4)         { pass++; } else { fail++; printf("FAIL: sizeof(int)=%d\n", (int)sizeof(int)); }
    if (sizeof(long long) == 8)   { pass++; } else { fail++; printf("FAIL: sizeof(long long)=%d\n", (int)sizeof(long long)); }
    if (sizeof(float) == 4)       { pass++; } else { fail++; printf("FAIL: sizeof(float)=%d\n", (int)sizeof(float)); }
    if (sizeof(double) == 8)      { pass++; } else { fail++; printf("FAIL: sizeof(double)=%d\n", (int)sizeof(double)); }
    if (sizeof(void*) == 8)       { pass++; } else { fail++; printf("FAIL: sizeof(void*)=%d\n", (int)sizeof(void*)); }

    // Fixed-width types
    if (sizeof(int8_t) == 1)      { pass++; } else { fail++; printf("FAIL: sizeof(int8_t)\n"); }
    if (sizeof(int16_t) == 2)     { pass++; } else { fail++; printf("FAIL: sizeof(int16_t)\n"); }
    if (sizeof(int32_t) == 4)     { pass++; } else { fail++; printf("FAIL: sizeof(int32_t)\n"); }
    if (sizeof(int64_t) == 8)     { pass++; } else { fail++; printf("FAIL: sizeof(int64_t)\n"); }
    if (sizeof(uint8_t) == 1)     { pass++; } else { fail++; printf("FAIL: sizeof(uint8_t)\n"); }
    if (sizeof(uint16_t) == 2)    { pass++; } else { fail++; printf("FAIL: sizeof(uint16_t)\n"); }
    if (sizeof(uint32_t) == 4)    { pass++; } else { fail++; printf("FAIL: sizeof(uint32_t)\n"); }
    if (sizeof(uint64_t) == 8)    { pass++; } else { fail++; printf("FAIL: sizeof(uint64_t)\n"); }

    // Limits macros must be real values
    if (INT_MAX == 2147483647)    { pass++; } else { fail++; printf("FAIL: INT_MAX=%d\n", INT_MAX); }
    if (INT_MIN < 0)              { pass++; } else { fail++; printf("FAIL: INT_MIN\n"); }
    if (CHAR_BIT == 8)            { pass++; } else { fail++; printf("FAIL: CHAR_BIT=%d\n", CHAR_BIT); }
    if (UINT_MAX > 0)             { pass++; } else { fail++; printf("FAIL: UINT_MAX\n"); }
    if (SHRT_MAX == 32767)        { pass++; } else { fail++; printf("FAIL: SHRT_MAX\n"); }

    // Casting safety
    double d = 3.14;
    int i = (int)d;
    if (i == 3)                   { pass++; } else { fail++; printf("FAIL: (int)3.14=%d\n", i); }

    uint32_t u = 0xDEADBEEF;
    int32_t s = (int32_t)u;
    if (s < 0)                    { pass++; } else { fail++; printf("FAIL: (int32_t)0xDEADBEEF\n"); }

    // NULL
    void *p = NULL;
    if (p == 0)                   { pass++; } else { fail++; printf("FAIL: NULL != 0\n"); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== PRODUCTION: Types %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
