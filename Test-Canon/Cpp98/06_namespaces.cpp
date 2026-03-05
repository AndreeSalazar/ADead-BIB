// ============================================================
// Canon C++98 — §7.3 Namespaces
// ============================================================
// Intención: Un namespace es organización pura — agrupar
// funciones y tipos bajo un nombre. CERO costo en runtime.
//
// C++98 §7.3.1: "A namespace-definition introduces a
// namespace-name."
//
// ADead-BIB: namespace::func → func renombrado con prefijo.
// Cero overhead — es solo un string más largo.
// ============================================================

int printf(const char *format, ...);

// --- Namespace math ---
namespace math {
    int add(int a, int b) { return a + b; }
    int sub(int a, int b) { return a - b; }
    int mul(int a, int b) { return a * b; }
    int div(int a, int b) { if (b == 0) return 0; return a / b; }

    int abs(int x) { if (x < 0) return 0 - x; return x; }
    int max(int a, int b) { if (a > b) return a; return b; }
    int min(int a, int b) { if (a < b) return a; return b; }
}

// --- Namespace string_utils ---
namespace string_utils {
    int length(const char *s) {
        int len = 0;
        while (s[len] != '\0') len++;
        return len;
    }

    int equal(const char *a, const char *b) {
        int i = 0;
        while (a[i] != '\0' && b[i] != '\0') {
            if (a[i] != b[i]) return 0;
            i++;
        }
        return a[i] == b[i];
    }
}

// --- Nested namespace ---
namespace game {
    namespace physics {
        int gravity(int mass, int height) {
            return mass * height * 10;
        }
    }

    namespace score {
        int calculate(int kills, int time) {
            return kills * 100 - time;
        }
    }
}

// --- Using declaration ---
int compute() {
    int x = math::add(10, 20);
    int y = math::mul(x, 2);
    return math::sub(y, 5);
}

int main() {
    printf("=== Canon C++98: Namespaces ===\n\n");

    // --- math:: ---
    printf("math:\n");
    printf("  add(10, 20) = %d\n", math::add(10, 20));
    printf("  sub(50, 8) = %d\n", math::sub(50, 8));
    printf("  mul(6, 7) = %d\n", math::mul(6, 7));
    printf("  div(100, 3) = %d\n", math::div(100, 3));
    printf("  abs(-42) = %d\n", math::abs(-42));
    printf("  max(10, 20) = %d\n", math::max(10, 20));
    printf("  min(10, 20) = %d\n", math::min(10, 20));

    // --- string_utils:: ---
    printf("\nstring_utils:\n");
    printf("  length(\"Hello\") = %d\n", string_utils::length("Hello"));
    printf("  equal(\"abc\", \"abc\") = %d\n", string_utils::equal("abc", "abc"));
    printf("  equal(\"abc\", \"xyz\") = %d\n", string_utils::equal("abc", "xyz"));

    // --- Nested namespaces ---
    printf("\nNested:\n");
    printf("  physics::gravity(10, 5) = %d\n", game::physics::gravity(10, 5));
    printf("  score::calculate(5, 100) = %d\n", game::score::calculate(5, 100));

    // --- Combined ---
    printf("\nCompute: %d\n", compute());

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (math::add(10, 20) == 30)            { pass++; } else { printf("FAIL: add\n"); }
    total++; if (math::mul(6, 7) == 42)              { pass++; } else { printf("FAIL: mul\n"); }
    total++; if (math::abs(-42) == 42)               { pass++; } else { printf("FAIL: abs\n"); }
    total++; if (string_utils::length("Hello") == 5) { pass++; } else { printf("FAIL: length\n"); }
    total++; if (string_utils::equal("abc", "abc"))  { pass++; } else { printf("FAIL: equal\n"); }
    total++; if (!string_utils::equal("abc", "xyz")) { pass++; } else { printf("FAIL: !equal\n"); }
    total++; if (game::physics::gravity(10, 5)==500)  { pass++; } else { printf("FAIL: gravity\n"); }
    total++; if (compute() == 55)                    { pass++; } else { printf("FAIL: compute\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
