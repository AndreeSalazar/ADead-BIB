// ============================================================
// Canon C++98 — §13.5 Operator Overloading
// ============================================================
// Intención: operator+, operator== etc. son funciones
// normales con nombre especial. El compilador las inlinea
// a instrucciones directas → zero overhead.
//
// C++98 §13.5: "A function declaration having one of the
// following operator-function-ids as its name declares
// an operator function."
// ============================================================

int printf(const char *format, ...);

// --- Vec2 con operators ---
class Vec2 {
public:
    int x;
    int y;

    Vec2() : x(0), y(0) {}
    Vec2(int x, int y) : x(x), y(y) {}

    Vec2 operator+(Vec2 other) {
        return Vec2(x + other.x, y + other.y);
    }

    Vec2 operator-(Vec2 other) {
        return Vec2(x - other.x, y - other.y);
    }

    Vec2 operator*(int scalar) {
        return Vec2(x * scalar, y * scalar);
    }

    int operator==(Vec2 other) {
        return x == other.x && y == other.y;
    }

    int operator!=(Vec2 other) {
        return x != other.x || y != other.y;
    }
};

// --- Fraction con operators ---
class Fraction {
public:
    int num;
    int den;

    Fraction() : num(0), den(1) {}
    Fraction(int n, int d) : num(n), den(d) {}

    Fraction operator+(Fraction other) {
        return Fraction(
            num * other.den + other.num * den,
            den * other.den
        );
    }

    Fraction operator*(Fraction other) {
        return Fraction(num * other.num, den * other.den);
    }

    int operator==(Fraction other) {
        return num * other.den == other.num * den;
    }

    void print() {
        printf("%d/%d", num, den);
    }
};

int main() {
    printf("=== Canon C++98: Operator Overloading ===\n\n");

    // --- Vec2 operators ---
    Vec2 a(3, 4);
    Vec2 b(1, 2);

    Vec2 sum = a + b;
    printf("Vec2:\n");
    printf("  (%d,%d) + (%d,%d) = (%d,%d)\n", a.x, a.y, b.x, b.y, sum.x, sum.y);

    Vec2 diff = a - b;
    printf("  (%d,%d) - (%d,%d) = (%d,%d)\n", a.x, a.y, b.x, b.y, diff.x, diff.y);

    Vec2 scaled = a * 3;
    printf("  (%d,%d) * 3 = (%d,%d)\n", a.x, a.y, scaled.x, scaled.y);

    Vec2 c(3, 4);
    printf("  (%d,%d) == (%d,%d) → %d\n", a.x, a.y, c.x, c.y, a == c);
    printf("  (%d,%d) != (%d,%d) → %d\n", a.x, a.y, b.x, b.y, a != b);

    // --- Fraction operators ---
    printf("\nFraction:\n");
    Fraction f1(1, 2);
    Fraction f2(1, 3);

    Fraction f_sum = f1 + f2;
    printf("  "); f1.print(); printf(" + "); f2.print();
    printf(" = "); f_sum.print(); printf("\n");

    Fraction f_mul = f1 * f2;
    printf("  "); f1.print(); printf(" * "); f2.print();
    printf(" = "); f_mul.print(); printf("\n");

    Fraction f3(2, 4);
    printf("  1/2 == 2/4 → %d\n", f1 == f3);

    // --- Chained operations ---
    Vec2 result = (a + b) * 2;
    printf("\nChained: ((%d,%d) + (%d,%d)) * 2 = (%d,%d)\n",
        a.x, a.y, b.x, b.y, result.x, result.y);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (sum.x == 4 && sum.y == 6)        { pass++; } else { printf("FAIL: vec+\n"); }
    total++; if (diff.x == 2 && diff.y == 2)      { pass++; } else { printf("FAIL: vec-\n"); }
    total++; if (scaled.x == 9 && scaled.y == 12)  { pass++; } else { printf("FAIL: vec*\n"); }
    total++; if (a == c)                           { pass++; } else { printf("FAIL: vec==\n"); }
    total++; if (a != b)                           { pass++; } else { printf("FAIL: vec!=\n"); }
    total++; if (f_sum.num == 5 && f_sum.den == 6) { pass++; } else { printf("FAIL: frac+\n"); }
    total++; if (f1 == f3)                         { pass++; } else { printf("FAIL: frac==\n"); }
    total++; if (result.x == 8 && result.y == 12)  { pass++; } else { printf("FAIL: chained\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
