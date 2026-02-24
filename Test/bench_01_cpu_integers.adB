// ============================================================
// ADead-BIB Benchmark Suite — 01: CPU Integer ALU
// ============================================================
//
// 10 integer-only CPU benchmarks designed to stress the ALU.
// No semicolons · No for-loops · printf() output · int types
// ============================================================

// ----------------------------------------------------------
// 1. Sum 64-bit loop: sum from 1 to 10,000,000
// ----------------------------------------------------------
int bench_sum(int limit) {
    int i = 1
    int sum = 0
    while i <= limit {
        sum = sum + i
        i = i + 1
    }
    return sum
}

// ----------------------------------------------------------
// 2. Multiply 64-bit loop: multiply chain with mod reduction
// ----------------------------------------------------------
int bench_multiply(int iterations) {
    int i = 0
    int acc = 1
    while i < iterations {
        acc = acc * 31
        acc = acc % 1000000007
        i = i + 1
    }
    return acc
}

// ----------------------------------------------------------
// 3. Division 64-bit loop: repeated division operations
// ----------------------------------------------------------
int bench_division(int iterations) {
    int i = 0
    int acc = 1000000000
    while i < iterations {
        acc = acc / 3
        if acc < 2 {
            acc = 1000000000
        }
        i = i + 1
    }
    return acc
}

// ----------------------------------------------------------
// 4. Fibonacci iterativo: compute fibonacci(45) iteratively
// ----------------------------------------------------------
int bench_fibonacci(int n) {
    if n <= 1 {
        return n
    }
    int a = 0
    int b = 1
    int i = 2
    while i <= n {
        int tmp = a + b
        a = b
        b = tmp
        i = i + 1
    }
    return b
}

// ----------------------------------------------------------
// 5. Factorial iterativo: compute factorial of 20 iteratively
// ----------------------------------------------------------
int bench_factorial(int n) {
    int result = 1
    int i = 2
    while i <= n {
        result = result * i
        i = i + 1
    }
    return result
}

// ----------------------------------------------------------
// 6. GCD Euclidean: compute GCD of large numbers 1M times
// ----------------------------------------------------------
int gcd(int a, int b) {
    while b != 0 {
        int t = b
        b = a % b
        a = t
    }
    return a
}

int bench_gcd(int iterations) {
    int i = 0
    int result = 0
    int a = 1071232171
    int b = 462830917
    while i < iterations {
        result = gcd(a + i, b + i)
        i = i + 1
    }
    return result
}

// ----------------------------------------------------------
// 7. Popcount manual: count bits set in integers, 1M iters
// ----------------------------------------------------------
int popcount(int x) {
    int count = 0
    while x != 0 {
        count = count + (x % 2)
        x = x / 2
    }
    return count
}

int bench_popcount(int iterations) {
    int i = 1
    int total = 0
    while i <= iterations {
        total = total + popcount(i)
        i = i + 1
    }
    return total
}

// ----------------------------------------------------------
// 8. Binary search simulation: simulate binary search 1M times
// ----------------------------------------------------------
int binary_search(int target, int size) {
    int lo = 0
    int hi = size - 1
    int steps = 0
    while lo <= hi {
        int mid = lo + (hi - lo) / 2
        if mid == target {
            return steps
        }
        if mid < target {
            lo = mid + 1
        }
        if mid > target {
            hi = mid - 1
        }
        steps = steps + 1
    }
    return steps
}

int bench_binary_search(int iterations) {
    int i = 0
    int total_steps = 0
    int size = 1000000
    while i < iterations {
        int target = (i * 7919) % size
        total_steps = total_steps + binary_search(target, size)
        i = i + 1
    }
    return total_steps
}

// ----------------------------------------------------------
// 9. Collatz conjecture: sequence length for 1..100000
// ----------------------------------------------------------
int collatz_length(int n) {
    int steps = 0
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2
        }
        if n % 2 != 0 {
            if n != 1 {
                n = 3 * n + 1
            }
        }
        steps = steps + 1
    }
    return steps
}

int bench_collatz(int limit) {
    int i = 2
    int max_steps = 0
    while i <= limit {
        int s = collatz_length(i)
        if s > max_steps {
            max_steps = s
        }
        i = i + 1
    }
    return max_steps
}

// ----------------------------------------------------------
// 10. Hashing (multiplicative): hash on integer sequences
// ----------------------------------------------------------
int mult_hash(int value, int hash) {
    hash = hash * 16777619 + value
    hash = hash % 2147483647
    hash = hash * 31 + value / 256
    hash = hash % 2147483647
    return hash
}

int bench_hash(int iterations) {
    int i = 0
    int hash = 2166136261 % 2147483647
    while i < iterations {
        hash = mult_hash(i, hash)
        i = i + 1
    }
    return hash
}

// ============================================================
// Main — run all 10 benchmarks sequentially
// ============================================================
int main() {
    printf("============================================================\n")
    printf("  ADead-BIB Benchmark Suite — 01: CPU Integer ALU\n")
    printf("============================================================\n\n")

    // 1. Sum 64-bit
    printf("[1/10] Sum 64-bit loop (1..10,000)\n")
    int r1 = bench_sum(10000)
    printf("  Result: ")
    printf(r1)
    printf("\n\n")

    // 2. Multiply 64-bit
    printf("[2/10] Multiply 64-bit loop (10K iters)\n")
    int r2 = bench_multiply(10000)
    printf("  Result: ")
    printf(r2)
    printf("\n\n")

    // 3. Division 64-bit
    printf("[3/10] Division 64-bit loop (100K iters)\n")
    int r3 = bench_division(10000)
    printf("  Result: ")
    printf(r3)
    printf("\n\n")

    // 4. Fibonacci iterativo
    printf("[4/10] Fibonacci iterativo (n=45)\n")
    int r4 = bench_fibonacci(45)
    printf("  Result: ")
    printf(r4)
    printf("\n\n")

    // 5. Factorial iterativo
    printf("[5/10] Factorial iterativo (n=20)\n")
    int r5 = bench_factorial(20)
    printf("  Result: ")
    printf(r5)
    printf("\n\n")

    // 6. GCD Euclidean
    printf("[6/10] GCD Euclidean (10K iters)\n")
    int r6 = bench_gcd(1000)
    printf("  Result: ")
    printf(r6)
    printf("\n\n")

    // 7. Popcount manual
    printf("[7/10] Popcount manual (1K iters)\n")
    int r7 = bench_popcount(1000)
    printf("  Result: ")
    printf(r7)
    printf("\n\n")

    // 8. Binary search simulation
    printf("[8/10] Binary search simulation (1K iters)\n")
    int r8 = bench_binary_search(1000)
    printf("  Result: ")
    printf(r8)
    printf("\n\n")

    // 9. Collatz conjecture
    printf("[9/10] Collatz conjecture (1..1000)\n")
    int r9 = bench_collatz(100)
    printf("  Result (max steps): ")
    printf(r9)
    printf("\n\n")

    // 10. Hashing multiplicative
    printf("[10/10] Hashing multiplicative (10K iters)\n")
    int r10 = bench_hash(1000)
    printf("  Result: ")
    printf(r10)
    printf("\n\n")

    printf("============================================================\n")
    printf("  All benchmarks complete.\n")
    printf("============================================================\n")

    return 0
}
