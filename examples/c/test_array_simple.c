#include <stdio.h>

int main() {
    int arr[5];
    arr[0] = 10;
    arr[1] = 20;
    arr[2] = 30;
    
    printf("arr[0] = %d\n", arr[0]);
    printf("arr[1] = %d\n", arr[1]);
    printf("arr[2] = %d\n", arr[2]);
    
    int sum = arr[0] + arr[1] + arr[2];
    printf("sum = %d\n", sum);
    
    return 0;
}
