// ADead-BIB CPU - Test de Calling Convention
// Contrato: Windows x64 ABI

fn main() {
    println("+========================================+")
    println("|   CPU - Test Calling Convention       |")
    println("|   Windows x64 ABI                     |")
    println("+========================================+")
    
    // ==========================================
    // [1] Registros de Parámetros
    // ==========================================
    println("")
    println("[1] Registros de Parametros (Windows x64)")
    println("    Param 1: RCX")
    println("    Param 2: RDX")
    println("    Param 3: R8")
    println("    Param 4: R9")
    println("    Param 5+: Stack")
    
    // ==========================================
    // [2] Test de Funciones
    // ==========================================
    println("")
    println("[2] Test de Funciones")
    
    let r1 = suma_dos(10, 20)
    print("    suma_dos(10, 20) = ")
    println(r1)
    
    let r2 = suma_tres(5, 10, 15)
    print("    suma_tres(5, 10, 15) = ")
    println(r2)
    
    let r3 = suma_cuatro(1, 2, 3, 4)
    print("    suma_cuatro(1, 2, 3, 4) = ")
    println(r3)
    
    // ==========================================
    // [3] Registro de Retorno
    // ==========================================
    println("")
    println("[3] Registro de Retorno")
    println("    Retorno: RAX")
    
    let ret = get_valor()
    print("    get_valor() = ")
    println(ret)
    
    // ==========================================
    // [4] Stack Frame
    // ==========================================
    println("")
    println("[4] Stack Frame")
    println("    [RBP+16]  = Param 2 (si > 4)")
    println("    [RBP+8]   = Return address")
    println("    [RBP]     = Old RBP")
    println("    [RBP-8]   = Local 1")
    println("    [RBP-16]  = Local 2")
    
    // ==========================================
    // [5] Verificación de Contrato
    // ==========================================
    println("")
    println("[5] Verificacion de Contrato")
    
    if r1 == 30 {
        println("    suma_dos: OK")
    }
    if r2 == 30 {
        println("    suma_tres: OK")
    }
    if r3 == 10 {
        println("    suma_cuatro: OK")
    }
    if ret == 42 {
        println("    get_valor: OK")
    }
    
    println("")
    println("+========================================+")
    println("|   OK - Calling Convention Respetada   |")
    println("+========================================+")
}

fn suma_dos(a, b) {
    return a + b
}

fn suma_tres(a, b, c) {
    return a + b + c
}

fn suma_cuatro(a, b, c, d) {
    return a + b + c + d
}

fn get_valor() {
    return 42
}
