#include <iostream>
struct Optional {
    int value;
    int has_val;
    Optional() : value(0), has_val(0) {}
    Optional(int v) : value(v), has_val(1) {}
    int has_value() { return has_val; }
    int get() { return value; }
};
Optional find_positive(int x) {
    if (x > 0) { return Optional(x); }
    return Optional();
}
int value_or(Optional o, int def) {
    if (o.has_value()) return o.get();
    return def;
}
int main() {
    Optional a = find_positive(42);
    Optional b = find_positive(-5);
    printf("a: has=%d val=%d\n", a.has_value(), value_or(a, -1));
    printf("b: has=%d val=%d\n", b.has_value(), value_or(b, -1));
    return 0;
}