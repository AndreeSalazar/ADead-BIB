// ============================================================
// ADead-BIB C++14 Intensive Test — Extended Features
// "Respetar Bits" — Type Strictness ULTRA
// ============================================================
// Ejecutar: adb cpp reportes/tests_cpp_intensive/test_cpp14_features.cpp
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// ============================================================
// Test 1: Generic Lambdas (auto parameters)
// ============================================================
void test_generic_lambdas() {
    printf("\n=== TEST 1: Generic Lambdas ===\n");
    
    // Generic lambda with auto
    auto print_value = [](auto x) {
        printf("  Value: %d\n", (int)x);
    };
    
    print_value(42);
    print_value(3.14f);
    print_value(100);
    
    // Generic lambda for comparison
    auto max_val = [](auto a, auto b) {
        return (a > b) ? a : b;
    };
    
    printf("  max(10, 20) = %d\n", max_val(10, 20));
    printf("  max(3.14, 2.71) = %.2f\n", max_val(3.14, 2.71));
    
    // Generic lambda with multiple operations
    auto apply_twice = [](auto f, auto x) {
        return f(f(x));
    };
    
    auto double_it = [](auto x) { return x * 2; };
    printf("  apply_twice(double, 5) = %d\n", apply_twice(double_it, 5));
}

// ============================================================
// Test 2: Return type deduction
// ============================================================
auto add_numbers(int a, int b) {
    return a + b;  // Return type deduced as int
}

auto multiply_numbers(float a, float b) {
    return a * b;  // Return type deduced as float
}

auto create_pair(int x, float y) {
    struct Pair { int first; float second; };
    Pair p;
    p.first = x;
    p.second = y;
    return p;  // Return type deduced as Pair
}

void test_return_type_deduction() {
    printf("\n=== TEST 2: Return type deduction ===\n");
    
    auto sum = add_numbers(10, 20);
    printf("  add_numbers(10, 20) = %d\n", sum);
    
    auto product = multiply_numbers(3.0f, 4.0f);
    printf("  multiply_numbers(3.0, 4.0) = %.2f\n", product);
    
    auto pair = create_pair(42, 3.14f);
    printf("  create_pair(42, 3.14) = {%d, %.2f}\n", pair.first, pair.second);
}

// ============================================================
// Test 3: Variable templates
// ============================================================
template<typename T>
constexpr T pi = T(3.14159265358979323846);

template<typename T>
constexpr T e = T(2.71828182845904523536);

template<typename T>
constexpr T golden_ratio = T(1.61803398874989484820);

void test_variable_templates() {
    printf("\n=== TEST 3: Variable templates ===\n");
    
    printf("  pi<float> = %.6f\n", pi<float>);
    printf("  pi<double> = %.15f\n", pi<double>);
    
    printf("  e<float> = %.6f\n", e<float>);
    printf("  e<double> = %.15f\n", e<double>);
    
    printf("  golden_ratio<float> = %.6f\n", golden_ratio<float>);
    printf("  golden_ratio<double> = %.15f\n", golden_ratio<double>);
    
    // Use in calculations
    float circle_area = pi<float> * 5.0f * 5.0f;
    printf("  Circle area (r=5): %.4f\n", circle_area);
}

// ============================================================
// Test 4: Relaxed constexpr
// ============================================================
constexpr int compute_factorial(int n) {
    int result = 1;
    for (int i = 2; i <= n; i++) {
        result = result * i;
    }
    return result;
}

