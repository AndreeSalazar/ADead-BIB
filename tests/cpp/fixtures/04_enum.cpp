// ADead-BIB C++ Fixture 04: Enum and Enum Class
// Tests C++11 scoped enums

int printf(const char *format, ...);

enum Color { Red, Green, Blue };

enum class Direction : int {
    North = 0,
    South = 1,
    East = 2,
    West = 3
};

int main() {
    int c = Red;
    printf("Color Red = %d\n", c);
    return 0;
}
