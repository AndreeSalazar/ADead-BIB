#include <iostream>
namespace Math { int add(int a, int b) { return a + b; } int sub(int a, int b) { return a - b; } }
namespace Utils { int abs_val(int x) { return x < 0 ? -x : x; } int max_val(int a, int b) { return a > b ? a : b; } }
int main() { printf("%d %d %d %d\n", Math::add(3,4), Math::sub(10,3), Utils::abs_val(-5), Utils::max_val(7,3)); return 0; }