// ============================================================
// Canon C++98 — STL Básico (Reconocido)
// ============================================================
// Intención: ADead-BIB reconoce los patrones STL comunes
// y los compila a código nativo. No implementa la STL
// completa — elimina el overhead y genera lo esencial.
//
// Lo que ADead-BIB hace con STL:
//   cout << x        → printf("%d", x)
//   vector<int>      → array dinámico (malloc/free)
//   unique_ptr<T>    → raw pointer (zero overhead)
//   string           → char* con length
//
// El programador escribe STL idiomático, ADead-BIB genera
// machine code sin el overhead de templates internos.
// ============================================================

int printf(const char *format, ...);

// --- Simulating STL patterns recognized by ADead-BIB ---

// Vector-like: fixed capacity, dynamic size
class IntVector {
public:
    int data[64];
    int size;
    int capacity;

    IntVector() : size(0), capacity(64) {}

    void push_back(int val) {
        if (size < capacity) {
            data[size] = val;
            size = size + 1;
        }
    }

    int at(int index) {
        if (index >= 0 && index < size) return data[index];
        return 0;
    }

    int front() { return data[0]; }
    int back() { return data[size - 1]; }
    int empty() { return size == 0; }

    void pop_back() {
        if (size > 0) size = size - 1;
    }

    void clear() { size = 0; }

    int sum() {
        int total = 0;
        int i;
        for (i = 0; i < size; i++) {
            total = total + data[i];
        }
        return total;
    }
};

// String-like: char array with length
class String {
public:
    char data[128];
    int len;

    String() : len(0) { data[0] = '\0'; }

    void set(const char *s) {
        int i = 0;
        while (s[i] != '\0' && i < 127) {
            data[i] = s[i];
            i++;
        }
        data[i] = '\0';
        len = i;
    }

    int length() { return len; }
    int empty() { return len == 0; }

    int equals(const char *s) {
        int i = 0;
        while (data[i] != '\0' && s[i] != '\0') {
            if (data[i] != s[i]) return 0;
            i++;
        }
        return data[i] == s[i];
    }

    char charAt(int index) {
        if (index >= 0 && index < len) return data[index];
        return '\0';
    }

    void print() {
        printf("%s", data);
    }
};

// Pair-like
class IntPair {
public:
    int first;
    int second;

    IntPair() : first(0), second(0) {}
    IntPair(int a, int b) : first(a), second(b) {}
};

int main() {
    printf("=== Canon C++98: STL Básico ===\n\n");

    // --- IntVector (como std::vector<int>) ---
    printf("IntVector (std::vector pattern):\n");
    IntVector vec;
    printf("  empty: %d\n", vec.empty());

    vec.push_back(10);
    vec.push_back(20);
    vec.push_back(30);
    vec.push_back(40);
    vec.push_back(50);

    printf("  size: %d\n", vec.size);
    printf("  front: %d\n", vec.front());
    printf("  back: %d\n", vec.back());
    printf("  sum: %d\n", vec.sum());

    printf("  contents: ");
    int i;
    for (i = 0; i < vec.size; i++) {
        printf("%d ", vec.at(i));
    }
    printf("\n");

    vec.pop_back();
    printf("  after pop_back: size=%d back=%d\n", vec.size, vec.back());

    // --- String (como std::string) ---
    printf("\nString (std::string pattern):\n");
    String s;
    printf("  empty: %d\n", s.empty());

    s.set("Hello ADead-BIB");
    printf("  value: "); s.print(); printf("\n");
    printf("  length: %d\n", s.length());
    printf("  charAt(0): '%c'\n", s.charAt(0));
    printf("  charAt(6): '%c'\n", s.charAt(6));
    printf("  equals(\"Hello ADead-BIB\"): %d\n", s.equals("Hello ADead-BIB"));
    printf("  equals(\"wrong\"): %d\n", s.equals("wrong"));

    // --- IntPair (como std::pair<int,int>) ---
    printf("\nIntPair (std::pair pattern):\n");
    IntPair p(42, 99);
    printf("  first=%d second=%d\n", p.first, p.second);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (vec.size == 4)            { pass++; } else { printf("FAIL: vec size\n"); }
    total++; if (vec.sum() == 100)         { pass++; } else { printf("FAIL: vec sum\n"); }
    total++; if (vec.front() == 10)        { pass++; } else { printf("FAIL: vec front\n"); }
    total++; if (s.length() == 15)         { pass++; } else { printf("FAIL: str len\n"); }
    total++; if (s.charAt(0) == 'H')       { pass++; } else { printf("FAIL: charAt\n"); }
    total++; if (s.equals("Hello ADead-BIB")) { pass++; } else { printf("FAIL: equals\n"); }
    total++; if (p.first == 42)            { pass++; } else { printf("FAIL: pair\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
