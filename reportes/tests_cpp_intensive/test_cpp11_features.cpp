// ============================================================
// ADead-BIB C++11 Intensive Test — Modern Features
// "Respetar Bits" — Type Strictness ULTRA
// ============================================================
// Ejecutar: adb cpp reportes/tests_cpp_intensive/test_cpp11_features.cpp
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// ============================================================
// Test 1: auto keyword
// ============================================================
void test_auto() {
    printf("\n=== TEST 1: auto keyword ===\n");
    
    auto x = 42;
    auto y = 3.14f;
    auto z = 2.718;
    
    printf("  auto x = 42 -> type int, value: %d\n", x);
    printf("  auto y = 3.14f -> type float, value: %.2f\n", y);
    printf("  auto z = 2.718 -> type double, value: %.3f\n", z);
    
    auto sum = x + (int)y;
    printf("  auto sum = x + (int)y -> %d\n", sum);
}

// ============================================================
// Test 2: nullptr
// ============================================================
void test_nullptr() {
    printf("\n=== TEST 2: nullptr ===\n");
    
    int* ptr1 = nullptr;
    float* ptr2 = nullptr;
    
    printf("  int* ptr1 = nullptr: %s\n", ptr1 == nullptr ? "is null" : "not null");
    printf("  float* ptr2 = nullptr: %s\n", ptr2 == nullptr ? "is null" : "not null");
    
    ptr1 = (int*)malloc(sizeof(int));
    *ptr1 = 100;
    printf("  After malloc, *ptr1 = %d\n", *ptr1);
    
    if (ptr1 != nullptr) {
        printf("  ptr1 is not null, freeing...\n");
        free(ptr1);
        ptr1 = nullptr;
    }
    
    printf("  After free, ptr1 == nullptr: %s\n", ptr1 == nullptr ? "true" : "false");
}

// ============================================================
// Test 3: Range-based for (simulated with array)
// ============================================================
void test_range_for() {
    printf("\n=== TEST 3: Range-based for ===\n");
    
    int arr[] = {10, 20, 30, 40, 50};
    int sum = 0;
    
    printf("  Array: ");
    for (auto val : arr) {
        printf("%d ", val);
        sum = sum + val;
    }
    printf("\n  Sum: %d\n", sum);
    
    // Modify with reference
    printf("  Doubling values: ");
    for (auto& val : arr) {
        val = val * 2;
        printf("%d ", val);
    }
    printf("\n");
}

// ============================================================
// Test 4: Lambda expressions
// ============================================================
void test_lambda() {
    printf("\n=== TEST 4: Lambda expressions ===\n");
    
    // Simple lambda
    auto add = [](int a, int b) { return a + b; };
    printf("  Lambda add(5, 3) = %d\n", add(5, 3));
    
    // Lambda with capture
    int multiplier = 10;
    auto multiply = [multiplier](int x) { return x * multiplier; };
    printf("  Lambda multiply(7) with capture = %d\n", multiply(7));
    
    // Lambda with mutable capture
    int counter = 0;
    auto increment = [&counter]() { 
        counter = counter + 1; 
        return counter; 
    };
    printf("  Lambda increment(): %d, %d, %d\n", 
           increment(), increment(), increment());
    
    // Lambda returning lambda (higher-order)
    auto make_adder = [](int n) {
        return [n](int x) { return x + n; };
    };
    auto add5 = make_adder(5);
    printf("  Higher-order: add5(10) = %d\n", add5(10));
}

// ============================================================
// Test 5: constexpr
// ============================================================
constexpr int factorial(int n) {
    return (n <= 1) ? 1 : n * factorial(n - 1);
}

constexpr int fibonacci(int n) {
    return (n <= 1) ? n : fibonacci(n - 1) + fibonacci(n - 2);
}

constexpr int ARRAY_SIZE = 10;
constexpr int FACTORIAL_5 = factorial(5);

