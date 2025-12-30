// ADead-BIB Library for Python FFI
// Compila a DLL/SO para uso desde Python

// Funciones exportables para FFI
fn count_to(limit: i32) -> i32 {
    let counter = 0
    while counter < limit {
        counter = counter + 1
    }
    return counter
}

fn count_billion() -> i32 {
    return count_to(1000000000)
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n
    }
    let a = 0
    let b = 1
    let i = 2
    while i <= n {
        let temp = a + b
        a = b
        b = temp
        i = i + 1
    }
    return b
}

fn factorial(n: i32) -> i32 {
    let result = 1
    let i = 1
    while i <= n {
        result = result * i
        i = i + 1
    }
    return result
}

fn sum_array(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    return a + b + c + d + e
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b
}

fn power(base: i32, exp: i32) -> i32 {
    let result = 1
    let i = 0
    while i < exp {
        result = result * base
        i = i + 1
    }
    return result
}

// Test standalone
fn main() {
    println("ADead-BIB Library Test")
    println("======================")
    
    print("count_to(100) = ")
    println(count_to(100))
    
    print("fibonacci(10) = ")
    println(fibonacci(10))
    
    print("factorial(5) = ")
    println(factorial(5))
    
    print("multiply(7, 8) = ")
    println(multiply(7, 8))
    
    print("power(2, 10) = ")
    println(power(2, 10))
    
    println("")
    println("Library ready for Python FFI!")
}
