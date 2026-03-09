#include <type_traits>
#include <iostream>

template<typename T>
void check_type() {
    std::cout << "is_integral: " << std::is_integral<T>::value << std::endl;
    std::cout << "is_floating_point: " << std::is_floating_point<T>::value << std::endl;
    std::cout << "is_arithmetic: " << std::is_arithmetic<T>::value << std::endl;
    std::cout << "is_pointer: " << std::is_pointer<T>::value << std::endl;
    std::cout << "is_reference: " << std::is_reference<T>::value << std::endl;
    std::cout << "is_const: " << std::is_const<T>::value << std::endl;
    std::cout << "is_signed: " << std::is_signed<T>::value << std::endl;
    std::cout << "is_unsigned: " << std::is_unsigned<T>::value << std::endl;
}

int main() {
    // Type traits
    bool a = std::is_same<int, int>::value;
    bool b = std::is_same<int, double>::value;
    
    // Type modifications
    typedef std::remove_const<const int>::type no_const;
    typedef std::remove_reference<int&>::type no_ref;
    typedef std::add_pointer<int>::type ptr_type;
    typedef std::add_const<int>::type const_type;
    typedef std::decay<const int&>::type decayed;
    typedef std::make_signed<unsigned int>::type signed_t;
    typedef std::make_unsigned<int>::type unsigned_t;
    typedef std::remove_pointer<int*>::type no_ptr;
    
    // Conditional
    typedef std::conditional<true, int, double>::type cond_type;
    typedef std::enable_if<true, int>::type enabled;
    
    // C++17 _v shortcuts
    bool c = std::is_integral_v<int>;
    bool d = std::is_same_v<int, int>;
    
    check_type<int>();
    check_type<double>();
    return 0;
}
