#include <stdio.h>
static int max_int(int a, int b) { return a > b ? a : b; }
static int min_int(int a, int b) { return a < b ? a : b; }
static int clamp(int x, int lo, int hi) { return min_int(max_int(x, lo), hi); }
static int abs_int(int x) { return x < 0 ? -x : x; }
static int square(int x) { return x * x; }
static int is_even(int x) { return (x & 1) == 0; }
int main() {
    printf("max(3,7)=%d\n", max_int(3, 7));
    printf("min(3,7)=%d\n", min_int(3, 7));
    printf("clamp(15,0,10)=%d\n", clamp(15, 0, 10));
    printf("clamp(-5,0,10)=%d\n", clamp(-5, 0, 10));
    printf("abs(-42)=%d\n", abs_int(-42));
    printf("sq(9)=%d\n", square(9));
    printf("even(4)=%d even(7)=%d\n", is_even(4), is_even(7));
    return 0;
}