#include <header_main.h>

class Shape {
public:
    virtual const char* name() { return "Shape"; }
    virtual int sides() { return 0; }
    virtual ~Shape() {}
};

class Triangle : public Shape {
public:
    const char* name() override { return "Triangle"; }
    int sides() override { return 3; }
};

class Square : public Shape {
public:
    const char* name() override { return "Square"; }
    int sides() override { return 4; }
};

class Circle : public Shape {
    double radius;
public:
    Circle(double r) : radius(r) {}
    const char* name() override { return "Circle"; }
    int sides() override { return 0; }
};

void print_shape(Shape* s) {
    printf("%s has %d sides\n", s->name(), s->sides());
}

int main() {
    Triangle t;
    Square sq;
    Circle c(5.0);

    print_shape(&t);
    print_shape(&sq);
    print_shape(&c);

    // Array of base pointers
    Shape* shapes[3] = {&t, &sq, &c};
    int total_sides = 0;
    for (int i = 0; i < 3; i++) {
        total_sides += shapes[i]->sides();
    }
    printf("total sides: %d\n", total_sides);

    return 0;
}
