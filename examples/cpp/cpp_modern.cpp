#include <iostream>
auto add(int a, int b) { return a + b; }
int main() { auto x = 42; auto r = add(10, 20); int *p = nullptr;
printf("x=%d r=%d\n", x, r); if (p == nullptr) printf("null ok\n"); return 0; }