#include <stdio.h>
#include <stdlib.h>
void fill_array(int *arr, int n) {
    for (int i = 0; i < n; i++) arr[i] = i * i;
}
int sum_array(int *arr, int n) {
    int s = 0;
    for (int i = 0; i < n; i++) s += arr[i];
    return s;
}
int main() {
    int n = 10;
    int *arr = (int*)malloc(n * sizeof(int));
    fill_array(arr, n);
    printf("sum of squares(0..9) = %d\n", sum_array(arr, n));
    free(arr);
    int fixed[5];
    for (int i = 0; i < 5; i++) fixed[i] = i + 1;
    printf("fixed sum = %d\n", sum_array(fixed, 5));
    return 0;
}