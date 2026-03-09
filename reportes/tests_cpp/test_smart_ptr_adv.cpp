#include <memory>
#include <iostream>

struct Node {
    int value;
    std::shared_ptr<Node> next;
    std::weak_ptr<Node> parent;
    Node(int v) : value(v) {}
    ~Node() { std::cout << "~Node(" << value << ")" << std::endl; }
};

int main() {
    // unique_ptr
    auto up = std::make_unique<int>(42);
    int val = *up;
    up.reset();
    
    auto up2 = std::make_unique<int[]>(10);
    up2[0] = 1;
    
    std::unique_ptr<int> up3(new int(100));
    int* raw = up3.release();
    delete raw;

    // shared_ptr
    auto sp1 = std::make_shared<Node>(1);
    auto sp2 = sp1;
    int count = sp1.use_count();
    sp1.reset();
    
    // weak_ptr
    std::weak_ptr<Node> wp = sp2;
    if (auto locked = wp.lock()) {
        std::cout << "locked: " << locked->value << std::endl;
    }
    bool expired = wp.expired();
    
    // enable_shared_from_this — advanced
    // custom deleter
    auto custom = std::shared_ptr<int>(new int(50), [](int* p) {
        std::cout << "custom delete " << *p << std::endl;
        delete p;
    });

    std::cout << "use_count=" << count << std::endl;
    return 0;
}
