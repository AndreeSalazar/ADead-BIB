#include <header_main.h>
#include <iostream>
#include <string>

int main() {
    // Basic cout output
    std::cout << "Hello World" << std::endl;
    std::cout << "int: " << 42 << std::endl;

    // Chain multiple values
    int x = 10;
    int y = 20;
    std::cout << "x=" << x << " y=" << y << " sum=" << (x + y) << std::endl;

    // String output
    std::string name = "ADead-BIB";
    std::cout << "Compiler: " << name << std::endl;

    // Multiple lines
    std::cout << "line1" << std::endl;
    std::cout << "line2" << std::endl;

    // Printf still works alongside cout
    printf("printf works too: %d\n", 99);

    return 0;
}
