// ADead-BIB C++ Example — Templates & Generic Programming
// Compilar: adB cxx cpp_templates.cpp -o templates.exe
//
// Demuestra: function templates, class templates, template specialization
// ADead-BIB solo instancia templates usados — dead code elimination

int printf(const char *format, ...);

// Function template — generic add
template<typename T>
T add(T a, T b) {
    return a + b;
}

// Function template — generic max
template<typename T>
T max_val(T a, T b) {
    if (a > b) {
        return a;
    }
    return b;
}

// Function template — swap
template<typename T>
void swap_vals(T* a, T* b) {
    T temp = *a;
    *a = *b;
    *b = temp;
}

// Class template — Pair
template<typename T1, typename T2>
class Pair {
public:
    T1 first;
    T2 second;

    Pair(T1 f, T2 s) : first(f), second(s) {}

    T1 getFirst() { return first; }
    T2 getSecond() { return second; }
};

// Class template — Stack (fixed size)
template<typename T, int N = 16>
class Stack {
public:
    int top;

    Stack() : top(-1) {}

    void push(T val) {
        top = top + 1;
    }

    bool empty() {
        return top < 0;
    }

    int size() {
        return top + 1;
    }
};

// Namespace with templates
namespace math {
    template<typename T>
    T square(T x) {
        return x * x;
    }

    template<typename T>
    T cube(T x) {
        return x * x * x;
    }

    int factorial(int n) {
        if (n <= 1) return 1;
        return n * factorial(n - 1);
    }
}

int main() {
    printf("=== ADead-BIB C++ Templates Demo ===\n\n");

    // Function templates
    int sum_i = add(10, 20);
    printf("add<int>(10, 20) = %d\n", sum_i);

    int m = max_val(42, 17);
    printf("max_val(42, 17) = %d\n", m);

    // Swap
    int x = 100;
    int y = 200;
    printf("Before swap: x=%d, y=%d\n", x, y);
    swap_vals(&x, &y);
    printf("After swap:  x=%d, y=%d\n", x, y);

    // Class templates
    Pair<int, int> p(10, 20);
    printf("\nPair: (%d, %d)\n", p.getFirst(), p.getSecond());

    // Stack
    Stack<int> s;
    printf("Stack empty: %d\n", s.empty());
    s.push(42);
    s.push(17);
    printf("Stack size: %d\n", s.size());

    // Namespace functions
    int sq = math::square(7);
    int cb = math::cube(3);
    int fact = math::factorial(5);
    printf("\nmath::square(7) = %d\n", sq);
    printf("math::cube(3) = %d\n", cb);
    printf("math::factorial(5) = %d\n", fact);

    printf("\n=== Done! ===\n");
    return 0;
}
