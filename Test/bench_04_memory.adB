// ============================================================
// ADead-BIB Benchmark Suite — 04: Memory & Register Pressure
// ============================================================
// Tests register allocation, stack spilling, recursion depth,
// function call overhead, dependency chains, and ILP.
//
// Compilar: adeadc bench_04_memory.adB -o bench04.exe
// ============================================================

// ------------------------------------------------------------
// 1. Register Pressure Low — 3 variables, 10M iterations
// ------------------------------------------------------------
int bench_reg_low() {
    int a = 0
    int b = 1
    int i = 0
    while i < 10000000 {
        a = a + b
        b = b + 1
        i = i + 1
    }
    return a + b
}

// ------------------------------------------------------------
// 2. Register Pressure Medium — 8 variables, 10M iterations
// ------------------------------------------------------------
int bench_reg_medium() {
    int a = 1
    int b = 2
    int c = 3
    int d = 4
    int e = 5
    int f = 6
    int g = 7
    int h = 8
    int i = 0
    while i < 10000000 {
        a = a + h
        b = b + a
        c = c + b
        d = d + c
        e = e + d
        f = f + e
        g = g + f
        h = h + g
        i = i + 1
    }
    return a + b + c + d + e + f + g + h
}

// ------------------------------------------------------------
// 3. Register Pressure High — 16 variables, 5M iterations
// ------------------------------------------------------------
int bench_reg_high() {
    int v0 = 1
    int v1 = 2
    int v2 = 3
    int v3 = 4
    int v4 = 5
    int v5 = 6
    int v6 = 7
    int v7 = 8
    int v8 = 9
    int v9 = 10
    int v10 = 11
    int v11 = 12
    int v12 = 13
    int v13 = 14
    int v14 = 15
    int v15 = 16
    int i = 0
    while i < 5000000 {
        v0 = v0 + v15
        v1 = v1 + v0
        v2 = v2 + v1
        v3 = v3 + v2
        v4 = v4 + v3
        v5 = v5 + v4
        v6 = v6 + v5
        v7 = v7 + v6
        v8 = v8 + v7
        v9 = v9 + v8
        v10 = v10 + v9
        v11 = v11 + v10
        v12 = v12 + v11
        v13 = v13 + v12
        v14 = v14 + v13
        v15 = v15 + v14
        i = i + 1
    }
    return v0 + v1 + v2 + v3 + v4 + v5 + v6 + v7 + v8 + v9 + v10 + v11 + v12 + v13 + v14 + v15
}

// ------------------------------------------------------------
// 4. Stack Depth — Deep Recursion (10000 levels)
// ------------------------------------------------------------
int deep_recurse(int n, int acc) {
    if n <= 0 {
        return acc
    }
    return deep_recurse(n - 1, acc + n)
}

int bench_deep_recursion() {
    return deep_recurse(10000, 0)
}

// ------------------------------------------------------------
// 5. Function Call Overhead — tiny function, 10M calls
// ------------------------------------------------------------
int tiny_add(int x) {
    return x + 1
}

int bench_call_overhead() {
    int result = 0
    int i = 0
    while i < 10000000 {
        result = tiny_add(result)
        i = i + 1
    }
    return result
}

// ------------------------------------------------------------
// 6. Variable Dependency Chain — 10M iterations
// ------------------------------------------------------------
int bench_dep_chain() {
    int a = 1
    int b = 2
    int c = 3
    int d = 4
    int e = 5
    int f = 6
    int g = 7
    int h = 8
    int i = 0
    while i < 10000000 {
        a = a + 1
        b = b + a
        c = c + b
        d = d + c
        e = e + d
        f = f + e
        g = g + f
        h = h + g
        i = i + 1
    }
    return h
}

// ------------------------------------------------------------
// 7. Independent Operations — 10M iterations, ILP test
// ------------------------------------------------------------
int bench_independent() {
    int a = 0
    int b = 100
    int c = 200
    int d = 300
    int e = 400
    int f = 500
    int g = 600
    int h = 700
    int i = 0
    while i < 10000000 {
        a = a + 3
        b = b + 5
        c = c + 7
        d = d + 11
        e = e + 13
        f = f + 17
        g = g + 19
        h = h + 23
        i = i + 1
    }
    return a + b + c + d + e + f + g + h
}

