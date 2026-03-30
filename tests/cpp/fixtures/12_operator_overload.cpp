// ADead-BIB C++ Fixture 12: Operator Overloading
// Tests operator overload → mangled function names

int printf(const char *format, ...);

class Vec2 {
public:
    int x;
    int y;

    Vec2(int a, int b) { x = a; y = b; }

    int operator+(int rhs) { return x + y + rhs; }
    int operator==(int rhs) { return x == rhs; }
};

int main() {
    printf("operator overload test\n");
    return 0;
}
