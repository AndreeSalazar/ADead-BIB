// ADead-BIB v1.3.0 - Test de conversión int()
// Convertir valores a entero

fn main() {
    println("=== Test int() ===")
    
    // Convertir flotante a entero
    let f = 3.14
    let i = int(f)
    print("int(3.14) = ")
    println(i)
    
    // Convertir entero a entero (sin cambio)
    let x = 42
    let y = int(x)
    print("int(42) = ")
    println(y)
    
    println("Test int() completado!")
}
