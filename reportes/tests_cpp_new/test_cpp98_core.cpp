#include <header_main.h>
#include <iostream>
#include <vector>
#include <map>
#include <string>

namespace math {
    const double PI = 3.14159265;
    template<typename T>
    T max_val(T a, T b) { return (a > b) ? a : b; }
}

class Shape {
public:
    virtual double area() const = 0;
    virtual ~Shape() {}
};

class Circle : public Shape {
    double radius;
public:
    Circle(double r) : radius(r) {}
    double area() const { return math::PI * radius * radius; }
    friend bool operator==(const Circle& a, const Circle& b) { return a.radius == b.radius; }
};

class Rectangle : public Shape {
    double w, h;
public:
    Rectangle(double w, double h) : w(w), h(h) {}
    double area() const { return w * h; }
    static int count;
};
int Rectangle::count = 0;

template<typename T>
class Stack {
    std::vector<T> data;
public:
    void push(const T& val) { data.push_back(val); }
    T pop() { T v = data.back(); data.pop_back(); return v; }
    bool empty() const { return data.empty(); }
    int size() const { return data.size(); }
};

typedef void (*Callback)(int);
void on_event(int x) { printf("event: %d\n", x); }

int main() {
    Circle c(5.0);
    Rectangle r(3.0, 4.0);
    Shape* shapes[] = {&c, &r};
    for (int i = 0; i < 2; i++) {
        printf("area: %.2f\n", shapes[i]->area());
    }
    int m = math::max_val(10, 20);
    double dm = math::max_val(3.14, 2.71);
    Stack<int> s;
    s.push(1); s.push(2); s.push(3);
    printf("stack size: %d\n", s.size());
    std::vector<int> v = {1, 2, 3, 4, 5};
    std::map<std::string, int> ages;
    ages["alice"] = 30;
    ages["bob"] = 25;
    std::pair<int, double> p(42, 3.14);
    Callback cb = on_event;
    cb(99);
    const int& ref = m;
    printf("max=%d ref=%d\n", m, ref);
    return 0;
}
