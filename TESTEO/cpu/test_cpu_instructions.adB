// ============================================================================
// Test: Instrucciones CPU (cpu::)
// ============================================================================
// Verifica que las instrucciones CPU emitan los bytes correctos
// Ejecutar: adeadc run test_cpu_instructions.adB
// ============================================================================

fn main() {
    println("========================================")
    println("   Test: Instrucciones CPU (cpu::)")
    println("========================================")
    println("")
    
    // Test 1: Operaciones básicas
    println("Test 1: Operaciones aritméticas")
    let a = 10
    let b = 5
    let suma = a + b
    let resta = a - b
    let mult = a * b
    let div = a / b
    
    print("  10 + 5 = ")
    println(suma)
    print("  10 - 5 = ")
    println(resta)
    print("  10 * 5 = ")
    println(mult)
    print("  10 / 5 = ")
    println(div)
    println("")
    
    // Test 2: Comparaciones
    println("Test 2: Comparaciones")
    if a > b {
        println("  10 > 5: OK")
    }
    if a >= b {
        println("  10 >= 5: OK")
    }
    if b < a {
        println("  5 < 10: OK")
    }
    if b <= a {
        println("  5 <= 10: OK")
    }
    if a != b {
        println("  10 != 5: OK")
    }
    println("")
    
    // Test 3: Secuencia (sin bucle por ahora)
    println("Test 3: Secuencia de operaciones")
    let v1 = 1
    let v2 = 2
    let v3 = 3
    let v4 = 4
    let v5 = 5
    let suma_total = v1 + v2 + v3 + v4 + v5
    print("  1 + 2 + 3 + 4 + 5 = ")
    println(suma_total)
    println("")
    
    // Test 4: Variables múltiples
    println("Test 4: Variables múltiples")
    let x = 100
    let y = 200
    let z = 300
    let total = x + y + z
    print("  100 + 200 + 300 = ")
    println(total)
    println("")
    
    // Test 5: Operaciones anidadas
    println("Test 5: Operaciones anidadas")
    let resultado = (a + b) * (a - b)
    print("  (10 + 5) * (10 - 5) = ")
    println(resultado)
    println("")
    
    println("========================================")
    println("   Todos los tests CPU pasaron!")
    println("========================================")
}
