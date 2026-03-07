#include <stdio.h>
int main() {
    char c = 'A';
    short s = 1000;
    int i = 100000;
    long l = 1000000;
    long long ll = 1000000000;
    unsigned char uc = 255;
    unsigned short us = 65535;
    unsigned int ui = 4000000000;
    float f = 3.14;
    double d = 3.141592653589793;
    _Bool b = 1;
    printf("c=%c s=%d i=%d l=%ld ll=%lld\n", c, s, i, l, ll);
    printf("uc=%u us=%u ui=%u\n", uc, us, ui);
    printf("f=%.2f d=%.15f b=%d\n", f, d, b);
    printf("sizeof: char=%d short=%d int=%d long=%d ll=%d\n",
        (int)sizeof(char), (int)sizeof(short), (int)sizeof(int),
        (int)sizeof(long), (int)sizeof(long long));
    printf("sizeof: float=%d double=%d\n", (int)sizeof(float), (int)sizeof(double));
    return 0;
}