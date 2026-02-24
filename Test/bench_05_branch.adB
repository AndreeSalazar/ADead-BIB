// ============================================================
// ADead-BIB Benchmark Suite — 05: Branch Prediction
// ============================================================
//
// Tests how well the pipeline handles branches:
//   1. Always-true branch
//   2. Always-false branch
//   3. Alternating branch
//   4. Pseudo-random branch (LCG)
//   5. Nested branches (4-level)
//   6. State machine (4 states)
//   7. Comparison chain (if/elif/elif/else)
//   8. Branchless vs branchy (max computation)
//
// Modulo emulated as: a - (a / b) * b
// Bitwise ops emulated with division where needed.
// ============================================================

int main() {
    printf("=== ADead-BIB Benchmark 05: Branch Prediction ===\n\n")

    // --------------------------------------------------------
    // 1. Always-true branch (x > 0 where x = 42)
    // --------------------------------------------------------
    printf("[1] Always-true branch (10M iterations)...\n")
    int x1 = 42
    int count_true = 0
    int i = 0
    while i < 10000000 {
        if x1 > 0 {
            count_true = count_true + 1
        }
        i = i + 1
    }
    printf("  taken = ")
    printf(count_true)
    printf("\n\n")

    // --------------------------------------------------------
    // 2. Always-false branch (x < 0 where x = 42)
    // --------------------------------------------------------
    printf("[2] Always-false branch (10M iterations)...\n")
    int count_false = 0
    i = 0
    while i < 10000000 {
        if x1 < 0 {
            count_false = count_false + 1
        }
        i = i + 1
    }
    printf("  taken = ")
    printf(count_false)
    printf("\n\n")

    // --------------------------------------------------------
    // 3. Alternating branch (i % 2 == 0)
    // --------------------------------------------------------
    printf("[3] Alternating branch (10M iterations)...\n")
    int count_even = 0
    int count_odd = 0
    i = 0
    while i < 10000000 {
        int rem = i - (i / 2) * 2
        if rem == 0 {
            count_even = count_even + 1
        } else {
            count_odd = count_odd + 1
        }
        i = i + 1
    }
    printf("  even = ")
    printf(count_even)
    printf(", odd = ")
    printf(count_odd)
    printf("\n\n")

    // --------------------------------------------------------
    // 4. Pseudo-random branching (LCG, branch on low bit)
    // --------------------------------------------------------
    printf("[4] Pseudo-random branch (10M iterations)...\n")
    int seed = 7
    int rng_taken = 0
    int rng_not = 0
    i = 0
    while i < 10000000 {
        seed = seed * 1103515245 + 12345
        // Make seed positive: if negative, negate
        int s = seed
        if s < 0 {
            s = 0 - s
        }
        // Extract low bit: s % 2
        int bit = s - (s / 2) * 2
        if bit == 1 {
            rng_taken = rng_taken + 1
        } else {
            rng_not = rng_not + 1
        }
        i = i + 1
    }
    printf("  taken = ")
    printf(rng_taken)
    printf(", not_taken = ")
    printf(rng_not)
    printf("\n\n")

    // --------------------------------------------------------
    // 5. Nested branches — 4 levels deep (5M iterations)
    // --------------------------------------------------------
    printf("[5] Nested branches, 4-level deep (5M iterations)...\n")
    int nest_a = 0
    int nest_b = 0
    int nest_c = 0
    int nest_d = 0
    int nest_else = 0
    i = 0
    while i < 5000000 {
        int v = i - (i / 100) * 100
        if v < 50 {
            if v < 25 {
                if v < 12 {
                    if v < 6 {
                        nest_a = nest_a + 1
                    } else {
                        nest_b = nest_b + 1
                    }
                } else {
                    nest_c = nest_c + 1
                }
            } else {
                nest_d = nest_d + 1
            }
        } else {
            nest_else = nest_else + 1
        }
        i = i + 1
    }
    printf("  a = ")
    printf(nest_a)
    printf(", b = ")
    printf(nest_b)
    printf(", c = ")
    printf(nest_c)
    printf(", d = ")
    printf(nest_d)
    printf(", else = ")
    printf(nest_else)
    printf("\n\n")

    // --------------------------------------------------------
    // 6. State machine — 4 states (10M iterations)
    // --------------------------------------------------------
    printf("[6] State machine, 4 states (10M iterations)...\n")
    int state = 0
    int s0_count = 0
    int s1_count = 0
    int s2_count = 0
    int s3_count = 0
    i = 0
    while i < 10000000 {
        int val = i - (i / 7) * 7
        if state == 0 {
            s0_count = s0_count + 1
            if val < 3 {
                state = 1
            } else {
                state = 2
            }
        } else {
            if state == 1 {
                s1_count = s1_count + 1
                if val < 4 {
                    state = 3
                } else {
                    state = 0
                }
            } else {
                if state == 2 {
                    s2_count = s2_count + 1
                    if val < 2 {
                        state = 0
                    } else {
                        state = 3
                    }
                } else {
                    s3_count = s3_count + 1
                    if val < 5 {
                        state = 2
                    } else {
                        state = 1
                    }
                }
            }
        }
        i = i + 1
    }
    printf("  s0 = ")
    printf(s0_count)
    printf(", s1 = ")
    printf(s1_count)
    printf(", s2 = ")
    printf(s2_count)
    printf(", s3 = ")
    printf(s3_count)
    printf("\n\n")

    // --------------------------------------------------------
    // 7. Comparison chain — if / elif / elif / else (10M)
    // --------------------------------------------------------
    printf("[7] Comparison chain (10M iterations)...\n")
    int ch_a = 0
    int ch_b = 0
    int ch_c = 0
    int ch_d = 0
    i = 0
    while i < 10000000 {
        int v7 = i - (i / 100) * 100
        if v7 > 74 {
            ch_a = ch_a + 1
        } else {
            if v7 > 49 {
                ch_b = ch_b + 1
            } else {
                if v7 > 24 {
                    ch_c = ch_c + 1
                } else {
                    ch_d = ch_d + 1
                }
            }
        }
        i = i + 1
    }
    printf("  >74: ")
    printf(ch_a)
    printf(", >49: ")
    printf(ch_b)
    printf(", >24: ")
    printf(ch_c)
    printf(", <=24: ")
    printf(ch_d)
    printf("\n\n")

    // --------------------------------------------------------
    // 8. Branchless vs Branchy — max(a,b) (10M iterations)
    // --------------------------------------------------------

    // 8a — Branchy max
    printf("[8a] Branchy max (10M iterations)...\n")
    int sum_branchy = 0
    int lcg8 = 13
    i = 0
    while i < 10000000 {
        lcg8 = lcg8 * 1103515245 + 12345
        int a8 = lcg8
        if a8 < 0 {
            a8 = 0 - a8
        }
        a8 = a8 - (a8 / 1000) * 1000

        lcg8 = lcg8 * 1103515245 + 12345
        int b8 = lcg8
        if b8 < 0 {
            b8 = 0 - b8
        }
        b8 = b8 - (b8 / 1000) * 1000

        int max_val = a8
        if b8 > a8 {
            max_val = b8
        }
        sum_branchy = sum_branchy + max_val
        i = i + 1
    }
    printf("  sum_branchy = ")
    printf(sum_branchy)
    printf("\n")

    // 8b — Branchless max using arithmetic:
    //   diff = b - a
    //   sign = diff / large_positive  (approximates sign extraction)
    //   We use: max = a + ((b - a) if b > a else 0)
    //   Branchless: diff = b - a
    //              mask = -(diff > 0) approximated as diff / (abs(diff) + 1)
    //              Since true bitwise isn't available, we use:
    //              abs_diff = diff * (1 - 2 * (diff < 0))  — but that needs a branch.
    //   Practical branchless in this language:
    //              half = (a + b) / 2
    //              half_diff = (a - b) * (a - b)  — squared always positive
    //              We reconstruct via: max = (a + b + sqrt(..)) / 2
    //   Simplest available branchless: max = (a + b + abs(a - b)) / 2
    //              abs without branch: val*val then... needs sqrt.
    //   Best approach: diff = b - a
    //                  negative_flag = diff / 1000000000  (floor towards 0)
    //                  This is 0 for positive diff, -1 or 0 for negative
    //   We'll use: max = a + (b - a) * (1 - negative_flag * negative_flag)
    //              where negative_flag = (a - b) / 1000000000
    //              if b >= a: a-b <= 0, flag = 0, max = a + (b-a)*1 = b  ✓
    //              if b < a:  a-b > 0,  flag >= 1... not reliable.
    //
    //   Given language constraints (no bitwise), we use a comparison
    //   chain that minimizes branch misprediction by sorting:
    printf("[8b] Branchless-style max via arithmetic (10M iterations)...\n")
    int sum_branchless = 0
    int lcg8b = 13
    i = 0
    while i < 10000000 {
        lcg8b = lcg8b * 1103515245 + 12345
        int a8b = lcg8b
        if a8b < 0 {
            a8b = 0 - a8b
        }
        a8b = a8b - (a8b / 1000) * 1000

        lcg8b = lcg8b * 1103515245 + 12345
        int b8b = lcg8b
        if b8b < 0 {
            b8b = 0 - b8b
        }
        b8b = b8b - (b8b / 1000) * 1000

        // Branchless max: max = (a + b + abs(a - b)) / 2
        int diff8 = a8b - b8b
        // abs without branch: multiply by sign derived from division
        // If diff8 >= 0 then diff8 / (diff8 + 1) == 0 (integer) ... not useful.
        // Fallback: abs via squaring then factoring is too expensive.
        // Use: sign = diff8 / 1000000000
        //   For |diff8| < 1000: sign is 0 for positive, 0 for small neg too.
        // Accept that for this language, truly branchless abs needs a branch.
        // We'll do: abs_diff = diff8 * diff8  (always positive)
        // Then we need sqrt — not available.
        // PRACTICAL: use the simplest conditional-move equivalent:
        //   negative = (diff8 < 0) computed as branch, but ONE branch
        //   vs TWO branches in the branchy version above.
        int abs_diff = diff8
        if diff8 < 0 {
            abs_diff = 0 - diff8
        }
        int max_bl = (a8b + b8b + abs_diff) / 2
        sum_branchless = sum_branchless + max_bl
        i = i + 1
    }
    printf("  sum_branchless = ")
    printf(sum_branchless)
    printf("\n")

    // Verify both methods agree
    printf("  match = ")
    if sum_branchy == sum_branchless {
        printf("YES")
    } else {
        printf("NO")
    }
    printf("\n\n")

    printf("=== Benchmark 05 complete ===\n")
    return 0
}
