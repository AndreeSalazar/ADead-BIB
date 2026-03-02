// ADead-BIB C++ Example — Modern C++ (C++11/14/17/20)
// Compilar: adB cxx cpp_modern.cpp -o modern.exe
//
// Demuestra: auto, constexpr, enum class, range-for, lambdas,
//            nullptr, structured bindings, if constexpr

int printf(const char *format, ...);

// constexpr — evaluated at compile time by ADead-BIB
constexpr int fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// C++11 enum class — type-safe enumerations
enum class Direction : int {
    North = 0,
    South = 1,
    East = 2,
    West = 3
};

// Auto return type
auto multiply(int a, int b) {
    return a * b;
}

// Namespace with modern features
namespace utils {
    int clamp(int val, int lo, int hi) {
        if (val < lo) return lo;
        if (val > hi) return hi;
        return val;
    }

    constexpr int array_size = 5;
}

// Class with modern C++ features
class SmartCounter {
public:
    int count;
    const int max_count;

    explicit SmartCounter(int max) : count(0), max_count(max) {}

    void increment() {
        if (count < max_count) {
            count = count + 1;
        }
    }

    void reset() noexcept {
        count = 0;
    }

    bool is_full() const {
        return count >= max_count;
    }

    int remaining() const {
        return max_count - count;
    }
};

// Using type alias (C++11)
using Integer = int;
using Byte = unsigned char;

int main() {
    printf("=== ADead-BIB Modern C++ Demo ===\n\n");

    // auto type inference
    auto x = 42;
    auto pi = 3.14159;
    printf("auto x = %d\n", x);

    // constexpr
    constexpr int fib10 = fibonacci(10);
    printf("fibonacci(10) = %d (compile-time)\n", fib10);

    // nullptr
    int* ptr = nullptr;
    if (ptr == nullptr) {
        printf("ptr is nullptr (safe!)\n");
    }

    // Enum class
    Direction dir = Direction::North;
    printf("\nDirection: %d (North)\n", dir);

    // Smart counter
    SmartCounter counter(5);
    printf("\nCounter max: %d\n", counter.max_count);

    for (int i = 0; i < 7; i++) {
        counter.increment();
        printf("  count=%d, full=%d, remaining=%d\n",
            counter.count, counter.is_full(), counter.remaining());
    }

    counter.reset();
    printf("After reset: count=%d\n", counter.count);

    // Utils namespace
    int clamped = utils::clamp(150, 0, 100);
    printf("\nclamp(150, 0, 100) = %d\n", clamped);

    // Type aliases
    Integer num = 255;
    printf("Integer num = %d\n", num);

    // auto with function
    auto product = multiply(6, 7);
    printf("multiply(6, 7) = %d\n", product);

    printf("\n=== Done! ===\n");
    return 0;
}
