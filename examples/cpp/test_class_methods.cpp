int printf(const char *format, ...);

class Calculator {
public:
    int result;

    Calculator() : result(0) {}

    void add(int n) { result = result + n; }
    void sub(int n) { result = result - n; }
    void mul(int n) { result = result * n; }
    void clear() { result = 0; }
    int get() { return result; }
};

int main() {
    Calculator calc;
    calc.add(10);
    calc.add(20);
    printf("10+20 = %d\n", calc.get());
    calc.mul(3);
    printf("*3 = %d\n", calc.get());
    calc.sub(10);
    printf("-10 = %d\n", calc.get());
    calc.clear();
    printf("clear = %d\n", calc.get());
    return 0;
}
