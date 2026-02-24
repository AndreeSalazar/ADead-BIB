// ============================================================
// ADead-BIB Benchmark Suite — 08: Intensive Math
// ============================================================

// ----- Helper: modular exponentiation -----
int mod_exp(int base, int exp, int mod) {
    int result = 1
    base = base % mod
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % mod
        }
        exp = exp / 2
        base = (base * base) % mod
    }
    return result
}

// ----- Helper: Ackermann function -----
int ackermann(int m, int n) {
    if m == 0 {
        return n + 1
    }
    if n == 0 {
        return ackermann(m - 1, 1)
    }
    return ackermann(m - 1, ackermann(m, n - 1))
}

// ----- Bench 1: Matrix 4x4 Multiply (1M iterations) -----
void bench_matrix_multiply() {
    printf("--- Bench 1: Matrix 4x4 Multiply (1M iter) ---\n")

    // Matrix A
    int a00 = 1  int a01 = 2  int a02 = 3  int a03 = 4
    int a10 = 5  int a11 = 6  int a12 = 7  int a13 = 8
    int a20 = 9  int a21 = 10 int a22 = 11 int a23 = 12
    int a30 = 13 int a31 = 14 int a32 = 15 int a33 = 16

    // Matrix B
    int b00 = 16 int b01 = 15 int b02 = 14 int b03 = 13
    int b10 = 12 int b11 = 11 int b12 = 10 int b13 = 9
    int b20 = 8  int b21 = 7  int b22 = 6  int b23 = 5
    int b30 = 4  int b31 = 3  int b32 = 2  int b33 = 1

    // Result matrix
    int r00 = 0 int r01 = 0 int r02 = 0 int r03 = 0
    int r10 = 0 int r11 = 0 int r12 = 0 int r13 = 0
    int r20 = 0 int r21 = 0 int r22 = 0 int r23 = 0
    int r30 = 0 int r31 = 0 int r32 = 0 int r33 = 0

    int iter = 0
    while iter < 1000000 {
        // Row 0
        r00 = a00*b00 + a01*b10 + a02*b20 + a03*b30
        r01 = a00*b01 + a01*b11 + a02*b21 + a03*b31
        r02 = a00*b02 + a01*b12 + a02*b22 + a03*b32
        r03 = a00*b03 + a01*b13 + a02*b23 + a03*b33

        // Row 1
        r10 = a10*b00 + a11*b10 + a12*b20 + a13*b30
        r11 = a10*b01 + a11*b11 + a12*b21 + a13*b31
        r12 = a10*b02 + a11*b12 + a12*b22 + a13*b32
        r13 = a10*b03 + a11*b13 + a12*b23 + a13*b33

        // Row 2
        r20 = a20*b00 + a21*b10 + a22*b20 + a23*b30
        r21 = a20*b01 + a21*b11 + a22*b21 + a23*b31
        r22 = a20*b02 + a21*b12 + a22*b22 + a23*b32
        r23 = a20*b03 + a21*b13 + a22*b23 + a23*b33

        // Row 3
        r30 = a30*b00 + a31*b10 + a32*b20 + a33*b30
        r31 = a30*b01 + a31*b11 + a32*b21 + a33*b31
        r32 = a30*b02 + a31*b12 + a32*b22 + a33*b32
        r33 = a30*b03 + a31*b13 + a32*b23 + a33*b33

        iter = iter + 1
    }

    int trace = r00 + r11 + r22 + r33
    printf("  Trace (diagonal sum): ")
    printf(trace)
    printf("\n")
}

// ----- Bench 2: Determinant 3x3 (5M iterations) -----
void bench_determinant_3x3() {
    printf("--- Bench 2: Determinant 3x3 (5M iter) ---\n")

    int det = 0
    int iter = 0
    while iter < 5000000 {
        int m00 = iter % 7 + 1
        int m01 = iter % 5 + 2
        int m02 = iter % 3 + 3
        int m10 = iter % 11 + 1
        int m11 = iter % 9 + 2
        int m12 = iter % 4 + 1
        int m20 = iter % 6 + 3
        int m21 = iter % 8 + 1
        int m22 = iter % 13 + 2

        det = m00 * (m11*m22 - m12*m21) - m01 * (m10*m22 - m12*m20) + m02 * (m10*m21 - m11*m20)

        iter = iter + 1
    }

    printf("  Last determinant: ")
    printf(det)
    printf("\n")
}

// ----- Bench 3: Vector Cross Product (10M iterations) -----
void bench_cross_product() {
    printf("--- Bench 3: Vector Cross Product (10M iter) ---\n")

    int rx = 0
    int ry = 0
    int rz = 0
    int iter = 0
    while iter < 10000000 {
        int ax = iter % 17 + 1
        int ay = iter % 13 + 2
        int az = iter % 11 + 3
        int bx = iter % 7 + 4
        int by = iter % 19 + 1
        int bz = iter % 23 + 2

        rx = ay*bz - az*by
        ry = az*bx - ax*bz
        rz = ax*by - ay*bx

        iter = iter + 1
    }

    printf("  Last cross product: (")
    printf(rx)
    printf(", ")
    printf(ry)
    printf(", ")
    printf(rz)
    printf(")\n")
}

