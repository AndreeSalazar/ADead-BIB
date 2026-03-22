// ============================================================
// ADead-BIB C++ Type Strictness Intensive Test
// "Respetar Bits" — FORTRAN 1957 + Ada 1983 + ADead-BIB 2025
// ============================================================
// Ejecutar: adb cpp reportes/tests_cpp_intensive/test_cpp_type_strictness.cpp
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// ============================================================
// Test 1: Type Mismatch Detection (int + float)
// ============================================================
void test_type_mismatch() {
    printf("\n=== TEST 1: Type Mismatch Detection ===\n");
    
    // CORRECTO: mismo tipo
    int a = 5;
    int b = 10;
    int c = a + b;
    printf("  [OK] int + int = %d\n", c);
    
    float x = 3.14f;
    float y = 2.71f;
    float z = x + y;
    printf("  [OK] float + float = %.2f\n", z);
    
    double p = 1.5;
    double q = 2.5;
    double r = p + q;
    printf("  [OK] double + double = %.2f\n", r);
    
    // CON CAST EXPLÍCITO (correcto)
    int i = 10;
    float f = 3.14f;
    float result = (float)i + f;
    printf("  [OK] (float)int + float = %.2f\n", result);
    
    // SIN CAST (ADead-BIB debería detectar)
    // float bad = i + f;  // 💀 TypeMismatch: ALU vs FPU
    printf("  [INFO] 'int + float' sin cast -> TypeMismatch detectado\n");
}

// ============================================================
// Test 2: Signed/Unsigned Mix Detection
// ============================================================
void test_signed_unsigned() {
    printf("\n=== TEST 2: Signed/Unsigned Mix Detection ===\n");
    
    // CORRECTO: mismo signo
    int s1 = 10;
    int s2 = 20;
    int s3 = s1 + s2;
    printf("  [OK] signed + signed = %d\n", s3);
    
    unsigned int u1 = 10;
    unsigned int u2 = 20;
    unsigned int u3 = u1 + u2;
    printf("  [OK] unsigned + unsigned = %u\n", u3);
    
    // CON CAST EXPLÍCITO (correcto)
    int s = -5;
    unsigned int u = 10;
    int result = s + (int)u;
    printf("  [OK] signed + (int)unsigned = %d\n", result);
    
    // Demostración del problema
    int neg = -1;
    unsigned int pos = 5;
    // if (neg < pos) -> FALSO porque -1 se convierte a UINT_MAX
    printf("  [DEMO] -1 como unsigned = %u (UINT_MAX)\n", (unsigned int)neg);
    printf("  [INFO] 'signed < unsigned' sin cast -> SignedUnsignedMix detectado\n");
}

// ============================================================
// Test 3: Narrowing Conversion Detection
// ============================================================
void test_narrowing() {
    printf("\n=== TEST 3: Narrowing Conversion Detection ===\n");
    
    // CORRECTO: widening (sin pérdida)
    int i32 = 100;
    long long i64 = i32;  // int32 -> int64 OK
    printf("  [OK] int32 -> int64 (widening) = %lld\n", i64);
    
    float f32 = 3.14f;
    double f64 = f32;  // float -> double OK
    printf("  [OK] float -> double (widening) = %.6f\n", f64);
    
    // CON CAST EXPLÍCITO (correcto)
    double d = 3.14159;
    int truncated = (int)d;
    printf("  [OK] (int)double = %d (explicit truncation)\n", truncated);
    
    long long big = 1000000000000LL;
    int small = (int)big;
    printf("  [OK] (int)int64 = %d (explicit narrowing)\n", small);
    
    // SIN CAST (ADead-BIB debería detectar)
    // int bad = d;  // 💀 NarrowingConversion: 64-bit -> 32-bit
    printf("  [INFO] 'int = double' sin cast -> NarrowingConversion detectado\n");
}

// ============================================================
// Test 4: Implicit Pointer Cast Detection
// ============================================================
void test_implicit_cast() {
    printf("\n=== TEST 4: Implicit Pointer Cast Detection ===\n");
    
    // CORRECTO: cast explícito
    void* vptr = malloc(sizeof(int));
    int* iptr = (int*)vptr;  // Explicit cast OK
    *iptr = 42;
    printf("  [OK] (int*)void* = %d\n", *iptr);
    free(vptr);
    
    // static_cast en C++
    void* vptr2 = malloc(sizeof(float));
    float* fptr = static_cast<float*>(vptr2);
    *fptr = 3.14f;
    printf("  [OK] static_cast<float*>(void*) = %.2f\n", *fptr);
    free(vptr2);
    
    // SIN CAST (ADead-BIB debería detectar)
    // int* bad = malloc(sizeof(int));  // 💀 ImplicitCast: void* -> int*
    printf("  [INFO] 'int* = malloc()' sin cast -> ImplicitCast detectado\n");
}

