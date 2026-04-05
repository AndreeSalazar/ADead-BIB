// ADead-BIB Bridge Test 06 — Structs, Unions, Enums
// Level: INTERMEDIATE
// Tests: struct layout, union access, typedef, enum, nested structs

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
    printf("=== ADead-BIB Bridge Test 06: Structs ===\n");
    int pass = 0, fail = 0;

    // Basic struct
    Point p1;
    p1.x = 10;
    p1.y = 20;
    if (p1.x == 10 && p1.y == 20) { pass++; } else { fail++; printf("FAIL: struct init\n"); }

    // Struct function
    Point p2;
    p2.x = 30;
    p2.y = 40;
    Point sum = point_add(p1, p2);
    if (sum.x == 40 && sum.y == 60) { pass++; } else { fail++; printf("FAIL: point_add\n"); }

    // Nested struct
    Rect r;
    r.origin.x = 0;
    r.origin.y = 0;
    r.width = 100;
    r.height = 50;
    if (rect_area(r) == 5000) { pass++; } else { fail++; printf("FAIL: rect_area\n"); }

    // Enum
    Color c = COLOR_BLUE;
    if (c == 2) { pass++; } else { fail++; printf("FAIL: enum value\n"); }
    if (COLOR_COUNT == 3) { pass++; } else { fail++; printf("FAIL: enum count\n"); }

    // Union
    Value v;
    v.i = 0x41424344;
    if (v.bytes[0] == 0x44) { pass++; } else { fail++; printf("FAIL: union bytes[0]\n"); }

    // sizeof checks
    if (sizeof(Point) == 8) { pass++; } else { fail++; printf("FAIL: sizeof(Point)=%d\n", (int)sizeof(Point)); }
    if (sizeof(Color) == 4) { pass++; } else { fail++; printf("FAIL: sizeof(Color)=%d\n", (int)sizeof(Color)); }

    // Array of structs
    Point pts[3];
    pts[0].x = 1; pts[0].y = 2;
    pts[1].x = 3; pts[1].y = 4;
    pts[2].x = 5; pts[2].y = 6;
    int sx = 0, sy = 0;
    for (int i = 0; i < 3; i++) { sx += pts[i].x; sy += pts[i].y; }
    if (sx == 9 && sy == 12) { pass++; } else { fail++; printf("FAIL: array of structs\n"); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 06: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
