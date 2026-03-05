// ============================================================
// Canon C++98 — §12 Constructores y Destructores Avanzados
// ============================================================
// Intención: Constructores inicializan objetos.
// Member initializer list es la forma canónica.
// Multiple constructors = overloading.
//
// C++98 §12.1: "A constructor is used to initialize objects."
// C++98 §12.6.2: "Member initializers... are evaluated in
// the order of declaration."
//
// ADead-BIB: constructor → función init llamada después
// de allocar el objeto en stack o heap.
// ============================================================

int printf(const char *format, ...);

// --- Multiple constructors (overloading) ---
class Rect {
public:
    int x;
    int y;
    int w;
    int h;

    Rect() : x(0), y(0), w(0), h(0) {}
    Rect(int w, int h) : x(0), y(0), w(w), h(h) {}
    Rect(int x, int y, int w, int h) : x(x), y(y), w(w), h(h) {}

    int area() { return w * h; }
    int perimeter() { return 2 * (w + h); }

    int contains(int px, int py) {
        return px >= x && px < x + w && py >= y && py < y + h;
    }

    void print() {
        printf("  Rect(%d,%d,%d,%d) area=%d\n", x, y, w, h, area());
    }
};

// --- Constructor with computed fields ---
class Matrix2x2 {
public:
    int a;
    int b;
    int c;
    int d;

    Matrix2x2() : a(1), b(0), c(0), d(1) {}
    Matrix2x2(int a, int b, int c, int d) : a(a), b(b), c(c), d(d) {}

    int determinant() {
        return a * d - b * c;
    }

    int trace() {
        return a + d;
    }

    Matrix2x2 multiply(Matrix2x2 other) {
        return Matrix2x2(
            a * other.a + b * other.c,
            a * other.b + b * other.d,
            c * other.a + d * other.c,
            c * other.b + d * other.d
        );
    }

    void print() {
        printf("  | %d %d |\n", a, b);
        printf("  | %d %d |\n", c, d);
    }
};

// --- Composición de objetos ---
class Player {
public:
    int id;
    int health;
    int score;
    Rect hitbox;

    Player(int id) : id(id), health(100), score(0), hitbox(0, 0, 32, 32) {}
    Player(int id, int x, int y) : id(id), health(100), score(0), hitbox(x, y, 32, 32) {}

    void takeDamage(int dmg) {
        health = health - dmg;
        if (health < 0) health = 0;
    }

    void addScore(int points) {
        score = score + points;
    }

    int alive() { return health > 0; }

    void print() {
        printf("  Player #%d: HP=%d Score=%d\n", id, health, score);
        printf("    Hitbox: ");
        hitbox.print();
    }
};

int main() {
    printf("=== Canon C++98: Constructores Avanzados ===\n\n");

    // --- Rect overloads ---
    printf("Rect constructors:\n");
    Rect r1;
    r1.print();

    Rect r2(10, 5);
    r2.print();

    Rect r3(100, 200, 50, 30);
    r3.print();

    printf("  r3 contains (110,210): %d\n", r3.contains(110, 210));
    printf("  r3 contains (0,0): %d\n", r3.contains(0, 0));

    // --- Matrix ---
    printf("\nMatrix2x2:\n");
    Matrix2x2 identity;
    printf("Identity:\n");
    identity.print();
    printf("  det = %d, trace = %d\n", identity.determinant(), identity.trace());

    Matrix2x2 m(2, 3, 1, 4);
    printf("M:\n");
    m.print();
    printf("  det = %d, trace = %d\n", m.determinant(), m.trace());

    Matrix2x2 m2(1, 2, 3, 4);
    Matrix2x2 product = m.multiply(m2);
    printf("M * M2:\n");
    product.print();

    // --- Composición ---
    printf("\nPlayer (composición):\n");
    Player p1(1);
    p1.print();

    p1.takeDamage(30);
    p1.addScore(100);
    printf("  After damage and score:\n");
    p1.print();

    Player p2(2, 100, 50);
    p2.print();

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (r1.area() == 0)          { pass++; } else { printf("FAIL: default\n"); }
    total++; if (r2.area() == 50)         { pass++; } else { printf("FAIL: r2 area\n"); }
    total++; if (r3.area() == 1500)       { pass++; } else { printf("FAIL: r3 area\n"); }
    total++; if (r3.contains(110, 210))   { pass++; } else { printf("FAIL: contains\n"); }
    total++; if (identity.determinant()==1){ pass++; } else { printf("FAIL: det identity\n"); }
    total++; if (m.determinant() == 5)    { pass++; } else { printf("FAIL: det m\n"); }
    total++; if (p1.alive())              { pass++; } else { printf("FAIL: alive\n"); }
    total++; if (p1.health == 70)         { pass++; } else { printf("FAIL: health\n"); }
    total++; if (p1.score == 100)         { pass++; } else { printf("FAIL: score\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
