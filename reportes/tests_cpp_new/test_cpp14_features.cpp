#include <header_main.h>
#include <iostream>
#include <memory>
#include <vector>
#include <algorithm>

int main() {
    auto add = [](auto a, auto b) { return a + b; };
    printf("int: %d\n", add(3, 4));
    printf("double: %.2f\n", add(1.5, 2.5));
    int binary = 0b10101010;
    printf("0b10101010 = %d\n", binary);
    int million = 1000000;
    printf("million = %d\n", million);
    auto up = std::make_unique<int>(42);
    printf("unique: %d\n", *up);
    auto arr = std::make_unique<int[]>(5);
    arr[0] = 10; arr[1] = 20;
    printf("arr[0]=%d arr[1]=%d\n", arr[0], arr[1]);
    std::vector<int> v = {5, 3, 1, 4, 2};
    std::sort(v.begin(), v.end(), [](auto a, auto b) { return a < b; });
    for (auto& x : v) printf("%d ", x);
    printf("\n");
    return 0;
}
