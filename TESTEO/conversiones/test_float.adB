// ADead-BIB v1.3.0 - Test de conversión float()
// Convertir valores a flotante

fn main() {
    println("=== Test float() ===")
    
    // Convertir entero a flotante
    let x = 42
    let f = float(x)
    print("float(42) = ")
    println(f)
    
    // Convertir flotante a flotante (sin cambio)
    let pi = 3.14
    let pi2 = float(pi)
    print("float(3.14) = ")
    println(pi2)
    
    println("Test float() completado!")
}
