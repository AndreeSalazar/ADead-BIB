// ADead-BIB v2.0.0 - Test de Literales Binarios
// Prueba que los literales binarios (0b...) funcionan
// IMPLEMENTADO en lexer v2.0.0

fn main() {
    println("+========================================+")
    println("|  ADead-BIB v2.0.0 - Test Binary Literal|")
    println("|  Binary Is Binary                     |")
    println("+========================================+")
    
    // Test 1: Literales binarios reales
    let bit_pattern = 0b11110000
    let all_ones = 0b11111111
    let all_zeros = 0b00000000
    
    println("")
    println("[1] Literales binarios reales")
    print("    0b11110000 = ")
    println(bit_pattern)
    print("    0b11111111 = ")
    println(all_ones)
    print("    0b00000000 = ")
    println(all_zeros)
    
    // Test 2: Aritmetica con binarios
    let a = 0b00001000
    let b = 0b00000010
    let suma = a + b
    let mult = a * 2
    
    println("")
    println("[2] Aritmetica con binarios")
    print("    0b00001000 + 0b00000010 = ")
    println(suma)
    print("    0b00001000 * 2 = ")
    println(mult)
    
    // Test 3: Verificacion de valores
    println("")
    println("[3] Verificacion de valores")
    if bit_pattern == 240 {
        println("    0b11110000 == 240: OK")
    }
    if all_ones == 255 {
        println("    0b11111111 == 255: OK")
    }
    if all_zeros == 0 {
        println("    0b00000000 == 0: OK")
    }
    
    // Test 4: Binario con separadores estilo Rust
    let separated = 0b1111_0000
    println("")
    println("[4] Binario con separadores")
    print("    0b1111_0000 = ")
    println(separated)
    
    println("")
    println("+========================================+")
    println("|   OK - Literales binarios funcionan!  |")
    println("+========================================+")
}
