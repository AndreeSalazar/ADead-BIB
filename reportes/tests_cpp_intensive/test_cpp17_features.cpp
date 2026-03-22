// ============================================================
// ADead-BIB C++17 Intensive Test — Latest Features
// "Respetar Bits" — Type Strictness ULTRA
// ============================================================
// Ejecutar: adb cpp reportes/tests_cpp_intensive/test_cpp17_features.cpp
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// ============================================================
// Test 1: Structured bindings
// ============================================================
struct Point2D {
    int x, y;
};

struct RGB {
    unsigned char r, g, b;
};

Point2D get_point() {
    return {10, 20};
}

RGB get_color() {
    return {255, 128, 64};
}

void test_structured_bindings() {
    printf("\n=== TEST 1: Structured bindings ===\n");
    
    // Structured binding with struct
    auto [x, y] = get_point();
    printf("  Point: x=%d, y=%d\n", x, y);
    
    // Structured binding with RGB
    auto [r, g, b] = get_color();
    printf("  Color: R=%d, G=%d, B=%d\n", (int)r, (int)g, (int)b);
    
    // Structured binding with array
    int arr[] = {100, 200, 300};
    auto [a, b2, c] = arr;
    printf("  Array: a=%d, b=%d, c=%d\n", a, b2, c);
    
    // Modify through binding
    Point2D p = {5, 10};
    auto& [px, py] = p;
    px = 50;
    py = 100;
    printf("  Modified Point: x=%d, y=%d\n", p.x, p.y);
}

// ============================================================
// Test 2: if constexpr
// ============================================================
template<typename T>
void print_type_info() {
    if constexpr (sizeof(T) == 1) {
        printf("  Type is 1 byte (char/bool)\n");
    } else if constexpr (sizeof(T) == 4) {
        printf("  Type is 4 bytes (int/float)\n");
    } else if constexpr (sizeof(T) == 8) {
        printf("  Type is 8 bytes (long long/double/pointer)\n");
    } else {
        printf("  Type is %d bytes\n", (int)sizeof(T));
    }
}

template<typename T>
T process_value(T val) {
    if constexpr (sizeof(T) <= 4) {
        return val * 2;  // Double small types
    } else {
        return val + 1;  // Increment large types
    }
}

void test_if_constexpr() {
    printf("\n=== TEST 2: if constexpr ===\n");
    
    printf("  char: ");
    print_type_info<char>();
    
    printf("  int: ");
    print_type_info<int>();
    
    printf("  double: ");
    print_type_info<double>();
    
    printf("  int*: ");
    print_type_info<int*>();
    
    int i = 10;
    long long ll = 100LL;
    printf("  process_value(int 10) = %d\n", process_value(i));
    printf("  process_value(long long 100) = %lld\n", process_value(ll));
}

// ============================================================
// Test 3: Fold expressions
// ============================================================
template<typename... Args>
auto sum_all(Args... args) {
    return (args + ...);  // Unary right fold
}

template<typename... Args>
auto multiply_all(Args... args) {
    return (args * ...);  // Unary right fold
}

template<typename... Args>
void print_all(Args... args) {
    ((printf("%d ", (int)args)), ...);  // Fold with comma operator
    printf("\n");
}

template<typename... Args>
bool all_positive(Args... args) {
    return ((args > 0) && ...);  // Fold with &&
}

template<typename... Args>
bool any_zero(Args... args) {
    return ((args == 0) || ...);  // Fold with ||
}

void test_fold_expressions() {
    printf("\n=== TEST 3: Fold expressions ===\n");
    
    printf("  sum_all(1, 2, 3, 4, 5) = %d\n", sum_all(1, 2, 3, 4, 5));
    printf("  multiply_all(1, 2, 3, 4) = %d\n", multiply_all(1, 2, 3, 4));
    
    printf("  print_all(10, 20, 30): ");
    print_all(10, 20, 30);
    
    printf("  all_positive(1, 2, 3): %s\n", all_positive(1, 2, 3) ? "true" : "false");
    printf("  all_positive(1, -2, 3): %s\n", all_positive(1, -2, 3) ? "true" : "false");
    
    printf("  any_zero(1, 2, 0): %s\n", any_zero(1, 2, 0) ? "true" : "false");
    printf("  any_zero(1, 2, 3): %s\n", any_zero(1, 2, 3) ? "true" : "false");
}

// ============================================================
// Test 4: Inline variables
// ============================================================
struct Constants {
    static inline int MAX_SIZE = 1024;
    static inline float PI = 3.14159f;
    static inline const char* VERSION = "1.0.0";
};

class Counter {
public:
    static inline int instance_count = 0;
    
    Counter() { instance_count++; }
    ~Counter() { instance_count--; }
};

void test_inline_variables() {
    printf("\n=== TEST 4: Inline variables ===\n");
    
    printf("  Constants::MAX_SIZE = %d\n", Constants::MAX_SIZE);
    printf("  Constants::PI = %.5f\n", Constants::PI);
    printf("  Constants::VERSION = %s\n", Constants::VERSION);
    
    printf("  Counter::instance_count = %d\n", Counter::instance_count);
    {
        Counter c1;
        Counter c2;
        printf("  After creating 2: %d\n", Counter::instance_count);
    }
    printf("  After destruction: %d\n", Counter::instance_count);
}

// ============================================================
// Test 5: [[nodiscard]] attribute
// ============================================================
[[nodiscard]] int compute_important_value() {
    return 42;
}

[[nodiscard("Error code must be checked")]]
int perform_operation() {
    return 0;  // Success
}

class [[nodiscard]] ErrorCode {
public:
    int code;
    ErrorCode(int c) : code(c) {}
};

ErrorCode do_something() {
    return ErrorCode(0);
}

