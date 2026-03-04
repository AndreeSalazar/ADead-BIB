#include <stdio.h>

void increment(int *p) {
    *p = *p + 1;
}

int main() {
    int x = 10;
    int *p = &x;
    printf("x=%d *p=%d\n", x, *p);
    increment(&x);
    printf("after inc: x=%d\n", x);
    
    int arr[3];
    arr[0] = 100;
    arr[1] = 200;
    arr[2] = 300;
    int *q = arr;
    printf("arr[0]=%d arr[1]=%d arr[2]=%d\n", arr[0], arr[1], arr[2]);
    return 0;
}
