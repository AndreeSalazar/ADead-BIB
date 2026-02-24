// ============================================================
// ADead-BIB Benchmark Suite — 02: CPU Float (Fixed-Point)
// ============================================================
// All "floating-point" operations are simulated using integers
// with fixed-point scaling (x10000 or x1000).
//
// Compilar: adeadc bench_02_cpu_floats.adB -o bench02.exe
// ============================================================

// ---- Utility functions ----

int abs_val(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

int div_int(int a, int b) {
    if b == 0 {
        return 0
    }
    return a / b
}

int mod_int(int a, int b) {
    if b == 0 {
        return 0
    }
    return a - (a / b) * b
}

// ============================================================
// TEST 1: Fixed-point harmonic sum
// Sum of 1/1 + 1/2 + 1/3 + ... + 1/N  (scaled x10000)
// N = 1000
// ============================================================

void bench_harmonic_sum() {
    printf("--- Bench 1: Fixed-Point Harmonic Sum ---\n")
    printf("Computing 1/1 + 1/2 + ... + 1/1000 (x10000)\n")

    int scale = 10000
    int sum = 0
    int i = 1
    int n = 1000

    while i <= n {
        sum = sum + div_int(scale, i)
        i = i + 1
    }

    printf("Harmonic sum (x10000): ")
    printf(sum)
    printf("\n\n")
}

// ============================================================
// TEST 2: Fixed-point chained multiplications
// Multiply a chain: start with scale, multiply by factor,
// rescale, repeat. Tests multiply + divide throughput.
// 1000 iterations, factor = 1.0001 => 10001/10000
// ============================================================

void bench_chain_multiply() {
    printf("--- Bench 2: Fixed-Point Chain Multiply ---\n")
    printf("Chained multiply x1000 iters, factor=10001/10000\n")

    int scale = 10000
    int val = scale
    int i = 0
    int n = 1000
    int checksum = 0

    while i < n {
        // val = val * 10001 / 10000  (multiply by 1.0001)
        // To avoid overflow, split: val + val/10000
        val = val + div_int(val, scale)
        // keep val bounded to avoid overflow
        if val > 1000000000 {
            val = div_int(val, 1000)
            checksum = checksum + 1
        }
        i = i + 1
    }

    printf("Chain multiply result: ")
    printf(val)
    printf("\nOverflow resets: ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 3: Fixed-point dot product
// Simulate dot product of two "vectors" computed on the fly
// 1000000 iterations
// vec_a[i] = (i * 3 + 7) mod 1000
// vec_b[i] = (i * 5 + 13) mod 1000
// Accumulate: sum += a * b / scale
// ============================================================

void bench_dot_product() {
    printf("--- Bench 3: Fixed-Point Dot Product ---\n")
    printf("Dot product 1000000 element pairs (x1000)\n")

    int scale = 1000
    int sum = 0
    int i = 0
    int n = 10000
    int a = 0
    int b = 0

    while i < n {
        a = mod_int(i * 3 + 7, scale)
        b = mod_int(i * 5 + 13, scale)
        sum = sum + div_int(a * b, scale)
        // prevent overflow in sum
        if sum > 1000000000 {
            sum = mod_int(sum, 1000000000)
        }
        i = i + 1
    }

    printf("Dot product checksum: ")
    printf(sum)
    printf("\n\n")
}

// ============================================================
// TEST 4: 4x4 Matrix multiply (unrolled, fixed-point)
// Each matrix is 16 int variables (m00..m33), scale=1000
// Repeat 1000000 times: C = A * B, then A = C
// ============================================================

void bench_matrix4x4() {
    printf("--- Bench 4: 4x4 Matrix Multiply ---\n")
    printf("Unrolled 4x4 matmul x1000000 iters (x1000)\n")

    int s = 1000

    // Matrix A (identity-ish, scaled)
    int a00 = 1000  int a01 = 100   int a02 = 0     int a03 = 0
    int a10 = 0     int a11 = 1000  int a12 = 100   int a13 = 0
    int a20 = 0     int a21 = 0     int a22 = 1000  int a23 = 100
    int a30 = 100   int a31 = 0     int a32 = 0     int a33 = 1000

    // Matrix B (same as A initially)
    int b00 = 1000  int b01 = 100   int b02 = 0     int b03 = 0
    int b10 = 0     int b11 = 1000  int b12 = 100   int b13 = 0
    int b20 = 0     int b21 = 0     int b22 = 1000  int b23 = 100
    int b30 = 100   int b31 = 0     int b32 = 0     int b33 = 1000

    // Result matrix C
    int c00 = 0  int c01 = 0  int c02 = 0  int c03 = 0
    int c10 = 0  int c11 = 0  int c12 = 0  int c13 = 0
    int c20 = 0  int c21 = 0  int c22 = 0  int c23 = 0
    int c30 = 0  int c31 = 0  int c32 = 0  int c33 = 0

    int iter = 0
    int n = 10000

    while iter < n {
        // C = A * B  (each element = sum of row_A * col_B / scale)
        c00 = div_int(a00*b00 + a01*b10 + a02*b20 + a03*b30, s)
        c01 = div_int(a00*b01 + a01*b11 + a02*b21 + a03*b31, s)
        c02 = div_int(a00*b02 + a01*b12 + a02*b22 + a03*b32, s)
        c03 = div_int(a00*b03 + a01*b13 + a02*b23 + a03*b33, s)

        c10 = div_int(a10*b00 + a11*b10 + a12*b20 + a13*b30, s)
        c11 = div_int(a10*b01 + a11*b11 + a12*b21 + a13*b31, s)
        c12 = div_int(a10*b02 + a11*b12 + a12*b22 + a13*b32, s)
        c13 = div_int(a10*b03 + a11*b13 + a12*b23 + a13*b33, s)

        c20 = div_int(a20*b00 + a21*b10 + a22*b20 + a23*b30, s)
        c21 = div_int(a20*b01 + a21*b11 + a22*b21 + a23*b31, s)
        c22 = div_int(a20*b02 + a21*b12 + a22*b22 + a23*b32, s)
        c23 = div_int(a20*b03 + a21*b13 + a22*b23 + a23*b33, s)

        c30 = div_int(a30*b00 + a31*b10 + a32*b20 + a33*b30, s)
        c31 = div_int(a30*b01 + a31*b11 + a32*b21 + a33*b31, s)
        c32 = div_int(a30*b02 + a31*b12 + a32*b22 + a33*b32, s)
        c33 = div_int(a30*b03 + a31*b13 + a32*b23 + a33*b33, s)

        // A = C for next iteration (keep bounded)
        a00 = mod_int(c00, 100000)  a01 = mod_int(c01, 100000)
        a02 = mod_int(c02, 100000)  a03 = mod_int(c03, 100000)
        a10 = mod_int(c10, 100000)  a11 = mod_int(c11, 100000)
        a12 = mod_int(c12, 100000)  a13 = mod_int(c13, 100000)
        a20 = mod_int(c20, 100000)  a21 = mod_int(c21, 100000)
        a22 = mod_int(c22, 100000)  a23 = mod_int(c23, 100000)
        a30 = mod_int(c30, 100000)  a31 = mod_int(c31, 100000)
        a32 = mod_int(c32, 100000)  a33 = mod_int(c33, 100000)

        iter = iter + 1
    }

    int trace = c00 + c11 + c22 + c33
    printf("Matrix trace checksum: ")
    printf(trace)
    printf("\n\n")
}

// ============================================================
// TEST 5: Newton's method integer square root
// For each n in 1..100000, compute isqrt(n) via Newton
// Checksum = sum of all isqrt results
// ============================================================

int isqrt_newton(int n) {
    if n <= 0 {
        return 0
    }
    if n == 1 {
        return 1
    }
    int x = n
    int y = div_int(x + 1, 2)
    while y < x {
        x = y
        y = div_int(x + div_int(n, x), 2)
    }
    return x
}

void bench_newton_sqrt() {
    printf("--- Bench 5: Newton's Method Sqrt ---\n")
    printf("Integer sqrt for 1..100000\n")

    int sum = 0
    int i = 1
    int n = 1000

    while i <= n {
        sum = sum + isqrt_newton(i)
        i = i + 1
    }

    printf("Sum of isqrt(1..100000): ")
    printf(sum)
    printf("\n\n")
}

// ============================================================
// TEST 6: Mandelbrot escape iteration (fixed-point)
// Scale = 1000, so 2.0 = 2000, escape radius^2 = 4000000
// Grid: x from -2000 to 1000 step 100, y from -1500 to 1500 step 100
// Count total iterations across all points (max 100 per point)
// ============================================================

void bench_mandelbrot() {
    printf("--- Bench 6: Mandelbrot Escape (Fixed-Point) ---\n")
    printf("Grid scan, scale=1000, max_iter=100\n")

    int scale = 1000
    int escape_sq = 4000000
    int max_iter = 100
    int total_iters = 0
    int points = 0

    int cy = 0 - 1500
    while cy <= 1500 {
        int cx = 0 - 2000
        while cx <= 1000 {
            int zr = 0
            int zi = 0
            int iter = 0

            while iter < max_iter {
                // zr^2 and zi^2 in fixed-point
                int zr2 = div_int(zr * zr, scale)
                int zi2 = div_int(zi * zi, scale)

                if zr2 + zi2 > escape_sq {
                    iter = max_iter + 1
                }

                if iter <= max_iter {
                    // new_zr = zr^2 - zi^2 + cx
                    int new_zr = zr2 - zi2 + cx
                    // new_zi = 2*zr*zi/scale + cy
                    zi = div_int(2 * zr * zi, scale) + cy
                    zr = new_zr
                    iter = iter + 1
                }
            }

            if iter > max_iter {
                total_iters = total_iters + max_iter
            }
            if iter <= max_iter {
                total_iters = total_iters + iter
            }

            points = points + 1
            cx = cx + 100
        }
        cy = cy + 100
    }

    printf("Mandelbrot points: ")
    printf(points)
    printf("\nTotal iterations: ")
    printf(total_iters)
    printf("\n\n")
}

// ============================================================
// TEST 7: Taylor series sin(x) approximation (fixed-point)
// sin(x) ~ x - x^3/3! + x^5/5! - x^7/7!  (scale=10000)
// Compute for 100000 values of x from 0 to 31415 (0 to pi)
// Checksum = sum of all sin approximations
// ============================================================

void bench_taylor_sin() {
    printf("--- Bench 7: Taylor Sin Approximation ---\n")
    printf("sin(x) Taylor 4-term, 100000 values (x10000)\n")

    int scale = 10000
    int n = 1000
    int checksum = 0
    int i = 0

    // x goes from 0 to 31415 (pi*10000) in steps of 31415/100000
    // step ~ 0, so use step = 1 (covers 0..99999 in scale units)
    // We'll compute sin for x = i * 31415 / 100000

    while i < n {
        // x in fixed-point (x10000), range 0..31415
        int x = div_int(i * 31415, n)

        // Normalize x to range for Taylor: we need x/scale for powers
        // term1 = x
        int term1 = x

        // term2 = -x^3 / 6
        // x^3 / scale^2 = x * (x/scale) * (x/scale) / 6 ... careful with overflow
        int x_over_s = div_int(x * x, scale)
        int x3 = div_int(x_over_s * x, scale)
        int term2 = 0 - div_int(x3, 6)

        // term3 = x^5 / 120
        int x5 = div_int(x3 * x_over_s, scale)
        int term3 = div_int(x5, 120)

        // term4 = -x^7 / 5040
        int x7 = div_int(x5 * x_over_s, scale)
        int term4 = 0 - div_int(x7, 5040)

        int sinx = term1 + term2 + term3 + term4
        checksum = checksum + sinx

        // prevent overflow in checksum
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000000)
        }
        if checksum < 0 - 1000000000 {
            checksum = mod_int(checksum, 1000000000)
        }

        i = i + 1
    }

    printf("Taylor sin checksum: ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// MAIN — Run all benchmarks
// ============================================================

int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite -- 02: CPU Float (Fixed-Point)\n")
    printf("============================================================\n")
    printf("All tests use integer arithmetic to simulate floats.\n\n")

    bench_harmonic_sum()
    bench_chain_multiply()
    bench_dot_product()
    bench_matrix4x4()
    bench_newton_sqrt()
    bench_mandelbrot()
    bench_taylor_sin()

    printf("============================================================\n")
    printf("All float benchmarks completed!\n")
    printf("============================================================\n")

    return 0
}
