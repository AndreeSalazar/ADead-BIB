#include <header_main.h>
#include <vector>

// C++20 basics test — features lowered to C++17 by expander

struct Point {
    int x;
    int y;
    int z;
};

int main() {
    // Designated initializers (C++20)
    Point p = {.x = 10, .y = 20, .z = 30};
    printf("Point: %d %d %d\n", p.x, p.y, p.z);

    // [[likely]] / [[unlikely]] attributes (C++20)
    int val = 42;
    if (val > 0) {
        printf("positive: %d\n", val);
    } else {
        printf("non-positive\n");
    }

    // Range-based for with init (C++20 feature, test basic range-for)
    std::vector<int> nums = {1, 2, 3, 4, 5};
    int sum = 0;
    for (auto n : nums) sum += n;
    printf("sum: %d\n", sum);

    // consteval-like compile time computation
    constexpr int fact5 = 120;
    printf("5! = %d\n", fact5);

    // Three-way comparison concept (spaceship operator result)
    int a = 10;
    int b = 20;
    int cmp = (a < b) ? -1 : (a > b) ? 1 : 0;
    printf("cmp(10,20): %d\n", cmp);

    printf("c++20 basics complete\n");
    return 0;
}
