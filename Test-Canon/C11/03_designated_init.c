// Canon C11 -- Designated initializers
// C11 6.7.9: initialize specific struct/array members
int printf(const char *format, ...);

struct Color {
    int r;
    int g;
    int b;
    int a;
};

int main() {
    printf("=== Canon C11: Designated Initializers ===\n\n");
    int pass = 0;
    int total = 0;

    // Struct designated init
    struct Color red = { .r = 255, .g = 0, .b = 0, .a = 255 };
    printf("red: r=%d g=%d b=%d a=%d\n", red.r, red.g, red.b, red.a);
    total++; if (red.r == 255) { pass++; } else { printf("FAIL: red.r\n"); }
    total++; if (red.g == 0) { pass++; } else { printf("FAIL: red.g\n"); }
    total++; if (red.b == 0) { pass++; } else { printf("FAIL: red.b\n"); }
    total++; if (red.a == 255) { pass++; } else { printf("FAIL: red.a\n"); }

    // Array designated init
    int arr[5] = { [0] = 10, [2] = 30, [4] = 50 };
    printf("arr: [0]=%d [2]=%d [4]=%d\n", arr[0], arr[2], arr[4]);
    total++; if (arr[0] == 10) { pass++; } else { printf("FAIL: arr[0]\n"); }
    total++; if (arr[2] == 30) { pass++; } else { printf("FAIL: arr[2]\n"); }
    total++; if (arr[4] == 50) { pass++; } else { printf("FAIL: arr[4]\n"); }

    // Partial init with designators
    struct Color blue = { .b = 255, .a = 128 };
    printf("blue: r=%d g=%d b=%d a=%d\n", blue.r, blue.g, blue.b, blue.a);
    total++; if (blue.b == 255) { pass++; } else { printf("FAIL: blue.b\n"); }
    total++; if (blue.a == 128) { pass++; } else { printf("FAIL: blue.a\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}