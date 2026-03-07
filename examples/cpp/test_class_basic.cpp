#include <iostream>
class Counter {
public:
    int value;
    Counter() : value(0) {}
    void inc() { value++; }
    void dec() { value--; }
    int get() { return value; }
};
int main() { Counter c; c.inc(); c.inc(); c.inc(); c.dec(); printf("count=%d\n", c.get()); return 0; }