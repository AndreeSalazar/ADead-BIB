#include <stdio.h>

typedef union {
    int i;
    float f;
    char bytes[4];
} Value;

int main() {
    printf("start\n");
    Value v;
    printf("after decl\n");
    v.i = 0x41424344;
    printf("after assign\n");
    printf("done\n");
    return 0;
}
