// ADead-BIB v2.3.0 - Test de Post-Procesamiento
// Compara tamaños de binario con y sin limpieza

fn main() {
    println("+========================================+")
    println("|  ADead-BIB v2.3.0 - Size Comparison   |")
    println("|  Post-Procesamiento: Eliminar Ruido   |")
    println("+========================================+")
    
    // Este test genera código que puede ser optimizado
    // para demostrar la reducción de tamaño
    
    println("")
    println("[1] Codigo con potencial de optimizacion")
    
    // Constantes que pueden ser folded
    let a = 10
    let b = 20
    let c = a + b
    print("    10 + 20 = ")
    println(c)
    
    // Strings que podrían deduplicarse
    println("")
    println("[2] Strings duplicados (dedup test)")
    println("    Hello")
    println("    Hello")
    println("    Hello")
    
    // Loop simple
    println("")
    println("[3] Loop simple")
    let sum = 0
    for i in 0..5 {
        sum = sum + i
    }
    print("    sum(0..5) = ")
    println(sum)
    
    // Resultado
    println("")
    println("+========================================+")
    println("|  Compilar con: --clean aggressive     |")
    println("|  para ver reduccion de tamano         |")
    println("+========================================+")
}
