// ============================================================
// Canon C++98 — §8.3.2 Referencias
// ============================================================
// Intención: Una referencia (T&) es un alias para un objeto
// existente. El compilador la implementa como un puntero,
// pero con semántica de acceso directo.
//
// C++98 §8.3.2: "A reference shall be initialized to
// refer to a valid object."
//
// ADead-BIB: T& → puntero implícito, auto-deref.
// Cero overhead: misma implementación que T*.
// ============================================================

int printf(const char *format, ...);

// --- Pass by reference ---
void increment(int &x) {
    x = x + 1;
}

void swap_ref(int &a, int &b) {
    int temp = a;
    a = b;
    b = temp;
}

void triple(int &x) {
    x = x * 3;
}

// --- Const reference (read-only) ---
int sum_ref(const int &a, const int &b) {
    return a + b;
}

// --- Reference return (alias to existing) ---
class Container {
public:
    int data[4];
    int size;

    Container() : size(0) {
        data[0] = 0;
        data[1] = 0;
        data[2] = 0;
        data[3] = 0;
    }

    void set(int index, int value) {
        if (index >= 0 && index < 4) {
            data[index] = value;
            if (index >= size) size = index + 1;
        }
    }

    int get(int index) {
        if (index >= 0 && index < 4) return data[index];
        return 0;
    }
};

// --- Functions taking struct by reference ---
class Vec2 {
public:
    int x;
    int y;
    Vec2() : x(0), y(0) {}
    Vec2(int x, int y) : x(x), y(y) {}
};

void vec2_scale(Vec2 &v, int factor) {
    v.x = v.x * factor;
    v.y = v.y * factor;
}

void vec2_translate(Vec2 &v, int dx, int dy) {
    v.x = v.x + dx;
    v.y = v.y + dy;
}

int main() {
    printf("=== Canon C++98: Referencias ===\n\n");

    // --- Basic reference ---
    int x = 10;
    int &ref = x;
    printf("x = %d, ref = %d\n", x, ref);

    ref = 42;
    printf("after ref=42: x = %d\n", x);

    // --- Pass by reference ---
    printf("\nPass by reference:\n");
    int a = 5;
    printf("  before increment: a = %d\n", a);
    increment(a);
    printf("  after increment: a = %d\n", a);
    increment(a);
    printf("  after 2nd increment: a = %d\n", a);

    // --- Swap ---
    int p = 100;
    int q = 200;
    printf("\nSwap:\n");
    printf("  before: p=%d q=%d\n", p, q);
    swap_ref(p, q);
    printf("  after: p=%d q=%d\n", p, q);

    // --- Triple ---
    int val = 7;
    triple(val);
    printf("\nTriple(7) = %d\n", val);

    // --- Const reference ---
    printf("\nConst ref:\n");
    int m = 30;
    int n = 12;
    printf("  sum(%d, %d) = %d\n", m, n, sum_ref(m, n));

    // --- Container ---
    printf("\nContainer:\n");
    Container c;
    c.set(0, 10);
    c.set(1, 20);
    c.set(2, 30);
    printf("  [0]=%d [1]=%d [2]=%d\n", c.get(0), c.get(1), c.get(2));

    // --- Vec2 by reference ---
    printf("\nVec2 by reference:\n");
    Vec2 v(3, 4);
    printf("  original: (%d, %d)\n", v.x, v.y);
    vec2_scale(v, 2);
    printf("  scaled x2: (%d, %d)\n", v.x, v.y);
    vec2_translate(v, 10, 20);
    printf("  translated (+10,+20): (%d, %d)\n", v.x, v.y);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (x == 42)                { pass++; } else { printf("FAIL: ref assign\n"); }
    total++; if (a == 7)                 { pass++; } else { printf("FAIL: increment\n"); }
    total++; if (p == 200 && q == 100)   { pass++; } else { printf("FAIL: swap\n"); }
    total++; if (val == 21)              { pass++; } else { printf("FAIL: triple\n"); }
    total++; if (sum_ref(m, n) == 42)    { pass++; } else { printf("FAIL: const ref\n"); }
    total++; if (v.x == 16 && v.y == 28) { pass++; } else { printf("FAIL: vec2 ref\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
