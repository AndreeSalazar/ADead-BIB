// ADead-BIB C++ Fixture 11: Virtual Methods & vtable
// Tests virtual method lowering with vtable generation

int printf(const char *format, ...);

class Animal {
public:
    int legs;
    virtual int speak() { return 0; }
    virtual int move() { return legs; }
};

class Dog : public Animal {
public:
    int speak() { return 1; }
};

int main() {
    printf("virtual test\n");
    return 0;
}
