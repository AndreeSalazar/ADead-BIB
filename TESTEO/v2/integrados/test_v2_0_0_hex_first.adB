// ADead-BIB v2.0.0 - Test Completo HEX-First Architecture
// Test integrado de la filosofia Binary Is Binary
// IMPLEMENTADO: Literales 0x/0b funcionan en lexer v2.0.0

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v2.0.0 - HEX-First Test   |")
    println("|   Binary Is Binary                    |")
    println("+========================================+")
    
    // ==========================================
    // [1] Literales HEX reales
    // ==========================================
    println("")
    println("[1] Literales HEX")
    let hex_byte = 0xFF
    let hex_word = 0x1234
    
    print("    0xFF = ")
    println(hex_byte)
    print("    0x1234 = ")
    println(hex_word)
    
    // ==========================================
    // [2] Literales Binarios reales
    // ==========================================
    println("")
    println("[2] Literales Binarios")
    let bin_byte = 0b11110000
    let bin_mask = 0b10101010
    
    print("    0b11110000 = ")
    println(bin_byte)
    print("    0b10101010 = ")
    println(bin_mask)
    
    // ==========================================
    // [3] Aritmetica con HEX
    // ==========================================
    println("")
    println("[3] Aritmetica HEX")
    let a = 0xF0
    let b = 0x0F
    let suma_hex = a + b
    let mult_hex = a * 2
    
    print("    0xF0 + 0x0F = ")
    println(suma_hex)
    print("    0xF0 * 2 = ")
    println(mult_hex)
    
    // ==========================================
    // [4] Sintaxis Humana (Simple)
    // ==========================================
    println("")
    println("[4] Sintaxis Humana")
    
    let nombre = "ADead-BIB"
    let version = 2
    let activo = true
    
    print("    Nombre: ")
    println(nombre)
    print("    Version: ")
    println(version)
    print("    Activo: ")
    println(activo)
    
    // ==========================================
    // [5] Control de Flujo
    // ==========================================
    println("")
    println("[5] Control de Flujo")
    
    if hex_byte == 255 {
        println("    if 0xFF == 255: OK")
    }
    
    let contador = 0
    for i in 0..3 {
        contador = contador + 1
    }
    print("    for 0..3 contador = ")
    println(contador)
    
    // ==========================================
    // [6] Funciones
    // ==========================================
    println("")
    println("[6] Funciones")
    let suma = sumar(10, 20)
    print("    sumar(10, 20) = ")
    println(suma)
    
    let doble = duplicar(16)
    print("    duplicar(16) = ")
    println(doble)
    
    // ==========================================
    // Resultado Final
    // ==========================================
    println("")
    println("+========================================+")
    println("|     OK - v2.0.0 HEX-First funciona!   |")
    println("|     Codigo -> Bytes -> Binario        |")
    println("+========================================+")
}

fn sumar(a, b) {
    return a + b
}

fn duplicar(valor) {
    return valor * 2
}
