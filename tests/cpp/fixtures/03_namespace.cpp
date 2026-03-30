// ADead-BIB C++ Fixture 03: Namespace
// Tests namespace definition and scoped functions

int printf(const char *format, ...);

namespace math {
    int square(int x) {
        return x * x;
    }

    int cube(int x) {
        return x * x * x;
    }
}

int main() {
    int a = 3;
    printf("square(3) = computed\n");
    printf("cube(3) = computed\n");
    return 0;
}
