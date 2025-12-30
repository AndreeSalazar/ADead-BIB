// ADead-BIB GPU - Test de Literales HEX
// Contrato: Opcodes HEX directos sin interpretación

fn main() {
    println("+========================================+")
    println("|   GPU - Test Literales HEX            |")
    println("|   Contrato HEX Directo                |")
    println("+========================================+")
    
    // ==========================================
    // [1] Literales HEX Básicos
    // ==========================================
    println("")
    println("[1] Literales HEX")
    
    let byte = 0xFF
    let word = 0x1234
    let dword = 0xDEAD
    
    print("    0xFF = ")
    println(byte)
    print("    0x1234 = ")
    println(word)
    print("    0xDEAD = ")
    println(dword)
    
    // ==========================================
    // [2] Separadores Estilo Rust
    // ==========================================
    println("")
    println("[2] Separadores Estilo Rust")
    
    let separated = 0xFF_FF
    let big = 0xAB_CD_EF
    
    print("    0xFF_FF = ")
    println(separated)
    print("    0xAB_CD_EF = ")
    println(big)
    
    // ==========================================
    // [3] Aritmética HEX
    // ==========================================
    println("")
    println("[3] Aritmetica HEX")
    
    let a = 0xF0
    let b = 0x0F
    let suma = a + b
    let mult = a * 2
    
    print("    0xF0 + 0x0F = ")
    println(suma)
    print("    0xF0 * 2 = ")
    println(mult)
    
    // ==========================================
    // [4] Verificación de Contrato
    // ==========================================
    println("")
    println("[4] Verificacion de Contrato")
    
    if byte == 255 {
        println("    0xFF == 255: OK")
    }
    if word == 4660 {
        println("    0x1234 == 4660: OK")
    }
    if separated == 65535 {
        println("    0xFF_FF == 65535: OK")
    }
    
    println("")
    println("+========================================+")
    println("|   OK - Contrato HEX Respetado         |")
    println("+========================================+")
}
