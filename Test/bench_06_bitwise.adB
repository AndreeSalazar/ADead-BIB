// ============================================================
// ADead-BIB Benchmark Suite -- 06: Arithmetic Intensity
// ============================================================
// Since bitwise operators (&, |, ^, <<, >>) are not yet in the
// parser, this benchmark simulates bit-level operations using
// arithmetic: multiply/divide by powers of 2, modulo extraction,
// and intensive ALU chains.
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

// ============================================================
// 1. Shift-left simulation: multiply by 2 (100K iterations)
// ============================================================
void bench_shift_left_sim() {
    printf("[1] Shift-left simulation (x2, 100K)...\n")
    int val = 1
    int i = 0
    while i < 100000 {
        val = val * 2
        if val > 1000000000 {
            val = 1
        }
        i = i + 1
    }
    printf("  result = ")
    printf(val)
    printf("\n\n")
}

// ============================================================
// 2. Shift-right simulation: divide by 2 (100K iterations)
// ============================================================
void bench_shift_right_sim() {
    printf("[2] Shift-right simulation (/2, 100K)...\n")
    int val = 1000000000
    int resets = 0
    int i = 0
    while i < 100000 {
        val = val / 2
        if val < 1 {
            val = 1000000000
            resets = resets + 1
        }
        i = i + 1
    }
    printf("  result = ")
    printf(val)
    printf(", resets = ")
    printf(resets)
    printf("\n\n")
}

// ============================================================
// 3. Bit extraction via modulo: extract "bits" (100K iterations)
// ============================================================
void bench_bit_extract() {
    printf("[3] Bit extraction via modulo (100K)...\n")
    int sum = 0
    int i = 1
    while i <= 100000 {
        int bit0 = mod_int(i, 2)
        int bit1 = mod_int(i / 2, 2)
        int bit2 = mod_int(i / 4, 2)
        int bit3 = mod_int(i / 8, 2)
        sum = sum + bit0 + bit1 + bit2 + bit3
        i = i + 1
    }
    printf("  total bits set (low 4): ")
    printf(sum)
    printf("\n\n")
}

// ============================================================
// 4. Popcount via division: count bits set (10K numbers)
// ============================================================
void bench_popcount() {
    printf("[4] Popcount via division (10K)...\n")
    int total_bits = 0
    int i = 1
    while i <= 10000 {
        int n = i
        int bits = 0
        while n > 0 {
            bits = bits + mod_int(n, 2)
            n = n / 2
        }
        total_bits = total_bits + bits
        i = i + 1
    }
    printf("  total bits set = ")
    printf(total_bits)
    printf("\n\n")
}

// ============================================================
// 5. Power-of-2 detection via division (100K)
// ============================================================
void bench_power_of_2() {
    printf("[5] Power-of-2 detection (100K)...\n")
    int count = 0
    int i = 1
    while i <= 100000 {
        int n = i
        int is_pow2 = 1
        if n <= 0 {
            is_pow2 = 0
        }
        if is_pow2 == 1 {
            while n > 1 {
                if mod_int(n, 2) != 0 {
                    is_pow2 = 0
                    n = 1
                }
                if is_pow2 == 1 {
                    n = n / 2
                }
            }
        }
        count = count + is_pow2
        i = i + 1
    }
    printf("  powers of 2 found = ")
    printf(count)
    printf("\n\n")
}

// ============================================================
// 6. Multiplicative hash (no XOR needed): 10K values
// ============================================================
void bench_mult_hash() {
    printf("[6] Multiplicative hash (10K)...\n")
    int hash = 2166136261 % 2147483647
    int i = 0
    while i < 10000 {
        hash = hash * 16777619 + i
        hash = mod_int(hash, 2147483647)
        i = i + 1
    }
    printf("  hash = ")
    printf(hash)
    printf("\n\n")
}

// ============================================================
// 7. Bit reversal via division/multiply (1K numbers)
// ============================================================
void bench_bit_reversal() {
    printf("[7] Bit reversal via div/mul (1K)...\n")
    int checksum = 0
    int i = 1
    while i <= 1000 {
        int n = i
        int rev = 0
        int b = 0
        while b < 20 {
            int bit = mod_int(n, 2)
            rev = rev * 2 + bit
            n = n / 2
            b = b + 1
        }
        checksum = checksum + rev
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000000)
        }
        i = i + 1
    }
    printf("  reversal checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// 8. Abs value computation (100K)
// ============================================================
void bench_abs_val() {
    printf("[8] Abs value computation (100K)...\n")
    int sum = 0
    int i = 0
    while i < 100000 {
        int x = i - 50000
        sum = sum + abs_val(x)
        i = i + 1
    }
    printf("  abs sum = ")
    printf(sum)
    printf("\n\n")
}

// ============================================================
// 9. Division-heavy chain (100K)
// ============================================================
void bench_div_chain() {
    printf("[9] Division-heavy chain (100K)...\n")
    int val = 1000000000
    int i = 0
    while i < 100000 {
        val = val / 3 + i
        if val > 2000000000 {
            val = mod_int(val, 1000000)
        }
        if val < 1 {
            val = 1000000000
        }
        i = i + 1
    }
    printf("  result = ")
    printf(val)
    printf("\n\n")
}

// ============================================================
// 10. Modulo-heavy chain (100K)
// ============================================================
void bench_mod_chain() {
    printf("[10] Modulo-heavy chain (100K)...\n")
    int val = 123456789
    int sum = 0
    int i = 1
    while i <= 100000 {
        val = val * 31 + i
        val = mod_int(val, 1000000007)
        sum = sum + mod_int(val, 1000)
        if sum > 2000000000 {
            sum = mod_int(sum, 1000000)
        }
        i = i + 1
    }
    printf("  checksum = ")
    printf(sum)
    printf("\n\n")
}

// ============================================================
// Main
// ============================================================
int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite -- 06: Arithmetic Intensity\n")
    printf("============================================================\n\n")

    bench_shift_left_sim()
    bench_shift_right_sim()
    bench_bit_extract()
    bench_popcount()
    bench_power_of_2()
    bench_mult_hash()
    bench_bit_reversal()
    bench_abs_val()
    bench_div_chain()
    bench_mod_chain()

    printf("============================================================\n")
    printf("All arithmetic benchmarks complete.\n")
    printf("============================================================\n")
    return 0
}
