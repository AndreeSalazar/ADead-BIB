// ============================================================
// C Comparison Benchmark (compile with: gcc -O3 -o bench_c.exe)
// ============================================================
// Matches ADead-BIB benchmark suite for direct comparison.
// Uses procedural C-style + OOP-light patterns (struct + funcs).
// ============================================================

#include <stdio.h>
#include <stdint.h>
#include <time.h>
#include <stdlib.h>

// ---- Timer utility ----

typedef struct {
    clock_t start;
} Timer;

void timer_start(Timer *t) {
    t->start = clock();
}

double timer_elapsed_ms(Timer *t) {
    return (double)(clock() - t->start) / CLOCKS_PER_SEC * 1000.0;
}

void bench_header(const char *name) {
    printf("%-45s", name);
    fflush(stdout);
}

void bench_result(Timer *t, int64_t result) {
    printf("%10.3f ms   (result: %lld)\n", timer_elapsed_ms(t), (long long)result);
}

// ---- OOP-light: Point2D "class" ----

typedef struct { int64_t x, y; } Point2D;

int64_t point_distance_sq(Point2D a, Point2D b) {
    int64_t dx = b.x - a.x;
    int64_t dy = b.y - a.y;
    return dx*dx + dy*dy;
}

// ---- OOP-light: Vec3 "class" ----

typedef struct { int64_t x, y, z; } Vec3;

int64_t vec3_dot(Vec3 a, Vec3 b) {
    return a.x*b.x + a.y*b.y + a.z*b.z;
}

Vec3 vec3_cross(Vec3 a, Vec3 b) {
    Vec3 r;
    r.x = a.y*b.z - a.z*b.y;
    r.y = a.z*b.x - a.x*b.z;
    r.z = a.x*b.y - a.y*b.x;
    return r;
}

// ============================================================
// Benchmarks matching ADead-BIB suite
// ============================================================

// 1. Sum to 10M
int64_t sum_to_10m(void) {
    int64_t sum = 0;
    for (int64_t i = 1; i <= 10000000; i++) sum += i;
    return sum;
}

// 2. Fibonacci(45) iterative
int64_t fibonacci_iter(int n) {
    int64_t a = 0, b = 1;
    for (int i = 0; i < n; i++) {
        int64_t tmp = a + b;
        a = b;
        b = tmp;
    }
    return a;
}

// 3. Factorial(20)
int64_t factorial_iter(int n) {
    int64_t result = 1;
    for (int i = 2; i <= n; i++) result *= i;
    return result;
}

// 4. GCD marathon
int64_t gcd(int64_t a, int64_t b) {
    while (b) { int64_t t = b; b = a % b; a = t; }
    return a;
}

int64_t gcd_marathon(void) {
    int64_t sum = 0;
    for (int64_t i = 1; i <= 1000000; i++) sum += gcd(i, i + 1);
    return sum;
}

// 5. Prime count
int is_prime(int64_t n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    for (int64_t i = 5; i * i <= n; i += 6)
        if (n % i == 0 || n % (i+2) == 0) return 0;
    return 1;
}

int64_t prime_count(int64_t limit) {
    int64_t count = 0;
    for (int64_t n = 2; n <= limit; n++)
        if (is_prime(n)) count++;
    return count;
}

// 6. Collatz max
int64_t collatz_length(int64_t n) {
    int64_t steps = 0;
    while (n != 1) {
        if (n % 2 == 0) n /= 2; else n = 3*n + 1;
        steps++;
    }
    return steps;
}

int64_t collatz_max(int64_t limit) {
    int64_t max_len = 0;
    for (int64_t i = 1; i <= limit; i++) {
        int64_t len = collatz_length(i);
        if (len > max_len) max_len = len;
    }
    return max_len;
}

// 7. Popcount sum
int64_t popcount_sum(int64_t limit) {
    int64_t sum = 0;
    for (int64_t i = 1; i <= limit; i++)
        sum += __builtin_popcountll(i);
    return sum;
}

