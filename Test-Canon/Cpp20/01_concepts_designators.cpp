// Canon C++20 -- Concepts, spaceship operator, designated initializers
// NOTE: C++20 features -- separate folder per ARCHITECTURE.md
int printf(const char *format, ...);

// C++20 designated initializers for classes
struct Config {
    int width;
    int height;
    int fps;
    int fullscreen;
};

// C++20-style three-way comparison concept (manual implementation)
// ADead-BIB spaceship operator support
int compare(int a, int b) {
    if (a < b) return -1;
    if (a > b) return 1;
    return 0;
}

// Concept-like constrained template (manual SFINAE style)
template<typename T>
T clamp(T val, T lo, T hi) {
    if (val < lo) return lo;
    if (val > hi) return hi;
    return val;
}

template<typename T>
class Span {
public:
    T* ptr;
    int len;
    Span(T* p, int n) : ptr(p), len(n) {}
    int size() { return len; }
    T get(int i) { return ptr[i]; }
};

int main() {
    printf("=== Canon C++20: Modern Features ===\n\n");
    int pass = 0;
    int total = 0;

    // Designated initializers (C++20 for aggregates)
    Config cfg = { .width = 1920, .height = 1080, .fps = 60, .fullscreen = 1 };
    printf("Config: %dx%d @%dfps fs=%d\n", cfg.width, cfg.height, cfg.fps, cfg.fullscreen);
    total++; if (cfg.width == 1920) { pass++; } else { printf("FAIL: width\n"); }
    total++; if (cfg.height == 1080) { pass++; } else { printf("FAIL: height\n"); }
    total++; if (cfg.fps == 60) { pass++; } else { printf("FAIL: fps\n"); }

    // Three-way comparison
    int cmp1 = compare(5, 10);
    int cmp2 = compare(10, 10);
    int cmp3 = compare(15, 10);
    printf("compare(5,10)=%d (10,10)=%d (15,10)=%d\n", cmp1, cmp2, cmp3);
    total++; if (cmp1 == -1) { pass++; } else { printf("FAIL: cmp<\n"); }
    total++; if (cmp2 == 0) { pass++; } else { printf("FAIL: cmp==\n"); }
    total++; if (cmp3 == 1) { pass++; } else { printf("FAIL: cmp>\n"); }

    // Clamp
    int c1 = clamp(5, 0, 10);
    int c2 = clamp(-5, 0, 10);
    int c3 = clamp(15, 0, 10);
    printf("clamp(5,0,10)=%d (-5)=%d (15)=%d\n", c1, c2, c3);
    total++; if (c1 == 5) { pass++; } else { printf("FAIL: clamp mid\n"); }
    total++; if (c2 == 0) { pass++; } else { printf("FAIL: clamp lo\n"); }
    total++; if (c3 == 10) { pass++; } else { printf("FAIL: clamp hi\n"); }

    // Span
    int arr[5] = {10, 20, 30, 40, 50};
    Span<int> sp(arr, 5);
    printf("Span size=%d [0]=%d [4]=%d\n", sp.size(), sp.get(0), sp.get(4));
    total++; if (sp.size() == 5) { pass++; } else { printf("FAIL: span size\n"); }
    total++; if (sp.get(0) == 10) { pass++; } else { printf("FAIL: span[0]\n"); }
    total++; if (sp.get(4) == 50) { pass++; } else { printf("FAIL: span[4]\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}