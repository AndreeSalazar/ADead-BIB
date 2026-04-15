// ============================================================
// Test 09: Structs Básicos — declaración, campos, inicialización
// ============================================================
// ADead-BIB Test Canon — C99 §6.7.2.1
// Verifica: struct decl, field access (.), typedef
// ============================================================

#include <stdio.h>

struct Point {
    int x;
    int y;
};

struct Color {
    unsigned char r;
    unsigned char g;
    unsigned char b;
    unsigned char a;
};

typedef struct {
    int width;
    int height;
} Size;

struct Rect {
    struct Point origin;
    Size size;
};

struct Point make_point(int x, int y) {
    struct Point p;
    p.x = x;
    p.y = y;
    return p;
}

int point_distance_sq(struct Point a, struct Point b) {
    int dx = a.x - b.x;
    int dy = a.y - b.y;
    return dx * dx + dy * dy;
}

int rect_area(struct Rect r) {
    return r.size.width * r.size.height;
}

int rect_perimeter(struct Rect r) {
    return 2 * (r.size.width + r.size.height);
}

int main() {
    // --- Inicialización directa ---
    struct Point p1 = {10, 20};
    struct Point p2 = {30, 40};
    printf("p1=(%d,%d) p2=(%d,%d)\n", p1.x, p1.y, p2.x, p2.y);

    // --- Asignación campo a campo ---
    struct Point p3;
    p3.x = 100;
    p3.y = 200;
    printf("p3=(%d,%d)\n", p3.x, p3.y);

    // --- make_point ---
    struct Point p4 = make_point(5, 15);
    printf("p4=(%d,%d)\n", p4.x, p4.y);

    // --- Distancia ---
    int dist = point_distance_sq(p1, p2);
    printf("dist_sq=%d\n", dist);

    // --- Color ---
    struct Color red = {255, 0, 0, 255};
    printf("color=(%d,%d,%d,%d)\n", red.r, red.g, red.b, red.a);

    // --- Rect con struct anidado ---
    struct Rect r;
    r.origin.x = 0;
    r.origin.y = 0;
    r.size.width = 100;
    r.size.height = 50;
    printf("area=%d perimeter=%d\n", rect_area(r), rect_perimeter(r));

    // --- typedef struct ---
    Size s = {640, 480};
    printf("size=%dx%d\n", s.width, s.height);

    // --- Asignación struct a struct ---
    struct Point copy = p1;
    printf("copy=(%d,%d)\n", copy.x, copy.y);

    // --- Array de structs ---
    struct Point pts[3] = {{1, 2}, {3, 4}, {5, 6}};
    int sum_x = 0, sum_y = 0;
    int i;
    for (i = 0; i < 3; i++) {
        sum_x += pts[i].x;
        sum_y += pts[i].y;
    }
    printf("sum_x=%d sum_y=%d\n", sum_x, sum_y);

    return 0;
}
// Expected:
// p1=(10,20) p2=(30,40)
// p3=(100,200)
// p4=(5,15)
// dist_sq=800
// color=(255,0,0,255)
// area=5000 perimeter=300
// size=640x480
// copy=(10,20)
// sum_x=9 sum_y=12
