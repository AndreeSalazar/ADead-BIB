// ADead-BIB CPU - Test de Literales Binarios
// Contrato: Bytes directos sin interpretación

fn main() {
    println("+========================================+")
    println("|   CPU - Test Literales Binarios       |")
    println("|   Contrato Binario Directo            |")
    println("+========================================+")
    
    // ==========================================
    // [1] Literales Binarios Básicos
    // ==========================================
    println("")
    println("[1] Literales Binarios")
    
    let byte_full = 0b11111111
    let byte_zero = 0b00000000
    let byte_half = 0b11110000
    
    print("    0b11111111 = ")
    println(byte_full)
    print("    0b00000000 = ")
    println(byte_zero)
    print("    0b11110000 = ")
    println(byte_half)
    
    // ==========================================
    // [2] Separadores Estilo Rust
    // ==========================================
    println("")
    println("[2] Separadores Estilo Rust")
    
    let separated = 0b1111_0000
    let word = 0b1010_1010_1010_1010
    
    print("    0b1111_0000 = ")
    println(separated)
    print("    0b1010_1010_1010_1010 = ")
    println(word)
    
    // ==========================================
    // [3] Aritmética Binaria
    // ==========================================
    println("")
    println("[3] Aritmetica Binaria")
    
    let a = 0b00001111
    let b = 0b00000001
    let suma = a + b
    let mult = a * 2
    
    print("    0b00001111 + 0b00000001 = ")
    println(suma)
    print("    0b00001111 * 2 = ")
    println(mult)
    
    // ==========================================
    // [4] Verificación de Contrato
    // ==========================================
    println("")
    println("[4] Verificacion de Contrato")
    
    if byte_full == 255 {
        println("    0b11111111 == 255: OK")
    }
    if byte_half == 240 {
        println("    0b11110000 == 240: OK")
    }
    if separated == 240 {
        println("    0b1111_0000 == 240: OK")
    }
    
    println("")
    println("+========================================+")
    println("|   OK - Contrato Binario Respetado     |")
    println("+========================================+")
}
