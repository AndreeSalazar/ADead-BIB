// ============================================================
// Canon C++98 — §14 Function Templates
// ============================================================
// Intención: Un template genera código SOLO para las
// instancias que el programador usa. Si escribes
// max<int>, se genera max_int. Si no usas max<float>,
// NO se genera max_float → zero dead code.
//
// C++98 §14: "A function template defines a family of
// functions."
//
// ADead-BIB: monomorphización — cada instancia se compila
// como función independiente con tipos concretos.
// ============================================================

int printf(const char *format, ...);

// --- Templates de función ---
template<typename T>
T max_val(T a, T b) {
    if (a > b) return a;
    return b;
}

template<typename T>
T min_val(T a, T b) {
    if (a < b) return a;
    return b;
}

template<typename T>
T abs_val(T x) {
    if (x < 0) return 0 - x;
    return x;
}

template<typename T>
T clamp(T value, T lo, T hi) {
    if (value < lo) return lo;
    if (value > hi) return hi;
    return value;
}

template<typename T>
void swap_val(T *a, T *b) {
    T temp = *a;
    *a = *b;
    *b = temp;
}

// --- Template con dos tipos ---
template<typename T>
T accumulate(T arr[], int len) {
    T total = 0;
    int i;
    for (i = 0; i < len; i++) {
        total = total + arr[i];
    }
    return total;
}

// --- Template para algoritmos ---
template<typename T>
int find(T arr[], int len, T target) {
    int i;
    for (i = 0; i < len; i++) {
        if (arr[i] == target) return i;
    }
    return -1;
}

template<typename T>
T array_max(T arr[], int len) {
    T m = arr[0];
    int i;
    for (i = 1; i < len; i++) {
        if (arr[i] > m) m = arr[i];
    }
    return m;
}

int main() {
    printf("=== Canon C++98: Function Templates ===\n\n");

    // --- max/min ---
    printf("max/min:\n");
    printf("  max(3, 7) = %d\n", max_val(3, 7));
    printf("  max(10, 2) = %d\n", max_val(10, 2));
    printf("  min(3, 7) = %d\n", min_val(3, 7));
    printf("  min(10, 2) = %d\n", min_val(10, 2));

    // --- abs ---
    printf("\nabs:\n");
    printf("  abs(-5) = %d\n", abs_val(-5));
    printf("  abs(5) = %d\n", abs_val(5));
    printf("  abs(-42) = %d\n", abs_val(-42));

    // --- clamp ---
    printf("\nclamp:\n");
    printf("  clamp(150, 0, 100) = %d\n", clamp(150, 0, 100));
    printf("  clamp(-5, 0, 100) = %d\n", clamp(-5, 0, 100));
    printf("  clamp(50, 0, 100) = %d\n", clamp(50, 0, 100));

    // --- swap ---
    int a = 42;
    int b = 99;
    printf("\nswap: before a=%d b=%d\n", a, b);
    swap_val(&a, &b);
    printf("  after a=%d b=%d\n", a, b);

    // --- accumulate ---
    int nums[5];
    nums[0] = 10; nums[1] = 20; nums[2] = 30; nums[3] = 40; nums[4] = 50;
    printf("\naccumulate [10,20,30,40,50] = %d\n", accumulate(nums, 5));

    // --- find ---
    printf("\nfind:\n");
    printf("  find(30) = index %d\n", find(nums, 5, 30));
    printf("  find(99) = index %d\n", find(nums, 5, 99));

    // --- array_max ---
    printf("  array_max = %d\n", array_max(nums, 5));

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (max_val(3, 7) == 7)          { pass++; } else { printf("FAIL: max\n"); }
    total++; if (min_val(3, 7) == 3)          { pass++; } else { printf("FAIL: min\n"); }
    total++; if (abs_val(-42) == 42)          { pass++; } else { printf("FAIL: abs\n"); }
    total++; if (clamp(150, 0, 100) == 100)   { pass++; } else { printf("FAIL: clamp hi\n"); }
    total++; if (clamp(-5, 0, 100) == 0)      { pass++; } else { printf("FAIL: clamp lo\n"); }
    total++; if (a == 99 && b == 42)          { pass++; } else { printf("FAIL: swap\n"); }
    total++; if (accumulate(nums, 5) == 150)  { pass++; } else { printf("FAIL: accum\n"); }
    total++; if (find(nums, 5, 30) == 2)      { pass++; } else { printf("FAIL: find\n"); }
    total++; if (find(nums, 5, 99) == -1)     { pass++; } else { printf("FAIL: find miss\n"); }
    total++; if (array_max(nums, 5) == 50)    { pass++; } else { printf("FAIL: array_max\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