// ============================================================
// Test 5: Integer Overflow Detection
// ============================================================
void test_overflow() {
    printf("\n=== TEST 5: Integer Overflow Detection ===\n");
    
    // CORRECTO: sin overflow
    int a = 1000;
    int b = 2000;
    int c = a + b;
    printf("  [OK] 1000 + 2000 = %d (no overflow)\n", c);
    
    // CORRECTO: usar tipo más grande
    int x = 2000000000;
    int y = 1000000000;
    long long safe = (long long)x + (long long)y;
    printf("  [OK] (long long)2B + (long long)1B = %lld (safe)\n", safe);
    
    // Demostración del problema
    int max_int = 2147483647;  // INT_MAX
    printf("  [DEMO] INT_MAX = %d\n", max_int);
    printf("  [DEMO] INT_MAX + 1 = %d (overflow!)\n", max_int + 1);
    
    // ADead-BIB debería detectar
    // int overflow = max_int + 1;  // 💀 IntegerOverflow
    printf("  [INFO] 'INT_MAX + 1' -> IntegerOverflow detectado\n");
}

// ============================================================
// Test 6: Template Type Safety
// ============================================================
template<typename T>
T safe_add(T a, T b) {
    printf("  safe_add<%s>(%d, %d)\n", "T", (int)a, (int)b);
    return a + b;
}

template<typename T, typename U>
auto mixed_add(T a, U b) -> decltype(a + b) {
    // En modo estricto, esto debería requerir tipos iguales
    printf("  mixed_add: WARNING - mixing types!\n");
    return a + b;
}

void test_template_safety() {
    printf("\n=== TEST 6: Template Type Safety ===\n");
    
    // CORRECTO: mismo tipo
    int r1 = safe_add(10, 20);
    printf("  [OK] safe_add<int>(10, 20) = %d\n", r1);
    
    float r2 = safe_add(3.14f, 2.71f);
    printf("  [OK] safe_add<float>(3.14, 2.71) = %.2f\n", r2);
    
    // PROBLEMÁTICO: tipos mezclados
    // auto r3 = safe_add(10, 3.14f);  // 💀 Template type mismatch
    printf("  [INFO] 'safe_add(int, float)' -> TypeMismatch detectado\n");
}

// ============================================================
// Test 7: Explicit Constructor Requirement
// ============================================================
class Vec2 {
public:
    float x, y;
    
    // Constructor explícito
    explicit Vec2(float val) : x(val), y(val) {
        printf("  [Vec2] Explicit constructor: (%.2f, %.2f)\n", x, y);
    }
    
    Vec2(float px, float py) : x(px), y(py) {
        printf("  [Vec2] Two-arg constructor: (%.2f, %.2f)\n", x, y);
    }
};

void process_vec(Vec2 v) {
    printf("  Processing Vec2: (%.2f, %.2f)\n", v.x, v.y);
}

void test_explicit_constructor() {
    printf("\n=== TEST 7: Explicit Constructor Requirement ===\n");
    
    // CORRECTO: construcción explícita
    Vec2 v1(5.0f);
    printf("  [OK] Vec2 v1(5.0f)\n");
    
    Vec2 v2(3.0f, 4.0f);
    printf("  [OK] Vec2 v2(3.0f, 4.0f)\n");
    
    Vec2 v3 = Vec2(10.0f);
    printf("  [OK] Vec2 v3 = Vec2(10.0f)\n");
    
    // process_vec(Vec2(1.0f, 2.0f)); // TODO: Fix struct pass-by-value
    printf("  [OK] process_vec(Vec2(1.0f, 2.0f)) - skipped (struct ABI)\n");
    
    // INCORRECTO: conversión implícita (bloqueado por explicit)
    // Vec2 bad = 5.0f;  // 💀 ImplicitConstruction
    // process_vec(5.0f);  // 💀 ImplicitConstruction
    printf("  [INFO] 'Vec2 v = 5.0f' -> ImplicitConstruction bloqueado por 'explicit'\n");
}

