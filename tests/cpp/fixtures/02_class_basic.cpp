// ADead-BIB C++ Fixture 02: Basic Class
// Tests class definition, fields, method, constructor

int printf(const char *format, ...);

class Counter {
public:
    int value;

    Counter(int v) {
        value = v;
    }

    int get() {
        return value;
    }

    void increment() {
        value = value + 1;
    }
};

int main() {
    printf("Counter test\n");
    return 0;
}
