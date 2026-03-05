// ============================================================
// Canon C++98 — §14 Class Templates
// ============================================================
// Intención: Un class template genera UNA clase concreta
// por cada tipo con el que se instancia. Pair<int> y
// Pair<char> son dos structs diferentes en memoria.
//
// Monomorphización = solo se genera código para lo que
// el programador realmente usa.
// ============================================================

int printf(const char *format, ...);

// --- Pair template ---
template<typename T>
class Pair {
public:
    T first;
    T second;

    Pair(T a, T b) : first(a), second(b) {}

    T getFirst() { return first; }
    T getSecond() { return second; }

    void swap() {
        T temp = first;
        first = second;
        second = temp;
    }
};

// --- Box template (contenedor) ---
template<typename T>
class Box {
public:
    T value;
    int valid;

    Box() : value(0), valid(0) {}
    Box(T v) : value(v), valid(1) {}

    T get() { return value; }

    void set(T v) {
        value = v;
        valid = 1;
    }

    int hasValue() { return valid; }
};

// --- Array template (contenedor fijo) ---
template<typename T>
class FixedArray {
public:
    T data[8];
    int count;

    FixedArray() : count(0) {}

    void push(T val) {
        if (count < 8) {
            data[count] = val;
            count = count + 1;
        }
    }

    T at(int index) {
        if (index >= 0 && index < count) {
            return data[index];
        }
        return 0;
    }

    int size() { return count; }

    T sum() {
        T total = 0;
        int i;
        for (i = 0; i < count; i++) {
            total = total + data[i];
        }
        return total;
    }
};

// --- Template con non-type param ---
template<typename T>
class MinMax {
public:
    T min_val;
    T max_val;
    int has_data;

    MinMax() : min_val(0), max_val(0), has_data(0) {}

    void observe(T val) {
        if (!has_data) {
            min_val = val;
            max_val = val;
            has_data = 1;
        } else {
            if (val < min_val) min_val = val;
            if (val > max_val) max_val = val;
        }
    }

    T getMin() { return min_val; }
    T getMax() { return max_val; }
    T range() { return max_val - min_val; }
};

int main() {
    printf("=== Canon C++98: Class Templates ===\n\n");

    // --- Pair<int> ---
    Pair<int> p(10, 20);
    printf("Pair<int>:\n");
    printf("  first=%d second=%d\n", p.getFirst(), p.getSecond());
    p.swap();
    printf("  after swap: first=%d second=%d\n", p.getFirst(), p.getSecond());

    // --- Box<int> ---
    printf("\nBox<int>:\n");
    Box<int> empty;
    printf("  empty.hasValue = %d\n", empty.hasValue());
    Box<int> full(42);
    printf("  full.get = %d, hasValue = %d\n", full.get(), full.hasValue());
    full.set(99);
    printf("  after set(99): %d\n", full.get());

    // --- FixedArray<int> ---
    printf("\nFixedArray<int>:\n");
    FixedArray<int> arr;
    arr.push(10);
    arr.push(20);
    arr.push(30);
    arr.push(40);
    printf("  size = %d\n", arr.size());
    printf("  at(0) = %d\n", arr.at(0));
    printf("  at(3) = %d\n", arr.at(3));
    printf("  sum = %d\n", arr.sum());

    // --- MinMax<int> ---
    printf("\nMinMax<int>:\n");
    MinMax<int> mm;
    mm.observe(42);
    mm.observe(17);
    mm.observe(89);
    mm.observe(3);
    mm.observe(55);
    printf("  min = %d\n", mm.getMin());
    printf("  max = %d\n", mm.getMax());
    printf("  range = %d\n", mm.range());

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (p.getFirst() == 20)     { pass++; } else { printf("FAIL: pair swap\n"); }
    total++; if (p.getSecond() == 10)    { pass++; } else { printf("FAIL: pair swap2\n"); }
    total++; if (full.get() == 99)       { pass++; } else { printf("FAIL: box set\n"); }
    total++; if (arr.size() == 4)        { pass++; } else { printf("FAIL: array size\n"); }
    total++; if (arr.sum() == 100)       { pass++; } else { printf("FAIL: array sum\n"); }
    total++; if (mm.getMin() == 3)       { pass++; } else { printf("FAIL: minmax min\n"); }
    total++; if (mm.getMax() == 89)      { pass++; } else { printf("FAIL: minmax max\n"); }
    total++; if (mm.range() == 86)       { pass++; } else { printf("FAIL: minmax range\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
