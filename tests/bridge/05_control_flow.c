// ADead-BIB Bridge Test 05 — Control Flow
// Level: INTERMEDIATE
// Tests: if/else, for, while, do-while, switch, goto, nested loops

#include <stdio.h>

int fibonacci(int n) {
    if (n <= 1) return n;
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        int tmp = a + b;
        a = b;
        b = tmp;
    }
    return b;
}

int is_prime(int n) {
    if (n < 2) return 0;
    if (n == 2) return 1;
    if (n % 2 == 0) return 0;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return 0;
    }
    return 1;
}

int main() {
    printf("=== ADead-BIB Bridge Test 05: Control Flow ===\n");
    int pass = 0, fail = 0;

    // if/else chain
    int x = 42;
    if (x > 100) { fail++; printf("FAIL: if>100\n"); }
    else if (x > 50) { fail++; printf("FAIL: if>50\n"); }
    else if (x > 40) { pass++; }
    else { fail++; printf("FAIL: if else\n"); }

    // for loop — sum 1..100
    int sum = 0;
    for (int i = 1; i <= 100; i++) sum += i;
    if (sum == 5050) { pass++; } else { fail++; printf("FAIL: for sum=%d\n", sum); }

    // while loop
    int count = 0;
    int val = 1024;
    while (val > 1) { val /= 2; count++; }
    if (count == 10) { pass++; } else { fail++; printf("FAIL: while count=%d\n", count); }

    // do-while
    int dw = 0;
    do { dw++; } while (dw < 5);
    if (dw == 5) { pass++; } else { fail++; printf("FAIL: do-while\n"); }

    // switch
    int grade = 85;
    char letter = 'F';
    switch (grade / 10) {
        case 10: case 9: letter = 'A'; break;
        case 8: letter = 'B'; break;
        case 7: letter = 'C'; break;
        case 6: letter = 'D'; break;
        default: letter = 'F'; break;
    }
    if (letter == 'B') { pass++; } else { fail++; printf("FAIL: switch %c\n", letter); }

    // nested loops — multiplication table check
    int ok = 1;
    for (int i = 1; i <= 9; i++) {
        for (int j = 1; j <= 9; j++) {
            if (i * j != j * i) { ok = 0; break; }
        }
    }
    if (ok) { pass++; } else { fail++; printf("FAIL: nested loops\n"); }

    // fibonacci
    if (fibonacci(10) == 55) { pass++; } else { fail++; printf("FAIL: fib(10)=%d\n", fibonacci(10)); }
    if (fibonacci(20) == 6765) { pass++; } else { fail++; printf("FAIL: fib(20)\n"); }

    // primes
    if (is_prime(2) && is_prime(17) && is_prime(97)) { pass++; } else { fail++; printf("FAIL: primes\n"); }
    if (!is_prime(0) && !is_prime(1) && !is_prime(100)) { pass++; } else { fail++; printf("FAIL: non-primes\n"); }

    // break/continue
    int bc_sum = 0;
    for (int i = 0; i < 20; i++) {
        if (i % 2 == 0) continue;
        if (i > 10) break;
        bc_sum += i;
    }
    if (bc_sum == 1+3+5+7+9) { pass++; } else { fail++; printf("FAIL: break/continue %d\n", bc_sum); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 05: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
