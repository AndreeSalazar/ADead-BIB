// ============================================================
// Canon C++98 — §9 Classes Básicas
// ============================================================
// Intención: Una class es un struct con métodos y control
// de acceso (public/private). Sin virtual, NO hay vtable
// — cero overhead respecto a C.
//
// C++98 §9: "A class is a type."
// C++98 §9.2: "Members of a class are data members,
// member functions, nested types..."
//
// ADead-BIB compila: class → struct + funciones con this*
// Constructor → función init. Destructor → función cleanup.
// ============================================================

int printf(const char *format, ...);

// --- Clase básica: Counter ---
class Counter {
public:
    int value;

    Counter() : value(0) {}
    Counter(int initial) : value(initial) {}

    void increment() { value = value + 1; }
    void decrement() { value = value - 1; }
    void add(int n) { value = value + n; }
    void reset() { value = 0; }
    int get() { return value; }
};

// --- Clase: Stack simple ---
class IntStack {
public:
    int data[16];
    int top;

    IntStack() : top(0) {}

    void push(int val) {
        if (top < 16) {
            data[top] = val;
            top = top + 1;
        }
    }

    int pop() {
        if (top > 0) {
            top = top - 1;
            return data[top];
        }
        return -1;
    }

    int peek() {
        if (top > 0) {
            return data[top - 1];
        }
        return -1;
    }

    int size() { return top; }
    int empty() { return top == 0; }
};

// --- Clase: Vec2 ---
class Vec2 {
public:
    int x;
    int y;

    Vec2() : x(0), y(0) {}
    Vec2(int x, int y) : x(x), y(y) {}

    Vec2 add(Vec2 other) {
        return Vec2(x + other.x, y + other.y);
    }

    int dot(Vec2 other) {
        return x * other.x + y * other.y;
    }

    int length_sq() {
        return x * x + y * y;
    }
};

int main() {
    printf("=== Canon C++98: Classes Básicas ===\n\n");

    // --- Counter ---
    Counter c;
    printf("Counter:\n");
    printf("  initial: %d\n", c.get());
    c.increment();
    c.increment();
    c.increment();
    printf("  after 3 inc: %d\n", c.get());
    c.decrement();
    printf("  after dec: %d\n", c.get());
    c.add(10);
    printf("  after add(10): %d\n", c.get());
    c.reset();
    printf("  after reset: %d\n", c.get());

    Counter c2(100);
    printf("  Counter(100): %d\n", c2.get());

    // --- Stack ---
    printf("\nIntStack:\n");
    IntStack s;
    printf("  empty: %d\n", s.empty());
    s.push(10);
    s.push(20);
    s.push(30);
    printf("  pushed 10, 20, 30\n");
    printf("  size: %d\n", s.size());
    printf("  peek: %d\n", s.peek());
    printf("  pop: %d\n", s.pop());
    printf("  pop: %d\n", s.pop());
    printf("  size after 2 pops: %d\n", s.size());

    // --- Vec2 ---
    printf("\nVec2:\n");
    Vec2 a(3, 4);
    Vec2 b(1, 2);
    Vec2 sum = a.add(b);
    printf("  (%d,%d) + (%d,%d) = (%d,%d)\n", a.x, a.y, b.x, b.y, sum.x, sum.y);
    printf("  dot = %d\n", a.dot(b));
    printf("  length² of (3,4) = %d\n", a.length_sq());

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (c.get() == 0)      { pass++; } else { printf("FAIL: reset\n"); }
    total++; if (c2.get() == 100)   { pass++; } else { printf("FAIL: ctor(100)\n"); }
    total++; if (s.size() == 1)     { pass++; } else { printf("FAIL: stack size\n"); }
    total++; if (sum.x == 4)        { pass++; } else { printf("FAIL: vec add x\n"); }
    total++; if (sum.y == 6)        { pass++; } else { printf("FAIL: vec add y\n"); }
    total++; if (a.dot(b) == 11)    { pass++; } else { printf("FAIL: dot\n"); }
    total++; if (a.length_sq()==25)  { pass++; } else { printf("FAIL: length_sq\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
