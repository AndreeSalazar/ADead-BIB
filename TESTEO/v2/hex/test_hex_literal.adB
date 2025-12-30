// ADead-BIB v2.0.0 - Test de Valores HEX
// Demuestra el concepto de Binary Is Binary
// NOTA: Literales 0x... son v2.0.0 (pendiente implementar en lexer)
// Por ahora usamos valores decimales equivalentes

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v2.0.0 - Test HEX Values  |")
    println("|   Binary Is Binary                    |")
    println("+========================================+")
    
    // Test 1: Valores equivalentes a HEX
    // 0xFF = 255, 0x00 = 0, 0x1234 = 4660
    let byte1 = 255
    let byte2 = 0
    let word = 4660
    
    println("")
    println("[1] Valores equivalentes a HEX")
    print("    255 (0xFF) = ")
    println(byte1)
    print("    0 (0x00) = ")
    println(byte2)
    print("    4660 (0x1234) = ")
    println(word)
    
    // Test 2: Aritmetica
    // 0x10 = 16, 0x20 = 32
    let a = 16
    let b = 32
    let suma = a + b
    
    println("")
    println("[2] Aritmetica (valores HEX)")
    print("    16 (0x10) + 32 (0x20) = ")
    println(suma)
    
    // Test 3: Verificacion
    println("")
    println("[3] Verificacion de valores")
    if byte1 == 255 {
        println("    255 == 0xFF: OK")
    }
    
    if word == 4660 {
        println("    4660 == 0x1234: OK")
    }
    
    // Test 4: Multiplicacion
    let mult = 16 * 2
    println("")
    println("[4] Operaciones")
    print("    16 (0x10) * 2 = ")
    println(mult)
    
    println("")
    println("+========================================+")
    println("|     OK - Concepto HEX verificado!     |")
    println("|     Literales 0x... pendiente v2.0   |")
    println("+========================================+")
}
