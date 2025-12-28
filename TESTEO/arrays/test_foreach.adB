// ADead-BIB v1.3.0 - Test de for x in arr
// Iteracion sobre arrays

fn main() {
    println("=== Test for x in arr ===")
    
    // Crear array
    let numeros = [10, 20, 30]
    
    println("Iterando sobre [10, 20, 30]:")
    
    // Iterar sobre el array
    for x in numeros {
        print("  x = ")
        println(x)
    }
    
    println("Test foreach completado!")
}
