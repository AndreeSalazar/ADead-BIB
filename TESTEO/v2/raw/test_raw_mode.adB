// ADead-BIB v2.0.0 - Test de Modo Raw
// Prueba el concepto de compilacion a bytes puros sin headers
// NOTA: #![mode(raw)] es una caracteristica FUTURA (v2.1.0)
// Este test demuestra el concepto

fn main() {
    println("+========================================+")
    println("|  ADead-BIB v2.0.0 - Test Raw Mode     |")
    println("|  Binary Is Binary                     |")
    println("+========================================+")
    
    // Concepto de modo raw:
    // #![mode(raw)]
    // #![base(0x1000)]
    // Genera solo bytes de codigo, sin headers PE/ELF
    
    println("")
    println("[1] Concepto modo raw")
    println("    #![mode(raw)] = bytes puros")
    println("    #![base(0x1000)] = direccion base")
    
    // Tamaños tipicos
    let pe_hello = 2048
    let raw_hello = 64
    let reduccion = pe_hello - raw_hello
    
    println("")
    println("[2] Comparacion de tamaños")
    print("    PE Hello World: ")
    print(pe_hello)
    println(" bytes")
    print("    Raw Hello World: ~")
    print(raw_hello)
    println(" bytes")
    print("    Reduccion: ")
    print(reduccion)
    println(" bytes")
    
    // Bytes de un programa minimo
    println("")
    println("[3] Bytes de programa minimo")
    println("    push rbp        = 0x55")
    println("    mov rbp, rsp    = 0x48 0x89 0xE5")
    println("    xor eax, eax    = 0x31 0xC0")
    println("    pop rbp         = 0x5D")
    println("    ret             = 0xC3")
    
    // Verificacion con literales HEX
    let push_rbp = 0x55
    let pop_rbp = 0x5D
    let ret = 0xC3
    
    println("")
    println("[4] Verificacion de bytes")
    if push_rbp == 85 {
        println("    0x55 == 85: OK")
    }
    if ret == 195 {
        println("    0xC3 == 195: OK")
    }
    
    println("")
    println("+========================================+")
    println("|   #![mode(raw)] pendiente v2.1.0     |")
    println("|   Concepto verificado                 |")
    println("+========================================+")
}
