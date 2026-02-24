// ============================================================
// ADead-BIB — Benchmark Completo (versión reducida para timing)
// ============================================================
// Ejecuta todos los algoritmos con iteraciones menores
// para medir que FUNCIONA y dar estadísticas reales.
// ============================================================

int mod_int(int a, int b) {
    return a - (a / b) * b
}

int abs_val(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

// --- CPU Integer: Sum ---
int bench_sum(int limit) {
    int i = 1
    int sum = 0
    while i <= limit {
        sum = sum + i
        i = i + 1
    }
    return sum
}

// --- CPU Integer: Multiply chain ---
int bench_multiply(int iterations) {
    int i = 0
    int acc = 1
    while i < iterations {
        acc = acc * 31
        acc = mod_int(acc, 1000000007)
        i = i + 1
    }
    return acc
}

// --- CPU Integer: Division ---
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

// --- Fibonacci iterativo ---
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

// --- Factorial ---
int bench_factorial(int n) {
    int result = 1
    int i = 2
    while i <= n {
        result = result * i
        i = i + 1
    }
    return result
}

// --- GCD ---
int gcd(int a, int b) {
    while b != 0 {
        int t = b
        b = mod_int(a, b)
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

// --- Popcount ---
int popcount(int x) {
    int count = 0
    while x != 0 {
        count = count + mod_int(x, 2)
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

// --- Binary Search ---
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
    int size = 100000
    while i < iterations {
        int target = mod_int(i * 7919, size)
        total_steps = total_steps + binary_search(target, size)
        i = i + 1
    }
    return total_steps
}

// --- Collatz ---
int collatz_length(int n) {
    int steps = 0
    while n != 1 {
        if mod_int(n, 2) == 0 {
            n = n / 2
        } else {
            n = 3 * n + 1
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

// --- Hash ---
int bench_hash(int iterations) {
    int i = 0
    int hash = mod_int(2166136261, 2147483647)
    while i < iterations {
        hash = hash * 16777619 + i
        hash = mod_int(hash, 2147483647)
        i = i + 1
    }
    return hash
}

// --- Primes (trial division) ---
int bench_primes(int limit) {
    int count = 0
    int n = 2
    while n <= limit {
        int is_prime = 1
        int d = 2
        while d * d <= n {
            if mod_int(n, d) == 0 {
                is_prime = 0
                d = n
            }
            d = d + 1
        }
        if is_prime == 1 {
            count = count + 1
        }
        n = n + 1
    }
    return count
}

// --- Newton sqrt ---
int isqrt(int n) {
    if n < 2 {
        return n
    }
    int x = n
    int y = (x + 1) / 2
    while y < x {
        x = y
        y = (x + n / x) / 2
    }
    return x
}

int bench_isqrt(int limit) {
    int checksum = 0
    int i = 1
    while i <= limit {
        checksum = checksum + isqrt(i)
        i = i + 1
    }
    return checksum
}

// --- Matrix 4x4 multiply ---
int bench_matrix(int iterations) {
    int a00 = 1000  int a01 = 100   int a02 = 0     int a03 = 0
    int a10 = 0     int a11 = 1000  int a12 = 100   int a13 = 0
    int a20 = 0     int a21 = 0     int a22 = 1000  int a23 = 100
    int a30 = 100   int a31 = 0     int a32 = 0     int a33 = 1000
    int s = 1000
    int iter = 0
    while iter < iterations {
        int c00 = (a00*a00 + a01*a10 + a02*a20 + a03*a30) / s
        int c11 = (a10*a01 + a11*a11 + a12*a21 + a13*a31) / s
        int c22 = (a20*a02 + a21*a12 + a22*a22 + a23*a32) / s
        int c33 = (a30*a03 + a31*a13 + a32*a23 + a33*a33) / s
        a00 = mod_int(c00, 100000)
        a11 = mod_int(c11, 100000)
        a22 = mod_int(c22, 100000)
        a33 = mod_int(c33, 100000)
        iter = iter + 1
    }
    return a00 + a11 + a22 + a33
}

// --- Sorting Network 8 elements ---
int bench_sort_network(int iterations) {
    int checksum = 0
    int seed = 73
    int iter = 0
    while iter < iterations {
        seed = seed * 1103515245 + 12345
        int s = seed  if s < 0 { s = 0 - s }
        int a = mod_int(s / 256, 1000)
        seed = seed * 1103515245 + 12345
        s = seed  if s < 0 { s = 0 - s }
        int b = mod_int(s / 256, 1000)
        seed = seed * 1103515245 + 12345
        s = seed  if s < 0 { s = 0 - s }
        int c = mod_int(s / 256, 1000)
        seed = seed * 1103515245 + 12345
        s = seed  if s < 0 { s = 0 - s }
        int d = mod_int(s / 256, 1000)
        int t = 0
        if a > b { t = a  a = b  b = t }
        if c > d { t = c  c = d  d = t }
        if a > c { t = a  a = c  c = t }
        if b > d { t = b  b = d  d = t }
        if b > c { t = b  b = c  c = t }
        checksum = checksum + a + d
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        iter = iter + 1
    }
    return checksum
}

// --- OOP-light: Point distance ---
int point_distance_sq(int x1, int y1, int x2, int y2) {
    int dx = x2 - x1
    int dy = y2 - y1
    return dx * dx + dy * dy
}

int bench_point_distance(int iterations) {
    int total = 0
    int i = 0
    while i < iterations {
        int x1 = mod_int(i, 100)
        int y1 = mod_int(i, 73)
        int x2 = mod_int(i, 41) + 50
        int y2 = mod_int(i, 37) + 30
        total = total + point_distance_sq(x1, y1, x2, y2)
        if total > 1000000000 {
            total = mod_int(total, 1000000)
        }
        i = i + 1
    }
    return total
}

// --- OOP-light: Vec3 cross product ---
int bench_vec3_cross(int iterations) {
    int checksum = 0
    int i = 0
    while i < iterations {
        int ax = mod_int(i, 17) + 1
        int ay = mod_int(i, 13) + 2
        int az = mod_int(i, 11) + 3
        int bx = mod_int(i, 7) + 4
        int by = mod_int(i, 19) + 1
        int bz = mod_int(i, 23) + 2
        int rx = ay * bz - az * by
        int ry = az * bx - ax * bz
        int rz = ax * by - ay * bx
        checksum = checksum + rx + ry + rz
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        i = i + 1
    }
    return checksum
}

// --- Crypto: Hash chain ---
int bench_hash_chain(int iterations) {
    int hash = mod_int(2166136261, 2147483647)
    int i = 0
    while i < iterations {
        hash = hash * 16777619 + i
        hash = mod_int(hash, 2147483647)
        hash = hash * 31 + i / 256
        hash = mod_int(hash, 2147483647)
        i = i + 1
    }
    return hash
}

// --- Pathfinding: Manhattan distance ---
int bench_manhattan(int iterations) {
    int total = 0
    int seed = 17
    int i = 0
    while i < iterations {
        seed = seed * 1103515245 + 12345
        int s = seed  if s < 0 { s = 0 - s }
        int x1 = mod_int(s, 1000)
        seed = seed * 1103515245 + 12345
        s = seed  if s < 0 { s = 0 - s }
        int y1 = mod_int(s, 1000)
        seed = seed * 1103515245 + 12345
        s = seed  if s < 0 { s = 0 - s }
        int x2 = mod_int(s, 1000)
        seed = seed * 1103515245 + 12345
        s = seed  if s < 0 { s = 0 - s }
        int y2 = mod_int(s, 1000)
        total = total + abs_val(x2 - x1) + abs_val(y2 - y1)
        if total > 1000000000 {
            total = mod_int(total, 1000000)
        }
        i = i + 1
    }
    return total
}

// --- Branch prediction ---
int bench_branches(int iterations) {
    int seed = 7
    int taken = 0
    int not_taken = 0
    int i = 0
    while i < iterations {
        seed = seed * 1103515245 + 12345
        int s = seed  if s < 0 { s = 0 - s }
        int bit = mod_int(s, 2)
        if bit == 1 {
            taken = taken + 1
        } else {
            not_taken = not_taken + 1
        }
        i = i + 1
    }
    return taken
}

// --- Deep recursion ---
int deep_sum(int n) {
    if n <= 0 {
        return 0
    }
    return n + deep_sum(n - 1)
}

// --- Modular exponentiation ---
int mod_pow(int base, int exp, int m) {
    int result = 1
    base = mod_int(base, m)
    while exp > 0 {
        if mod_int(exp, 2) == 1 {
            result = mod_int(result * base, m)
        }
        exp = exp / 2
        base = mod_int(base * base, m)
    }
    return result
}

// ============================================================
// MAIN — Run all benchmarks with timing-friendly sizes
// ============================================================
int main() {
    printf("============================================================\n")
    printf("  ADead-BIB Benchmark Suite — Full Results\n")
    printf("  Compiler: ADead-BIB v1.2.0 (Rust backend)\n")
    printf("  Target: Windows x86-64 PE\n")
    printf("============================================================\n\n")

    // === CPU INTEGER ===
    printf("=== CPU INTEGER ALU ===\n")

    printf("[01] Sum 1..10000: ")
    int r = bench_sum(10000)
    printf(r)
    printf("\n")

    printf("[02] Multiply chain 5000: ")
    r = bench_multiply(5000)
    printf(r)
    printf("\n")

    printf("[03] Division chain 5000: ")
    r = bench_division(5000)
    printf(r)
    printf("\n")

    printf("[04] Fibonacci(45): ")
    r = bench_fibonacci(45)
    printf(r)
    printf("\n")

    printf("[05] Factorial(20): ")
    r = bench_factorial(20)
    printf(r)
    printf("\n")

    printf("[06] GCD 1000 iters: ")
    r = bench_gcd(1000)
    printf(r)
    printf("\n")

    printf("[07] Popcount 1000: ")
    r = bench_popcount(1000)
    printf(r)
    printf("\n")

    printf("[08] Binary search 500: ")
    r = bench_binary_search(500)
    printf(r)
    printf("\n")

    printf("[09] Collatz max 1..1000: ")
    r = bench_collatz(1000)
    printf(r)
    printf("\n")

    printf("[10] Hash chain 5000: ")
    r = bench_hash(5000)
    printf(r)
    printf("\n")

    // === REAL ALGORITHMS ===
    printf("\n=== REAL ALGORITHMS ===\n")

    printf("[11] Primes up to 10000: ")
    r = bench_primes(10000)
    printf(r)
    printf("\n")

    printf("[12] Integer sqrt sum 1..10000: ")
    r = bench_isqrt(10000)
    printf(r)
    printf("\n")

    printf("[13] Matrix 4x4 multiply 1000: ")
    r = bench_matrix(1000)
    printf(r)
    printf("\n")

    printf("[14] Modular exp 7^65537 mod 1000003: ")
    r = mod_pow(7, 65537, 1000003)
    printf(r)
    printf("\n")

    // === OOP-LIGHT PATTERNS ===
    printf("\n=== OOP-LIGHT PATTERNS ===\n")

    printf("[15] Point2D distance 10000: ")
    r = bench_point_distance(10000)
    printf(r)
    printf("\n")

    printf("[16] Vec3 cross product 10000: ")
    r = bench_vec3_cross(10000)
    printf(r)
    printf("\n")

    printf("[17] Sorting network 10000: ")
    r = bench_sort_network(10000)
    printf(r)
    printf("\n")

    // === CRYPTO ===
    printf("\n=== CRYPTO & HASHING ===\n")

    printf("[18] Hash chain 5000: ")
    r = bench_hash_chain(5000)
    printf(r)
    printf("\n")

    // === PATHFINDING ===
    printf("\n=== PATHFINDING ===\n")

    printf("[19] Manhattan distance 10000: ")
    r = bench_manhattan(10000)
    printf(r)
    printf("\n")

    // === BRANCH PREDICTION ===
    printf("\n=== BRANCH PREDICTION ===\n")

    printf("[20] Random branches 10000: ")
    r = bench_branches(10000)
    printf(r)
    printf("\n")

    // === STRESS ===
    printf("\n=== STRESS TESTS ===\n")

    printf("[21] Deep recursion sum(5000): ")
    r = deep_sum(5000)
    printf(r)
    printf("\n")

    printf("\n============================================================\n")
    printf("  All 21 benchmarks completed successfully!\n")
    printf("============================================================\n")

    return 0
}
