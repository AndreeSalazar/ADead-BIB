#include <cstdio>

int is_prime(int n) {
    if (n < 2) return 0;
    for (int i = 2; i * i <= n; i++) {
        if (n % i == 0) return 0;
    }
    return 1;
}

int main() {
    printf("prime(2)=%d\n", is_prime(2));
    printf("prime(17)=%d\n", is_prime(17));
    printf("prime(100)=%d\n", is_prime(100));
    return 0;
}
