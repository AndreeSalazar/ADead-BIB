// Canon C++17 -- if constexpr, template specialization, type traits
int printf(const char *format, ...);

// Full template specialization
template<typename T>
class TypeName {
public:
    int id;
    TypeName() : id(0) {}
    int get() { return id; }
};

template<>
class TypeName<int> {
public:
    int id;
    TypeName() : id(1) {}
    int get() { return id; }
};

template<>
class TypeName<char> {
public:
    int id;
    TypeName() : id(2) {}
    int get() { return id; }
};

// Template function specialization
template<typename T>
int type_id() { return 0; }

template<>
int type_id<int>() { return 10; }

template<>
int type_id<char>() { return 20; }

// Template with non-type param
template<int N>
int fixed_value() { return N; }

// Template type alias
template<typename T>
using Ref = T;

int main() {
    printf("=== Canon C++17: Specialization + constexpr ===\n\n");
    int pass = 0;
    int total = 0;

    // Class specialization
    TypeName<int> ti;
    TypeName<char> tc;
    printf("TypeName<int>.id = %d\n", ti.get());
    printf("TypeName<char>.id = %d\n", tc.get());
    total++; if (ti.get() == 1) { pass++; } else { printf("FAIL: TypeName<int>\n"); }
    total++; if (tc.get() == 2) { pass++; } else { printf("FAIL: TypeName<char>\n"); }

    // Function specialization
    int id_int = type_id<int>();
    int id_char = type_id<char>();
    printf("type_id<int>() = %d\n", id_int);
    printf("type_id<char>() = %d\n", id_char);
    total++; if (id_int == 10) { pass++; } else { printf("FAIL: type_id<int>\n"); }
    total++; if (id_char == 20) { pass++; } else { printf("FAIL: type_id<char>\n"); }

    // Non-type template
    int v5 = fixed_value<5>();
    int v42 = fixed_value<42>();
    printf("fixed_value<5>() = %d\n", v5);
    printf("fixed_value<42>() = %d\n", v42);
    total++; if (v5 == 5) { pass++; } else { printf("FAIL: fixed<5>\n"); }
    total++; if (v42 == 42) { pass++; } else { printf("FAIL: fixed<42>\n"); }

    // Template alias
    Ref<int> ri = 99;
    printf("Ref<int> = %d\n", ri);
    total++; if (ri == 99) { pass++; } else { printf("FAIL: Ref<int>\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}