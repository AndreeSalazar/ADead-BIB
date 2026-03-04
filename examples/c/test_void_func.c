#include <stdio.h>

int global_x = 0;

void set_x(int val) {
    global_x = val;
}

int get_x(void) {
    return global_x;
}

void print_separator(void) {
    printf("--------\n");
}

int main() {
    set_x(42);
    printf("x=%d\n", get_x());
    print_separator();
    set_x(100);
    printf("x=%d\n", get_x());
    return 0;
}
