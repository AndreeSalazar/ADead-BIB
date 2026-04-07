#include <stdio.h>

typedef struct { int x; int y; } Point;
typedef enum { COLOR_RED = 0, COLOR_GREEN = 1, COLOR_BLUE = 2, COLOR_COUNT } Color;

int main() {
    printf("start\n");
    Color c = COLOR_BLUE;
    printf("c=%d\n", c);
    if (c == 2) { printf("PASS enum\n"); }
    if (COLOR_COUNT == 3) { printf("PASS count\n"); }
    printf("done\n");
    return 0;
}
