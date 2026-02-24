// ============================================================
// Rust Comparison Benchmark (compile with: rustc -O)
// ============================================================
// Matches ADead-BIB benchmark suite for direct comparison

use std::hint::black_box;
use std::time::Instant;

fn bench<F: FnOnce() -> u64>(name: &str, f: F) {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    let ms = elapsed.as_secs_f64() * 1000.0;
    println!("{:<40} {:>10.3} ms   (result: {})", name, ms, result);
}

// --------------- benchmarks ---------------

fn sum_to_10m() -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=10_000_000u64 {
        sum += i;
    }
    sum
}

fn fibonacci_iterative(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let tmp = a.wrapping_add(b);
        a = b;
        b = tmp;
    }
    a
}

fn factorial_iterative(n: u64) -> u64 {
    let mut result: u64 = 1;
    for i in 2..=n {
        result = result.wrapping_mul(i);
    }
    result
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn gcd_marathon() -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=1_000_000u64 {
        sum += gcd(i, i + 1);
    }
    sum
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn prime_count(limit: u64) -> u64 {
    let mut count = 0u64;
    for n in 2..=limit {
        if is_prime(n) {
            count += 1;
        }
    }
    count
}

fn collatz_length(mut n: u64) -> u64 {
    let mut steps = 0u64;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    steps
}

fn collatz_max(limit: u64) -> u64 {
    let mut max_len = 0u64;
    for i in 1..=limit {
        let len = collatz_length(i);
        if len > max_len {
            max_len = len;
        }
    }
    max_len
}

fn popcount_sum(limit: u64) -> u64 {
    let mut sum = 0u64;
    for i in 1..=limit {
        sum += i.count_ones() as u64;
    }
    sum
}

fn matrix_4x4_multiply(iterations: u64) -> u64 {
    let mut a = [
        [1.0f64, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ];
    let b = [
        [0.5, 0.1, 0.2, 0.3],
        [0.4, 0.6, 0.1, 0.2],
        [0.3, 0.2, 0.7, 0.1],
        [0.1, 0.3, 0.2, 0.8],
    ];

    for _ in 0..iterations {
        let mut c = [[0.0f64; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut s = 0.0;
                for k in 0..4 {
                    s += a[i][k] * b[k][j];
                }
                c[i][j] = s;
            }
        }
        a = c;
    }

    // Return a hash-like value from the final matrix
    let mut h = 0u64;
    for i in 0..4 {
        for j in 0..4 {
            h ^= (a[i][j] * 1_000_000.0) as u64;
        }
    }
    h
}

fn fibonacci_recursive(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn sum_of_squares(limit: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=limit {
        sum += i * i;
    }
    sum
}

fn main() {
    println!("============================================================");
    println!(" Rust Comparison Benchmark");
    println!("============================================================");
    println!();

    bench("1. Sum to 10M", || sum_to_10m());

    bench("2. Fibonacci(45) iterative", || {
        black_box(fibonacci_iterative(45))
    });

    bench("3. Factorial(20) iterative", || {
        black_box(factorial_iterative(20))
    });

    bench("4. GCD marathon (1..1M)", || gcd_marathon());

    bench("5. Prime count up to 100000", || prime_count(100_000));

    bench("6. Collatz max length (1..100000)", || {
        collatz_max(100_000)
    });

    bench("7. Popcount sum (1..1M)", || popcount_sum(1_000_000));

    bench("8. Matrix 4x4 multiply 1M times", || {
        matrix_4x4_multiply(1_000_000)
    });

    bench("9. Recursive fibonacci(40)", || {
        black_box(fibonacci_recursive(40))
    });

    bench("10. Sum of squares (1..10M)", || sum_of_squares(10_000_000));

    bench("11. Sorting network 8-elem (100K)", || {
        sorting_network_bench(100_000)
    });

    bench("12. Point2D distance (10M) [OOP]", || {
        point_distance_bench()
    });

    bench("13. Vec3 cross product (10M) [OOP]", || {
        vec3_cross_bench()
    });

    bench("14. Dijkstra 4-node (100K) [OOP]", || {
        dijkstra_bench()
    });

    bench("15. Modular exp (100K calls)", || {
        mod_pow_bench()
    });

    println!();
    println!("============================================================");
    println!(" Done");
    println!("============================================================");
}

// ---- OOP-light: Point2D ----

struct Point2D { x: i64, y: i64 }

impl Point2D {
    fn distance_sq(&self, other: &Point2D) -> i64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx*dx + dy*dy
    }
}

fn point_distance_bench() -> u64 {
    let mut total: i64 = 0;
    for i in 0..10_000_000i64 {
        let a = Point2D { x: i % 100, y: i % 73 };
        let b = Point2D { x: i % 41 + 50, y: i % 37 + 30 };
        total += a.distance_sq(&b);
        if total > 1_000_000_000 { total %= 1_000_000; }
    }
    total as u64
}

// ---- OOP-light: Vec3 ----

struct Vec3 { x: i64, y: i64, z: i64 }

impl Vec3 {
    fn cross(&self, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y*b.z - self.z*b.y,
            y: self.z*b.x - self.x*b.z,
            z: self.x*b.y - self.y*b.x,
        }
    }
}

fn vec3_cross_bench() -> u64 {
    let mut checksum: i64 = 0;
    for i in 0..10_000_000i64 {
        let a = Vec3 { x: i%17+1, y: i%13+2, z: i%11+3 };
        let b = Vec3 { x: i%7+4, y: i%19+1, z: i%23+2 };
        let r = a.cross(&b);
        checksum += r.x + r.y + r.z;
        if checksum > 1_000_000_000 { checksum %= 1_000_000; }
    }
    checksum as u64
}

// ---- Sorting network ----

fn sorting_network_bench(iterations: u64) -> u64 {
    let mut checksum: u64 = 0;
    let mut seed: i32 = 73;
    for _ in 0..iterations {
        let mut v = [0i32; 8];
        for j in 0..8 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            v[j] = (seed / 256).abs() % 1000;
        }
        macro_rules! cswap { ($a:expr,$b:expr) => { if v[$a] > v[$b] { v.swap($a,$b); } } }
        cswap!(0,1); cswap!(2,3); cswap!(4,5); cswap!(6,7);
        cswap!(0,2); cswap!(1,3); cswap!(4,6); cswap!(5,7);
        cswap!(1,2); cswap!(5,6);
        cswap!(0,4); cswap!(1,5); cswap!(2,6); cswap!(3,7);
        cswap!(2,4); cswap!(3,5);
        cswap!(1,2); cswap!(3,4); cswap!(5,6);
        checksum += (v[0] + v[7]) as u64;
    }
    checksum
}