// ----- Bench 4: Distance Squared (10M iterations) -----
void bench_distance_squared() {
    printf("--- Bench 4: Distance Squared (10M iter) ---\n")

    int dist2 = 0
    int iter = 0
    while iter < 10000000 {
        int x1 = iter % 100
        int y1 = iter % 73
        int z1 = iter % 59
        int x2 = iter % 41 + 50
        int y2 = iter % 37 + 30
        int z2 = iter % 29 + 20

        int dx = x2 - x1
        int dy = y2 - y1
        int dz = z2 - z1
        dist2 = dx*dx + dy*dy + dz*dz

        iter = iter + 1
    }

    printf("  Last distance squared: ")
    printf(dist2)
    printf("\n")
}

// ----- Bench 5: Ackermann(3, 8) -----
void bench_ackermann() {
    printf("--- Bench 5: Ackermann(3, 8) ---\n")

    int result = ackermann(3, 8)

    printf("  Ackermann(3, 8) = ")
    printf(result)
    printf("\n")
}

// ----- Bench 6: Modular Power Tower -----
void bench_power_tower() {
    printf("--- Bench 6: Modular Power Tower ---\n")

    // Compute a^(b^c) mod m using mod_exp
    int a = 7
    int b = 13
    int c = 5
    int m = 1000000007

    // First compute b^c mod (m-1) by Fermat's little theorem for prime m
    int exp_inner = mod_exp(b, c, m - 1)

    // Then compute a^exp_inner mod m
    int result = mod_exp(a, exp_inner, m)

    printf("  7^(13^5) mod 1000000007 = ")
    printf(result)
    printf("\n")
}

// ----- Bench 7: Sum of Squares (i=1 to 10M) -----
void bench_sum_of_squares() {
    printf("--- Bench 7: Sum of Squares (10M) ---\n")

    int sum = 0
    int i = 1
    while i <= 10000000 {
        sum = sum + i * i
        i = i + 1
    }

    printf("  Sum of squares (mod 2^32 overflow): ")
    printf(sum)
    printf("\n")
}

// ----- Bench 8: Sum of Cubes (i=1 to 10M) -----
void bench_sum_of_cubes() {
    printf("--- Bench 8: Sum of Cubes (10M) ---\n")

    int sum = 0
    int i = 1
    while i <= 10000000 {
        sum = sum + i * i * i
        i = i + 1
    }

    printf("  Sum of cubes (mod 2^32 overflow): ")
    printf(sum)
    printf("\n")
}

// ----- Bench 9: Harmonic Series Fixed-Point (1M terms) -----
void bench_harmonic_fixed() {
    printf("--- Bench 9: Harmonic Series Fixed-Point x100000 (1M terms) ---\n")

    int sum = 0
    int scale = 100000
    int i = 1
    while i <= 1000000 {
        sum = sum + scale / i
        i = i + 1
    }

    int whole = sum / scale
    int frac = sum % scale

    printf("  H(1000000) approx = ")
    printf(whole)
    printf(".")
    printf(frac)
    printf("\n")
}

// ----- Bench 10: Pi via Leibniz Series Fixed-Point (1M terms) -----
void bench_pi_leibniz() {
    printf("--- Bench 10: Pi Leibniz Fixed-Point x100000 (1M terms) ---\n")

    int sum = 0
    int scale = 100000
    int sign = 1
    int denom = 1
    int i = 0
    while i < 1000000 {
        sum = sum + sign * (4 * scale / denom)
        sign = 0 - sign
        denom = denom + 2
        i = i + 1
    }

    int whole = sum / scale
    int frac = sum % scale
    if frac < 0 {
        frac = 0 - frac
    }

    printf("  Pi approx = ")
    printf(whole)
    printf(".")
    printf(frac)
    printf("\n")
}

// ----- Main -----
int main() {
    printf("// ============================================================\n")
    printf("// ADead-BIB Benchmark Suite — 08: Intensive Math\n")
    printf("// ============================================================\n\n")

    bench_matrix_multiply()
    bench_determinant_3x3()
    bench_cross_product()
    bench_distance_squared()
    bench_ackermann()
    bench_power_tower()
    bench_sum_of_squares()
    bench_sum_of_cubes()
    bench_harmonic_fixed()
    bench_pi_leibniz()

    printf("\n// ============================================================\n")
    printf("// All 10 intensive math benchmarks complete.\n")
    printf("// ============================================================\n")

    return 0
}
