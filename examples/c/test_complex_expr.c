#include <stdio.h>

int main() {
    int a = 2 + 3 * 4;
    printf("2+3*4=%d\n", a);
    
    int b = (2 + 3) * 4;
    printf("(2+3)*4=%d\n", b);
    
    int c = 10 - 2 - 3;
    printf("10-2-3=%d\n", c);
    
    int d = 100 / 10 / 2;
    printf("100/10/2=%d\n", d);
    
    int e = 1 + 2 * 3 + 4 * 5 + 6;
    printf("1+2*3+4*5+6=%d\n", e);
    
    int f = (1 < 2) && (3 > 2);
    printf("(1<2)&&(3>2)=%d\n", f);
    
    int g = (1 > 2) || (3 > 2);
    printf("(1>2)||(3>2)=%d\n", g);
    
    int h = !(0);
    printf("!(0)=%d\n", h);
    
    int x = 5;
    int y = (x > 3) ? x * 2 : x + 1;
    printf("ternary=%d\n", y);
    return 0;
}
