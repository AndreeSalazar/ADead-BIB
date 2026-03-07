// Canon C++11 -- Variadic templates, static_assert, type aliases
int printf(const char *format, ...);

// using type alias (C++11)
using Integer = int;
using UInt = unsigned int;

// Class template with multiple params
template<typename T, typename U>
class Pair {
public:
    T first;
    U second;
    Pair(T f, U s) : first(f), second(s) {}
    T getFirst() { return first; }
    U getSecond() { return second; }
};

// Template function with auto return deduced
template<typename T>
T identity(T val) { return val; }

template<typename T>
T square(T x) { return x * x; }

int main() {
    printf("=== Canon C++11: Type Aliases + Templates ===\n\n");
    int pass = 0;
    int total = 0;

    // using aliases
    Integer a = 42;
    UInt b = 100;
    printf("Integer a=%d UInt b=%d\n", a, (int)b);
    total++; if (a == 42) { pass++; } else { printf("FAIL: Integer\n"); }
    total++; if (b == 100) { pass++; } else { printf("FAIL: UInt\n"); }

    // Pair template
    Pair<int, int> p(10, 20);
    printf("Pair<%s,%s>(%d,%d)\n", "int", "int", p.getFirst(), p.getSecond());
    total++; if (p.getFirst() == 10) { pass++; } else { printf("FAIL: pair.first\n"); }
    total++; if (p.getSecond() == 20) { pass++; } else { printf("FAIL: pair.second\n"); }

    // identity
    int id_val = identity(77);
    printf("identity(77) = %d\n", id_val);
    total++; if (id_val == 77) { pass++; } else { printf("FAIL: identity\n"); }

    // square
    int sq = square(9);
    printf("square(9) = %d\n", sq);
    total++; if (sq == 81) { pass++; } else { printf("FAIL: square\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}