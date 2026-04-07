#include <stdio.h>

typedef struct {
    int x;
    int y;
} Point;

Point point_add(Point a, Point b) {
    Point r;
    r.x = a.x + b.x;
    r.y = a.y + b.y;
    return r;
}

int main() {
    printf("start\n");
    Point p1;
    p1.x = 10;
    p1.y = 20;
    Point p2;
    p2.x = 30;
    p2.y = 40;
    printf("before call\n");
    Point sum = point_add(p1, p2);
    printf("after call\n");
    printf("sum.x=%d sum.y=%d\n", sum.x, sum.y);
    return 0;
}
