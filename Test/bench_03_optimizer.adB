// ============================================================
// ADead-BIB Benchmark Suite — 03: Compiler Optimization Tests
// ============================================================
// Validates whether the compiler performs specific optimizations.
// Compare timing between optimized and non-optimized builds to
// measure the impact of each optimization pass.
//
// Compilar: adeadc bench_03_optimizer.adB -o bench03.exe
// ============================================================

// ----------------------------------------------------------------
// 1. Dead Code Elimination (DCE)
// ----------------------------------------------------------------
// The code after `return` is unreachable. If DCE works, the
// compiler removes it and this function is trivially fast.

int dce_function(int x) {
    return x + 1
    int dead = x * x * x * x
    dead = dead + dead + dead
    dead = dead * dead
    return dead
}

void bench_dce() {
    printf("--- Test 1: Dead Code Elimination ---\n")
    int i = 0
    int result = 0
    while i < 10000 {
        result = dce_function(i)
        i = i + 1
    }
    printf("Result: ")
    printf(result)
    printf("\n\n")
}

// ----------------------------------------------------------------
// 2. Constant Folding
// ----------------------------------------------------------------
// The expression `3 * 7 + 2 - 1` is all constants (= 22).
// If constant folding works, it becomes a single load per iter.

void bench_constant_folding() {
    printf("--- Test 2: Constant Folding ---\n")
    int i = 0
    int result = 0
    while i < 100000 {
        result = 3 * 7 + 2 - 1
        i = i + 1
    }
    printf("Result: ")
    printf(result)
    printf("\n\n")
}

// ----------------------------------------------------------------
// 3. Loop with Constant Trip Count (Unrolling)
// ----------------------------------------------------------------
// Inner loop has a fixed trip count of 10. If the compiler
// unrolls it, the inner loop overhead disappears.

void bench_loop_unroll() {
    printf("--- Test 3: Loop Unrolling (constant trip count) ---\n")
    int i = 0
    int sum = 0
    while i < 10000 {
        int j = 0
        while j < 10 {
            sum = sum + j
            j = j + 1
        }
        i = i + 1
    }
    printf("Result: ")
    printf(sum)
    printf("\n\n")
}

// ----------------------------------------------------------------
// 4. Inlining
// ----------------------------------------------------------------
// Tiny identity function called 10M times. If inlined, the call
// overhead (push/pop/call/ret) disappears entirely.

int identity(int x) {
    return x
}

void bench_inlining() {
    printf("--- Test 4: Function Inlining ---\n")
    int i = 0
    int result = 0
    while i < 100000 {
        result = identity(i)
        i = i + 1
    }
    printf("Result: ")
    printf(result)
    printf("\n\n")
}

// ----------------------------------------------------------------
// 5. Strength Reduction
// ----------------------------------------------------------------
// Multiplications by powers of 2 can be replaced with shifts:
//   x * 2  -> x << 1
//   x * 4  -> x << 2
//   x * 8  -> x << 3

void bench_strength_reduction() {
    printf("--- Test 5: Strength Reduction ---\n")
    int i = 0
    int r1 = 0
    int r2 = 0
    int r3 = 0
    while i < 100000 {
        r1 = i * 2
        r2 = i * 4
        r3 = i * 8
        i = i + 1
    }
    int result = r1 + r2 + r3
    printf("Result: ")
    printf(result)
    printf("\n\n")
}

// ----------------------------------------------------------------
// 6. Algebraic Simplification
// ----------------------------------------------------------------
// x + 0, x * 1, x - 0 are all identity operations.
// If the compiler recognizes them, they collapse to just x.

void bench_algebraic_simplification() {
    printf("--- Test 6: Algebraic Simplification ---\n")
    int i = 0
    int result = 0
    while i < 100000 {
        int a = i + 0
        int b = a * 1
        int c = b - 0
        result = c
        i = i + 1
    }
    printf("Result: ")
    printf(result)
    printf("\n\n")
}

// ----------------------------------------------------------------
// 7. Common Subexpression Elimination (CSE)
// ----------------------------------------------------------------
// (a + b) appears twice in the expression (a + b) * (a + b).
// If CSE works, the addition is computed only once per iteration.

void bench_cse() {
    printf("--- Test 7: Common Subexpression Elimination ---\n")
    int i = 0
    int result = 0
    int a = 3
    int b = 4
    while i < 100000 {
        result = (a + b) * (a + b)
        i = i + 1
    }
    printf("Result: ")
    printf(result)
    printf("\n\n")
}

// ----------------------------------------------------------------
// 8. Loop-Invariant Code Motion (LICM)
// ----------------------------------------------------------------
// The computation `base * factor + offset` does not depend on
// the loop variable. If LICM works, it is hoisted outside.

void bench_licm() {
    printf("--- Test 8: Loop-Invariant Code Motion ---\n")
    int base = 17
    int factor = 13
    int offset = 5
    int i = 0
    int result = 0
    while i < 100000 {
        result = base * factor + offset
        i = i + 1
    }
    printf("Result: ")
    printf(result)
    printf("\n\n")
}

// ----------------------------------------------------------------
// Main — run all optimization benchmarks
// ----------------------------------------------------------------

int main() {
    printf("============================================================\n")
    printf("  ADead-BIB Benchmark Suite - 03: Compiler Optimizations\n")
    printf("============================================================\n")
    printf("  Compare optimized vs non-optimized build times.\n")
    printf("============================================================\n\n")

    bench_dce()
    bench_constant_folding()
    bench_loop_unroll()
    bench_inlining()
    bench_strength_reduction()
    bench_algebraic_simplification()
    bench_cse()
    bench_licm()

    printf("============================================================\n")
    printf("  All optimization benchmarks complete.\n")
    printf("============================================================\n")
    return 0
}
