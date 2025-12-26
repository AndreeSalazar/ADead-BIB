// ADead-BIB v0.7.0 - Recursion
// Factorial recursivo

fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

fn main() {
    println("=== Recursion ===")
    
    let f5 = factorial(5)
    print("factorial(5) = ")
    println(f5)
    
    let f6 = factorial(6)
    print("factorial(6) = ")
    println(f6)
    
    println("Recursion funciona!")
}
