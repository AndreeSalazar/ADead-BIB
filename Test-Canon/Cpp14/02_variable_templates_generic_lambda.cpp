// Canon C++14 -- Variable templates, generic lambdas
int printf(const char *format, ...);

// Template class with default
template<typename T = int>
class Stack {
public:
    T data[16];
    int top;

    Stack() : top(0) { }

    void push(T val) {
        data[top] = val;
        top = top + 1;
    }

    T pop() {
        top = top - 1;
        return data[top];
    }

    T peek() { return data[top - 1]; }
    int size() { return top; }
    int empty() { return top == 0; }
};

int main() {
    printf("=== Canon C++14: Variable Templates + Stack ===\n\n");
    int pass = 0;
    int total = 0;

    // Stack with default type (int)
    Stack<int> s;
    s.push(10);
    s.push(20);
    s.push(30);
    printf("Stack size=%d peek=%d\n", s.size(), s.peek());
    total++; if (s.size() == 3) { pass++; } else { printf("FAIL: size\n"); }
    total++; if (s.peek() == 30) { pass++; } else { printf("FAIL: peek\n"); }

    int v1 = s.pop();
    int v2 = s.pop();
    printf("pop=%d pop=%d\n", v1, v2);
    total++; if (v1 == 30) { pass++; } else { printf("FAIL: pop1\n"); }
    total++; if (v2 == 20) { pass++; } else { printf("FAIL: pop2\n"); }
    total++; if (s.size() == 1) { pass++; } else { printf("FAIL: size after pop\n"); }

    // Generic lambda (C++14) -- auto parameters
    auto multiply = [](auto a, auto b) -> auto { return a * b; };
    int r1 = multiply(6, 7);
    printf("multiply(6,7) = %d\n", r1);
    total++; if (r1 == 42) { pass++; } else { printf("FAIL: generic lambda\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}