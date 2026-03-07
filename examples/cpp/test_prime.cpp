#include <iostream>
int is_prime(int n) { if (n < 2) return 0; for (int i = 2; i * i <= n; i++) if (n % i == 0) return 0; return 1; }
int next_prime(int n) { n++; while (!is_prime(n)) n++; return n; }
int main() { printf("prime7=%d next10=%d\n", is_prime(7), next_prime(10)); return 0; }