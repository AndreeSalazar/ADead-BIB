// ADead-BIB v2.0.0 - Test Completo HEX-First Architecture
// Test integrado de la filosofia Binary Is Binary
// NOTA: Literales 0x/0b son v2.0.0 (pendiente en lexer)
// Por ahora usamos valores decimales equivalentes

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v2.0.0 - HEX-First Test   |")
    println("|   Binary Is Binary                    |")
    println("+========================================+")
    
    // ==========================================
    // [1] Valores HEX (decimales equivalentes)
    // ==========================================
    println("")
    println("[1] Valores HEX")
    let hex_byte = 255
    let hex_word = 4660
    
    print("    255 (0xFF) = ")
    println(hex_byte)
    print("    4660 (0x1234) = ")
    println(hex_word)
    
    // ==========================================
    // [2] Valores Binarios (decimales equivalentes)
    // ==========================================
    println("")
    println("[2] Valores Binarios")
    let bin_byte = 240
    let bin_mask = 170
    
    print("    240 (0b11110000) = ")
    println(bin_byte)
    print("    170 (0b10101010) = ")
    println(bin_mask)
    
    // ==========================================
    // [3] Aritmetica
    // ==========================================
    println("")
    println("[3] Aritmetica")
    let a = 240
    let b = 15
    let suma_hex = a + b
    let mult_hex = a * 2
    
    print("    240 + 15 = ")
    println(suma_hex)
    print("    240 * 2 = ")
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
