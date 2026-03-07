#include <iostream>
int classify(int n) { if (n < 0) return -1; if (n == 0) return 0; return 1; }
int factorial(int n) { if (n <= 1) return 1; return n * factorial(n - 1); }
int fib(int n) { if (n <= 1) return n; int a=0,b=1; for(int i=2;i<=n;i++){int t=a+b;a=b;b=t;} return b; }
int main() { printf("%d %d %d %d\n", classify(-5), classify(0), factorial(6), fib(10)); return 0; }