// Test: Structs, enums, typedefs, unions
// Expected: All parse + lower correctly

typedef unsigned int u32;
typedef int i32;

enum Color { RED = 0, GREEN = 1, BLUE = 2 };

struct Point {
    int x;
    int y;
};

struct Rect {
    struct Point origin;
    struct Point size;
};

union Value {
    int i;
    float f;
    char c;
};

struct Point make_point(int x, int y) {
    struct Point p;
    p.x = x;
    p.y = y;
    return p;
}

int rect_area(struct Rect *r) {
    return r->size.x * r->size.y;
}

int color_value(enum Color c) {
    switch (c) {
        case RED: return 0xFF0000;
        case GREEN: return 0x00FF00;
        case BLUE: return 0x0000FF;
        default: return 0;
    }
}

int main() {
    struct Point p = {10, 20};
    struct Rect r;
    r.origin.x = 0;
    r.origin.y = 0;
    r.size.x = 100;
    r.size.y = 50;
    int area = rect_area(&r);

    enum Color c = GREEN;
    int cv = color_value(c);

    u32 x = 42;
    i32 y = -10;

    union Value v;
    v.i = 123;

    return 0;
}