// 8. Matrix 4x4 multiply
int64_t matrix_4x4_multiply(int64_t iterations) {
    double a[4][4] = {{1,2,3,4},{5,6,7,8},{9,10,11,12},{13,14,15,16}};
    double b[4][4] = {{0.5,0.1,0.2,0.3},{0.4,0.6,0.1,0.2},{0.3,0.2,0.7,0.1},{0.1,0.3,0.2,0.8}};

    for (int64_t iter = 0; iter < iterations; iter++) {
        double c[4][4] = {0};
        for (int i = 0; i < 4; i++)
            for (int j = 0; j < 4; j++)
                for (int k = 0; k < 4; k++)
                    c[i][j] += a[i][k] * b[k][j];
        for (int i = 0; i < 4; i++)
            for (int j = 0; j < 4; j++)
                a[i][j] = c[i][j];
    }

    int64_t h = 0;
    for (int i = 0; i < 4; i++)
        for (int j = 0; j < 4; j++)
            h ^= (int64_t)(a[i][j] * 1000000.0);
    return h;
}

// 9. Recursive fibonacci
int64_t fib_recursive(int n) {
    if (n <= 1) return n;
    return fib_recursive(n-1) + fib_recursive(n-2);
}

// 10. Sum of squares
int64_t sum_of_squares(int64_t limit) {
    int64_t sum = 0;
    for (int64_t i = 1; i <= limit; i++) sum += i*i;
    return sum;
}

// 11. Sorting network 8 elements (OOP-light pattern)
int64_t sorting_network_bench(int64_t iterations) {
    int64_t checksum = 0;
    int seed = 73;
    for (int64_t iter = 0; iter < iterations; iter++) {
        int v[8];
        for (int i = 0; i < 8; i++) {
            seed = seed * 1103515245 + 12345;
            v[i] = abs(seed / 256) % 1000;
        }
        // Sorting network
        #define SWAP(a,b) if(v[a]>v[b]){int t=v[a];v[a]=v[b];v[b]=t;}
        SWAP(0,1) SWAP(2,3) SWAP(4,5) SWAP(6,7)
        SWAP(0,2) SWAP(1,3) SWAP(4,6) SWAP(5,7)
        SWAP(1,2) SWAP(5,6)
        SWAP(0,4) SWAP(1,5) SWAP(2,6) SWAP(3,7)
        SWAP(2,4) SWAP(3,5)
        SWAP(1,2) SWAP(3,4) SWAP(5,6)
        #undef SWAP
        checksum += v[0] + v[7];
    }
    return checksum;
}

// 12. Point distance (OOP-light)
int64_t point_distance_bench(void) {
    int64_t total = 0;
    for (int64_t i = 0; i < 10000000; i++) {
        Point2D a = { i % 100, i % 73 };
        Point2D b = { i % 41 + 50, i % 37 + 30 };
        total += point_distance_sq(a, b);
        if (total > 1000000000LL) total %= 1000000;
    }
    return total;
}

// 13. Vec3 cross product (OOP-light)
int64_t vec3_cross_bench(void) {
    int64_t checksum = 0;
    for (int64_t i = 0; i < 10000000; i++) {
        Vec3 a = { i%17+1, i%13+2, i%11+3 };
        Vec3 b = { i%7+4, i%19+1, i%23+2 };
        Vec3 r = vec3_cross(a, b);
        checksum += r.x + r.y + r.z;
        if (checksum > 1000000000LL) checksum %= 1000000;
    }
    return checksum;
}

