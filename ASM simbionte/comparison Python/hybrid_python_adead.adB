// ============================================
// HYBRID: Python + ADead-BIB
// Sintaxis Python-style + Compilación nativa
// ============================================

// Python-style classes con def y self
class Counter:
    def run(self, limit):
        let i = 0
        while i < limit {
            i = i + 1
        }
        return i

class Timer:
    def benchmark(self, iterations):
        return Counter::run(iterations)

// Rust-style function como entry point
fn main() {
    // Python: counter = 0
    // ADead:  let counter = 0
    let counter = 0
    
    // Python: while (counter < 1000000000):
    // ADead:  while counter < 1000000000 {
    while counter < 1000000000 {
        // Python: counter += 1
        // ADead:  counter = counter + 1
        counter = counter + 1
    }
    
    // Python: print(counter)
    // ADead:  println(counter)
    println(counter)
    
    // Bonus: usando clase Python-style
    println(Timer::benchmark(100))
}
