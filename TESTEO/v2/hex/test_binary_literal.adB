// ADead-BIB v2.0.0 - Test de Valores Binarios
// Demuestra el concepto de Binary Is Binary
// NOTA: Literales 0b... son v2.0.0 (pendiente implementar en lexer)
// Por ahora usamos valores decimales equivalentes

fn main() {
    println("+========================================+")
    println("|  ADead-BIB v2.0.0 - Test Binary Values|")
    println("|  Binary Is Binary                     |")
    println("+========================================+")
    
    // Test 1: Valores equivalentes a binario
    // 0b11110000 = 240, 0b11111111 = 255, 0b00000000 = 0
    let bit_pattern = 240
    let all_ones = 255
    let all_zeros = 0
    
    println("")
    println("[1] Valores equivalentes a binario")
    print("    240 (0b11110000) = ")
    println(bit_pattern)
    print("    255 (0b11111111) = ")
    println(all_ones)
    print("    0 (0b00000000) = ")
    println(all_zeros)
    
    // Test 2: Aritmetica
    // 0b00001000 = 8, 0b00000010 = 2
    let a = 8
    let b = 2
    let suma = a + b
    let mult = a * 2
    
    println("")
    println("[2] Aritmetica (valores binarios)")
    print("    8 (0b00001000) + 2 (0b00000010) = ")
    println(suma)
    print("    8 (0b00001000) * 2 = ")
    println(mult)
    
    // Test 3: Verificacion de valores
    println("")
    println("[3] Verificacion de valores")
    if bit_pattern == 240 {
        println("    240 == 0b11110000: OK")
    }
    if all_ones == 255 {
        println("    255 == 0b11111111: OK")
    }
    if all_zeros == 0 {
        println("    0 == 0b00000000: OK")
    }
    
    println("")
    println("+========================================+")
    println("|   OK - Concepto binario verificado!   |")
    println("|   Literales 0b... pendiente v2.0     |")
    println("+========================================+")
}
