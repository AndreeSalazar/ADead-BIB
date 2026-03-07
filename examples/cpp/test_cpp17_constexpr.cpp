#include <iostream>
constexpr int factorial(int n) { return n <= 1 ? 1 : n * factorial(n - 1); }
constexpr int fib(int n) { return n <= 1 ? n : fib(n - 1) + fib(n - 2); }
constexpr int square(int x) { return x * x; }
constexpr int cube(int x) { return x * x * x; }
constexpr int power(int base, int exp) { return exp == 0 ? 1 : base * power(base, exp - 1); }
constexpr int gcd(int a, int b) { return b == 0 ? a : gcd(b, a % b); }
constexpr int abs_val(int x) { return x < 0 ? -x : x; }
constexpr int max_val(int a, int b) { return a > b ? a : b; }
int main() {
    printf("fact(6)=%d fib(10)=%d\n", factorial(6), fib(10));
    printf("sq(9)=%d cube(3)=%d\n", square(9), cube(3));
    printf("pow(2,10)=%d gcd(48,18)=%d\n", power(2, 10), gcd(48, 18));
    printf("abs(-42)=%d max(7,3)=%d\n", abs_val(-42), max_val(7, 3));
    return 0;
}