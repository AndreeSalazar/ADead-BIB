// ADead-BIB CPU - Test de Opcodes x86-64
// Contrato: Bytes de instrucciones directos

fn main() {
    println("+========================================+")
    println("|   CPU - Test Opcodes x86-64           |")
    println("|   Contrato Binario Directo            |")
    println("+========================================+")
    
    // ==========================================
    // [1] Prólogo de Función
    // ==========================================
    println("")
    println("[1] Prologo de Funcion")
    
    let push_rbp = 0x55
    let mov_rbp_rsp_1 = 0x48
    let mov_rbp_rsp_2 = 0x89
    let mov_rbp_rsp_3 = 0xE5
    
    print("    push rbp     = 0x")
    println(push_rbp)
    print("    mov rbp,rsp  = 0x")
    print(mov_rbp_rsp_1)
    print(" 0x")
    print(mov_rbp_rsp_2)
    print(" 0x")
    println(mov_rbp_rsp_3)
    
    // ==========================================
    // [2] Epílogo de Función
    // ==========================================
    println("")
    println("[2] Epilogo de Funcion")
    
    let pop_rbp = 0x5D
    let ret = 0xC3
    
    print("    pop rbp      = 0x")
    println(pop_rbp)
    print("    ret          = 0x")
    println(ret)
    
    // ==========================================
    // [3] Operaciones Aritméticas
    // ==========================================
    println("")
    println("[3] Operaciones Aritmeticas")
    
    let xor_opcode = 0x31
    let add_opcode = 0x01
    let sub_opcode = 0x29
    let inc_opcode = 0xFF
    
    print("    xor          = 0x")
    println(xor_opcode)
    print("    add          = 0x")
    println(add_opcode)
    print("    sub          = 0x")
    println(sub_opcode)
    print("    inc/dec      = 0x")
    println(inc_opcode)
    
    // ==========================================
    // [4] Saltos y Llamadas
    // ==========================================
    println("")
    println("[4] Saltos y Llamadas")
    
    let jmp_short = 0xEB
    let jmp_near = 0xE9
    let call_near = 0xE8
    let je_short = 0x74
    let jne_short = 0x75
    
    print("    jmp short    = 0x")
    println(jmp_short)
    print("    jmp near     = 0x")
    println(jmp_near)
    print("    call         = 0x")
    println(call_near)
    print("    je           = 0x")
    println(je_short)
    print("    jne          = 0x")
    println(jne_short)
    
    // ==========================================
    // [5] Verificación de Contrato
    // ==========================================
    println("")
    println("[5] Verificacion de Contrato")
    
    if push_rbp == 85 {
        println("    push rbp (0x55) = 85: OK")
    }
    if ret == 195 {
        println("    ret (0xC3) = 195: OK")
    }
    if call_near == 232 {
        println("    call (0xE8) = 232: OK")
    }
    
    println("")
    println("+========================================+")
    println("|   OK - Opcodes x86-64 Verificados     |")
    println("+========================================+")
}
