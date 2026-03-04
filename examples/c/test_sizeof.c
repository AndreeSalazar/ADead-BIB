#include <stdio.h>

struct Pair { int a; int b; };

int main() {
    printf("sizeof(int)=%d\n", (int)sizeof(int));
    printf("sizeof(char)=%d\n", (int)sizeof(char));
    printf("sizeof(long)=%d\n", (int)sizeof(long));
    
    int arr[10];
    printf("sizeof(arr)=%d\n", (int)sizeof(arr));
    
    int x = 42;
    printf("sizeof(x)=%d\n", (int)sizeof(x));
    return 0;
}
