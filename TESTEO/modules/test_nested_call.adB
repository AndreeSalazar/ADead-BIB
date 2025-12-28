// Test nested call - println(double(x))
fn double(x) {
    return x * 2
}

fn main() {
    println("Testing nested call...")
    let x = 5
    println(double(x))
    println("Done!")
}
