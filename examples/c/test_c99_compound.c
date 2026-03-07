#include <stdio.h>
struct Vec2 { int x; int y; };
struct Vec2 vec2_add(struct Vec2 a, struct Vec2 b) {
    struct Vec2 r;
    r.x = a.x + b.x;
    r.y = a.y + b.y;
    return r;
}
int dot(struct Vec2 a, struct Vec2 b) { return a.x * b.x + a.y * b.y; }
int length_sq(struct Vec2 v) { return v.x * v.x + v.y * v.y; }
int main() {
    struct Vec2 a; a.x = 3; a.y = 4;
    struct Vec2 b; b.x = 1; b.y = 2;
    struct Vec2 sum = vec2_add(a, b);
    printf("(%d,%d) + (%d,%d) = (%d,%d)\n", a.x, a.y, b.x, b.y, sum.x, sum.y);
    printf("dot = %d\n", dot(a, b));
    printf("len_sq = %d\n", length_sq(a));
    int arr[] = {10, 20, 30, 40, 50};
    int total = 0;
    for (int i = 0; i < 5; i++) total += arr[i];
    printf("array sum = %d\n", total);
    return 0;
}