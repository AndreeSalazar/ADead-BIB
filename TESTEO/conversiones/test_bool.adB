// ADead-BIB v1.3.0 - Test de conversión bool()
// Convertir valores a booleano

fn main() {
    println("=== Test bool() ===")
    
    // Convertir número a booleano
    let x = 42
    let b1 = bool(x)
    print("bool(42) = ")
    println(b1)
    
    // Convertir 0 a booleano (false)
    let zero = 0
    let b2 = bool(zero)
    print("bool(0) = ")
    println(b2)
    
    // Convertir 1 a booleano (true)
    let one = 1
    let b3 = bool(one)
    print("bool(1) = ")
    println(b3)
    
    println("Test bool() completado!")
}
