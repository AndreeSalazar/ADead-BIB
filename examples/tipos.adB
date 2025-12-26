// ADead-BIB v0.8.0 - Tipos de Datos
// Booleanos, enteros, flotantes

fn main() {
    println("=== Tipos de Datos ===")
    
    // Booleanos
    println("--- Booleanos ---")
    let verdadero = true
    let falso = false
    
    if verdadero {
        println("true = 1")
    }
    
    if falso {
        println("ERROR")
    } else {
        println("false = 0")
    }
    
    // Enteros
    println("--- Enteros ---")
    let a = 100
    let b = 25
    print("100 + 25 = ")
    println(a + b)
    print("100 - 25 = ")
    println(a - b)
    print("100 * 25 = ")
    println(a * b)
    print("100 / 25 = ")
    println(a / b)
    
    // Flotantes (con decimales reales)
    println("--- Flotantes ---")
    print("PI = ")
    println(3.14159)
    print("E = ")
    println(2.71828)
    print("Gravedad = ")
    println(9.81)
    
    println("Tipos funcionan!")
}
