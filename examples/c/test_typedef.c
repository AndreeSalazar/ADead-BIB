#include <stdio.h>

typedef int i32;
typedef unsigned int u32;
typedef long long i64;
typedef unsigned char byte;

typedef struct {
    i32 x;
    i32 y;
} Point;

i32 add(i32 a, i32 b) {
    return a + b;
}

int main() {
    i32 x = 42;
    u32 y = 100;
    byte b = 255;
    printf("x=%d y=%d b=%d\n", x, y, b);
    printf("add=%d\n", add(x, 10));
    return 0;
}
