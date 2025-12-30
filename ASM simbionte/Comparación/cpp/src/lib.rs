// ============================================================
// C++ SIMULADO con Rust/LLVM (mismo backend que clang++ -O3)
// ============================================================
// BENCHMARK BLINDADO - LLVM NO PUEDE ELIMINAR ESTE LOOP
// ============================================================

use std::arch::asm;
use std::ptr;

static mut SINK: i64 = 0;

/// Barrera que fuerza que el valor sea "observable"
#[inline(always)]
fn escape(val: i64) {
    unsafe {
        asm!(
            "",
            in("rax") val,
            options(nostack, preserves_flags, readonly)
        );
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn count_to(limit: i64) -> i64 {
    let mut counter: i64 = 0;
    escape(limit);
    
    while counter < limit {
        counter += 1;
        escape(counter);
    }
    
    unsafe {
        ptr::write_volatile(&mut SINK, counter);
    }
    
    counter
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn count_billion() -> i64 {
    let limit: i64 = 1_000_000_000;
    escape(limit);
    count_to(limit)
}

#[no_mangle]
pub extern "C" fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[no_mangle]
pub extern "C" fn factorial(n: i64) -> i64 {
    let mut result: i64 = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[no_mangle]
pub extern "C" fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

#[no_mangle]
pub extern "C" fn power(base: i64, exp: i64) -> i64 {
    let mut result: i64 = 1;
    for _ in 0..exp {
        result *= base;
    }
    result
}