// ---- Dijkstra 4-node graph ----

fn dijkstra_bench() -> u64 {
    let mut total: i64 = 0;
    let mut seed: i32 = 99;
    for _ in 0..100_000u64 {
        let mut w = [0i32; 6];
        for i in 0..6 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            w[i] = seed.abs() % 100 + 1;
        }
        let mut d = [0i64, 999999, 999999, 999999];
        let mut visited = [false; 4];

        for _ in 0..4 {
            let mut u = usize::MAX;
            let mut best = 999999i64;
            for j in 0..4 {
                if !visited[j] && d[j] < best { best = d[j]; u = j; }
            }
            if u == usize::MAX { break; }
            visited[u] = true;
            for v in 0..4 {
                if visited[v] || u == v { continue; }
                let (a, b) = if u < v { (u, v) } else { (v, u) };
                let edge = match (a, b) {
                    (0,1) => w[0], (0,2) => w[1], (0,3) => w[2],
                    (1,2) => w[3], (1,3) => w[4], _ => w[5],
                } as i64;
                if d[u] + edge < d[v] { d[v] = d[u] + edge; }
            }
        }
        total += d[3];
        if total > 1_000_000_000 { total %= 1_000_000; }
    }
    total as u64
}

// ---- Modular exponentiation ----

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % modulus; }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

fn mod_pow_bench() -> u64 {
    let mut sum = 0u64;
    for base in 1..=100_000u64 {
        sum += mod_pow(base, 65537, 1000003);
    }
    sum
}
