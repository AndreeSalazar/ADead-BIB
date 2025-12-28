// Test de metodos con self v1.6.0

impl Counter {
    fn create() -> i32 {
        return 0
    }
    
    fn increment(&self, amount: i32) -> i32 {
        return amount + 1
    }
    
    fn double(&self) -> i32 {
        return 2
    }
}

trait Resettable {
    fn reset(&self) -> i32;
}

impl Resettable for Counter {
    fn reset(&self) -> i32 {
        return 0
    }
}

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.6.0 - Self Methods     |")
    println("+========================================+")
    println("")
    
    println("[1] Counter methods")
    print("    Counter::create() = ")
    println(Counter::create())
    
    print("    Counter::increment(5) = ")
    println(Counter::increment(5))
    
    print("    Counter::double() = ")
    println(Counter::double())
    
    println("")
    
    println("[2] Resettable trait")
    print("    Counter::Resettable::reset() = ")
    println(Counter::Resettable::reset())
    
    println("")
    println("+========================================+")
    println("|     OK - Self methods work!           |")
    println("+========================================+")
}
