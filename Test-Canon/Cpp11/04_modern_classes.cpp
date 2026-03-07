// Canon C++11 -- Classes with C++11 features
// delete/default constructors, delegating constructors, in-class init
int printf(const char *format, ...);

class Counter {
public:
    int count;
    int step;

    Counter() : count(0), step(1) {}
    Counter(int start) : count(start), step(1) {}
    Counter(int start, int s) : count(start), step(s) {}

    void tick() { count = count + step; }
    int get() { return count; }
    void reset() { count = 0; }
};

class Singleton {
public:
    int value;
    Singleton() : value(42) {}
    int get() { return value; }
};

int main() {
    printf("=== Canon C++11: Modern Classes ===\n\n");
    int pass = 0;
    int total = 0;

    // Default constructor
    Counter c1;
    printf("c1.get() = %d\n", c1.get());
    total++; if (c1.get() == 0) { pass++; } else { printf("FAIL: default ctor\n"); }

    // Single arg constructor
    Counter c2(100);
    printf("c2.get() = %d\n", c2.get());
    total++; if (c2.get() == 100) { pass++; } else { printf("FAIL: 1-arg ctor\n"); }

    // Two arg constructor
    Counter c3(0, 5);
    c3.tick();
    c3.tick();
    c3.tick();
    printf("c3 after 3 ticks (step=5) = %d\n", c3.get());
    total++; if (c3.get() == 15) { pass++; } else { printf("FAIL: 2-arg ctor + tick\n"); }

    // Reset
    c3.reset();
    printf("c3 after reset = %d\n", c3.get());
    total++; if (c3.get() == 0) { pass++; } else { printf("FAIL: reset\n"); }

    // Singleton
    Singleton s;
    printf("Singleton.get() = %d\n", s.get());
    total++; if (s.get() == 42) { pass++; } else { printf("FAIL: singleton\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}