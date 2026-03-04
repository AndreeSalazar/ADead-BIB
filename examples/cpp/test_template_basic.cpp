int printf(const char *format, ...);

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

int main() {
    printf("max(3,7)=%d\n", max_val(3, 7));
    printf("min(3,7)=%d\n", min_val(3, 7));
    printf("abs(-5)=%d\n", abs_val(-5));
    printf("abs(5)=%d\n", abs_val(5));
    return 0;
}
