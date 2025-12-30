// ADead-BIB v2.0.0 - Test de emit![] Macro
// Prueba la insercion de bytes directos en el codigo
// NOTA: emit![] es una caracteristica FUTURA (v2.1.0)
// Este test demuestra el concepto con sintaxis actual

fn main() {
    println("+========================================+")
    println("|  ADead-BIB v2.0.0 - Test emit![] Macro|")
    println("|  Binary Is Binary                     |")
    println("+========================================+")
    
    // Concepto de emit![]:
    // emit![0x48, 0x31, 0xC0]  // xor rax, rax
    // emit![0xC3]              // ret
    
    println("")
    println("[1] Concepto emit![]")
    println("    emit![0x48, 0x31, 0xC0] = xor rax, rax")
    println("    emit![0xC3] = ret")
    
    // Por ahora, demostramos los bytes como valores
    let xor_rax_rax_1 = 0x48
    let xor_rax_rax_2 = 0x31
    let xor_rax_rax_3 = 0xC0
    let ret_opcode = 0xC3
    
    println("")
    println("[2] Bytes de instrucciones x86-64")
    print("    xor rax, rax = [")
    print(xor_rax_rax_1)
    print(", ")
    print(xor_rax_rax_2)
    print(", ")
    print(xor_rax_rax_3)
    println("]")
    print("    ret = [")
    print(ret_opcode)
    println("]")
    
    // Verificacion
    println("")
    println("[3] Verificacion de opcodes")
    if xor_rax_rax_1 == 72 {
        println("    0x48 == 72: OK")
    }
    if ret_opcode == 195 {
        println("    0xC3 == 195: OK")
    }
    
    println("")
    println("+========================================+")
    println("|   emit![] pendiente para v2.1.0       |")
    println("|   Concepto verificado con literales   |")
    println("+========================================+")
}
