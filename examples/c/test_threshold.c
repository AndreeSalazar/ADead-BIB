#include <stdio.h>

int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
int mul(int a, int b) { return a * b; }
int my_abs(int x) { if (x < 0) return 0 - x; return x; }
int my_max(int a, int b) { if (a > b) return a; return b; }

int main() {
    printf("START\n");
    int r1 = add(3, 4);
    int r2 = sub(10, 3);
    int r3 = mul(3, 4);
    int r4 = my_abs(-5);
    int r5 = my_max(3, 7);
    printf("add=%d sub=%d mul=%d abs=%d max=%d\n", r1, r2, r3, r4, r5);
    printf("DONE\n");
    return 0;
}
