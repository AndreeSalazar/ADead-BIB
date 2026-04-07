#include <stdio.h>

typedef union {
    int i;
    char bytes[4];
} Value;

int main() {
    printf("start\n");
    Value v;
    v.i = 0x41424344;
    printf("v.i=%d\n", v.i);
    return 0;
}
