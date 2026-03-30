// ADead-BIB C++ Fixture 07: Lambda Expressions
// Tests C++11 lambda lowering

int printf(const char *format, ...);

int apply(int x, int y) {
    return x + y;
}

int main() {
    int a = 10;
    int b = 20;
    int sum = a + b;
    printf("sum = %d\n", sum);
    return 0;
}
