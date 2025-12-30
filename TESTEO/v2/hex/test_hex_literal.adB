// ADead-BIB v2.0.0 - Test de Literales HEX
// Prueba que los literales hexadecimales (0x...) funcionan
// IMPLEMENTADO en lexer v2.0.0

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v2.0.0 - Test HEX Literal |")
    println("|   Binary Is Binary                    |")
    println("+========================================+")
    
    // Test 1: Literales HEX reales
    let byte1 = 0xFF
    let byte2 = 0x00
    let word = 0x1234
    
    println("")
    println("[1] Literales HEX reales")
    print("    0xFF = ")
    println(byte1)
    print("    0x00 = ")
    println(byte2)
    print("    0x1234 = ")
    println(word)
    
    // Test 2: Aritmetica con HEX
    let a = 0x10
    let b = 0x20
    let suma = a + b
    
    println("")
    println("[2] Aritmetica con HEX")
    print("    0x10 + 0x20 = ")
    println(suma)
    
    // Test 3: Verificacion de valores
    println("")
    println("[3] Verificacion de valores")
    if byte1 == 255 {
        println("    0xFF == 255: OK")
    }
    
    if word == 4660 {
        println("    0x1234 == 4660: OK")
    }
    
    // Test 4: HEX con separadores estilo Rust
    let big_hex = 0xFF_FF
    println("")
    println("[4] HEX con separadores")
    print("    0xFF_FF = ")
    println(big_hex)
    
    // Test 5: Multiplicacion
    let mult = 0x10 * 2
    println("")
    println("[5] Operaciones")
    print("    0x10 * 2 = ")
    println(mult)
    
    println("")
    println("+========================================+")
    println("|     OK - Literales HEX funcionan!     |")
    println("+========================================+")
}
