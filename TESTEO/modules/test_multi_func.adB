// Test multiple functions (same as import test but all in one file)
fn double(x) {
    return x * 2
}

fn triple(x) {
    return x * 3
}

fn square(x) {
    return x * x
}

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.5.0 - Multi Functions  |")
    println("+========================================+")
    println("")
    
    let x = 5
    
    print("[1] double(5) = ")
    println(double(x))
    
    print("[2] triple(5) = ")
    println(triple(x))
    
    print("[3] square(5) = ")
    println(square(x))
    
    println("")
    println("+========================================+")
    println("|     OK - All functions work!          |")
    println("+========================================+")
}
