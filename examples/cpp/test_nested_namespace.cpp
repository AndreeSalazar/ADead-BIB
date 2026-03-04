int printf(const char *format, ...);

namespace outer {
    int value = 100;

    namespace inner {
        int compute(int x) { return x * 2; }
    }

    int process(int x) {
        return inner::compute(x) + value;
    }
}

namespace math {
    int add(int a, int b) { return a + b; }
    int sub(int a, int b) { return a - b; }
    int mul(int a, int b) { return a * b; }

    int sum_range(int n) {
        int total = 0;
        for (int i = 1; i <= n; i++) {
            total = add(total, i);
        }
        return total;
    }
}

int main() {
    printf("outer::process(5) = %d\n", outer::process(5));
    printf("math::add(3,4) = %d\n", math::add(3, 4));
    printf("math::sum_range(10) = %d\n", math::sum_range(10));
    return 0;
}