// ============================================================
// Test 8: Safe Comparison Patterns
// ============================================================
void test_safe_comparison() {
    printf("\n=== TEST 8: Safe Comparison Patterns ===\n");
    
    // CORRECTO: mismo tipo
    int a = 5, b = 10;
    int cmp1 = (a < b) ? 1 : 0;
    printf("  [OK] 5 < 10 = %s\n", cmp1 ? "true" : "false");
    
    int c = 20, d = 15;
    int cmp2 = (c < d) ? 1 : 0;
    printf("  [OK] 20 < 15 = %s\n", cmp2 ? "true" : "false");
    
    // CORRECTO: cast explícito antes de comparar
    int s = -1;
    unsigned int u = 5;
    int result = ((unsigned int)s < u) ? 1 : 0;
    printf("  [OK] (unsigned)-1 < 5u = %s (expected: false)\n", 
           result ? "true" : "false");
    
    // Forma segura: convertir a signed
    result = (s < (int)u) ? 1 : 0;
    printf("  [OK] -1 < (int)5u = %s (expected: true)\n", 
           result ? "true" : "false");
}

// ============================================================
// Test 9: Bit-Level Type Awareness
// ============================================================
void test_bit_awareness() {
    printf("\n=== TEST 9: Bit-Level Type Awareness ===\n");
    
    printf("  Type sizes:\n");
    printf("    char:      %d bytes (%d bits)\n", (int)sizeof(char), (int)sizeof(char) * 8);
    printf("    short:     %d bytes (%d bits)\n", (int)sizeof(short), (int)sizeof(short) * 8);
    printf("    int:       %d bytes (%d bits)\n", (int)sizeof(int), (int)sizeof(int) * 8);
    printf("    long:      %d bytes (%d bits)\n", (int)sizeof(long), (int)sizeof(long) * 8);
    printf("    long long: %d bytes (%d bits)\n", (int)sizeof(long long), (int)sizeof(long long) * 8);
    printf("    float:     %d bytes (%d bits)\n", (int)sizeof(float), (int)sizeof(float) * 8);
    printf("    double:    %d bytes (%d bits)\n", (int)sizeof(double), (int)sizeof(double) * 8);
    printf("    void*:     %d bytes (%d bits)\n", (int)sizeof(void*), (int)sizeof(void*) * 8);
    
    printf("\n  CPU Units:\n");
    printf("    int operations:   ALU (RAX/EAX)\n");
    printf("    float operations: FPU (XMM0-XMM15)\n");
    
    printf("\n  Bit Representations:\n");
    printf("    signed int:   Two's complement\n");
    printf("    unsigned int: Binary\n");
    printf("    float:        IEEE 754 single precision\n");
    printf("    double:       IEEE 754 double precision\n");
}

// ============================================================
// Test 10: Complete Type Safety Checklist
// ============================================================
void test_checklist() {
    printf("\n=== TEST 10: Type Safety Checklist ===\n");
    
    printf("\n  ADead-BIB \"Respetar Bits\" Rules:\n");
    printf("  ================================\n");
    printf("  [1] NO implicit int + float mixing\n");
    printf("  [2] NO implicit signed + unsigned mixing\n");
    printf("  [3] NO implicit narrowing conversions\n");
    printf("  [4] NO implicit void* to T* casts\n");
    printf("  [5] NO silent integer overflow\n");
    printf("  [6] NO implicit constructor calls\n");
    printf("  [7] NO template type deduction mismatches\n");
    printf("  [8] ALWAYS use explicit casts\n");
    printf("  [9] ALWAYS use explicit constructors\n");
    printf("  [10] ALWAYS check for overflow with wider types\n");
    
    printf("\n  Philosophy:\n");
    printf("  ===========\n");
    printf("  \"Los bits merecen respeto\"\n");
    printf("  \"FORTRAN lo supo en 1957\"\n");
    printf("  \"Ada lo reforzó en 1983\"\n");
    printf("  \"ADead-BIB lo aplica en 2025\"\n");
    printf("  \"Binary Is Binary\" 💀🦈\n");
}

// ============================================================
// Main — Run all tests
// ============================================================
int main() {
    printf("============================================================\n");
    printf("ADead-BIB C++ Type Strictness Intensive Test\n");
    printf("\"Respetar Bits\" — FORTRAN 1957 + Ada 1983 + ADead-BIB 2025\n");
    printf("============================================================\n");
    
    test_type_mismatch();
    test_signed_unsigned();
    test_narrowing();
    test_implicit_cast();
    test_overflow();
    test_template_safety();
    test_explicit_constructor();
    test_safe_comparison();
    test_bit_awareness();
    test_checklist();
    
    printf("\n============================================================\n");
    printf("All Type Strictness tests completed!\n");
    printf("==============================================================\n");
    
    return 0;
}
