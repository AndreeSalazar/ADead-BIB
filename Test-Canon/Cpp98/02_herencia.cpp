// ============================================================
// Canon C++98 — §10 Herencia (Single Inheritance)
// ============================================================
// Intención: Herencia extiende un struct base con campos
// y métodos adicionales. Sin virtual, es simplemente
// concatenación de campos + funciones con prefijo.
//
// C++98 §10: "A class can be derived from one or more
// base classes."
//
// ADead-BIB: Base fields primero, luego Derived fields.
// Métodos no-virtual → llamada directa (call addr).
// ============================================================

int printf(const char *format, ...);

// --- Base class ---
class Shape {
public:
    int id;
    int color;

    Shape() : id(0), color(0) {}
    Shape(int id, int color) : id(id), color(color) {}

    int getId() { return id; }
    int getColor() { return color; }
};

// --- Derived: Circle ---
class Circle : public Shape {
public:
    int radius;

    Circle(int r) : Shape(1, 0xFF0000), radius(r) {}

    int area() {
        return 3 * radius * radius;
    }

    int circumference() {
        return 2 * 3 * radius;
    }
};

// --- Derived: Rectangle ---
class Rectangle : public Shape {
public:
    int width;
    int height;

    Rectangle(int w, int h) : Shape(2, 0x00FF00), width(w), height(h) {}

    int area() {
        return width * height;
    }

    int perimeter() {
        return 2 * (width + height);
    }
};

// --- Derived: Triangle ---
class Triangle : public Shape {
public:
    int base;
    int height;

    Triangle(int b, int h) : Shape(3, 0x0000FF), base(b), height(h) {}

    int area() {
        return (base * height) / 2;
    }
};

// --- Multi-level inheritance ---
class Animal {
public:
    int age;
    int weight;

    Animal(int a, int w) : age(a), weight(w) {}
    int getAge() { return age; }
};

class Dog : public Animal {
public:
    int tricks;

    Dog(int a, int w, int t) : Animal(a, w), tricks(t) {}

    int totalSkills() {
        return tricks + age;
    }
};

int main() {
    printf("=== Canon C++98: Herencia ===\n\n");

    // --- Circle ---
    Circle c(10);
    printf("Circle (r=10):\n");
    printf("  id = %d\n", c.getId());
    printf("  area = %d\n", c.area());
    printf("  circumference = %d\n", c.circumference());

    // --- Rectangle ---
    Rectangle r(5, 8);
    printf("\nRectangle (5x8):\n");
    printf("  id = %d\n", r.getId());
    printf("  area = %d\n", r.area());
    printf("  perimeter = %d\n", r.perimeter());

    // --- Triangle ---
    Triangle t(6, 10);
    printf("\nTriangle (base=6, h=10):\n");
    printf("  id = %d\n", t.getId());
    printf("  area = %d\n", t.area());

    // --- Multi-level ---
    Dog rex(5, 30, 10);
    printf("\nDog:\n");
    printf("  age = %d\n", rex.getAge());
    printf("  tricks = %d\n", rex.tricks);
    printf("  totalSkills = %d\n", rex.totalSkills());

    // --- Base members accessed from derived ---
    printf("\nInherited members:\n");
    printf("  circle.color = 0x%X\n", c.color);
    printf("  rect.color = 0x%X\n", r.color);
    printf("  triangle.color = 0x%X\n", t.color);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (c.area() == 300)         { pass++; } else { printf("FAIL: circle area\n"); }
    total++; if (c.getId() == 1)          { pass++; } else { printf("FAIL: circle id\n"); }
    total++; if (r.area() == 40)          { pass++; } else { printf("FAIL: rect area\n"); }
    total++; if (r.perimeter() == 26)     { pass++; } else { printf("FAIL: rect perim\n"); }
    total++; if (t.area() == 30)          { pass++; } else { printf("FAIL: tri area\n"); }
    total++; if (rex.totalSkills() == 15) { pass++; } else { printf("FAIL: dog skills\n"); }
    total++; if (rex.getAge() == 5)       { pass++; } else { printf("FAIL: dog age\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
