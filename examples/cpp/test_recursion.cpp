#include <iostream>
int power(int base, int exp) { if (exp == 0) return 1; return base * power(base, exp - 1); }
int sum_to(int n) { if (n <= 0) return 0; return n + sum_to(n - 1); }
int main() { printf("pow=%d sum=%d\n", power(2, 10), sum_to(100)); return 0; }