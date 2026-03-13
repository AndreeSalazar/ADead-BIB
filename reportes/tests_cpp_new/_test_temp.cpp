#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <functional>
#include <memory>

enum class Color { Red, Green, Blue };
using IntVec = std::vector<int>;
constexpr int square(int x) { return x * x; }

class Widget {
    int id;
    std::string name;
public:
    Widget() : Widget(0, "default") {}
    Widget(int i, std::string n) : id(i), name(n) {}
    int get_id() const { return id; }
    std::string get_name() const { return name; }
};

int main() {
    auto x = 42;
    int* p = nullptr;
    static_assert(sizeof(int) >= 4, "int must be at least 4 bytes");
    constexpr int sq = square(5);
    printf("square(5) = %d\n", sq);
    IntVec nums = {10, 20, 30, 40, 50};
    auto sum_fn = [](int a, int b) { return a + b; };
    int total = 0;
    for (auto& n : nums) { total = sum_fn(total, n); }
    printf("total = %d\n", total);
    Color c = Color::Green;
    if (c == Color::Green) printf("green\n");
    Widget w1;
    Widget w2(1, "custom");
    auto up = std::make_unique<int>(100);
    printf("unique_ptr: %d\n", *up);
    auto sp = std::make_shared<int>(200);
    printf("shared_ptr: %d\n", *sp);
    std::function<int(int)> factorial = [&](int n) { return n <= 1 ? 1 : n * factorial(n - 1); };
    printf("5! = %d\n", factorial(5));
    return 0;
}
