#include <stdio.h>

int array_sum(int *arr, int len) {
    int total = 0;
    int i;
    for (i = 0; i < len; i++) {
        total = total + arr[i];
    }
    return total;
}

int array_max(int *arr, int len) {
    int max = arr[0];
    int i;
    for (i = 1; i < len; i++) {
        if (arr[i] > max) max = arr[i];
    }
    return max;
}

int main() {
    // Test 1: 5-element explicit
    int arr1[5];
    arr1[0] = 10;
    arr1[1] = 20;
    arr1[2] = 30;
    arr1[3] = 40;
    arr1[4] = 50;
    printf("Test1 sum=%d (150)\n", array_sum(arr1, 5));
    printf("Test1 max=%d (50)\n", array_max(arr1, 5));
    
    // Test 2: 10-element initializer
    int arr2[] = {5, 3, 8, 1, 9, 2, 7, 4, 6, 10};
    printf("Test2 sum=%d (55)\n", array_sum(arr2, 10));
    printf("Test2 max=%d (10)\n", array_max(arr2, 10));
    
    // Test 3: Read back initializer
    int v0 = arr2[0];
    int v4 = arr2[4];
    int v9 = arr2[9];
    printf("arr2[0]=%d(5) [4]=%d(9) [9]=%d(10)\n", v0, v4, v9);
    
    return 0;
}
