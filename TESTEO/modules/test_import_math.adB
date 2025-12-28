// ADead-BIB v1.5.0 - Test Module Import
// Tests importing from std::math

from std::math import abs, max, min, pow

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.5.0 - Test Modules     |")
    println("+========================================+")
    println("")
    
    // Test abs()
    print("[1] abs(-5) = ")
    println(abs(-5))
    
    print("    abs(10) = ")
    println(abs(10))
    
    // Test max()
    print("[2] max(3, 7) = ")
    println(max(3, 7))
    
    // Test min()
    print("[3] min(3, 7) = ")
    println(min(3, 7))
    
    // Test pow()
    print("[4] pow(2, 8) = ")
    println(pow(2, 8))
    
    print("    pow(3, 4) = ")
    println(pow(3, 4))
    
    println("")
    println("+========================================+")
    println("|     OK - Modules work!                |")
    println("+========================================+")
}
