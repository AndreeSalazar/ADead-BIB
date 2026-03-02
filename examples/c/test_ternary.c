#include <stdio.h>

int safe_div(int a, int b) {
    return (b != 0) ? a / b : 0;
}

int safe_mod(int a, int b) {
    return (b != 0) ? a % b : 0;
}

int max2(int a, int b) {
    return (a > b) ? a : b;
}

int min2(int a, int b) {
    return (a < b) ? a : b;
}

int main() {
    printf("=== Ternary Test ===\n");
    printf("safe_div(100,4) = %d\n", safe_div(100, 4));
    printf("safe_div(10,0) = %d\n", safe_div(10, 0));
    printf("safe_mod(17,5) = %d\n", safe_mod(17, 5));
    printf("max2(3,7) = %d\n", max2(3, 7));
    printf("min2(3,7) = %d\n", min2(3, 7));

    int x = 10;
    int y = 20;
    int z = (x < y) ? x : y;
    printf("min(%d,%d) = %d\n", x, y, z);
    return 0;
}