constexpr int compute_fibonacci(int n) {
    if (n <= 1) return n;
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        int temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

constexpr int compute_power(int base, int exp) {
    int result = 1;
    for (int i = 0; i < exp; i++) {
        result = result * base;
    }
    return result;
}

void test_relaxed_constexpr() {
    printf("\n=== TEST 4: Relaxed constexpr ===\n");
    
    constexpr int fact5 = compute_factorial(5);
    constexpr int fact7 = compute_factorial(7);
    printf("  constexpr factorial(5) = %d\n", fact5);
    printf("  constexpr factorial(7) = %d\n", fact7);
    
    constexpr int fib10 = compute_fibonacci(10);
    constexpr int fib15 = compute_fibonacci(15);
    printf("  constexpr fibonacci(10) = %d\n", fib10);
    printf("  constexpr fibonacci(15) = %d\n", fib15);
    
    constexpr int pow2_10 = compute_power(2, 10);
    constexpr int pow3_5 = compute_power(3, 5);
    printf("  constexpr power(2, 10) = %d\n", pow2_10);
    printf("  constexpr power(3, 5) = %d\n", pow3_5);
}

// ============================================================
// Test 5: Binary literals and digit separators
// ============================================================
void test_binary_literals() {
    printf("\n=== TEST 5: Binary literals & digit separators ===\n");
    
    // Binary literals
    int bin1 = 0b1010;      // 10
    int bin2 = 0b11110000;  // 240
    int bin3 = 0b10101010;  // 170
    
    printf("  0b1010 = %d\n", bin1);
    printf("  0b11110000 = %d\n", bin2);
    printf("  0b10101010 = %d\n", bin3);
    
    // Digit separators
    int million = 1'000'000;
    long long billion = 1'000'000'000LL;
    float precise = 3.141'592'653f;
    
    printf("  1'000'000 = %d\n", million);
    printf("  1'000'000'000 = %lld\n", billion);
    printf("  3.141'592'653 = %.9f\n", precise);
    
    // Binary with separators
    int byte_pattern = 0b1111'0000'1010'0101;
    printf("  0b1111'0000'1010'0101 = %d (0x%04X)\n", byte_pattern, byte_pattern);
}

// ============================================================
// Test 6: [[deprecated]] attribute
// ============================================================
[[deprecated("Use new_function instead")]]
void old_function() {
    printf("  old_function() called (deprecated)\n");
}

void new_function() {
    printf("  new_function() called (recommended)\n");
}

[[deprecated]]
int legacy_add(int a, int b) {
    return a + b;
}

void test_deprecated_attribute() {
    printf("\n=== TEST 6: [[deprecated]] attribute ===\n");
    
    // These would generate warnings in strict mode
    old_function();
    new_function();
    
    int result = legacy_add(5, 3);
    printf("  legacy_add(5, 3) = %d (deprecated)\n", result);
}

// ============================================================
// Test 7: Lambda capture expressions
// ============================================================
void test_lambda_capture_expressions() {
    printf("\n=== TEST 7: Lambda capture expressions ===\n");
    
    int x = 10;
    int y = 20;
    
    // Capture with initialization
    auto lambda1 = [z = x + y]() {
        printf("  Captured z = x + y = %d\n", z);
    };
    lambda1();
    
    // Move capture (simulated)
    auto lambda2 = [captured_x = x * 2]() {
        printf("  Captured captured_x = x * 2 = %d\n", captured_x);
    };
    lambda2();
    
    // Multiple capture expressions
    auto lambda3 = [a = x, b = y, sum = x + y]() {
        printf("  Captured a=%d, b=%d, sum=%d\n", a, b, sum);
    };
    lambda3();
}

// ============================================================
// Test 8: decltype(auto)
// ============================================================
int global_value = 42;

decltype(auto) get_value() {
    return global_value;  // Returns int
}

decltype(auto) get_reference() {
    return (global_value);  // Returns int& (parentheses matter!)
}

void test_decltype_auto() {
    printf("\n=== TEST 8: decltype(auto) ===\n");
    
    auto val = get_value();
    printf("  get_value() = %d\n", val);
    
    decltype(auto) ref = get_reference();
    printf("  get_reference() = %d\n", ref);
    
    // Modify through reference
    ref = 100;
    printf("  After ref = 100, global_value = %d\n", global_value);
    
    // Reset
    global_value = 42;
}

// ============================================================
// Test 9: Aggregate member initialization
// ============================================================
struct Point3D {
    float x = 0.0f;
    float y = 0.0f;
    float z = 0.0f;
};

struct Rectangle {
    int width = 10;
    int height = 20;
    int area() { return width * height; }
};

struct Config {
    int max_connections = 100;
    int timeout_ms = 5000;
    bool debug_mode = false;
};

void test_aggregate_init() {
    printf("\n=== TEST 9: Aggregate member initialization ===\n");
    
    Point3D p1;  // Uses defaults
    printf("  Point3D default: (%.1f, %.1f, %.1f)\n", p1.x, p1.y, p1.z);
    
    Point3D p2 = {1.0f, 2.0f, 3.0f};
    printf("  Point3D initialized: (%.1f, %.1f, %.1f)\n", p2.x, p2.y, p2.z);
    
    Rectangle r1;  // Uses defaults
    printf("  Rectangle default: %dx%d, area=%d\n", r1.width, r1.height, r1.area());
    
    Rectangle r2 = {30, 40};
    printf("  Rectangle initialized: %dx%d, area=%d\n", r2.width, r2.height, r2.area());
    
    Config cfg;  // Uses all defaults
    printf("  Config: max_conn=%d, timeout=%d, debug=%s\n", 
           cfg.max_connections, cfg.timeout_ms, cfg.debug_mode ? "true" : "false");
}

// ============================================================
// Test 10: constexpr member functions
// ============================================================
class ConstexprPoint {
public:
    int x, y;
    
    constexpr ConstexprPoint(int px = 0, int py = 0) : x(px), y(py) {}
    
    constexpr int distanceSquared() const {
        return x * x + y * y;
    }
    
    constexpr ConstexprPoint add(const ConstexprPoint& other) const {
        return ConstexprPoint(x + other.x, y + other.y);
    }
    
    constexpr bool operator==(const ConstexprPoint& other) const {
        return x == other.x && y == other.y;
    }
};

void test_constexpr_members() {
    printf("\n=== TEST 10: constexpr member functions ===\n");
    
    constexpr ConstexprPoint p1(3, 4);
    constexpr ConstexprPoint p2(1, 2);
    
    constexpr int dist = p1.distanceSquared();
    printf("  Point(3,4).distanceSquared() = %d\n", dist);
    
    constexpr ConstexprPoint p3 = p1.add(p2);
    printf("  Point(3,4) + Point(1,2) = Point(%d,%d)\n", p3.x, p3.y);
    
    constexpr bool equal = (p1 == p1);
    printf("  Point(3,4) == Point(3,4): %s\n", equal ? "true" : "false");
    
    constexpr bool not_equal = (p1 == p2);
    printf("  Point(3,4) == Point(1,2): %s\n", not_equal ? "true" : "false");
}

// ============================================================
// Main — Run all tests
// ============================================================
int main() {
    printf("============================================================\n");
    printf("ADead-BIB C++14 Intensive Test — Extended Features\n");
    printf("============================================================\n");
    
    test_generic_lambdas();
    test_return_type_deduction();
    test_variable_templates();
    test_relaxed_constexpr();
    test_binary_literals();
    test_deprecated_attribute();
    test_lambda_capture_expressions();
    test_decltype_auto();
    test_aggregate_init();
    test_constexpr_members();
    
    printf("\n============================================================\n");
    printf("All C++14 Feature tests completed!\n");
    printf("==============================================================\n");
    
    return 0;
}
