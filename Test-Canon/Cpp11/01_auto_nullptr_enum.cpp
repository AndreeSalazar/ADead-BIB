// Canon C++11 -- auto, nullptr, enum class, range-for
int printf(const char *format, ...);

enum class Color { Red = 1, Green = 2, Blue = 3 };

int main() {
    printf("=== Canon C++11: auto, nullptr, enum class ===\n\n");
    int pass = 0;
    int total = 0;

    // auto type deduction
    auto x = 42;
    auto y = 100;
    printf("auto x=%d y=%d\n", x, y);
    total++; if (x == 42) { pass++; } else { printf("FAIL: auto x\n"); }
    total++; if (y == 100) { pass++; } else { printf("FAIL: auto y\n"); }

    // nullptr
    int *p = nullptr;
    total++; if (p == nullptr) { pass++; } else { printf("FAIL: nullptr\n"); }

    // enum class (scoped enum)
    Color c = Color::Blue;
    int cv = (int)c;
    printf("Color::Blue = %d\n", cv);
    total++; if (cv == 3) { pass++; } else { printf("FAIL: enum class\n"); }

    // auto with expressions
    auto sum = x + y;
    printf("auto sum = %d\n", sum);
    total++; if (sum == 142) { pass++; } else { printf("FAIL: auto sum\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}