#include <tuple>
#include <iostream>

int main() {
    // tuple
    std::tuple<int, double, std::string> t = std::make_tuple(1, 2.0, "hello");
    int a = std::get<0>(t);
    double b = std::get<1>(t);
    int sz = std::tuple_size<decltype(t)>::value;

    // tie
    int x;
    double y;
    std::string z;
    std::tie(x, y, z) = t;

    // structured bindings (C++17)
    auto [p, q, r] = t;

    // tuple_cat
    auto t2 = std::make_tuple(4, 5.0);
    auto t3 = std::tuple_cat(t, t2);

    std::cout << "tuple: " << a << ", " << b << std::endl;
    return 0;
}
