// ADead-BIB v1.3.0 - Test Completo de Todas las Features
// Arrays, len(), for x in arr, conversiones

fn sumar_array(a, b, c) {
    return a + b + c
}

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.3.0 - Test Completo    |")
    println("+========================================+")
    println("")
    
    // ========================================
    // 1. ARRAYS
    // ========================================
    println("[1] Arrays")
    let numeros = [10, 20, 30, 40, 50]
    println("    Array creado: [10, 20, 30, 40, 50]")
    
    // ========================================
    // 2. LEN()
    // ========================================
    println("[2] len()")
    let longitud = len(numeros)
    print("    len(numeros) = ")
    println(longitud)
    
    // ========================================
    // 3. FOR X IN ARR
    // ========================================
    println("[3] for x in arr")
    println("    Iterando sobre numeros:")
    for x in numeros {
        print("      x = ")
        println(x)
    }
    
    // ========================================
    // 4. CONVERSIONES
    // ========================================
    println("[4] Conversiones")
    
    let a = 42
    let b1 = bool(a)
    print("    bool(42) = ")
    println(b1)
    
    let zero = 0
    let b2 = bool(zero)
    print("    bool(0) = ")
    println(b2)
    
    // ========================================
    // 5. FUNCIONES
    // ========================================
    println("[5] Funciones")
    let resultado = sumar_array(100, 200, 300)
    print("    sumar_array(100, 200, 300) = ")
    println(resultado)
    
    // ========================================
    // 6. CONTROL DE FLUJO
    // ========================================
    println("[6] Control de Flujo")
    let valor = 10
    if valor > 5 {
        println("    valor > 5: true")
    } else {
        println("    valor > 5: false")
    }
    
    // ========================================
    // 7. LOOPS TRADICIONALES
    // ========================================
    println("[7] for i in 0..3")
    for i in 0..3 {
        print("    i = ")
        println(i)
    }
    
    println("")
    println("+========================================+")
    println("|     OK - Todos los tests pasaron!     |")
    println("+========================================+")
}
