#include <header_main.h>
#include <iostream>
#include <memory>

struct Node {
    int value;
    std::shared_ptr<Node> next;
    Node(int v) : value(v) {}
    ~Node() { printf("~Node(%d)\n", value); }
};

int main() {
    auto up = std::make_unique<int>(42);
    printf("unique: %d\n", *up);
    up.reset();
    auto up2 = std::make_unique<int[]>(5);
    up2[0] = 10;
    printf("array: %d\n", up2[0]);
    auto sp1 = std::make_shared<Node>(1);
    auto sp2 = sp1;
    printf("use_count=%d\n", sp1.use_count());
    sp1.reset();
    printf("sp2 value=%d\n", sp2->value);
    std::weak_ptr<Node> wp = sp2;
    if (auto locked = wp.lock()) {
        printf("locked: %d\n", locked->value);
    }
    printf("expired=%d\n", wp.expired());
    std::unique_ptr<int> up3(new int(99));
    int* raw = up3.release();
    printf("raw=%d\n", *raw);
    delete raw;
    return 0;
}
