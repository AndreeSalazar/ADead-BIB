#include <header_main.h>
#include <string>
#include <type_traits>

// if constexpr is already parsed by cpp_parser (parse_if handles Constexpr token)
// This test verifies it compiles through the full pipeline

template<typename T>
int type_id() {
    if constexpr (std::is_integral<T>::value) {
        return 1;
    } else if constexpr (std::is_floating_point<T>::value) {
        return 2;
    } else {
        return 0;
    }
}

template<typename T>
T double_value(T val) {
    if constexpr (std::is_integral<T>::value) {
        return val * 2;
    } else {
        return val;
    }
}

int main() {
    printf("int type_id: %d\n", type_id<int>());
    printf("double type_id: %d\n", type_id<double>());
    printf("double 5: %d\n", double_value(5));
    return 0;
}