void test_nodiscard() {
    printf("\n=== TEST 5: [[nodiscard]] attribute ===\n");
    
    int val = compute_important_value();
    printf("  compute_important_value() = %d\n", val);
    
    int result = perform_operation();
    printf("  perform_operation() = %d\n", result);
    
    ErrorCode ec = do_something();
    printf("  do_something().code = %d\n", ec.code);
}

// ============================================================
// Test 6: [[maybe_unused]] attribute
// ============================================================
[[maybe_unused]] static void unused_helper() {
    printf("  This function might not be used\n");
}

void test_maybe_unused([[maybe_unused]] int debug_param) {
    printf("\n=== TEST 6: [[maybe_unused]] attribute ===\n");
    
    [[maybe_unused]] int local_debug = 100;
    
    printf("  Function with [[maybe_unused]] parameter\n");
    printf("  Local [[maybe_unused]] variable defined\n");
    
    // In debug mode, we might use these
    #ifdef DEBUG
    printf("  debug_param = %d\n", debug_param);
    printf("  local_debug = %d\n", local_debug);
    #endif
    
    printf("  No warnings for unused variables!\n");
}

// ============================================================
// Test 7: Nested namespaces
// ============================================================
namespace Company::Project::Module {
    int version = 1;
    
    void print_info() {
        printf("  Company::Project::Module::version = %d\n", version);
    }
    
    namespace SubModule {
        int sub_version = 2;
        
        void print_sub_info() {
            printf("  Company::Project::Module::SubModule::sub_version = %d\n", sub_version);
        }
    }
}

void test_nested_namespaces() {
    printf("\n=== TEST 7: Nested namespaces ===\n");
    
    Company::Project::Module::print_info();
    Company::Project::Module::SubModule::print_sub_info();
    
    printf("  Accessing directly: %d\n", Company::Project::Module::version);
}

// ============================================================
// Test 8: constexpr if with type traits
// ============================================================
template<typename T>
constexpr bool is_integral_v = false;

template<>
constexpr bool is_integral_v<int> = true;

template<>
constexpr bool is_integral_v<long> = true;

template<>
constexpr bool is_integral_v<long long> = true;

template<typename T>
constexpr bool is_floating_v = false;

template<>
constexpr bool is_floating_v<float> = true;

template<>
constexpr bool is_floating_v<double> = true;

template<typename T>
void describe_type() {
    if constexpr (is_integral_v<T>) {
        printf("  Type is integral\n");
    } else if constexpr (is_floating_v<T>) {
        printf("  Type is floating point\n");
    } else {
        printf("  Type is unknown\n");
    }
}

void test_constexpr_type_traits() {
    printf("\n=== TEST 8: constexpr if with type traits ===\n");
    
    printf("  int: ");
    describe_type<int>();
    
    printf("  float: ");
    describe_type<float>();
    
    printf("  double: ");
    describe_type<double>();
    
    printf("  char*: ");
    describe_type<char*>();
}

// ============================================================
// Test 9: Init statements in if/switch
// ============================================================
int get_status() {
    static int call_count = 0;
    call_count++;
    return call_count % 3;  // Returns 0, 1, 2, 0, 1, 2...
}

void test_init_statements() {
    printf("\n=== TEST 9: Init statements in if/switch ===\n");
    
    // if with init statement
    if (int status = get_status(); status == 0) {
        printf("  Status is 0 (OK)\n");
    } else if (status == 1) {
        printf("  Status is 1 (Warning)\n");
    } else {
        printf("  Status is %d (Error)\n", status);
    }
    
    // Another if with init
    if (auto val = get_status(); val != 0) {
        printf("  Non-zero status: %d\n", val);
    }
    
    // switch with init statement
    switch (int code = get_status(); code) {
        case 0:
            printf("  Switch: code 0\n");
            break;
        case 1:
            printf("  Switch: code 1\n");
            break;
        default:
            printf("  Switch: code %d\n", code);
            break;
    }
}

// ============================================================
// Test 10: Template argument deduction for class templates
// ============================================================
template<typename T>
class Wrapper {
public:
    T value;
    
    Wrapper(T v) : value(v) {
        printf("[Wrapper] Created with value\n");
    }
    
    T get() { return value; }
};

template<typename T, typename U>
class Pair17 {
public:
    T first;
    U second;
    
    Pair17(T f, U s) : first(f), second(s) {
        printf("[Pair17] Created\n");
    }
};

void test_class_template_deduction() {
    printf("\n=== TEST 10: Class template argument deduction ===\n");
    
    // CTAD - compiler deduces template arguments
    Wrapper w1(42);        // Wrapper<int>
    Wrapper w2(3.14f);     // Wrapper<float>
    Wrapper w3(100LL);     // Wrapper<long long>
    
    printf("  Wrapper(42).get() = %d\n", w1.get());
    printf("  Wrapper(3.14f).get() = %.2f\n", w2.get());
    printf("  Wrapper(100LL).get() = %lld\n", w3.get());
    
    Pair17 p1(10, 20.5f);  // Pair17<int, float>
    printf("  Pair17(10, 20.5f) = {%d, %.1f}\n", p1.first, p1.second);
}

// ============================================================
// Main — Run all tests
// ============================================================
int main() {
    printf("============================================================\n");
    printf("ADead-BIB C++17 Intensive Test — Latest Features\n");
    printf("============================================================\n");
    
    test_structured_bindings();
    test_if_constexpr();
    test_fold_expressions();
    test_inline_variables();
    test_nodiscard();
    test_maybe_unused(42);
    test_nested_namespaces();
    test_constexpr_type_traits();
    test_init_statements();
    test_class_template_deduction();
    
    printf("\n============================================================\n");
    printf("All C++17 Feature tests completed!\n");
    printf("==============================================================\n");
    
    return 0;
}
