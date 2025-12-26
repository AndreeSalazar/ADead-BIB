// ADead-BIB v0.7.0 - Test de Funciones
// Funciones propias con parámetros y return

fn suma(a, b) {
    return a + b
}

fn resta(a, b) {
    return a - b
}

fn cuadrado(x) {
    return x * x
}

fn main() {
    println("=== Funciones Propias ===")
    
    let r1 = suma(10, 5)
    print("suma(10, 5) = ")
    println(r1)
    
    let r2 = resta(20, 8)
    print("resta(20, 8) = ")
    println(r2)
    
    let r3 = cuadrado(7)
    print("cuadrado(7) = ")
    println(r3)
    
    println("Funciones funcionan!")
}
