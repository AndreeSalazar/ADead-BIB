#include <iostream>
using Integer = int;
using Float = double;
Integer add(Integer a, Integer b) { return a + b; }
Float multiply(Float a, Float b) { return a * b; }
Integer negate(Integer x) { return -x; }
int main() { Integer a = add(3, 4); printf("%d %d\n", a, negate(a)); return 0; }