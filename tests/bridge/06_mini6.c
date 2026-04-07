#include <stdio.h>
#include <string.h>

typedef struct {
    int x;
    int y;
} Point;

typedef struct {
    Point origin;
    int width;
    int height;
} Rect;

typedef enum {
    COLOR_RED = 0,
    COLOR_GREEN = 1,
    COLOR_BLUE = 2,
    COLOR_COUNT
} Color;

typedef union {
    int i;
    float f;
    char bytes[4];
} Value;

Point point_add(Point a, Point b) {
    Point r;
    r.x = a.x + b.x;
    r.y = a.y + b.y;
    return r;
}

int rect_area(Rect r) {
    return r.width * r.height;
}

int main() {
    printf("=== Test 06 ===\n");
    int pass = 0, fail = 0;

    Point p1;
    p1.x = 10;
    p1.y = 20;
    if (p1.x == 10 && p1.y == 20) { pass++; } else { fail++; printf("FAIL: struct init\n"); }

    Point p2;
    p2.x = 30;
    p2.y = 40;
    Point sum = point_add(p1, p2);
    if (sum.x == 40 && sum.y == 60) { pass++; } else { fail++; printf("FAIL: point_add\n"); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    return fail;
}
