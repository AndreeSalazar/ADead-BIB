#include <header_main.h>

namespace Math {
    int add(int a, int b) { return a + b; }
    int multiply(int a, int b) { return a * b; }
    double pi() { return 3.14159; }
}

class Counter {
    int value;
public:
    Counter() : value(0) {}
    Counter(int v) : value(v) {}
    void increment() { value++; }
    void decrement() { value--; }
    int get() const { return value; }
    void reset() { value = 0; }
};

namespace Utils {
    namespace StringOps {
        int length(const char* s) {
            int n = 0;
            while (s[n]) n++;
            return n;
        }
    }
}

int main() {
    // Namespace functions
    int r = Math::add(3, 4);
    printf("add: %d\n", r);
    printf("mul: %d\n", Math::multiply(5, 6));

    // Class methods
    Counter c;
    c.increment();
    c.increment();
    c.increment();
    printf("counter: %d\n", c.get());
    c.decrement();
    printf("after dec: %d\n", c.get());
    c.reset();
    printf("after reset: %d\n", c.get());

    // Nested namespace
    printf("strlen: %d\n", Utils::StringOps::length("hello"));

    // Constructor with args
    Counter c2(100);
    printf("c2: %d\n", c2.get());

    return 0;
}
