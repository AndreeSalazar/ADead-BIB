#include <stdio.h>

int main() {
    int x = 100;
    x += 50;
    printf("+=50: %d\n", x);
    x -= 30;
    printf("-=30: %d\n", x);
    x *= 2;
    printf("*=2: %d\n", x);
    x /= 3;
    printf("/=3: %d\n", x);
    x %= 7;
    printf("%%=7: %d\n", x);
    
    int y = 0xFF;
    y &= 0x0F;
    printf("&=0F: %d\n", y);
    y |= 0xF0;
    printf("|=F0: %d\n", y);
    y ^= 0xFF;
    printf("^=FF: %d\n", y);
    
    int z = 1;
    z <<= 4;
    printf("<<=4: %d\n", z);
    z >>= 2;
    printf(">>=2: %d\n", z);
    return 0;
}
