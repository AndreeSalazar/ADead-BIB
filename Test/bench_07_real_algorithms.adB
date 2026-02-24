// ============================================================
// ADead-BIB Benchmark Suite — 07: Real-World Algorithms
// ============================================================

// --- Helper: modulo (a mod b) ---
int mod_int(int a, int b) {
    return a - (a / b) * b
}

// --- Helper: integer square root (Newton's method) ---
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

// --- Helper: GCD (Euclidean algorithm) ---
int gcd(int a, int b) {
    while b != 0 {
        int t = mod_int(a, b)
        a = b
        b = t
    }
    return a
}

// --- Helper: modular exponentiation (base^exp mod m) ---
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

// --- Helper: count digits ---
int count_digits(int n) {
    if n == 0 {
        return 1
    }
    int count = 0
    while n > 0 {
        count = count + 1
        n = n / 10
    }
    return count
}

// --- Helper: reverse digits of a number ---
int reverse_number(int n) {
    int rev = 0
    while n > 0 {
        rev = rev * 10 + mod_int(n, 10)
        n = n / 10
    }
    return rev
}

// --- Helper: sum of digits ---
int digit_sum(int n) {
    int s = 0
    while n > 0 {
        s = s + mod_int(n, 10)
        n = n / 10
    }
    return s
}

// --- Helper: sum of proper divisors ---
int sum_divisors(int n) {
    int s = 1
    int d = 2
    while d * d <= n {
        if mod_int(n, d) == 0 {
            s = s + d
            if d != n / d {
                s = s + n / d
            }
        }
        d = d + 1
    }
    return s
}

// ============================================================
// Test 1: Prime sieve (scalar trial division)
// Count all primes up to 100000
// ============================================================
void bench_primes() {
    printf("Test 1: Prime count up to 100000\n")
    int count = 0
    int n = 2
    while n <= 100000 {
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
    printf("  Primes found: ")
    printf(count)
    printf("\n")
}

// ============================================================
// Test 2: Collatz sequence
// For each number 1..100000, compute sequence length, print max
// ============================================================
void bench_collatz() {
    printf("Test 2: Collatz max sequence length (1..100000)\n")
    int max_len = 0
    int i = 1
    while i <= 100000 {
        int n = i
        int len = 0
        while n != 1 {
            if mod_int(n, 2) == 0 {
                n = n / 2
            } else {
                n = 3 * n + 1
            }
            len = len + 1
        }
        if len > max_len {
            max_len = len
        }
        i = i + 1
    }
    printf("  Max Collatz length: ")
    printf(max_len)
    printf("\n")
}

// ============================================================
// Test 3: Integer square root (Newton's method)
// Compute isqrt for all numbers 1..1000000, print checksum
// ============================================================
void bench_isqrt() {
    printf("Test 3: Integer sqrt checksum (1..1000000)\n")
    int checksum = 0
    int i = 1
    while i <= 1000000 {
        checksum = checksum + isqrt(i)
        i = i + 1
    }
    printf("  isqrt checksum: ")
    printf(checksum)
    printf("\n")
}

// ============================================================
// Test 4: GCD marathon
// Compute GCD(i, i+1) for i = 1..1000000, print checksum
// ============================================================
void bench_gcd() {
    printf("Test 4: GCD marathon checksum (1..1000000)\n")
    int checksum = 0
    int i = 1
    while i <= 1000000 {
        checksum = checksum + gcd(i, i + 1)
        i = i + 1
    }
    printf("  GCD checksum: ")
    printf(checksum)
    printf("\n")
}

// ============================================================
// Test 5: Modular exponentiation
// Compute base^65537 mod 1000003 for base = 1..100000
// ============================================================
void bench_mod_pow() {
    printf("Test 5: Modular exponentiation checksum (100000 calls)\n")
    int checksum = 0
    int base = 1
    while base <= 100000 {
        checksum = checksum + mod_pow(base, 65537, 1000003)
        base = base + 1
    }
    printf("  mod_pow checksum: ")
    printf(checksum)
    printf("\n")
}

// ============================================================
// Test 6: Digit sum
// Sum of digits of all numbers from 1..1000000
// ============================================================
void bench_digit_sum() {
    printf("Test 6: Digit sum total (1..1000000)\n")
    int total = 0
    int i = 1
    while i <= 1000000 {
        total = total + digit_sum(i)
        i = i + 1
    }
    printf("  Digit sum total: ")
    printf(total)
    printf("\n")
}

// ============================================================
// Test 7: Number reversal
// Reverse digits of all numbers 1..1000000, print checksum
// ============================================================
void bench_reverse() {
    printf("Test 7: Number reversal checksum (1..1000000)\n")
    int checksum = 0
    int i = 1
    while i <= 1000000 {
        checksum = checksum + reverse_number(i)
        i = i + 1
    }
    printf("  Reverse checksum: ")
    printf(checksum)
    printf("\n")
}

// ============================================================
// Test 8: Palindrome count
// Count palindromic numbers from 1..1000000
// ============================================================
void bench_palindromes() {
    printf("Test 8: Palindrome count (1..1000000)\n")
    int count = 0
    int i = 1
    while i <= 1000000 {
        if reverse_number(i) == i {
            count = count + 1
        }
        i = i + 1
    }
    printf("  Palindromes found: ")
    printf(count)
    printf("\n")
}

// ============================================================
// Test 9: Perfect number search
// Find all perfect numbers up to 100000
// ============================================================
void bench_perfect_numbers() {
    printf("Test 9: Perfect numbers up to 100000\n")
    int count = 0
    int n = 2
    while n <= 100000 {
        if sum_divisors(n) == n {
            printf("  Found perfect number: ")
            printf(n)
            printf("\n")
            count = count + 1
        }
        n = n + 1
    }
    printf("  Perfect numbers found: ")
    printf(count)
    printf("\n")
}

// ============================================================
// Test 10: Euler totient
// Compute phi(n) for n = 1..10000, sum results
// ============================================================
void bench_euler_totient() {
    printf("Test 10: Euler totient sum (1..10000)\n")
    int total = 0
    int n = 1
    while n <= 10000 {
        int phi = 0
        int k = 1
        while k <= n {
            if gcd(n, k) == 1 {
                phi = phi + 1
            }
            k = k + 1
        }
        total = total + phi
        n = n + 1
    }
    printf("  Totient sum: ")
    printf(total)
    printf("\n")
}

// ============================================================
// Main — run all benchmarks
// ============================================================
int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite — 07: Real-World Algorithms\n")
    printf("============================================================\n\n")

    bench_primes()
    printf("\n")

    bench_collatz()
    printf("\n")

    bench_isqrt()
    printf("\n")

    bench_gcd()
    printf("\n")

    bench_mod_pow()
    printf("\n")

    bench_digit_sum()
    printf("\n")

    bench_reverse()
    printf("\n")

    bench_palindromes()
    printf("\n")

    bench_perfect_numbers()
    printf("\n")

    bench_euler_totient()
    printf("\n")

    printf("============================================================\n")
    printf("All benchmarks complete.\n")
    printf("============================================================\n")

    return 0
}
