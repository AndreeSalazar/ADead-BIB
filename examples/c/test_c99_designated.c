#include <stdio.h>
struct Point { int x; int y; int z; };
struct Color { unsigned char r; unsigned char g; unsigned char b; unsigned char a; };
struct Config {
    int width;
    int height;
    int fullscreen;
    struct Color bg;
};
int main() {
    struct Point p;
    p.x = 10;
    p.y = 20;
    p.z = 30;
    printf("Point: (%d, %d, %d)\n", p.x, p.y, p.z);
    struct Color red;
    red.r = 255; red.g = 0; red.b = 0; red.a = 255;
    printf("Color: rgba(%d,%d,%d,%d)\n", red.r, red.g, red.b, red.a);
    struct Config cfg;
    cfg.width = 1920;
    cfg.height = 1080;
    cfg.fullscreen = 1;
    cfg.bg = red;
    printf("Config: %dx%d fs=%d bg=(%d,%d,%d)\n",
        cfg.width, cfg.height, cfg.fullscreen,
        cfg.bg.r, cfg.bg.g, cfg.bg.b);
    return 0;
}