// ============================================================
// ADead-BIB C Type Strictness Tests
// "Respetar Bits" — FORTRAN 1957 + Ada 1983 + ADead-BIB 2025
// ============================================================
// Ejecutar: adb step tests/c_abi/test_type_strictness.c
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

// ============================================================
// === DEBEN FALLAR (UB detectado) ===
// ============================================================

// TypeMismatch: int + float
void test_int_plus_float() {
    int x = 5;
    float y = 3.14f;
    float z = x + y;      // 💀 TypeMismatch: ALU vs FPU
    printf("z = %f\n", z);
}

// TypeMismatch: float + int
void test_float_plus_int() {
    float x = 3.14f;
    int y = 5;
    float z = x + y;      // 💀 TypeMismatch: FPU vs ALU
    printf("z = %f\n", z);
}

// TypeMismatch: int + double
void test_int_plus_double() {
    int x = 5;
    double y = 3.14;
    double z = x + y;     // 💀 TypeMismatch: ALU vs FPU
    printf("z = %lf\n", z);
}

// TypeMismatch: float + double
void test_float_plus_double() {
    float x = 1.5f;
    double y = 2.5;
    double z = x + y;     // 💀 TypeMismatch: float32 vs float64
    printf("z = %lf\n", z);
}

// SignedUnsignedMix: signed < unsigned comparison
void test_signed_unsigned_compare() {
    int s = -1;
    unsigned int u = 5;
    if (s < u) {          // 💀 SignedUnsignedMix
        printf("s < u\n");
    }
}

// SignedUnsignedMix: signed + unsigned arithmetic
void test_signed_unsigned_add() {
    int s = 10;
    unsigned int u = 5;
    int r = s + u;        // 💀 SignedUnsignedMix
    printf("r = %d\n", r);
}

// ImplicitCast: void* to int* without explicit cast
void test_implicit_ptr_cast() {
    void* ptr = malloc(4);
    int* iptr = ptr;      // 💀 ImplicitCast: void* → int*
    *iptr = 42;
    printf("*iptr = %d\n", *iptr);
    free(ptr);
}

// IntegerOverflow: INT_MAX + 1
void test_integer_overflow() {
    int x = 2147483647;   // INT_MAX
    x = x + 1;            // 💀 IntegerOverflow
    printf("x = %d\n", x);
}

// NarrowingConversion: double to int
void test_narrowing_double_int() {
    double d = 3.14;
    int x = d;            // 💀 NarrowingConversion: float64 → int32
    printf("x = %d\n", x);
}

// NarrowingConversion: float to int
void test_narrowing_float_int() {
    float f = 1.5f;
    int x = f;            // 💀 NarrowingConversion: float32 → int32
    printf("x = %d\n", x);
}

// TypeMismatch: int32 + int64
void test_int32_int64_mix() {
    int a = 5;            // int32
    long long b = 10;     // int64
    long long c = a + b;  // 💀 TypeMismatch: int32 vs int64
    printf("c = %lld\n", c);
}

// NarrowingConversion: int64 to int32
void test_narrowing_int64_int32() {
    long long big = 10000000000LL;
    int small = big;      // 💀 NarrowingConversion: int64 → int32
    printf("small = %d\n", small);
}

// ============================================================
// === DEBEN PASAR (válidos) ===
// ============================================================

// Same type: int + int
void test_int_plus_int() {
    int x = 5 + 3;        // ✅ int32 + int32 = int32
    printf("x = %d\n", x);
}

// Same type: float + float
void test_float_plus_float() {
    float x = 1.5f + 2.5f; // ✅ float32 + float32 = float32
    printf("x = %f\n", x);
}

// Same type: double + double
void test_double_plus_double() {
    double x = 1.5 + 2.5; // ✅ float64 + float64 = float64
    printf("x = %lf\n", x);
}

// Explicit cast: int to float
void test_explicit_cast_int_float() {
    int x = 5;
    float y = (float)x + 3.14f; // ✅ cast explícito
    printf("y = %f\n", y);
}

// Explicit pointer cast
void test_explicit_ptr_cast() {
    void* ptr = malloc(4);
    int* iptr = (int*)ptr; // ✅ cast explícito
    *iptr = 42;
    printf("*iptr = %d\n", *iptr);
    free(ptr);
}

// Explicit narrowing cast
void test_explicit_narrowing() {
    double d = 3.14;
    int x = (int)d;       // ✅ cast explícito (con warning)
    printf("x = %d\n", x);
}

// Explicit signed to unsigned cast
void test_explicit_signed_to_unsigned() {
    int s = -1;
    unsigned int u = 5;
    if ((unsigned int)s < u) { // ✅ cast explícito
        printf("(unsigned)s < u\n");
    }
}

// Explicit overflow prevention with wider type
void test_explicit_overflow_prevention() {
    int x = 2147483647;   // INT_MAX
    long long y = (long long)x + 1; // ✅ no overflow
    printf("y = %lld\n", y);
}

// Same size integers
void test_same_size_ints() {
    int a = 5;
    int b = 10;
    int c = a + b;        // ✅ int32 + int32 = int32
    printf("c = %d\n", c);
}

// Unsigned same type
void test_unsigned_same_type() {
    unsigned int a = 5;
    unsigned int b = 10;
    unsigned int c = a + b; // ✅ uint32 + uint32 = uint32
    printf("c = %u\n", c);
}

// ============================================================
// Main — Run all tests
// ============================================================

int main() {
    printf("=== ADead-BIB Type Strictness Tests ===\n\n");
    
    printf("--- Tests that SHOULD FAIL (UB detected) ---\n");
    // Uncomment to test each failure case:
    // test_int_plus_float();
    // test_float_plus_int();
    // test_int_plus_double();
    // test_float_plus_double();
    // test_signed_unsigned_compare();
    // test_signed_unsigned_add();
    // test_implicit_ptr_cast();
    // test_integer_overflow();
    // test_narrowing_double_int();
    // test_narrowing_float_int();
    // test_int32_int64_mix();
    // test_narrowing_int64_int32();
    
    printf("\n--- Tests that SHOULD PASS (valid) ---\n");
    test_int_plus_int();
    test_float_plus_float();
    test_double_plus_double();
    test_explicit_cast_int_float();
    test_explicit_ptr_cast();
    test_explicit_narrowing();
    test_explicit_signed_to_unsigned();
    test_explicit_overflow_prevention();
    test_same_size_ints();
    test_unsigned_same_type();
    
    printf("\n=== All valid tests passed! ===\n");
    return 0;
}
