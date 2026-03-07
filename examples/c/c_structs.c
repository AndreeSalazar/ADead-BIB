#include <stdio.h>
struct Point { int x; int y; };
struct Rect { struct Point tl; struct Point br; };
int area(struct Rect r) { return (r.br.x - r.tl.x) * (r.br.y - r.tl.y); }
struct Point make_point(int x, int y) { struct Point p; p.x = x; p.y = y; return p; }
int main() { struct Rect r; r.tl = make_point(0, 0); r.br = make_point(10, 5); printf("area=%d\n", area(r)); return 0; }