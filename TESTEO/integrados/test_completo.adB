// ADead-BIB v1.3.0 - Test Integrado Completo
// Combina arrays, conversiones y funciones

fn suma_array(a, b, c) {
    return a + b + c
}

fn main() {
    println("=== Test Integrado v1.3.0 ===")
    
    // 1. Arrays
    println("--- Arrays ---")
    let numeros = [1, 2, 3]
    println("Array creado: [1, 2, 3]")
    
    // 2. Conversiones
    println("--- Conversiones ---")
    let x = 100
    let b = bool(x)
    print("bool(100) = ")
    println(b)
    
    let y = 0
    let b2 = bool(y)
    print("bool(0) = ")
    println(b2)
    
    // 3. Funciones con arrays
    println("--- Funciones ---")
    let resultado = suma_array(10, 20, 30)
    print("suma_array(10, 20, 30) = ")
    println(resultado)
    
    // 4. Control de flujo
    println("--- Control de Flujo ---")
    let valor = 5
    if valor > 0 {
        println("valor es positivo")
    } else {
        println("valor es cero o negativo")
    }
    
    // 5. Loops
    println("--- Loops ---")
    for i in 0..3 {
        print("i = ")
        println(i)
    }
    
    println("=== Test Integrado Completado! ===")
}
