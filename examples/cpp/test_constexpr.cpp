#include <iostream>
constexpr int square(int x) { return x * x; }
constexpr int cube(int x) { return x * x * x; }
constexpr int add(int a, int b) { return a + b; }
constexpr int factorial(int n) { return n <= 1 ? 1 : n * factorial(n - 1); }
int main() { printf("%d %d %d %d\n", square(5), cube(3), add(10,20), factorial(6)); return 0; }