#include <header_main.h>
#include <type_traits>

int main() {
    // type_traits compile-time values
    printf("is_integral<int>: %d\n", std::is_integral<int>::value);
    printf("is_integral<double>: %d\n", std::is_integral<double>::value);
    printf("is_floating_point<double>: %d\n", std::is_floating_point<double>::value);
    printf("is_floating_point<int>: %d\n", std::is_floating_point<int>::value);
    printf("is_pointer<int*>: %d\n", std::is_pointer<int*>::value);
    printf("is_same<int,int>: %d\n", std::is_same<int,int>::value);
    printf("is_same<int,double>: %d\n", std::is_same<int,double>::value);

    // enable_if used with constexpr
    constexpr bool int_ok = std::is_integral<int>::value;
    constexpr bool dbl_ok = std::is_floating_point<double>::value;
    printf("int_ok: %d\n", int_ok);
    printf("dbl_ok: %d\n", dbl_ok);

    return 0;
}
