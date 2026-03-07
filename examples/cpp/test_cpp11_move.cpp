#include <iostream>
class Buffer {
public:
    int *data;
    int size;
    Buffer(int n) : size(n) { data = (int*)0; }
    Buffer(const Buffer &other) : size(other.size) { data = other.data; }
    int get_size() { return size; }
    void set(int idx, int val) { }
};
class Container {
public:
    int value;
    Container(int v) : value(v) {}
    Container(const Container &other) : value(other.value) {}
    int get() { return value; }
};
int main() {
    Buffer b1(100);
    Buffer b2(b1);
    printf("b1.size=%d b2.size=%d\n", b1.get_size(), b2.get_size());
    Container c1(42);
    Container c2(c1);
    printf("c1=%d c2=%d\n", c1.get(), c2.get());
    return 0;
}