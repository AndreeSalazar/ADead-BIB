#include <header_main.h>
#include <iostream>
#include <optional>
#include <variant>
#include <any>
#include <string_view>
#include <string>
#include <map>
#include <type_traits>
#include <vector>

std::optional<int> find_value(int key) {
    if (key == 42) return 100;
    return {};
}

int main() {
    auto [x, y] = std::make_pair(10, 20);
    printf("x=%d y=%d\n", x, y);
    std::map<std::string, int> m;
    m["a"] = 1; m["b"] = 2;
    for (auto& [key, val] : m) {
        printf("%s=%d\n", key.c_str(), val);
    }
    auto opt = find_value(42);
    if (opt.has_value()) {
        printf("found: %d\n", opt.value());
    }
    auto opt2 = find_value(0);
    printf("default: %d\n", opt2.value_or(-1));
    std::variant<int, double, std::string> var = 42;
    std::any a = 3.14;
    std::string_view sv = "hello world";
    printf("sv: %s len=%d\n", sv.data(), sv.size());
    bool is_int = std::is_integral<int>::value;
    bool is_flt = std::is_floating_point<double>::value;
    printf("int=%d float=%d\n", is_int, is_flt);
    std::vector<int> v = {1, 2, 3, 4, 5};
    int sum = 0;
    for (auto& n : v) sum += n;
    printf("sum=%d\n", sum);
    return 0;
}
