#include <iostream>
auto double_it(int x) { return x * 2; }
auto negate(int x) { return -x; }
auto is_positive(int x) { return x > 0; }
int main() { auto a = 42; auto b = double_it(a); auto c = negate(b); int *p = nullptr;
printf("a=%d b=%d c=%d\n", a, b, c); return 0; }