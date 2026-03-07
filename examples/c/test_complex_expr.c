#include <stdio.h>
int main() { int a=2,b=3,c=4,d=5; int r1=(a+b)*(c-d)+a*b/(c+1); int r2=a>b?a:b; int r3=(a<b)&&(c>d)||(a==2); printf("%d %d %d\n",r1,r2,r3); return 0; }