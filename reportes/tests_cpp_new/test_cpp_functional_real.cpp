#include <header_main.h>
#include <functional>

int double_it(int x) { return x * 2; }
int triple_it(int x) { return x * 3; }
int square_it(int x) { return x * x; }
int negate_it(int x) { return -x; }

int main() {
    // Direct function calls — type erasure concept
    printf("double(5): %d\n", double_it(5));
    printf("triple(5): %d\n", triple_it(5));
    printf("square(5): %d\n", square_it(5));
    printf("negate(5): %d\n", negate_it(5));

    // Lambda (auto-typed) — compiler resolves at compile time
    auto f1 = [](int x) { return x * x; };
    printf("lambda square(6): %d\n", f1(6));

    auto f2 = [](int a, int b) { return a + b; };
    printf("lambda add(3,4): %d\n", f2(3, 4));

    // Lambda with capture
    int factor = 4;
    auto f3 = [factor](int x) { return x * factor; };
    printf("capture lambda(5): %d\n", f3(5));

    // Composing results
    int r1 = double_it(10);
    int r2 = triple_it(10);
    int r3 = square_it(10);
    int r4 = negate_it(10);
    printf("double(10)=%d triple(10)=%d square(10)=%d negate(10)=%d\n", r1, r2, r3, r4);

    // Chained calls
    printf("double(triple(2)): %d\n", double_it(triple_it(2)));
    printf("square(double(3)): %d\n", square_it(double_it(3)));

    return 0;
}
