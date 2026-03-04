int printf(const char *format, ...);

auto add(int a, int b) { return a + b; }
auto multiply(int a, int b) { return a * b; }

int main() {
    auto x = 42;
    auto y = 10;
    auto sum = add(x, y);
    printf("x=%d y=%d sum=%d\n", x, y, sum);

    int *ptr = nullptr;
    if (ptr == nullptr) {
        printf("ptr is null\n");
    }

    auto prod = multiply(6, 7);
    printf("6*7=%d\n", prod);
    return 0;
}
