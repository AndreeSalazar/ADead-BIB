#include <optional>
#include <variant>
#include <any>
#include <iostream>

int main() {
    // optional
    std::optional<int> opt = 42;
    if (opt.has_value()) {
        int v = opt.value();
        int v2 = *opt;
    }
    std::optional<int> empty;
    int def = empty.value_or(99);
    opt.reset();

    // variant
    std::variant<int, double, std::string> var = 42;
    int idx = var.index();
    int ival = std::get<int>(var);
    var = 3.14;
    double dval = std::get<double>(var);
    bool holds = std::holds_alternative<double>(var);

    // any
    std::any a = 42;
    int aval = std::any_cast<int>(a);
    a = std::string("hello");
    bool has = a.has_value();
    a.reset();

    std::cout << "optional=" << def << " variant=" << dval << std::endl;
    return 0;
}
