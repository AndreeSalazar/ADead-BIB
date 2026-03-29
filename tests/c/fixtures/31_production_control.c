// Test: Production control flow — complex patterns used in real code
// Expected: Compile + Run — every path verified
// Strict: Return codes validate correctness

#include <stdio.h>

// Fibonacci iterative
int fib(int n) {
    if (n <= 1) return n;
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        int t = a + b;
        a = b;
        b = t;
    }
    return b;
}

// Binary search
int bsearch_idx(const int *arr, int n, int target) {
    int lo = 0, hi = n - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (arr[mid] == target) return mid;
        else if (arr[mid] < target) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}

// State machine
int parse_number(const char *s) {
    int state = 0; // 0=start, 1=digits, 2=done
    int result = 0;
    while (*s) {
        switch (state) {
            case 0:
                if (*s >= '0' && *s <= '9') { state = 1; result = *s - '0'; }
                else if (*s == ' ') { /* skip */ }
                else return -1;
                break;
            case 1:
                if (*s >= '0' && *s <= '9') { result = result * 10 + (*s - '0'); }
                else { state = 2; }
                break;
            case 2:
                return result;
        }
        s++;
    }
    return result;
}

// Nested loops with break/continue
int count_primes(int limit) {
    int count = 0;
    for (int n = 2; n < limit; n++) {
        int is_prime = 1;
        for (int d = 2; d * d <= n; d++) {
            if (n % d == 0) { is_prime = 0; break; }
        }
        if (is_prime) count++;
    }
    return count;
}

int main() {
    printf("=== PRODUCTION: Control Flow ===\n");
    int pass = 0, fail = 0;

    // Fibonacci
    if (fib(0) == 0)   { pass++; } else { fail++; printf("FAIL: fib(0)\n"); }
    if (fib(1) == 1)   { pass++; } else { fail++; printf("FAIL: fib(1)\n"); }
    if (fib(10) == 55) { pass++; } else { fail++; printf("FAIL: fib(10)=%d\n", fib(10)); }
    if (fib(20) == 6765) { pass++; } else { fail++; printf("FAIL: fib(20)=%d\n", fib(20)); }

    // Binary search
    int sorted[] = {1, 3, 5, 7, 9, 11, 13, 15, 17, 19};
    if (bsearch_idx(sorted, 10, 7) == 3)    { pass++; } else { fail++; printf("FAIL: bsearch 7\n"); }
    if (bsearch_idx(sorted, 10, 1) == 0)    { pass++; } else { fail++; printf("FAIL: bsearch 1\n"); }
    if (bsearch_idx(sorted, 10, 19) == 9)   { pass++; } else { fail++; printf("FAIL: bsearch 19\n"); }
    if (bsearch_idx(sorted, 10, 8) == -1)   { pass++; } else { fail++; printf("FAIL: bsearch miss\n"); }

    // State machine
    if (parse_number("42") == 42)            { pass++; } else { fail++; printf("FAIL: parse 42\n"); }
    if (parse_number("  123x") == 123)       { pass++; } else { fail++; printf("FAIL: parse 123\n"); }
    if (parse_number("0") == 0)              { pass++; } else { fail++; printf("FAIL: parse 0\n"); }

    // Primes
    if (count_primes(10) == 4)   { pass++; } else { fail++; printf("FAIL: primes<10=%d\n", count_primes(10)); }
    if (count_primes(100) == 25) { pass++; } else { fail++; printf("FAIL: primes<100=%d\n", count_primes(100)); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== PRODUCTION: Control %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
