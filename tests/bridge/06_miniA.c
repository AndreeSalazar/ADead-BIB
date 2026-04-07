#include <stdio.h>

typedef union {
    int i;
    float f;
    char bytes[4];
} Value;

int main() {
    printf("start\n");
    Value v;
    v.i = 0x41424344;
    printf("v.i=%d\n", v.i);
    printf("v.bytes[0]=%d\n", v.bytes[0]);
    if (v.bytes[0] == 0x44) { printf("PASS\n"); } else { printf("FAIL\n"); }
    printf("done\n");
    return 0;
}
