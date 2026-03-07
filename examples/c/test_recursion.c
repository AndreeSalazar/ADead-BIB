#include <stdio.h>
int fibonacci(int n) { if(n<=1) return n; return fibonacci(n-1)+fibonacci(n-2); }
int power(int b, int e) { if(e==0) return 1; return b*power(b,e-1); }
int factorial(int n) { if(n<=1) return 1; return n*factorial(n-1); }
int main() { printf("fib=%d pow=%d fact=%d\n", fibonacci(10), power(2,8), factorial(6)); return 0; }