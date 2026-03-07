// Canon C11 -- Compound literals and flexible features
// C11: compound literals, generic selections concept
int printf(const char *format, ...);

struct Point { int x; int y; };

int distance_sq(struct Point a, struct Point b) {
    int dx = a.x - b.x;
    int dy = a.y - b.y;
    return dx * dx + dy * dy;
}

int max_int(int a, int b) { return (a > b) ? a : b; }
int min_int(int a, int b) { return (a < b) ? a : b; }
int abs_int(int x) { return (x < 0) ? -x : x; }

int main() {
    printf("=== Canon C11: Compound Literals ===\n\n");
    int pass = 0;
    int total = 0;

    // Compound literal as function argument
    int d = distance_sq((struct Point){3, 0}, (struct Point){0, 4});
    printf("distance_sq((3,0),(0,4)) = %d\n", d);
    total++; if (d == 25) { pass++; } else { printf("FAIL: distance_sq\n"); }

    // Compound literal assignment
    struct Point origin = (struct Point){0, 0};
    struct Point target = (struct Point){10, 20};
    printf("origin=(%d,%d) target=(%d,%d)\n", origin.x, origin.y, target.x, target.y);
    total++; if (origin.x == 0 && origin.y == 0) { pass++; } else { printf("FAIL: origin\n"); }
    total++; if (target.x == 10 && target.y == 20) { pass++; } else { printf("FAIL: target\n"); }

    // Array compound literal
    int *arr = (int[]){10, 20, 30, 40, 50};
    printf("arr: %d %d %d\n", arr[0], arr[2], arr[4]);
    total++; if (arr[0] == 10) { pass++; } else { printf("FAIL: arr[0]\n"); }
    total++; if (arr[2] == 30) { pass++; } else { printf("FAIL: arr[2]\n"); }
    total++; if (arr[4] == 50) { pass++; } else { printf("FAIL: arr[4]\n"); }

    // Generic-style dispatch (manual, since _Generic needs preprocessor)
    int a = max_int(42, 17);
    int b = min_int(42, 17);
    int c = abs_int(-99);
    printf("max=%d min=%d abs=%d\n", a, b, c);
    total++; if (a == 42) { pass++; } else { printf("FAIL: max\n"); }
    total++; if (b == 17) { pass++; } else { printf("FAIL: min\n"); }
    total++; if (c == 99) { pass++; } else { printf("FAIL: abs\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}