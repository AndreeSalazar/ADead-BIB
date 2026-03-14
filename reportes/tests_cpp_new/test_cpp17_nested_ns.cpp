#include <header_main.h>

// C++17 nested namespace declarations
namespace A::B::C {
    int value = 42;
    int get_value() { return value; }
}

namespace X::Y {
    int compute(int a, int b) { return a + b; }
}

// Traditional nested namespaces (should also work)
namespace Outer {
    namespace Inner {
        int multiply(int a, int b) { return a * b; }
    }
}

int main() {
    printf("A::B::C::value = %d\n", A::B::C::get_value());
    printf("X::Y::compute(3,4) = %d\n", X::Y::compute(3, 4));
    printf("Outer::Inner::multiply(5,6) = %d\n", Outer::Inner::multiply(5, 6));
    return 0;
}
