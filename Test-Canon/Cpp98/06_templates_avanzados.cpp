// Canon C++11/14/17 -- Advanced Templates
int printf(const char *format, ...);
template<typename T> T add(T a, T b) { return a + b; }
template<typename T, typename U> T convert(U val) { return val; }
template<typename T, int N> struct StaticArray { T data[N]; int size() { return N; } };
template<typename T = int> T zero() { return 0; }
template<typename T> T describe_type() { return 0; }
template<> int describe_type<int>() { return 1; }
template<> int describe_type<char>() { return 2; }
template<typename T> class TypeInfo { public: int id; TypeInfo() : id(0) {} int get_id() { return id; } };
template<> class TypeInfo<int> { public: int id; TypeInfo() : id(42) {} int get_id() { return id; } };
template<> class TypeInfo<char> { public: int id; TypeInfo() : id(99) {} int get_id() { return id; } };
template<typename T> using Ptr = T*;
template<typename T> class Optional { public: T value; int has_val; Optional() : value(0), has_val(0) {} Optional(T v) : value(v), has_val(1) {} T get() { return value; } int has_value() { return has_val; } T value_or(T def) { if (has_val) return value; return def; } };
int main() {
    int pass = 0; int total = 0;
    int sum = add(10, 20);
    total++; if (sum == 30) { pass++; } else { printf("FAIL add\n"); }
    StaticArray<int, 5> arr;
    total++; if (arr.size() == 5) { pass++; } else { printf("FAIL static_array\n"); }
    int id = describe_type<int>();
    total++; if (id == 1) { pass++; } else { printf("FAIL func spec\n"); }
    TypeInfo<int> ti;
    total++; if (ti.get_id() == 42) { pass++; } else { printf("FAIL class spec\n"); }
    Optional<int> empty;
    Optional<int> full(77);
    total++; if (empty.has_value() == 0) { pass++; } else { printf("FAIL opt empty\n"); }
    total++; if (full.get() == 77) { pass++; } else { printf("FAIL opt full\n"); }
    total++; if (empty.value_or(42) == 42) { pass++; } else { printf("FAIL opt val_or\n"); }
    total++; if (full.value_or(42) == 77) { pass++; } else { printf("FAIL opt val_or2\n"); }
    printf("%d/%d passed\n", pass, total);
    return 0;
}