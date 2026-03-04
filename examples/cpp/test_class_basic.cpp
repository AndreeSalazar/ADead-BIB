int printf(const char *format, ...);

class Counter {
public:
    int value;

    Counter() : value(0) {}

    void increment() { value = value + 1; }
    void decrement() { value = value - 1; }
    int get() { return value; }
    void reset() { value = 0; }
};

int main() {
    Counter c;
    c.increment();
    c.increment();
    c.increment();
    printf("after 3 inc: %d\n", c.get());
    c.decrement();
    printf("after dec: %d\n", c.get());
    c.reset();
    printf("after reset: %d\n", c.get());
    return 0;
}
