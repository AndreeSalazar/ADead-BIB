// ADead-BIB C++ Fixture 14: extern "C" and Forward Declarations
// Tests extern C block lowering and forward declaration emission

int printf(const char *format, ...);

extern "C" {
    int abs(int x);
    int atoi(const char *str);
}

int helper(int x);

int helper(int x) {
    return x * 2;
}

int main() {
    int val = helper(21);
    printf("helper(21) = %d\n", val);
    return 0;
}
