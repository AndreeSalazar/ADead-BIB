// ADead-BIB Test: inttypes.h macros + fenv.h basics
#include <stdio.h>
#include <stdint.h>
#include <limits.h>

int main() {
    int pass = 0, fail = 0;

    // stdint exact-width types
    int8_t i8 = -128;
    if (i8 == -128) { pass++; printf("PASS: int8_t min=%d\n", i8); }
    else { fail++; printf("FAIL: int8_t\n"); }

    uint8_t u8 = 255;
    if (u8 == 255) { pass++; printf("PASS: uint8_t max=%u\n", (unsigned)u8); }
    else { fail++; printf("FAIL: uint8_t\n"); }

    int16_t i16 = -32768;
    if (i16 == -32768) { pass++; printf("PASS: int16_t min=%d\n", i16); }
    else { fail++; printf("FAIL: int16_t\n"); }

    uint16_t u16 = 65535;
    if (u16 == 65535) { pass++; printf("PASS: uint16_t max=%u\n", (unsigned)u16); }
    else { fail++; printf("FAIL: uint16_t\n"); }

    int32_t i32v = -2147483647 - 1;
    if (i32v == INT_MIN) { pass++; printf("PASS: int32_t min\n"); }
    else { fail++; printf("FAIL: int32_t min\n"); }

    uint32_t u32v = 4294967295U;
    if (u32v == 4294967295U) { pass++; printf("PASS: uint32_t max=%u\n", u32v); }
    else { fail++; printf("FAIL: uint32_t\n"); }

    int64_t i64v = -9223372036854775807LL - 1;
    if (i64v < 0) { pass++; printf("PASS: int64_t min is negative\n"); }
    else { fail++; printf("FAIL: int64_t min\n"); }

    uint64_t u64v = 18446744073709551615ULL;
    if (u64v > 0) { pass++; printf("PASS: uint64_t max > 0\n"); }
    else { fail++; printf("FAIL: uint64_t\n"); }

    // size_t
    if (sizeof(size_t) == 8) { pass++; printf("PASS: sizeof(size_t)=8\n"); }
    else { fail++; printf("FAIL: sizeof(size_t)=%d\n", (int)sizeof(size_t)); }

    // ptrdiff_t
    if (sizeof(ptrdiff_t) == 8) { pass++; printf("PASS: sizeof(ptrdiff_t)=8\n"); }
    else { fail++; printf("FAIL: sizeof(ptrdiff_t)\n"); }

    // intptr_t
    if (sizeof(intptr_t) == 8) { pass++; printf("PASS: sizeof(intptr_t)=8\n"); }
    else { fail++; printf("FAIL: sizeof(intptr_t)\n"); }

    printf("\n=== inttypes_fenv: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
