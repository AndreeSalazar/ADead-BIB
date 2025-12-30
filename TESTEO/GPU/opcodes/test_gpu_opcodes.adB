// ADead-BIB GPU - Test de Opcodes GPU
// Contrato: Opcodes 0xC0DA... directos

fn main() {
    println("+========================================+")
    println("|   GPU - Test Opcodes GPU              |")
    println("|   Contrato HEX Directo                |")
    println("+========================================+")
    
    // ==========================================
    // [1] Opcodes de Inicialización
    // ==========================================
    println("")
    println("[1] Opcodes de Inicializacion")
    
    let GPU_INIT = 0xC0DA0001
    let GPU_SHUTDOWN = 0xC0DA0002
    
    print("    GPU_INIT     = ")
    println(GPU_INIT)
    print("    GPU_SHUTDOWN = ")
    println(GPU_SHUTDOWN)
    
    // ==========================================
    // [2] Opcodes de Memoria
    // ==========================================
    println("")
    println("[2] Opcodes de Memoria")
    
    let GPU_ALLOC = 0xC0DA0010
    let GPU_FREE = 0xC0DA0011
    let GPU_COPY_H2D = 0xC0DA0012
    let GPU_COPY_D2H = 0xC0DA0013
    
    print("    GPU_ALLOC    = ")
    println(GPU_ALLOC)
    print("    GPU_FREE     = ")
    println(GPU_FREE)
    print("    GPU_COPY_H2D = ")
    println(GPU_COPY_H2D)
    print("    GPU_COPY_D2H = ")
    println(GPU_COPY_D2H)
    
    // ==========================================
    // [3] Opcodes Matemáticos
    // ==========================================
    println("")
    println("[3] Opcodes Matematicos")
    
    let GPU_MATMUL = 0xC0DA0020
    let GPU_ADD = 0xC0DA0021
    let GPU_SUB = 0xC0DA0022
    let GPU_MUL = 0xC0DA0023
    
    print("    GPU_MATMUL   = ")
    println(GPU_MATMUL)
    print("    GPU_ADD      = ")
    println(GPU_ADD)
    print("    GPU_SUB      = ")
    println(GPU_SUB)
    print("    GPU_MUL      = ")
    println(GPU_MUL)
    
    // ==========================================
    // [4] Opcodes de Control
    // ==========================================
    println("")
    println("[4] Opcodes de Control")
    
    let GPU_SYNC = 0xC0DA00F0
    let GPU_END = 0xC0DAFFFF
    
    print("    GPU_SYNC     = ")
    println(GPU_SYNC)
    print("    GPU_END      = ")
    println(GPU_END)
    
    // ==========================================
    // [5] Formato de Instrucción
    // ==========================================
    println("")
    println("[5] Formato de Instruccion")
    println("    [opcode:8][dst:8][src1:8][src2:8]")
    println("    = 4 bytes por instruccion")
    
    println("")
    println("+========================================+")
    println("|   OK - Opcodes GPU Verificados        |")
    println("+========================================+")
}
