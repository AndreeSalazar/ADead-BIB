#include <stdio.h>

struct Point {
    int x;
    int y;
};

struct Rect {
    struct Point origin;
    int width;
    int height;
};

int area(int w, int h) {
    return w * h;
}

int main() {
    struct Point p;
    p.x = 10;
    p.y = 20;
    printf("point=(%d,%d)\n", p.x, p.y);
    
    struct Rect r;
    r.width = 100;
    r.height = 50;
    printf("rect=%dx%d area=%d\n", r.width, r.height, area(r.width, r.height));
    return 0;
}
