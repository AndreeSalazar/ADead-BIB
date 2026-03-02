#include <stdio.h>

int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
int mul(int a, int b) { return a * b; }
int my_abs(int x) { if (x < 0) return 0 - x; return x; }
int my_max(int a, int b) { if (a > b) return a; return b; }
int my_min(int a, int b) { if (a < b) return a; return b; }
int clamp(int x, int lo, int hi) { if (x < lo) return lo; if (x > hi) return hi; return x; }
int factorial(int n) { if (n <= 1) return 1; return n * factorial(n - 1); }
int array_sum(int *arr, int len) { int t = 0; int i; for (i = 0; i < len; i++) { t = t + arr[i]; } return t; }

void my_sort(int *arr, int len) {
    int i;
    int j;
    for (i = 0; i < len - 1; i++) {
        for (j = 0; j < len - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

int main() {
    printf("START\n");
    int r = add(3, 4);
    printf("add=%d\n", r);
    int s = sub(10, 3);
    printf("sub=%d\n", s);
    int m = mul(3, 4);
    printf("mul=%d\n", m);
    
    int arr[] = {3, 1, 2};
    my_sort(arr, 3);
    int v0 = arr[0];
    int v1 = arr[1];
    int v2 = arr[2];
    printf("sorted=[%d,%d,%d]\n", v0, v1, v2);
    printf("DONE\n");
    return 0;
}
