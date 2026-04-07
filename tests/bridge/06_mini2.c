#include <stdio.h>

typedef struct {
    int x;
    int y;
} Point;

int get_x(Point p) {
    return p.x;
}

int main() {
    printf("start\n");
    Point p1;
    p1.x = 42;
    p1.y = 99;
    int val = get_x(p1);
    printf("val=%d\n", val);
    return 0;
}
