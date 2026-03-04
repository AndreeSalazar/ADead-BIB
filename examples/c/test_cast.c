#include <stdio.h>

int main() {
    int x = 65;
    char c = (char)x;
    printf("x=%d c=%c\n", x, c);
    
    int a = 7;
    int b = 2;
    int result = a / b;
    printf("7/2=%d\n", result);
    
    unsigned int u = 0xFFFFFFFF;
    int s = (int)u;
    printf("u=%u s=%d\n", u, s);
    return 0;
}
