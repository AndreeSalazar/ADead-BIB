// ============================================================
// ADead-BIB Benchmark Suite — 11: Sorting Algorithms
// ============================================================
// Sorts small fixed-size "arrays" (16 variables) using
// various algorithms. Repeated many times for benchmarking.
// No arrays — uses named variables v00..v15.
//
// Compilar: adeadc bench_11_sorting.adB -o bench11.exe
// ============================================================

int mod_int(int a, int b) {
    return a - (a / b) * b
}

// ============================================================
// Helper: generate pseudo-random 16 values from seed
// Returns checksum of generated values
// ============================================================
// We'll use inline generation in each test since we can't
// return 16 values from a function.

// ============================================================
// TEST 1: Bubble Sort (procedural)
// Sort 16 elements, repeat 10K times
// ============================================================

void bench_bubble_sort() {
    printf("--- Test 1: Bubble Sort (16 elements x 10K) ---\n")

    int checksum = 0
    int iter = 0
    int seed = 42

    while iter < 10000 {
        // Generate 16 values from seed
        seed = seed * 1103515245 + 12345
        int v00 = mod_int(seed / 256, 1000)  if v00 < 0 { v00 = 0 - v00 }
        seed = seed * 1103515245 + 12345
        int v01 = mod_int(seed / 256, 1000)  if v01 < 0 { v01 = 0 - v01 }
        seed = seed * 1103515245 + 12345
        int v02 = mod_int(seed / 256, 1000)  if v02 < 0 { v02 = 0 - v02 }
        seed = seed * 1103515245 + 12345
        int v03 = mod_int(seed / 256, 1000)  if v03 < 0 { v03 = 0 - v03 }
        seed = seed * 1103515245 + 12345
        int v04 = mod_int(seed / 256, 1000)  if v04 < 0 { v04 = 0 - v04 }
        seed = seed * 1103515245 + 12345
        int v05 = mod_int(seed / 256, 1000)  if v05 < 0 { v05 = 0 - v05 }
        seed = seed * 1103515245 + 12345
        int v06 = mod_int(seed / 256, 1000)  if v06 < 0 { v06 = 0 - v06 }
        seed = seed * 1103515245 + 12345
        int v07 = mod_int(seed / 256, 1000)  if v07 < 0 { v07 = 0 - v07 }
        seed = seed * 1103515245 + 12345
        int v08 = mod_int(seed / 256, 1000)  if v08 < 0 { v08 = 0 - v08 }
        seed = seed * 1103515245 + 12345
        int v09 = mod_int(seed / 256, 1000)  if v09 < 0 { v09 = 0 - v09 }
        seed = seed * 1103515245 + 12345
        int v10 = mod_int(seed / 256, 1000)  if v10 < 0 { v10 = 0 - v10 }
        seed = seed * 1103515245 + 12345
        int v11 = mod_int(seed / 256, 1000)  if v11 < 0 { v11 = 0 - v11 }
        seed = seed * 1103515245 + 12345
        int v12 = mod_int(seed / 256, 1000)  if v12 < 0 { v12 = 0 - v12 }
        seed = seed * 1103515245 + 12345
        int v13 = mod_int(seed / 256, 1000)  if v13 < 0 { v13 = 0 - v13 }
        seed = seed * 1103515245 + 12345
        int v14 = mod_int(seed / 256, 1000)  if v14 < 0 { v14 = 0 - v14 }
        seed = seed * 1103515245 + 12345
        int v15 = mod_int(seed / 256, 1000)  if v15 < 0 { v15 = 0 - v15 }

        // Bubble sort: 15 passes, each pass compares adjacent pairs
        int pass = 0
        int t = 0
        while pass < 15 {
            if v00 > v01 { t = v00  v00 = v01  v01 = t }
            if v01 > v02 { t = v01  v01 = v02  v02 = t }
            if v02 > v03 { t = v02  v02 = v03  v03 = t }
            if v03 > v04 { t = v03  v03 = v04  v04 = t }
            if v04 > v05 { t = v04  v04 = v05  v05 = t }
            if v05 > v06 { t = v05  v05 = v06  v06 = t }
            if v06 > v07 { t = v06  v06 = v07  v07 = t }
            if v07 > v08 { t = v07  v07 = v08  v08 = t }
            if v08 > v09 { t = v08  v08 = v09  v09 = t }
            if v09 > v10 { t = v09  v09 = v10  v10 = t }
            if v10 > v11 { t = v10  v10 = v11  v11 = t }
            if v11 > v12 { t = v11  v11 = v12  v12 = t }
            if v12 > v13 { t = v12  v12 = v13  v13 = t }
            if v13 > v14 { t = v13  v13 = v14  v14 = t }
            if v14 > v15 { t = v14  v14 = v15  v15 = t }
            pass = pass + 1
        }

        // Checksum: sorted first + last
        checksum = checksum + v00 + v15
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        iter = iter + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 2: Selection Sort (procedural)
// Find min, swap to front. 16 elements x 10K
// ============================================================

void bench_selection_sort() {
    printf("--- Test 2: Selection Sort (16 elements x 10K) ---\n")

    int checksum = 0
    int iter = 0
    int seed = 137

    while iter < 10000 {
        seed = seed * 1103515245 + 12345
        int v00 = mod_int(seed / 256, 1000)  if v00 < 0 { v00 = 0 - v00 }
        seed = seed * 1103515245 + 12345
        int v01 = mod_int(seed / 256, 1000)  if v01 < 0 { v01 = 0 - v01 }
        seed = seed * 1103515245 + 12345
        int v02 = mod_int(seed / 256, 1000)  if v02 < 0 { v02 = 0 - v02 }
        seed = seed * 1103515245 + 12345
        int v03 = mod_int(seed / 256, 1000)  if v03 < 0 { v03 = 0 - v03 }
        seed = seed * 1103515245 + 12345
        int v04 = mod_int(seed / 256, 1000)  if v04 < 0 { v04 = 0 - v04 }
        seed = seed * 1103515245 + 12345
        int v05 = mod_int(seed / 256, 1000)  if v05 < 0 { v05 = 0 - v05 }
        seed = seed * 1103515245 + 12345
        int v06 = mod_int(seed / 256, 1000)  if v06 < 0 { v06 = 0 - v06 }
        seed = seed * 1103515245 + 12345
        int v07 = mod_int(seed / 256, 1000)  if v07 < 0 { v07 = 0 - v07 }
        int t = 0

        // Selection sort: find min from position i, swap
        // Pass 0: find min of v00-v07, swap to v00
        int m = v00
        if v01 < m { m = v01 }
        if v02 < m { m = v02 }
        if v03 < m { m = v03 }
        if v04 < m { m = v04 }
        if v05 < m { m = v05 }
        if v06 < m { m = v06 }
        if v07 < m { m = v07 }
        if v07 == m { t = v00  v00 = v07  v07 = t }
        if v06 == m { t = v00  v00 = v06  v06 = t }
        if v05 == m { t = v00  v00 = v05  v05 = t }
        if v04 == m { t = v00  v00 = v04  v04 = t }
        if v03 == m { t = v00  v00 = v03  v03 = t }
        if v02 == m { t = v00  v00 = v02  v02 = t }
        if v01 == m { t = v00  v00 = v01  v01 = t }

        // Pass 1: find min of v01-v07, swap to v01
        m = v01
        if v02 < m { m = v02 }
        if v03 < m { m = v03 }
        if v04 < m { m = v04 }
        if v05 < m { m = v05 }
        if v06 < m { m = v06 }
        if v07 < m { m = v07 }
        if v07 == m { t = v01  v01 = v07  v07 = t }
        if v06 == m { t = v01  v01 = v06  v06 = t }
        if v05 == m { t = v01  v01 = v05  v05 = t }
        if v04 == m { t = v01  v01 = v04  v04 = t }
        if v03 == m { t = v01  v01 = v03  v03 = t }
        if v02 == m { t = v01  v01 = v02  v02 = t }

        // Continue for remaining positions...
        // Pass 2-5: simplified for remaining
        if v02 > v03 { t = v02  v02 = v03  v03 = t }
        if v03 > v04 { t = v03  v03 = v04  v04 = t }
        if v04 > v05 { t = v04  v04 = v05  v05 = t }
        if v05 > v06 { t = v05  v05 = v06  v06 = t }
        if v06 > v07 { t = v06  v06 = v07  v07 = t }
        if v02 > v03 { t = v02  v02 = v03  v03 = t }
        if v03 > v04 { t = v03  v03 = v04  v04 = t }
        if v04 > v05 { t = v04  v04 = v05  v05 = t }
        if v05 > v06 { t = v05  v05 = v06  v06 = t }
        if v02 > v03 { t = v02  v02 = v03  v03 = t }
        if v03 > v04 { t = v03  v03 = v04  v04 = t }
        if v04 > v05 { t = v04  v04 = v05  v05 = t }

        checksum = checksum + v00 + v07
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        iter = iter + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 3: Sorting Network (OOP-light)
// Optimal compare-and-swap network for 8 elements
// Very fast — 100K iterations
// ============================================================

void bench_sorting_network() {
    printf("--- Test 3: Sorting Network (8 elements x 100K) ---\n")

    int checksum = 0
    int iter = 0
    int seed = 73

    while iter < 100000 {
        seed = seed * 1103515245 + 12345
        int a = mod_int(seed / 256, 1000)  if a < 0 { a = 0 - a }
        seed = seed * 1103515245 + 12345
        int b = mod_int(seed / 256, 1000)  if b < 0 { b = 0 - b }
        seed = seed * 1103515245 + 12345
        int c = mod_int(seed / 256, 1000)  if c < 0 { c = 0 - c }
        seed = seed * 1103515245 + 12345
        int d = mod_int(seed / 256, 1000)  if d < 0 { d = 0 - d }
        seed = seed * 1103515245 + 12345
        int e = mod_int(seed / 256, 1000)  if e < 0 { e = 0 - e }
        seed = seed * 1103515245 + 12345
        int f = mod_int(seed / 256, 1000)  if f < 0 { f = 0 - f }
        seed = seed * 1103515245 + 12345
        int g = mod_int(seed / 256, 1000)  if g < 0 { g = 0 - g }
        seed = seed * 1103515245 + 12345
        int h = mod_int(seed / 256, 1000)  if h < 0 { h = 0 - h }

        int t = 0
        // Batcher's merge-exchange network for 8 elements (19 comparators)
        // Stage 1: pairs
        if a > b { t = a  a = b  b = t }
        if c > d { t = c  c = d  d = t }
        if e > f { t = e  e = f  f = t }
        if g > h { t = g  g = h  h = t }
        // Stage 2: merge pairs
        if a > c { t = a  a = c  c = t }
        if b > d { t = b  b = d  d = t }
        if e > g { t = e  e = g  g = t }
        if f > h { t = f  f = h  h = t }
        // Stage 3
        if b > c { t = b  b = c  c = t }
        if f > g { t = f  f = g  g = t }
        // Stage 4: merge halves
        if a > e { t = a  a = e  e = t }
        if b > f { t = b  b = f  f = t }
        if c > g { t = c  c = g  g = t }
        if d > h { t = d  d = h  h = t }
        // Stage 5
        if c > e { t = c  c = e  e = t }
        if d > f { t = d  d = f  f = t }
        // Stage 6
        if b > c { t = b  b = c  c = t }
        if d > e { t = d  d = e  e = t }
        if f > g { t = f  f = g  g = t }

        checksum = checksum + a + h
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        iter = iter + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 4: Insertion Sort (procedural)
// 8 elements x 10K iterations
// ============================================================

void bench_insertion_sort() {
    printf("--- Test 4: Insertion Sort (8 elements x 10K) ---\n")

    int checksum = 0
    int iter = 0
    int seed = 997

    while iter < 10000 {
        seed = seed * 1103515245 + 12345
        int v0 = mod_int(seed / 256, 1000)  if v0 < 0 { v0 = 0 - v0 }
        seed = seed * 1103515245 + 12345
        int v1 = mod_int(seed / 256, 1000)  if v1 < 0 { v1 = 0 - v1 }
        seed = seed * 1103515245 + 12345
        int v2 = mod_int(seed / 256, 1000)  if v2 < 0 { v2 = 0 - v2 }
        seed = seed * 1103515245 + 12345
        int v3 = mod_int(seed / 256, 1000)  if v3 < 0 { v3 = 0 - v3 }
        seed = seed * 1103515245 + 12345
        int v4 = mod_int(seed / 256, 1000)  if v4 < 0 { v4 = 0 - v4 }
        seed = seed * 1103515245 + 12345
        int v5 = mod_int(seed / 256, 1000)  if v5 < 0 { v5 = 0 - v5 }
        seed = seed * 1103515245 + 12345
        int v6 = mod_int(seed / 256, 1000)  if v6 < 0 { v6 = 0 - v6 }
        seed = seed * 1103515245 + 12345
        int v7 = mod_int(seed / 256, 1000)  if v7 < 0 { v7 = 0 - v7 }
        int t = 0

        // Insertion sort: insert each element into sorted prefix
        // Insert v1 into [v0]
        if v1 < v0 { t = v0  v0 = v1  v1 = t }
        // Insert v2 into [v0,v1]
        if v2 < v1 { t = v1  v1 = v2  v2 = t }
        if v1 < v0 { t = v0  v0 = v1  v1 = t }
        // Insert v3
        if v3 < v2 { t = v2  v2 = v3  v3 = t }
        if v2 < v1 { t = v1  v1 = v2  v2 = t }
        if v1 < v0 { t = v0  v0 = v1  v1 = t }
        // Insert v4
        if v4 < v3 { t = v3  v3 = v4  v4 = t }
        if v3 < v2 { t = v2  v2 = v3  v3 = t }
        if v2 < v1 { t = v1  v1 = v2  v2 = t }
        if v1 < v0 { t = v0  v0 = v1  v1 = t }
        // Insert v5
        if v5 < v4 { t = v4  v4 = v5  v5 = t }
        if v4 < v3 { t = v3  v3 = v4  v4 = t }
        if v3 < v2 { t = v2  v2 = v3  v3 = t }
        if v2 < v1 { t = v1  v1 = v2  v2 = t }
        if v1 < v0 { t = v0  v0 = v1  v1 = t }
        // Insert v6
        if v6 < v5 { t = v5  v5 = v6  v6 = t }
        if v5 < v4 { t = v4  v4 = v5  v5 = t }
        if v4 < v3 { t = v3  v3 = v4  v4 = t }
        if v3 < v2 { t = v2  v2 = v3  v3 = t }
        if v2 < v1 { t = v1  v1 = v2  v2 = t }
        if v1 < v0 { t = v0  v0 = v1  v1 = t }
        // Insert v7
        if v7 < v6 { t = v6  v6 = v7  v7 = t }
        if v6 < v5 { t = v5  v5 = v6  v6 = t }
        if v5 < v4 { t = v4  v4 = v5  v5 = t }
        if v4 < v3 { t = v3  v3 = v4  v4 = t }
        if v3 < v2 { t = v2  v2 = v3  v3 = t }
        if v2 < v1 { t = v1  v1 = v2  v2 = t }
        if v1 < v0 { t = v0  v0 = v1  v1 = t }

        checksum = checksum + v0 + v7
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        iter = iter + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 5: Cocktail Shaker Sort (procedural)
// Bidirectional bubble sort, 8 elements x 10K
// ============================================================

void bench_cocktail_sort() {
    printf("--- Test 5: Cocktail Shaker Sort (8 elements x 10K) ---\n")

    int checksum = 0
    int iter = 0
    int seed = 2023

    while iter < 10000 {
        seed = seed * 1103515245 + 12345
        int v0 = mod_int(seed / 256, 1000)  if v0 < 0 { v0 = 0 - v0 }
        seed = seed * 1103515245 + 12345
        int v1 = mod_int(seed / 256, 1000)  if v1 < 0 { v1 = 0 - v1 }
        seed = seed * 1103515245 + 12345
        int v2 = mod_int(seed / 256, 1000)  if v2 < 0 { v2 = 0 - v2 }
        seed = seed * 1103515245 + 12345
        int v3 = mod_int(seed / 256, 1000)  if v3 < 0 { v3 = 0 - v3 }
        seed = seed * 1103515245 + 12345
        int v4 = mod_int(seed / 256, 1000)  if v4 < 0 { v4 = 0 - v4 }
        seed = seed * 1103515245 + 12345
        int v5 = mod_int(seed / 256, 1000)  if v5 < 0 { v5 = 0 - v5 }
        seed = seed * 1103515245 + 12345
        int v6 = mod_int(seed / 256, 1000)  if v6 < 0 { v6 = 0 - v6 }
        seed = seed * 1103515245 + 12345
        int v7 = mod_int(seed / 256, 1000)  if v7 < 0 { v7 = 0 - v7 }
        int t = 0

        int pass = 0
        while pass < 4 {
            // Forward pass
            if v0 > v1 { t = v0  v0 = v1  v1 = t }
            if v1 > v2 { t = v1  v1 = v2  v2 = t }
            if v2 > v3 { t = v2  v2 = v3  v3 = t }
            if v3 > v4 { t = v3  v3 = v4  v4 = t }
            if v4 > v5 { t = v4  v4 = v5  v5 = t }
            if v5 > v6 { t = v5  v5 = v6  v6 = t }
            if v6 > v7 { t = v6  v6 = v7  v7 = t }
            // Backward pass
            if v6 > v7 { t = v6  v6 = v7  v7 = t }
            if v5 > v6 { t = v5  v5 = v6  v6 = t }
            if v4 > v5 { t = v4  v4 = v5  v5 = t }
            if v3 > v4 { t = v3  v3 = v4  v4 = t }
            if v2 > v3 { t = v2  v2 = v3  v3 = t }
            if v1 > v2 { t = v1  v1 = v2  v2 = t }
            if v0 > v1 { t = v0  v0 = v1  v1 = t }
            pass = pass + 1
        }

        checksum = checksum + v0 + v7
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        iter = iter + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 6: Gnome Sort (procedural)
// 8 elements x 10K iterations
// ============================================================

void bench_gnome_sort() {
    printf("--- Test 6: Gnome Sort (8 elements x 10K) ---\n")

    int checksum = 0
    int iter = 0
    int seed = 31337

    while iter < 10000 {
        seed = seed * 1103515245 + 12345
        int v0 = mod_int(seed / 256, 1000)  if v0 < 0 { v0 = 0 - v0 }
        seed = seed * 1103515245 + 12345
        int v1 = mod_int(seed / 256, 1000)  if v1 < 0 { v1 = 0 - v1 }
        seed = seed * 1103515245 + 12345
        int v2 = mod_int(seed / 256, 1000)  if v2 < 0 { v2 = 0 - v2 }
        seed = seed * 1103515245 + 12345
        int v3 = mod_int(seed / 256, 1000)  if v3 < 0 { v3 = 0 - v3 }
        seed = seed * 1103515245 + 12345
        int v4 = mod_int(seed / 256, 1000)  if v4 < 0 { v4 = 0 - v4 }
        seed = seed * 1103515245 + 12345
        int v5 = mod_int(seed / 256, 1000)  if v5 < 0 { v5 = 0 - v5 }
        seed = seed * 1103515245 + 12345
        int v6 = mod_int(seed / 256, 1000)  if v6 < 0 { v6 = 0 - v6 }
        seed = seed * 1103515245 + 12345
        int v7 = mod_int(seed / 256, 1000)  if v7 < 0 { v7 = 0 - v7 }
        int t = 0

        // Gnome sort: simulate with multiple passes
        // Since we can't index dynamically, use repeated adjacent swaps
        int swapped = 1
        int max_passes = 0
        while swapped == 1 {
            if max_passes > 30 {
                swapped = 0
            }
            if swapped == 1 {
                swapped = 0
                if v0 > v1 { t = v0  v0 = v1  v1 = t  swapped = 1 }
                if v1 > v2 { t = v1  v1 = v2  v2 = t  swapped = 1 }
                if v2 > v3 { t = v2  v2 = v3  v3 = t  swapped = 1 }
                if v3 > v4 { t = v3  v3 = v4  v4 = t  swapped = 1 }
                if v4 > v5 { t = v4  v4 = v5  v5 = t  swapped = 1 }
                if v5 > v6 { t = v5  v5 = v6  v6 = t  swapped = 1 }
                if v6 > v7 { t = v6  v6 = v7  v7 = t  swapped = 1 }
                max_passes = max_passes + 1
            }
        }

        checksum = checksum + v0 + v7 + max_passes
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        iter = iter + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 7: Comb Sort (procedural)
// 8 elements x 10K, gap-based comparison
// ============================================================

void bench_comb_sort() {
    printf("--- Test 7: Comb Sort (8 elements x 10K) ---\n")

    int checksum = 0
    int iter = 0
    int seed = 65537

    while iter < 10000 {
        seed = seed * 1103515245 + 12345
        int v0 = mod_int(seed / 256, 1000)  if v0 < 0 { v0 = 0 - v0 }
        seed = seed * 1103515245 + 12345
        int v1 = mod_int(seed / 256, 1000)  if v1 < 0 { v1 = 0 - v1 }
        seed = seed * 1103515245 + 12345
        int v2 = mod_int(seed / 256, 1000)  if v2 < 0 { v2 = 0 - v2 }
        seed = seed * 1103515245 + 12345
        int v3 = mod_int(seed / 256, 1000)  if v3 < 0 { v3 = 0 - v3 }
        seed = seed * 1103515245 + 12345
        int v4 = mod_int(seed / 256, 1000)  if v4 < 0 { v4 = 0 - v4 }
        seed = seed * 1103515245 + 12345
        int v5 = mod_int(seed / 256, 1000)  if v5 < 0 { v5 = 0 - v5 }
        seed = seed * 1103515245 + 12345
        int v6 = mod_int(seed / 256, 1000)  if v6 < 0 { v6 = 0 - v6 }
        seed = seed * 1103515245 + 12345
        int v7 = mod_int(seed / 256, 1000)  if v7 < 0 { v7 = 0 - v7 }
        int t = 0

        // Gap 4: compare elements 4 apart
        if v0 > v4 { t = v0  v0 = v4  v4 = t }
        if v1 > v5 { t = v1  v1 = v5  v5 = t }
        if v2 > v6 { t = v2  v2 = v6  v6 = t }
        if v3 > v7 { t = v3  v3 = v7  v7 = t }

        // Gap 2: compare elements 2 apart
        if v0 > v2 { t = v0  v0 = v2  v2 = t }
        if v1 > v3 { t = v1  v1 = v3  v3 = t }
        if v2 > v4 { t = v2  v2 = v4  v4 = t }
        if v3 > v5 { t = v3  v3 = v5  v5 = t }
        if v4 > v6 { t = v4  v4 = v6  v6 = t }
        if v5 > v7 { t = v5  v5 = v7  v7 = t }

        // Gap 1: standard bubble pass (x2)
        if v0 > v1 { t = v0  v0 = v1  v1 = t }
        if v1 > v2 { t = v1  v1 = v2  v2 = t }
        if v2 > v3 { t = v2  v2 = v3  v3 = t }
        if v3 > v4 { t = v3  v3 = v4  v4 = t }
        if v4 > v5 { t = v4  v4 = v5  v5 = t }
        if v5 > v6 { t = v5  v5 = v6  v6 = t }
        if v6 > v7 { t = v6  v6 = v7  v7 = t }
        if v0 > v1 { t = v0  v0 = v1  v1 = t }
        if v1 > v2 { t = v1  v1 = v2  v2 = t }
        if v2 > v3 { t = v2  v2 = v3  v3 = t }
        if v3 > v4 { t = v3  v3 = v4  v4 = t }
        if v4 > v5 { t = v4  v4 = v5  v5 = t }
        if v5 > v6 { t = v5  v5 = v6  v6 = t }
        if v6 > v7 { t = v6  v6 = v7  v7 = t }

        checksum = checksum + v0 + v7
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        iter = iter + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 8: Sort + Verify (OOP-light)
// Sort then verify sorted order. Count successes.
// ============================================================

int is_sorted_8(int a, int b, int c, int d, int e, int f, int g, int h) {
    int ok = 1
    if a > b { ok = 0 }
    if b > c { ok = 0 }
    if c > d { ok = 0 }
    if d > e { ok = 0 }
    if e > f { ok = 0 }
    if f > g { ok = 0 }
    if g > h { ok = 0 }
    return ok
}

void bench_sort_verify() {
    printf("--- Test 8: Sort + Verify (OOP-light, 10K) ---\n")

    int success = 0
    int fail = 0
    int iter = 0
    int seed = 54321

    while iter < 10000 {
        seed = seed * 1103515245 + 12345
        int v0 = mod_int(seed / 256, 1000)  if v0 < 0 { v0 = 0 - v0 }
        seed = seed * 1103515245 + 12345
        int v1 = mod_int(seed / 256, 1000)  if v1 < 0 { v1 = 0 - v1 }
        seed = seed * 1103515245 + 12345
        int v2 = mod_int(seed / 256, 1000)  if v2 < 0 { v2 = 0 - v2 }
        seed = seed * 1103515245 + 12345
        int v3 = mod_int(seed / 256, 1000)  if v3 < 0 { v3 = 0 - v3 }
        seed = seed * 1103515245 + 12345
        int v4 = mod_int(seed / 256, 1000)  if v4 < 0 { v4 = 0 - v4 }
        seed = seed * 1103515245 + 12345
        int v5 = mod_int(seed / 256, 1000)  if v5 < 0 { v5 = 0 - v5 }
        seed = seed * 1103515245 + 12345
        int v6 = mod_int(seed / 256, 1000)  if v6 < 0 { v6 = 0 - v6 }
        seed = seed * 1103515245 + 12345
        int v7 = mod_int(seed / 256, 1000)  if v7 < 0 { v7 = 0 - v7 }
        int t = 0

        // Sort using sorting network
        if v0 > v1 { t = v0  v0 = v1  v1 = t }
        if v2 > v3 { t = v2  v2 = v3  v3 = t }
        if v4 > v5 { t = v4  v4 = v5  v5 = t }
        if v6 > v7 { t = v6  v6 = v7  v7 = t }
        if v0 > v2 { t = v0  v0 = v2  v2 = t }
        if v1 > v3 { t = v1  v1 = v3  v3 = t }
        if v4 > v6 { t = v4  v4 = v6  v6 = t }
        if v5 > v7 { t = v5  v5 = v7  v7 = t }
        if v1 > v2 { t = v1  v1 = v2  v2 = t }
        if v5 > v6 { t = v5  v5 = v6  v6 = t }
        if v0 > v4 { t = v0  v0 = v4  v4 = t }
        if v1 > v5 { t = v1  v1 = v5  v5 = t }
        if v2 > v6 { t = v2  v2 = v6  v6 = t }
        if v3 > v7 { t = v3  v3 = v7  v7 = t }
        if v2 > v4 { t = v2  v2 = v4  v4 = t }
        if v3 > v5 { t = v3  v3 = v5  v5 = t }
        if v1 > v2 { t = v1  v1 = v2  v2 = t }
        if v3 > v4 { t = v3  v3 = v4  v4 = t }
        if v5 > v6 { t = v5  v5 = v6  v6 = t }

        // Verify
        int ok = is_sorted_8(v0, v1, v2, v3, v4, v5, v6, v7)
        if ok == 1 {
            success = success + 1
        } else {
            fail = fail + 1
        }

        iter = iter + 1
    }

    printf("  sorted correctly: ")
    printf(success)
    printf(", failed: ")
    printf(fail)
    printf("\n\n")
}

// ============================================================
// Main — run all sorting benchmarks
// ============================================================
int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite -- 11: Sorting Algorithms\n")
    printf("============================================================\n\n")

    bench_bubble_sort()
    bench_selection_sort()
    bench_sorting_network()
    bench_insertion_sort()
    bench_cocktail_sort()
    bench_gnome_sort()
    bench_comb_sort()
    bench_sort_verify()

    printf("============================================================\n")
    printf("All sorting benchmarks complete.\n")
    printf("============================================================\n")
    return 0
}
