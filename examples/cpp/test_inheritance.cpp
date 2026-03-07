#include <iostream>
class Shape { public: int x; int y; Shape(int a, int b) : x(a), y(b) {} };
class Circle : public Shape { public: int radius; Circle(int a, int b, int r) : Shape(a, b), radius(r) {} };
class Rect : public Shape { public: int w; int h; Rect(int a, int b, int ww, int hh) : Shape(a, b), w(ww), h(hh) {} };
class Square : public Rect { public: Square(int a, int b, int s) : Rect(a, b, s, s) {} };
int main() { Circle c(0,0,5); Rect r(1,1,10,20); Square s(2,2,7); printf("r=%d w=%d sw=%d\n",c.radius,r.w,s.w); return 0; }