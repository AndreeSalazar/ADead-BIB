#include <iostream>
struct Pair { int first; int second; };
struct Triple { int a; int b; int c; };
Pair make_pair(int a, int b) { Pair p; p.first = a; p.second = b; return p; }
Triple make_triple(int a, int b, int c) { Triple t; t.a = a; t.b = b; t.c = c; return t; }
int sum_pair(Pair p) { return p.first + p.second; }
int sum_triple(Triple t) { return t.a + t.b + t.c; }
int main() {
    Pair p = make_pair(10, 20);
    Triple t = make_triple(1, 2, 3);
    printf("pair=(%d,%d) sum=%d\n", p.first, p.second, sum_pair(p));
    printf("triple=(%d,%d,%d) sum=%d\n", t.a, t.b, t.c, sum_triple(t));
    return 0;
}