// ============================================================
// Canon C++98 — §10.3 Virtual Functions y Polimorfismo
// ============================================================
// Intención: virtual indica que el método puede ser
// sobreescrito y se resuelve en runtime via vtable.
// ADead-BIB devirtualiza cuando es posible (tipo conocido
// en compilación → llamada directa, sin vtable).
//
// C++98 §10.3: "A function specified as virtual in a base
// class can be overridden in a derived class."
//
// override = el programador confirma que está sobreescribiendo
// ============================================================

int printf(const char *format, ...);

// --- Base con virtual ---
class Shape {
public:
    int x;
    int y;

    Shape(int x, int y) : x(x), y(y) {}

    virtual int area() {
        return 0;
    }

    virtual void describe() {
        printf("  Shape at (%d, %d)\n", x, y);
    }

    int getX() { return x; }
    int getY() { return y; }
};

// --- Circle override ---
class Circle : public Shape {
public:
    int radius;

    Circle(int x, int y, int r) : Shape(x, y), radius(r) {}

    int area() override {
        return 3 * radius * radius;
    }

    void describe() override {
        printf("  Circle at (%d,%d) r=%d area=%d\n", x, y, radius, area());
    }
};

// --- Rectangle override ---
class Rect : public Shape {
public:
    int w;
    int h;

    Rect(int x, int y, int w, int h) : Shape(x, y), w(w), h(h) {}

    int area() override {
        return w * h;
    }

    void describe() override {
        printf("  Rect at (%d,%d) %dx%d area=%d\n", x, y, w, h, area());
    }
};

// --- Animal hierarchy with virtual ---
class Animal {
public:
    int age;

    Animal(int a) : age(a) {}

    virtual void speak() {
        printf("  ...\n");
    }

    virtual int legCount() {
        return 0;
    }
};

class Dog : public Animal {
public:
    Dog(int a) : Animal(a) {}

    void speak() override {
        printf("  Woof! (age %d)\n", age);
    }

    int legCount() override {
        return 4;
    }
};

class Bird : public Animal {
public:
    Bird(int a) : Animal(a) {}

    void speak() override {
        printf("  Tweet! (age %d)\n", age);
    }

    int legCount() override {
        return 2;
    }
};

int main() {
    printf("=== Canon C++98: Virtual y Polimorfismo ===\n\n");

    // --- Devirtualized calls (tipo conocido) ---
    printf("Shapes (tipo conocido → devirtualizado):\n");
    Circle c(10, 20, 5);
    c.describe();

    Rect r(0, 0, 8, 6);
    r.describe();

    // --- Area ---
    printf("\nAreas:\n");
    printf("  Circle area = %d\n", c.area());
    printf("  Rect area = %d\n", r.area());

    // --- Base method (no virtual) ---
    printf("\nBase methods:\n");
    printf("  Circle pos = (%d, %d)\n", c.getX(), c.getY());
    printf("  Rect pos = (%d, %d)\n", r.getX(), r.getY());

    // --- Animals ---
    printf("\nAnimals:\n");
    Dog dog(3);
    Bird bird(1);

    printf("Dog: ");
    dog.speak();
    printf("  legs = %d\n", dog.legCount());

    printf("Bird: ");
    bird.speak();
    printf("  legs = %d\n", bird.legCount());

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (c.area() == 75)       { pass++; } else { printf("FAIL: circle area\n"); }
    total++; if (r.area() == 48)       { pass++; } else { printf("FAIL: rect area\n"); }
    total++; if (c.getX() == 10)       { pass++; } else { printf("FAIL: circle x\n"); }
    total++; if (dog.legCount() == 4)  { pass++; } else { printf("FAIL: dog legs\n"); }
    total++; if (bird.legCount() == 2) { pass++; } else { printf("FAIL: bird legs\n"); }
    total++; if (dog.age == 3)         { pass++; } else { printf("FAIL: dog age\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