// ------------------------------------------------------------
// 8. Memcpy Simulation — copy 16 vars, 5M iterations
// ------------------------------------------------------------
int bench_memcpy_sim() {
    // Source group
    int s0 = 10
    int s1 = 20
    int s2 = 30
    int s3 = 40
    int s4 = 50
    int s5 = 60
    int s6 = 70
    int s7 = 80
    int s8 = 90
    int s9 = 100
    int s10 = 110
    int s11 = 120
    int s12 = 130
    int s13 = 140
    int s14 = 150
    int s15 = 160

    // Destination group
    int d0 = 0
    int d1 = 0
    int d2 = 0
    int d3 = 0
    int d4 = 0
    int d5 = 0
    int d6 = 0
    int d7 = 0
    int d8 = 0
    int d9 = 0
    int d10 = 0
    int d11 = 0
    int d12 = 0
    int d13 = 0
    int d14 = 0
    int d15 = 0

    int i = 0
    while i < 5000000 {
        d0 = s0
        d1 = s1
        d2 = s2
        d3 = s3
        d4 = s4
        d5 = s5
        d6 = s6
        d7 = s7
        d8 = s8
        d9 = s9
        d10 = s10
        d11 = s11
        d12 = s12
        d13 = s13
        d14 = s14
        d15 = s15

        // Mutate source for next iteration
        s0 = s0 + 1
        s1 = s1 + 1
        s2 = s2 + 1
        s3 = s3 + 1
        s4 = s4 + 1
        s5 = s5 + 1
        s6 = s6 + 1
        s7 = s7 + 1
        s8 = s8 + 1
        s9 = s9 + 1
        s10 = s10 + 1
        s11 = s11 + 1
        s12 = s12 + 1
        s13 = s13 + 1
        s14 = s14 + 1
        s15 = s15 + 1
        i = i + 1
    }
    return d0 + d1 + d2 + d3 + d4 + d5 + d6 + d7 + d8 + d9 + d10 + d11 + d12 + d13 + d14 + d15
}

// ============================================================
// Main — run all benchmarks and print checksums
// ============================================================
int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite - 04: Memory & Register Pressure\n")
    printf("============================================================\n\n")

    // 1. Register pressure low
    printf("[1] Register Pressure Low (3 vars, 10M iter)...\n")
    int r1 = bench_reg_low()
    printf("    checksum = ")
    printf(r1)
    printf("\n\n")

    // 2. Register pressure medium
    printf("[2] Register Pressure Medium (8 vars, 10M iter)...\n")
    int r2 = bench_reg_medium()
    printf("    checksum = ")
    printf(r2)
    printf("\n\n")

    // 3. Register pressure high
    printf("[3] Register Pressure High (16 vars, 5M iter)...\n")
    int r3 = bench_reg_high()
    printf("    checksum = ")
    printf(r3)
    printf("\n\n")

    // 4. Deep recursion
    printf("[4] Deep Recursion (10000 levels)...\n")
    int r4 = bench_deep_recursion()
    printf("    checksum = ")
    printf(r4)
    printf("\n\n")

    // 5. Function call overhead
    printf("[5] Function Call Overhead (10M calls)...\n")
    int r5 = bench_call_overhead()
    printf("    checksum = ")
    printf(r5)
    printf("\n\n")

    // 6. Dependency chain
    printf("[6] Variable Dependency Chain (10M iter)...\n")
    int r6 = bench_dep_chain()
    printf("    checksum = ")
    printf(r6)
    printf("\n\n")

    // 7. Independent operations
    printf("[7] Independent Operations / ILP (10M iter)...\n")
    int r7 = bench_independent()
    printf("    checksum = ")
    printf(r7)
    printf("\n\n")

    // 8. Memcpy simulation
    printf("[8] Memcpy Simulation (16 vars, 5M iter)...\n")
    int r8 = bench_memcpy_sim()
    printf("    checksum = ")
    printf(r8)
    printf("\n\n")

    printf("============================================================\n")
    printf("All benchmarks completed.\n")
    printf("============================================================\n")

    return 0
}
