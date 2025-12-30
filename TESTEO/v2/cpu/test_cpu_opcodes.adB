// ADead-BIB v2.0.0 - Test de Opcodes CPU x86-64
// Demuestra los bytes que genera el compilador
// Binary Is Binary - Codigo directo a bytes

fn main() {
    println("+========================================+")
    println("|  ADead-BIB v2.0.0 - Test CPU Opcodes  |")
    println("|  Binary Is Binary                     |")
    println("+========================================+")
    
    // Tabla de opcodes x86-64 comunes
    println("")
    println("[1] Opcodes x86-64 basicos")
    
    // Prologo de funcion
    let push_rbp = 0x55
    let mov_rbp_rsp_1 = 0x48
    let mov_rbp_rsp_2 = 0x89
    let mov_rbp_rsp_3 = 0xE5
    
    print("    push rbp = 0x")
    println(push_rbp)
    print("    mov rbp,rsp = 0x")
    print(mov_rbp_rsp_1)
    print(" 0x")
    print(mov_rbp_rsp_2)
    print(" 0x")
    println(mov_rbp_rsp_3)
    
    // Epilogo de funcion
    let pop_rbp = 0x5D
    let ret = 0xC3
    
    println("")
    println("[2] Epilogo de funcion")
    print("    pop rbp = 0x")
    println(pop_rbp)
    print("    ret = 0x")
    println(ret)
    
    // Operaciones aritmeticas
    println("")
    println("[3] Operaciones aritmeticas")
    let xor_rax_rax = 0x31
    let add_rax_rcx = 0x01
    let sub_rax_rcx = 0x29
    
    print("    xor = 0x")
    println(xor_rax_rax)
    print("    add = 0x")
    println(add_rax_rcx)
    print("    sub = 0x")
    println(sub_rax_rcx)
    
    // Saltos
    println("")
    println("[4] Instrucciones de salto")
    let jmp_short = 0xEB
    let jmp_near = 0xE9
    let call = 0xE8
    
    print("    jmp short = 0x")
    println(jmp_short)
    print("    jmp near = 0x")
    println(jmp_near)
    print("    call = 0x")
    println(call)
    
    // Verificacion
    println("")
    println("[5] Verificacion")
    if push_rbp == 85 {
        println("    push rbp (0x55) = 85: OK")
    }
    if ret == 195 {
        println("    ret (0xC3) = 195: OK")
    }
    if call == 232 {
        println("    call (0xE8) = 232: OK")
    }
    
    println("")
    println("+========================================+")
    println("|   OK - Opcodes CPU verificados!       |")
    println("+========================================+")
}
