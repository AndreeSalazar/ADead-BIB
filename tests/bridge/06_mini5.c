#include <stdio.h>
#include <string.h>

typedef struct {
    int x;
    int y;
} Point;

int main() {
    printf("start\n");
    Point p1;
    p1.x = 10;
    p1.y = 20;
    printf("p1.x=%d p1.y=%d\n", p1.x, p1.y);
    return 0;
}
