// ============================================================
// ADead-BIB Benchmark Suite — 09: Stress Tests
// ============================================================
// Pushes the compiler and runtime to their limits:
//   1. Deep recursion (depth 50000)
//   2. Many function calls (20-function chain, 1M iterations)
//   3. Long computation chain (20 ops/iter, 5M iterations)
//   4. Many local variables (32 locals, 5M iterations)
//   5. Nested loops (4 levels, 10M iterations)
//   6. Recursive fibonacci(40)
//   7. Mixed operations (add/sub/mul/div/cmp, 10M iterations)
//   8. Many parameters (6+ params, 5M iterations)
// ============================================================

// ============================================================
// Helper: modulo (no % operator)
// ============================================================

int mod_int(int a, int b) {
    return a - (a / b) * b
}

// ============================================================
// TEST 1: Deep Recursion — recursive sum to 50000
// ============================================================

int deep_sum(int n) {
    if n <= 0 {
        return 0
    }
    return n + deep_sum(n - 1)
}

void test_deep_recursion() {
    printf("--- Test 1: Deep Recursion (depth 50000) ---\n")
    int result = deep_sum(50000)
    printf("deep_sum(50000) = ")
    printf(result)
    printf("\n\n")
}

// ============================================================
// TEST 2: Many Function Calls — 20 chained functions, 1M times
// ============================================================

int chain_01(int x) { return x + 1 }
int chain_02(int x) { return chain_01(x) + 1 }
int chain_03(int x) { return chain_02(x) + 1 }
int chain_04(int x) { return chain_03(x) + 1 }
int chain_05(int x) { return chain_04(x) + 1 }
int chain_06(int x) { return chain_05(x) + 1 }
int chain_07(int x) { return chain_06(x) + 1 }
int chain_08(int x) { return chain_07(x) + 1 }
int chain_09(int x) { return chain_08(x) + 1 }
int chain_10(int x) { return chain_09(x) + 1 }
int chain_11(int x) { return chain_10(x) + 1 }
int chain_12(int x) { return chain_11(x) + 1 }
int chain_13(int x) { return chain_12(x) + 1 }
int chain_14(int x) { return chain_13(x) + 1 }
int chain_15(int x) { return chain_14(x) + 1 }
int chain_16(int x) { return chain_15(x) + 1 }
int chain_17(int x) { return chain_16(x) + 1 }
int chain_18(int x) { return chain_17(x) + 1 }
int chain_19(int x) { return chain_18(x) + 1 }
int chain_20(int x) { return chain_19(x) + 1 }

void test_many_calls() {
    printf("--- Test 2: Many Function Calls (20-chain x 1M) ---\n")
    int i = 0
    int acc = 0
    while i < 1000000 {
        acc = acc + chain_20(i)
        i = i + 1
    }
    printf("accumulated = ")
    printf(acc)
    printf("\n\n")
}

// ============================================================
// TEST 3: Long Computation Chain — 20 dependent ops, 5M iters
// ============================================================

void test_long_chain() {
    printf("--- Test 3: Long Computation Chain (20 ops x 5M) ---\n")
    int i = 0
    int a = 1
    while i < 5000000 {
        a = a + 7
        a = a - 3
        a = a + 11
        a = a - 5
        a = a + 2
        a = a * 3
        a = a / 2
        a = a + 13
        a = a - 9
        a = a + 1
        a = a + 6
        a = a - 4
        a = a + 8
        a = a - 2
        a = a + 5
        a = a * 2
        a = a / 3
        a = a + 10
        a = a - 7
        a = a + 3
        if a > 1000000000 {
            a = mod_int(a, 1000000)
        }
        i = i + 1
    }
    printf("final a = ")
    printf(a)
    printf("\n\n")
}

// ============================================================
// TEST 4: Many Local Variables — 32 locals, 5M iterations
// ============================================================

int many_locals_work(int seed) {
    int v00 = seed
    int v01 = seed + 1
    int v02 = seed + 2
    int v03 = seed + 3
    int v04 = seed + 4
    int v05 = seed + 5
    int v06 = seed + 6
    int v07 = seed + 7
    int v08 = seed + 8
    int v09 = seed + 9
    int v10 = seed + 10
    int v11 = seed + 11
    int v12 = seed + 12
    int v13 = seed + 13
    int v14 = seed + 14
    int v15 = seed + 15
    int v16 = seed + 16
    int v17 = seed + 17
    int v18 = seed + 18
    int v19 = seed + 19
    int v20 = seed + 20
    int v21 = seed + 21
    int v22 = seed + 22
    int v23 = seed + 23
    int v24 = seed + 24
    int v25 = seed + 25
    int v26 = seed + 26
    int v27 = seed + 27
    int v28 = seed + 28
    int v29 = seed + 29
    int v30 = seed + 30
    int v31 = seed + 31

    v00 = v00 + v01 + v02 + v03
    v04 = v04 + v05 + v06 + v07
    v08 = v08 + v09 + v10 + v11
    v12 = v12 + v13 + v14 + v15
    v16 = v16 + v17 + v18 + v19
    v20 = v20 + v21 + v22 + v23
    v24 = v24 + v25 + v26 + v27
    v28 = v28 + v29 + v30 + v31

    v00 = v00 + v04 + v08 + v12
    v16 = v16 + v20 + v24 + v28

    return v00 + v16
}

