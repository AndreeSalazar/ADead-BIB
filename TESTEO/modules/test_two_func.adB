// Test with exactly 2 auxiliary functions
fn double(x) {
    return x * 2
}

fn triple(x) {
    return x * 3
}

fn main() {
    println("Testing 2 functions...")
    
    let x = 5
    
    print("double(5) = ")
    println(double(x))
    
    print("triple(5) = ")
    println(triple(x))
    
    println("Done!")
}
