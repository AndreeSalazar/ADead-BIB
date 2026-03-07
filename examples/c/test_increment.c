#include <stdio.h>
int main() { int a=5; a++; ++a; a--; --a; int b=a++; int c=++a; printf("a=%d b=%d c=%d\n",a,b,c); return 0; }