void test_many_locals() {
    printf("--- Test 4: Many Local Variables (32 locals x 5M) ---\n")
    int i = 0
    int acc = 0
    while i < 5000000 {
        acc = acc + many_locals_work(i)
        if acc > 1000000000 {
            acc = mod_int(acc, 1000000)
        }
        i = i + 1
    }
    printf("accumulated = ")
    printf(acc)
    printf("\n\n")
}

// ============================================================
// TEST 5: Nested Loops — 100 x 100 x 100 x 10 = 10M iterations
// ============================================================

void test_nested_loops() {
    printf("--- Test 5: Nested Loops (100x100x100x10 = 10M) ---\n")
    int total = 0
    int a = 0
    while a < 100 {
        int b = 0
        while b < 100 {
            int c = 0
            while c < 100 {
                int d = 0
                while d < 10 {
                    total = total + 1
                    d = d + 1
                }
                c = c + 1
            }
            b = b + 1
        }
        a = a + 1
    }
    printf("total iterations = ")
    printf(total)
    printf("\n\n")
}

// ============================================================
// TEST 6: Recursive Fibonacci — fib(40)
// ============================================================

int fib(int n) {
    if n <= 0 {
        return 0
    }
    if n == 1 {
        return 1
    }
    return fib(n - 1) + fib(n - 2)
}

void test_recursive_fib() {
    printf("--- Test 6: Recursive Fibonacci(40) ---\n")
    int result = fib(40)
    printf("fib(40) = ")
    printf(result)
    printf("\n\n")
}

// ============================================================
// TEST 7: Mixed Operations — add/sub/mul/div/cmp, 10M iters
// ============================================================

void test_mixed_ops() {
    printf("--- Test 7: Mixed Operations (10M iterations) ---\n")
    int i = 0
    int a = 1
    int b = 2
    int c = 3
    int count = 0
    while i < 10000000 {
        a = a + b
        b = a - c
        c = b * 3
        a = c / 2
        if a > b {
            count = count + 1
        }
        if a > 1000000000 {
            a = mod_int(a, 100000)
        }
        if b > 1000000000 {
            b = mod_int(b, 100000)
        }
        if c > 1000000000 {
            c = mod_int(c, 100000)
        }
        i = i + 1
    }
    printf("a = ")
    printf(a)
    printf(", count = ")
    printf(count)
    printf("\n\n")
}

// ============================================================
// TEST 8: Many Parameters — 6+ params, 5M calls
// ============================================================

int sum6(int p1, int p2, int p3, int p4, int p5, int p6) {
    return p1 + p2 + p3 + p4 + p5 + p6
}

int weighted6(int p1, int p2, int p3, int p4, int p5, int p6) {
    return p1 * 1 + p2 * 2 + p3 * 3 + p4 * 4 + p5 * 5 + p6 * 6
}

int mix8(int a, int b, int c, int d, int e, int f, int g, int h) {
    return (a + b) * (c - d) + (e * f) - (g + h)
}

void test_many_params() {
    printf("--- Test 8: Many Parameters (6+ params x 5M) ---\n")
    int i = 0
    int acc = 0
    while i < 5000000 {
        int s = sum6(i, i + 1, i + 2, i + 3, i + 4, i + 5)
        int w = weighted6(1, 2, 3, 4, 5, 6)
        int m = mix8(i, i + 1, i + 2, i + 3, i + 4, i + 5, i + 6, i + 7)
        acc = acc + s + w + m
        if acc > 1000000000 {
            acc = mod_int(acc, 1000000)
        }
        i = i + 1
    }
    printf("accumulated = ")
    printf(acc)
    printf("\n\n")
}

// ============================================================
// Main — run all stress tests
// ============================================================

int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite — 09: Stress Tests\n")
    printf("============================================================\n\n")

    test_deep_recursion()
    test_many_calls()
    test_long_chain()
    test_many_locals()
    test_nested_loops()
    test_recursive_fib()
    test_mixed_ops()
    test_many_params()

    printf("============================================================\n")
    printf("All stress tests completed.\n")
    printf("============================================================\n")
    return 0
}
