#include <iostream>
class Adder {
public:
    int base;
    Adder(int b) : base(b) {}
    int apply(int x) { return base + x; }
};
class Multiplier {
public:
    int factor;
    Multiplier(int f) : factor(f) {}
    int apply(int x) { return factor * x; }
};
class Negator {
public:
    int apply(int x) { return -x; }
};
int compose(int x, int a, int b) { return a + b + x; }
int main() {
    Adder add10(10);
    Multiplier mul3(3);
    Negator neg;
    printf("add10(5) = %d\n", add10.apply(5));
    printf("mul3(7) = %d\n", mul3.apply(7));
    printf("neg(42) = %d\n", neg.apply(42));
    printf("compose(1,2,3) = %d\n", compose(1, 2, 3));
    return 0;
}