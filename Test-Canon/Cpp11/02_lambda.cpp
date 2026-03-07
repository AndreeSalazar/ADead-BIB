// Canon C++11 -- Lambda expressions
int printf(const char *format, ...);

int main() {
    printf("=== Canon C++11: Lambda Expressions ===\n\n");
    int pass = 0;
    int total = 0;

    // Basic lambda
    auto add = [](int a, int b) -> int { return a + b; };
    int r1 = add(10, 20);
    printf("add(10,20) = %d\n", r1);
    total++; if (r1 == 30) { pass++; } else { printf("FAIL: basic lambda\n"); }

    // Lambda with capture by value
    int factor = 5;
    auto mul = [factor](int x) -> int { return x * factor; };
    int r2 = mul(7);
    printf("mul(7) with factor=5: %d\n", r2);
    total++; if (r2 == 35) { pass++; } else { printf("FAIL: capture by value\n"); }

    // Lambda with capture by reference
    int counter = 0;
    auto inc = [&counter]() -> void { counter = counter + 1; };
    inc();
    inc();
    inc();
    printf("counter after 3 inc(): %d\n", counter);
    total++; if (counter == 3) { pass++; } else { printf("FAIL: capture by ref\n"); }

    // Lambda with default capture
    int a = 10;
    int b = 20;
    auto sum_all = [=]() -> int { return a + b; };
    int r3 = sum_all();
    printf("sum_all() = %d\n", r3);
    total++; if (r3 == 30) { pass++; } else { printf("FAIL: default capture\n"); }

    // Immediately invoked lambda
    int r4 = [](int x) -> int { return x * x; }(9);
    printf("[](9*9) = %d\n", r4);
    total++; if (r4 == 81) { pass++; } else { printf("FAIL: IIFE\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}