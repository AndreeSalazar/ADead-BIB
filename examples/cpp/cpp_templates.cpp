#include <iostream>
template<typename T>
T max_val(T a, T b) { return (a > b) ? a : b; }
template<typename T>
T min_val(T a, T b) { return (a < b) ? a : b; }
template<typename T, int N>
struct Array { T data[N]; int size() { return N; } };
template<typename T>
T identity(T x) { return x; }
int main() { int a = max_val(3, 7); int b = min_val(3, 7); printf("max=%d min=%d\n", a, b); return 0; }