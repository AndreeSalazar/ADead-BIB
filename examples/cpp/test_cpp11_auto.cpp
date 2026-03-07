#include <iostream>
auto square(int x) { return x * x; }
auto cube(int x) { return x * x * x; }
auto negate(int x) { return -x; }
auto abs_val(int x) { return x < 0 ? -x : x; }
int main() {
    auto a = 42;
    auto b = square(a);
    auto c = cube(3);
    auto d = negate(7);
    auto e = abs_val(-99);
    int *p = nullptr;
    printf("a=%d sq=%d cube=%d neg=%d abs=%d null=%d\n", a, b, c, d, e, p == nullptr);
    return 0;
}