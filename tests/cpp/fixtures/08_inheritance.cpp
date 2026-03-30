// ADead-BIB C++ Fixture 08: Inheritance
// Tests class inheritance, base class embedding

int printf(const char *format, ...);

class Shape {
public:
    int x;
    int y;
    int area() { return 0; }
};

class Rectangle : public Shape {
public:
    int width;
    int height;
    int area() { return width * height; }
};

int main() {
    printf("Inheritance test\n");
    return 0;
}
