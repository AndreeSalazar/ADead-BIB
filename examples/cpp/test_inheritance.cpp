int printf(const char *format, ...);

class Shape {
public:
    int id;
    Shape(int i) : id(i) {}
    int area() { return 0; }
};

class Circle : public Shape {
public:
    int radius;
    Circle(int r) : Shape(1), radius(r) {}
    int area() { return 3 * radius * radius; }
};

class Rectangle : public Shape {
public:
    int w;
    int h;
    Rectangle(int w, int h) : Shape(2), w(w), h(h) {}
    int area() { return w * h; }
};

int main() {
    Circle c(10);
    Rectangle r(5, 8);
    printf("circle area=%d\n", c.area());
    printf("rect area=%d\n", r.area());
    printf("circle id=%d rect id=%d\n", c.id, r.id);
    return 0;
}
