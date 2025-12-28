// ADead-BIB v1.5.0 - Test Local Module Import
// Tests importing from a local module

from mymath import double, triple, square

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.5.0 - Local Modules    |")
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
    println("|     OK - Local modules work!          |")
    println("+========================================+")
}
