#include <stdio.h>
struct Pixel { unsigned char r, g, b, a; };
int main() {
    printf("sizeof(Pixel)=%d\n", (int)sizeof(struct Pixel));
    return 0;
}
