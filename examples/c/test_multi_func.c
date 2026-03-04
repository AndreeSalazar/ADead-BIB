#include <stdio.h>

int square(int x) { return x * x; }
int cube(int x) { return x * x * x; }
int double_val(int x) { return x + x; }
int half(int x) { return x / 2; }
int negate(int x) { return 0 - x; }
int identity(int x) { return x; }

int apply_twice(int x, int first_op) {
    int r = x;
    if (first_op == 0) {
        r = square(r);
        r = square(r);
    } else {
        r = double_val(r);
        r = double_val(r);
    }
    return r;
}

int main() {
    printf("square(5)=%d\n", square(5));
    printf("cube(3)=%d\n", cube(3));
    printf("double(7)=%d\n", double_val(7));
    printf("half(100)=%d\n", half(100));
    printf("negate(42)=%d\n", negate(42));
    printf("apply_twice(3,0)=%d\n", apply_twice(3, 0));
    printf("apply_twice(3,1)=%d\n", apply_twice(3, 1));
    return 0;
}
