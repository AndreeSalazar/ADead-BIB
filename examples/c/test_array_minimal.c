#include <stdio.h>

int main() {
    int arr[5];
    arr[0] = 10;
    arr[1] = 20;
    arr[2] = 30;
    arr[3] = 40;
    arr[4] = 50;
    
    // Verify all values
    int v0 = arr[0];
    int v1 = arr[1];
    int v2 = arr[2];
    int v3 = arr[3];
    int v4 = arr[4];
    
    printf("arr = [%d, %d, %d, %d, %d]\n", v0, v1, v2, v3, v4);
    
    int sum = v0 + v1 + v2 + v3 + v4;
    printf("Sum = %d (expected 150)\n", sum);
    
    return 0;
}
