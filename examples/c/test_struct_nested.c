#include <stdio.h>
struct Point { int x; int y; };
struct Rect { struct Point tl; struct Point br; };
int main() { struct Point p; p.x=1; p.y=2; struct Rect r; r.tl=p; r.br.x=10; r.br.y=20; printf("(%d,%d)-(%d,%d)\n",r.tl.x,r.tl.y,r.br.x,r.br.y); return 0; }