// 14. Dijkstra 4-node (OOP-light graph)
int64_t dijkstra_bench(void) {
    int64_t total = 0;
    int seed = 99;
    for (int64_t iter = 0; iter < 100000; iter++) {
        int w[6];
        for (int i = 0; i < 6; i++) {
            seed = seed * 1103515245 + 12345;
            w[i] = abs(seed) % 100 + 1;
        }
        int d[4] = {0, 999999, 999999, 999999};
        int visited[4] = {0};

        for (int step = 0; step < 4; step++) {
            int u = -1, best = 999999;
            for (int j = 0; j < 4; j++)
                if (!visited[j] && d[j] < best) { best = d[j]; u = j; }
            if (u < 0) break;
            visited[u] = 1;
            // Relax edges from u
            for (int v = 0; v < 4; v++) {
                if (visited[v] || u == v) continue;
                int edge;
                int a = u < v ? u : v, b = u > v ? u : v;
                if (a==0&&b==1) edge=w[0]; else if(a==0&&b==2) edge=w[1];
                else if(a==0&&b==3) edge=w[2]; else if(a==1&&b==2) edge=w[3];
                else if(a==1&&b==3) edge=w[4]; else edge=w[5];
                if (d[u] + edge < d[v]) d[v] = d[u] + edge;
            }
        }
        total += d[3];
        if (total > 1000000000LL) total %= 1000000;
    }
    return total;
}

// 15. Modular exponentiation
int64_t mod_pow(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        exp >>= 1;
        base = base * base % mod;
    }
    return result;
}

int64_t mod_pow_bench(void) {
    int64_t sum = 0;
    for (int64_t base = 1; base <= 100000; base++)
        sum += mod_pow(base, 65537, 1000003);
    return sum;
}

// ============================================================
// Main
// ============================================================

int main(void) {
    Timer t;

    printf("============================================================\n");
    printf(" C (gcc -O3) Comparison Benchmark\n");
    printf("============================================================\n\n");

    bench_header("1. Sum to 10M");
    timer_start(&t); int64_t r = sum_to_10m(); bench_result(&t, r);

    bench_header("2. Fibonacci(45) iterative");
    timer_start(&t); r = fibonacci_iter(45); bench_result(&t, r);

    bench_header("3. Factorial(20)");
    timer_start(&t); r = factorial_iter(20); bench_result(&t, r);

    bench_header("4. GCD marathon (1..1M)");
    timer_start(&t); r = gcd_marathon(); bench_result(&t, r);

    bench_header("5. Prime count to 100K");
    timer_start(&t); r = prime_count(100000); bench_result(&t, r);

    bench_header("6. Collatz max (1..100K)");
    timer_start(&t); r = collatz_max(100000); bench_result(&t, r);

    bench_header("7. Popcount sum (1..1M)");
    timer_start(&t); r = popcount_sum(1000000); bench_result(&t, r);

    bench_header("8. Matrix 4x4 multiply 1M times");
    timer_start(&t); r = matrix_4x4_multiply(1000000); bench_result(&t, r);

    bench_header("9. Recursive fibonacci(40)");
    timer_start(&t); r = fib_recursive(40); bench_result(&t, r);

    bench_header("10. Sum of squares (1..10M)");
    timer_start(&t); r = sum_of_squares(10000000); bench_result(&t, r);

    bench_header("11. Sorting network 8-elem (100K)");
    timer_start(&t); r = sorting_network_bench(100000); bench_result(&t, r);

    bench_header("12. Point2D distance (10M) [OOP]");
    timer_start(&t); r = point_distance_bench(); bench_result(&t, r);

    bench_header("13. Vec3 cross product (10M) [OOP]");
    timer_start(&t); r = vec3_cross_bench(); bench_result(&t, r);

    bench_header("14. Dijkstra 4-node (100K) [OOP]");
    timer_start(&t); r = dijkstra_bench(); bench_result(&t, r);

    bench_header("15. Modular exp (100K calls)");
    timer_start(&t); r = mod_pow_bench(); bench_result(&t, r);

    printf("\n============================================================\n");
    printf(" Done\n");
    printf("============================================================\n");

    return 0;
}
