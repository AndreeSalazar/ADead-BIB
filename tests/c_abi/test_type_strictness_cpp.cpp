// ============================================================
// ADead-BIB C++ Type Strictness Tests
// "Respetar Bits" — FORTRAN 1957 + Ada 1983 + ADead-BIB 2025
// ============================================================
// Ejecutar: adb step tests/c_abi/test_type_strictness_cpp.cpp
// ============================================================

#include <stdio.h>

// ============================================================
// === DEBEN FALLAR (UB detectado) ===
// ============================================================

// Template type mixing
template<typename T>
T suma(T a, T b) { return a + b; }

void test_template_mix() {
    // suma(5, 3.14f);        // 💀 TypeMismatch: int vs float
    // suma(5, 3.14);         // 💀 TypeMismatch: int vs double
    printf("Template mix tests (commented out - should fail)\n");
}

// Implicit constructor
class Vec2 {
public:
    float x, y;
    Vec2(float val) : x(val), y(val) {}
};

void test_implicit_constructor() {
    // Vec2 v = 5.0f;         // 💀 ImplicitConstruction
    Vec2 v(5.0f);             // ✅ explicit
    printf("Vec2: x=%f, y=%f\n", v.x, v.y);
}

// Narrowing in initializer list (C++11)
void test_narrowing_init() {
    double d = 3.14;
    // int x{d};              // 💀 NarrowingConversion (C++11 brace init)
    // int y = d;             // 💀 NarrowingConversion (ADead-BIB strict)
    int x = (int)d;           // ✅ explicit cast
    printf("x = %d\n", x);
}

// Mixed arithmetic
void test_cpp_mixed() {
    int a = 10;
    float b = 3.14f;
    // auto c = a + b;        // 💀 TypeMismatch: int + float
    auto c = (float)a + b;    // ✅ explicit cast
    printf("c = %f\n", c);
}

// Signed/unsigned in comparison
void test_signed_unsigned_cpp() {
    int s = -1;
    unsigned int u = 5;
    // if (s < u) { }         // 💀 SignedUnsignedMix
    if ((unsigned int)s < u) { // ✅ explicit cast
        printf("(unsigned)s < u\n");
    }
}

// ============================================================
// === DEBEN PASAR (válidos) ===
// ============================================================

// Template same type
void test_template_same_type() {
    int r1 = suma(5, 3);           // ✅ int + int
    float r2 = suma(5.0f, 3.14f);  // ✅ float + float
    double r3 = suma(5.0, 3.14);   // ✅ double + double
    printf("r1=%d, r2=%f, r3=%lf\n", r1, r2, r3);
}

// Explicit constructor
void test_explicit_constructor() {
    Vec2 v(5.0f);              // ✅ explicit
    Vec2 v2 = Vec2(5.0f);      // ✅ explicit
    printf("v: x=%f, y=%f\n", v.x, v.y);
    printf("v2: x=%f, y=%f\n", v2.x, v2.y);
}

// Static cast
void test_static_cast() {
    int a = 10;
    float b = 3.14f;
    float c = static_cast<float>(a) + b; // ✅ explicit
    printf("c = %f\n", c);
}

// Explicit narrowing C++ style
void test_explicit_narrowing_cpp() {
    double d = 3.14;
    int x = static_cast<int>(d); // ✅ explicit
    printf("x = %d\n", x);
}

// Same type operations
void test_same_type_ops() {
    int a = 5, b = 10;
    int c = a + b;             // ✅ int + int
    
    float x = 1.5f, y = 2.5f;
    float z = x + y;           // ✅ float + float
    
    double p = 1.5, q = 2.5;
    double r = p + q;          // ✅ double + double
    
    printf("c=%d, z=%f, r=%lf\n", c, z, r);
}

// Pointer with explicit cast
void test_ptr_explicit_cast() {
    void* ptr = malloc(sizeof(int));
    int* iptr = static_cast<int*>(ptr); // ✅ explicit
    *iptr = 42;
    printf("*iptr = %d\n", *iptr);
    free(ptr);
}

// Class with explicit keyword (best practice)
class ExplicitVec2 {
public:
    float x, y;
    explicit ExplicitVec2(float val) : x(val), y(val) {}
};

void test_explicit_class() {
    // ExplicitVec2 v = 5.0f;     // ❌ Won't compile (explicit)
    ExplicitVec2 v(5.0f);          // ✅ OK
    ExplicitVec2 v2 = ExplicitVec2(5.0f); // ✅ OK
    printf("v: x=%f, y=%f\n", v.x, v.y);
}

// ============================================================
// Main — Run all tests
// ============================================================

int main() {
    printf("=== ADead-BIB C++ Type Strictness Tests ===\n\n");
    
    printf("--- Tests that SHOULD FAIL (UB detected) ---\n");
    test_template_mix();
    test_implicit_constructor();
    test_narrowing_init();
    test_cpp_mixed();
    test_signed_unsigned_cpp();
    
    printf("\n--- Tests that SHOULD PASS (valid) ---\n");
    test_template_same_type();
    test_explicit_constructor();
    test_static_cast();
    test_explicit_narrowing_cpp();
    test_same_type_ops();
    test_ptr_explicit_cast();
    test_explicit_class();
    
    printf("\n=== All valid tests passed! ===\n");
    return 0;
}
