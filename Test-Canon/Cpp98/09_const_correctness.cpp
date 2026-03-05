// ============================================================
// Canon C++98 — §7.1.5.1 Const Correctness
// ============================================================
// Intención: const es una promesa del programador de que
// un valor no se modificará. El compilador lo verifica
// en compilación y puede optimizar (constant folding).
//
// constexpr (C++11): evaluado en tiempo de compilación
// → 0 instrucciones en runtime, valor ya calculado.
// ============================================================

int printf(const char *format, ...);

// --- Constantes globales ---
const int MAX_SIZE = 100;
const int GRID_WIDTH = 16;
const int GRID_HEIGHT = 12;

// --- constexpr: evaluado en compilación ---
constexpr int square(int x) { return x * x; }
constexpr int cube(int x) { return x * x * x; }
constexpr int factorial_ce(int n) { return n <= 1 ? 1 : n * factorial_ce(n - 1); }

// --- Clase con const methods ---
class Circle {
public:
    int radius;

    Circle(int r) : radius(r) {}

    int area() { return 3 * radius * radius; }
    int diameter() { return 2 * radius; }
    int getRadius() { return radius; }
};

// --- Const parameters ---
int sum_array(const int arr[], int len) {
    int total = 0;
    int i;
    for (i = 0; i < len; i++) {
        total = total + arr[i];
    }
    return total;
}

int max_array(const int arr[], int len) {
    int m = arr[0];
    int i;
    for (i = 1; i < len; i++) {
        if (arr[i] > m) m = arr[i];
    }
    return m;
}

int main() {
    printf("=== Canon C++98: Const Correctness ===\n\n");

    // --- Const variables ---
    printf("Constants:\n");
    printf("  MAX_SIZE = %d\n", MAX_SIZE);
    printf("  GRID = %d x %d = %d cells\n", GRID_WIDTH, GRID_HEIGHT, GRID_WIDTH * GRID_HEIGHT);

    // --- constexpr (compile-time) ---
    printf("\nconstexpr (evaluado en compilación):\n");
    printf("  square(5) = %d\n", square(5));
    printf("  cube(3) = %d\n", cube(3));
    printf("  factorial(6) = %d\n", factorial_ce(6));

    // --- Compile-time constants ---
    constexpr int TABLE_SIZE = square(4);
    constexpr int VOLUME = cube(5);
    printf("  TABLE_SIZE = square(4) = %d\n", TABLE_SIZE);
    printf("  VOLUME = cube(5) = %d\n", VOLUME);

    // --- Const with class ---
    printf("\nConst class:\n");
    Circle c(10);
    printf("  radius = %d\n", c.getRadius());
    printf("  area = %d\n", c.area());
    printf("  diameter = %d\n", c.diameter());

    // --- Const arrays ---
    int nums[5];
    nums[0] = 10; nums[1] = 20; nums[2] = 30; nums[3] = 40; nums[4] = 50;
    printf("\nConst array params:\n");
    printf("  sum = %d\n", sum_array(nums, 5));
    printf("  max = %d\n", max_array(nums, 5));

    // --- Local const ---
    const int local_const = 42;
    printf("\nLocal const = %d\n", local_const);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (MAX_SIZE == 100)         { pass++; } else { printf("FAIL: MAX_SIZE\n"); }
    total++; if (square(5) == 25)         { pass++; } else { printf("FAIL: square\n"); }
    total++; if (cube(3) == 27)           { pass++; } else { printf("FAIL: cube\n"); }
    total++; if (factorial_ce(6) == 720)  { pass++; } else { printf("FAIL: factorial\n"); }
    total++; if (TABLE_SIZE == 16)        { pass++; } else { printf("FAIL: TABLE_SIZE\n"); }
    total++; if (VOLUME == 125)           { pass++; } else { printf("FAIL: VOLUME\n"); }
    total++; if (sum_array(nums, 5)==150) { pass++; } else { printf("FAIL: sum\n"); }
    total++; if (local_const == 42)       { pass++; } else { printf("FAIL: local const\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
