// Rust Library for Python FFI
// Compila con: cargo build --release
// ============================================================
// BENCHMARK BLINDADO - LLVM NO PUEDE ELIMINAR ESTE LOOP
// ============================================================
// Usamos:
// 1. core::arch::asm! con "memory" clobber
// 2. volatile write al final
// 3. inline(never) para prevenir inlining
// ============================================================

use std::arch::asm;
use std::ptr;

// Variable global volatile - LLVM no puede ignorarla
static mut SINK: i64 = 0;

/// Barrera de memoria REAL usando asm volatile
/// LLVM NO puede optimizar a través de esto
#[inline(always)]
fn memory_barrier(val: &mut i64) {
    unsafe {
        asm!(
            "", // instrucción vacía
            inout("rax") *val,  // val debe estar en rax
            options(nostack, preserves_flags)
        );
    }
}

/// Barrera que fuerza que el valor sea "observable"
#[inline(always)]
fn escape(val: i64) {
    unsafe {
        asm!(
            "", // instrucción vacía pero...
            in("rax") val,  // val DEBE existir en rax
            options(nostack, preserves_flags, readonly)
        );
    }
}

/// VERSION 1: Con barrera cada iteración (más lento pero seguro)
#[no_mangle]
#[inline(never)]
pub extern "C" fn count_to(limit: i64) -> i64 {
    let mut counter: i64 = 0;
    
    // Forzar que limit venga de "afuera" (no constante)
    escape(limit);
    
    while counter < limit {
        counter += 1;
        // Barrera cada iteración - LLVM NO puede eliminar
        escape(counter);
    }
    
    // Escribir a memoria volatile - efecto observable REAL
    unsafe {
        ptr::write_volatile(&mut SINK, counter);
    }
    
    counter
}

/// VERSION 2: Sin barrera por iteración - MÁS JUSTO vs ADead-BIB
/// Solo barreras al inicio y final
#[no_mangle]
#[inline(never)]
pub extern "C" fn count_to_fair(limit: i64) -> i64 {
    let mut counter: i64 = 0;
    
    // Barrera solo al inicio
    escape(limit);
    escape(counter);
    
    // Loop PURO - sin barreras internas
    while counter < limit {
        counter += 1;
    }
    
    // Barrera solo al final + volatile write
    unsafe {
        ptr::write_volatile(&mut SINK, counter);
    }
    escape(counter);
    
    counter
}

/// Versión "fair" de count_billion
#[no_mangle]
#[inline(never)]
pub extern "C" fn count_billion_fair() -> i64 {
    let limit: i64 = 1_000_000_000;
    escape(limit);
    count_to_fair(limit)
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn count_billion() -> i64 {
    // El límite viene de "afuera" - no es constante en compile time
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
