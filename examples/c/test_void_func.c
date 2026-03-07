#include <stdio.h>
int gx = 0;
void set_x(int v) { gx = v; }
int get_x() { return gx; }
void print_separator() { printf("---\n"); }
void print_value(int v) { printf("val=%d\n", v); }
int main() { set_x(42); print_value(get_x()); print_separator(); set_x(99); print_value(get_x()); return 0; }