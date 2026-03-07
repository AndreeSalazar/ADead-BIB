#include <iostream>
namespace A { int val() { return 1; } namespace B { int val() { return 2; } } }
namespace C { int val() { return 3; } }
int val() { return 0; }
int main() { printf("%d %d %d %d\n", val(), A::val(), A::B::val(), C::val()); return 0; }