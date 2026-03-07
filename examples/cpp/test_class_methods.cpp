#include <iostream>
class Calculator {
public:
    int result;
    Calculator() : result(0) {}
    void add(int x) { result += x; }
    void sub(int x) { result -= x; }
    void mul(int x) { result *= x; }
    int get() { return result; }
    void reset() { result = 0; }
};
int main() { Calculator c; c.add(10); c.mul(3); c.sub(5); printf("result=%d\n", c.get()); return 0; }