// C++ Library for Python FFI
// Compila con: g++ -shared -o cpp_lib.dll -fPIC cpp_lib.cpp

#include <cstdint>

extern "C" {

__declspec(dllexport) int64_t count_to(int64_t limit) {
    int64_t counter = 0;
    while (counter < limit) {
        counter++;
    }
    return counter;
}

__declspec(dllexport) int64_t count_billion() {
    return count_to(1000000000);
}

__declspec(dllexport) int64_t fibonacci(int64_t n) {
    if (n <= 1) return n;
    int64_t a = 0, b = 1;
    for (int64_t i = 2; i <= n; i++) {
        int64_t temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

__declspec(dllexport) int64_t factorial(int64_t n) {
    int64_t result = 1;
    for (int64_t i = 1; i <= n; i++) {
        result *= i;
    }
    return result;
}

__declspec(dllexport) int64_t multiply(int64_t a, int64_t b) {
    return a * b;
}

__declspec(dllexport) int64_t power(int64_t base, int64_t exp) {
    int64_t result = 1;
    for (int64_t i = 0; i < exp; i++) {
        result *= base;
    }
    return result;
}

}
