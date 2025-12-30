// ADead-BIB - Contador de 1 billón
// Compila a binario nativo x86-64 - MUCHO más rápido

fn main() {
    let counter = 0
    
    while counter < 1000000000 {
        counter = counter + 1
    }
    
    println(counter)
}
