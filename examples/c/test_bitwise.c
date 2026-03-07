#include <stdio.h>
int main() { int a=0xFF; int b=a&0x0F; int c=a|0xF00; int d=a^0xFF; int e=~a; int f=a<<4; int g=a>>4; printf("%d %d %d %d %d %d\n",b,c,d,e,f,g); return 0; }