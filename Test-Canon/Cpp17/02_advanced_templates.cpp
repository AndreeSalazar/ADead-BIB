// Canon C++17 -- Advanced templates: non-type params, multiple specialization
int printf(const char *format, ...);

// Matrix with compile-time dimensions
template<typename T, int Rows, int Cols>
class Matrix {
public:
    T data[16];
    int rows() { return Rows; }
    int cols() { return Cols; }
    T get(int r, int c) { return data[r * Cols + c]; }
    void set(int r, int c, T val) { data[r * Cols + c] = val; }
};

// Optional with has_value
template<typename T>
class Optional {
public:
    T val;
    int has;
    Optional() : val(0), has(0) {}
    Optional(T v) : val(v), has(1) {}
    int has_value() { return has; }
    T value() { return val; }
    T value_or(T def) { if (has) return val; return def; }
};

// Result type (like Rust Result<T,E>)
template<typename T>
class Result {
public:
    T val;
    int err_code;
    int is_ok;
    Result(T v) : val(v), err_code(0), is_ok(1) {}
    int ok() { return is_ok; }
    T unwrap() { return val; }
};

int main() {
    printf("=== Canon C++17: Advanced Template Patterns ===\n\n");
    int pass = 0;
    int total = 0;

    // Matrix<int, 2, 3>
    Matrix<int, 2, 3> m;
    m.set(0, 0, 1); m.set(0, 1, 2); m.set(0, 2, 3);
    m.set(1, 0, 4); m.set(1, 1, 5); m.set(1, 2, 6);
    printf("Matrix 2x3: [0,0]=%d [1,2]=%d\n", m.get(0,0), m.get(1,2));
    printf("  rows=%d cols=%d\n", m.rows(), m.cols());
    total++; if (m.get(0,0) == 1) { pass++; } else { printf("FAIL: m[0,0]\n"); }
    total++; if (m.get(1,2) == 6) { pass++; } else { printf("FAIL: m[1,2]\n"); }
    total++; if (m.rows() == 2) { pass++; } else { printf("FAIL: rows\n"); }
    total++; if (m.cols() == 3) { pass++; } else { printf("FAIL: cols\n"); }

    // Optional
    Optional<int> empty;
    Optional<int> full(42);
    printf("Optional empty: has=%d value_or(99)=%d\n", empty.has_value(), empty.value_or(99));
    printf("Optional full: has=%d value=%d\n", full.has_value(), full.value());
    total++; if (empty.has_value() == 0) { pass++; } else { printf("FAIL: empty.has\n"); }
    total++; if (empty.value_or(99) == 99) { pass++; } else { printf("FAIL: empty.value_or\n"); }
    total++; if (full.has_value() == 1) { pass++; } else { printf("FAIL: full.has\n"); }
    total++; if (full.value() == 42) { pass++; } else { printf("FAIL: full.value\n"); }

    // Result
    Result<int> ok(100);
    printf("Result ok=%d unwrap=%d\n", ok.ok(), ok.unwrap());
    total++; if (ok.ok() == 1) { pass++; } else { printf("FAIL: ok.ok\n"); }
    total++; if (ok.unwrap() == 100) { pass++; } else { printf("FAIL: ok.unwrap\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}