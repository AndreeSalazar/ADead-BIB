#include <iostream>
template<typename T> T add(T a, T b) { return a + b; }
template<typename T> T sub(T a, T b) { return a - b; }
template<typename T> T mul(T a, T b) { return a * b; }
template<typename T> T identity(T x) { return x; }
int main() { printf("%d %d %d %d\n", add(1,2), sub(5,3), mul(3,4), identity(42)); return 0; }