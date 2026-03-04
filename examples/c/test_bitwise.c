#include <stdio.h>

int main() {
    int a = 0xAA;
    int b = 0x55;
    
    printf("a & b = 0x%x\n", a & b);
    printf("a | b = 0x%x\n", a | b);
    printf("a ^ b = 0x%x\n", a ^ b);
    printf("~a = 0x%x\n", ~a & 0xFF);
    printf("a << 4 = 0x%x\n", a << 4);
    printf("a >> 4 = 0x%x\n", a >> 4);
    
    unsigned int flags = 0;
    flags = flags | (1 << 0);
    flags = flags | (1 << 3);
    flags = flags | (1 << 7);
    printf("flags = 0x%x\n", flags);
    
    int bit3 = (flags >> 3) & 1;
    int bit4 = (flags >> 4) & 1;
    printf("bit3=%d bit4=%d\n", bit3, bit4);
    return 0;
}
