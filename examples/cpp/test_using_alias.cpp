int printf(const char *format, ...);

using Integer = int;
using Byte = unsigned char;

Integer double_val(Integer x) { return x + x; }
Integer triple_val(Integer x) { return x * 3; }

class Point {
public:
    Integer x;
    Integer y;
    Point(Integer px, Integer py) : x(px), y(py) {}
    Integer distance_sq() { return x * x + y * y; }
};

int main() {
    Integer a = 42;
    printf("double(%d) = %d\n", a, double_val(a));
    printf("triple(%d) = %d\n", a, triple_val(a));
    Point p(3, 4);
    printf("dist_sq = %d\n", p.distance_sq());
    return 0;
}
