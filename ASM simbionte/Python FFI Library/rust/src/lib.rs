// Rust Library for Python FFI
// Compila con: cargo build --release

#[no_mangle]
pub extern "C" fn count_to(limit: i64) -> i64 {
    let mut counter: i64 = 0;
    while counter < limit {
        counter += 1;
    }
    counter
}

#[no_mangle]
pub extern "C" fn count_billion() -> i64 {
    count_to(1_000_000_000)
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
