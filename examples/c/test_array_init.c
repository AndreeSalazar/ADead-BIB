#include <stdio.h>

int sum(int *arr, int n) {
    int total = 0;
    for (int i = 0; i < n; i++) {
        total += arr[i];
    }
    return total;
}

int main() {
    int a[] = {10, 20, 30, 40, 50};
    printf("sum=%d\n", sum(a, 5));
    
    int b[5];
    for (int i = 0; i < 5; i++) {
        b[i] = (i + 1) * 10;
    }
    printf("b[0]=%d b[4]=%d\n", b[0], b[4]);
    
    int matrix[3];
    matrix[0] = 1;
    matrix[1] = 2;
    matrix[2] = 3;
    printf("matrix sum=%d\n", matrix[0] + matrix[1] + matrix[2]);
    return 0;
}