void test_constexpr() {
    printf("\n=== TEST 5: constexpr ===\n");
    
    printf("  constexpr ARRAY_SIZE = %d\n", ARRAY_SIZE);
    printf("  constexpr factorial(5) = %d\n", FACTORIAL_5);
    printf("  constexpr factorial(6) = %d\n", factorial(6));
    printf("  constexpr fibonacci(10) = %d\n", fibonacci(10));
    
    // Use constexpr as array size
    int arr[ARRAY_SIZE];
    for (int i = 0; i < ARRAY_SIZE; i++) {
        arr[i] = i * i;
    }
    printf("  Array[%d] squares: ", ARRAY_SIZE);
    for (int i = 0; i < ARRAY_SIZE; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

// ============================================================
// Test 6: enum class (scoped enums)
// ============================================================
enum class Color { Red, Green, Blue };
enum class Size { Small = 1, Medium = 2, Large = 3 };
enum class Status { OK = 0, Error = -1, Pending = 1 };

void test_enum_class() {
    printf("\n=== TEST 6: enum class (scoped enums) ===\n");
    
    Color c = Color::Red;
    Size s = Size::Medium;
    Status st = Status::OK;
    
    printf("  Color::Red = %d\n", (int)c);
    printf("  Size::Medium = %d\n", (int)s);
    printf("  Status::OK = %d\n", (int)st);
    
    // Switch on enum class
    switch (c) {
        case Color::Red:   printf("  Color is Red\n"); break;
        case Color::Green: printf("  Color is Green\n"); break;
        case Color::Blue:  printf("  Color is Blue\n"); break;
    }
    
    // Compare
    if (s == Size::Medium) {
        printf("  Size is Medium\n");
    }
}

// ============================================================
// Test 7: Initializer lists
// ============================================================
class IntArray {
private:
    int* data;
    int size;
public:
    IntArray(int sz) : size(sz) {
        data = (int*)malloc(sizeof(int) * sz);
        printf("[IntArray] Constructor with size %d\n", sz);
    }
    
    IntArray(int a, int b, int c) : size(3) {
        data = (int*)malloc(sizeof(int) * 3);
        data[0] = a;
        data[1] = b;
        data[2] = c;
        printf("[IntArray] Constructor with {%d, %d, %d}\n", a, b, c);
    }
    
    ~IntArray() {
        if (data) free(data);
    }
    
    void print() {
        printf("  IntArray[%d]: ", size);
        for (int i = 0; i < size; i++) {
            printf("%d ", data[i]);
        }
        printf("\n");
    }
};

void test_initializer_list() {
    printf("\n=== TEST 7: Initializer lists ===\n");
    
    // Brace initialization
    int x{42};
    float y{3.14f};
    printf("  int x{42} = %d\n", x);
    printf("  float y{3.14f} = %.2f\n", y);
    
    // Array initialization
    int arr[]{1, 2, 3, 4, 5};
    printf("  int arr[]{1,2,3,4,5}: ");
    for (int i = 0; i < 5; i++) printf("%d ", arr[i]);
    printf("\n");
    
    // Class with initializer
    IntArray ia(10, 20, 30);
    ia.print();
}

// ============================================================
// Test 8: static_assert
// ============================================================
static_assert(sizeof(int) == 4, "int must be 4 bytes");
static_assert(sizeof(long long) == 8, "long long must be 8 bytes");
static_assert(sizeof(float) == 4, "float must be 4 bytes");
static_assert(sizeof(double) == 8, "double must be 8 bytes");

void test_static_assert() {
    printf("\n=== TEST 8: static_assert ===\n");
    
    printf("  static_assert(sizeof(int) == 4) PASSED\n");
    printf("  static_assert(sizeof(long long) == 8) PASSED\n");
    printf("  static_assert(sizeof(float) == 4) PASSED\n");
    printf("  static_assert(sizeof(double) == 8) PASSED\n");
    
    printf("  sizeof(int) = %d\n", (int)sizeof(int));
    printf("  sizeof(long long) = %d\n", (int)sizeof(long long));
    printf("  sizeof(float) = %d\n", (int)sizeof(float));
    printf("  sizeof(double) = %d\n", (int)sizeof(double));
}

// ============================================================
// Test 9: decltype
// ============================================================
void test_decltype() {
    printf("\n=== TEST 9: decltype ===\n");
    
    int x = 42;
    decltype(x) y = 100;  // y is int
    
    float f = 3.14f;
    decltype(f) g = 2.71f;  // g is float
    
    printf("  int x = 42; decltype(x) y = 100 -> y = %d\n", y);
    printf("  float f = 3.14f; decltype(f) g = 2.71f -> g = %.2f\n", g);
    
    // decltype with expression
    decltype(x + f) result = x + f;
    printf("  decltype(x + f) result = %.2f\n", result);
}

// ============================================================
// Test 10: Move semantics (simplified)
// ============================================================
class Buffer {
private:
    int* data;
    int size;
    
public:
    Buffer(int sz) : size(sz) {
        data = (int*)malloc(sizeof(int) * sz);
        printf("[Buffer] Constructor: allocated %d ints\n", sz);
    }
    
    // Move constructor
    Buffer(Buffer&& other) : data(other.data), size(other.size) {
        other.data = nullptr;
        other.size = 0;
        printf("[Buffer] Move constructor: took ownership\n");
    }
    
    ~Buffer() {
        if (data) {
            printf("[Buffer] Destructor: freeing %d ints\n", size);
            free(data);
        } else {
            printf("[Buffer] Destructor: nothing to free (moved)\n");
        }
    }
    
    int getSize() { return size; }
};

Buffer createBuffer(int size) {
    Buffer b(size);
    return b;  // Move semantics
}

void test_move_semantics() {
    printf("\n=== TEST 10: Move semantics ===\n");
    
    Buffer b1(10);
    printf("  b1 size: %d\n", b1.getSize());
    
    Buffer b2 = createBuffer(20);
    printf("  b2 size: %d\n", b2.getSize());
}

// ============================================================
// Main — Run all tests
// ============================================================
int main() {
    printf("============================================================\n");
    printf("ADead-BIB C++11 Intensive Test — Modern Features\n");
    printf("============================================================\n");
    
    test_auto();
    test_nullptr();
    test_range_for();
    test_lambda();
    test_constexpr();
    test_enum_class();
    test_initializer_list();
    test_static_assert();
    test_decltype();
    test_move_semantics();
    
    printf("\n============================================================\n");
    printf("All C++11 Feature tests completed!\n");
    printf("==============================================================\n");
    
    return 0;
}
