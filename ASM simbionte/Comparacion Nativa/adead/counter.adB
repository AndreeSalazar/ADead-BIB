// ADead-BIB Nativo - Benchmark de Loop
// Compila con: adeadc build counter.adB -o counter.exe

fn main() {
    let counter = 0
    
    while counter < 1000000000 {
        counter += 1
    }
    
    println(counter)
}
