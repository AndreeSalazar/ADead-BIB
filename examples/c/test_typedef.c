#include <stdio.h>
typedef int Integer;
typedef struct { int x; int y; } Point;
Integer add(Integer a, Integer b) { return a + b; }
int main() { Integer x = add(3,4); Point p; p.x=1; p.y=2; printf("x=%d p=(%d,%d)\n",x,p.x,p.y); return 0; }