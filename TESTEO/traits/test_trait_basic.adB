// Test básico de traits v1.6.0
// Definición de trait y su implementación

trait Calculable {
    fn compute(x: i32) -> i32;
}

impl Calculable for Calculator {
    fn compute(x: i32) -> i32 {
        return x * 2
    }
}

impl Calculator {
    fn add(a: i32, b: i32) -> i32 {
        return a + b
    }
    
    fn multiply(a: i32, b: i32) -> i32 {
        return a * b
    }
}

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.6.0 - Traits Test      |")
    println("+========================================+")
    println("")
    
    // Test impl Struct - static methods
    print("[1] Calculator::add(10, 20) = ")
    println(Calculator::add(10, 20))
    
    print("[2] Calculator::multiply(5, 6) = ")
    println(Calculator::multiply(5, 6))
    
    // Test impl Trait for Struct
    print("[3] Calculator::Calculable::compute(25) = ")
    println(Calculator::Calculable::compute(25))
    
    println("")
    println("+========================================+")
    println("|     OK - Traits work!                 |")
    println("+========================================+")
}
