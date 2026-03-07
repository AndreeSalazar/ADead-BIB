#include <iostream>
template<typename T> T add(T a, T b) { return a + b; }
template<typename T> T mul(T a, T b) { return a * b; }
template<typename T> T identity(T x) { return x; }
template<typename T, typename U> T convert(U x) { return (T)x; }
auto double_it(int x) { return x * 2; }
auto triple_it(int x) { return x * 3; }
int main() {
    printf("add=%d mul=%d id=%d\n", add(3, 4), mul(5, 6), identity(99));
    printf("dbl=%d trp=%d\n", double_it(7), triple_it(7));
    return 0;
}