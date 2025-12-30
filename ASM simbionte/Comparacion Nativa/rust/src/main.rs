// Rust Nativo - Benchmark de Loop
// Compila con: cargo build --release

use std::time::Instant;
use std::hint::black_box;

// VERSION 1: Con black_box (previene optimización)
fn count_to_safe(limit: i64) -> i64 {
    let mut counter: i64 = 0;
    let limit = black_box(limit);
    while counter < limit {
        counter += 1;
    }
    black_box(counter)
}

// VERSION 2: Sin black_box (LLVM puede optimizar)
#[inline(never)]
fn count_to_raw(limit: i64) -> i64 {
    let mut counter: i64 = 0;
    while counter < limit {
        counter += 1;
    }
    counter
}

fn main() {
    const ITERATIONS: i64 = 1_000_000_000;  // 1 billón
    
    println!("Rust Nativo - Loop de {} iteraciones", ITERATIONS);
    println!("");
    
    // Test 1: Con black_box
    println!("[Con black_box - trabajo real]");
    let start = Instant::now();
    let result = count_to_safe(ITERATIONS);
    let duration = start.elapsed();
    println!("Resultado: {}", result);
    println!("Tiempo: {:.3}s", duration.as_secs_f64());
    println!("");
    
    // Test 2: Sin black_box
    println!("[Sin black_box - LLVM puede optimizar]");
    let start = Instant::now();
    let result = count_to_raw(ITERATIONS);
    let duration = start.elapsed();
    println!("Resultado: {}", result);
    println!("Tiempo: {:.3}s", duration.as_secs_f64());
}
