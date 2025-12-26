// ============================================
// ADead-BIB - Ejemplo 10: Directo al Binario
// La esencia de ADead-BIB: ASM -> Binario
// ============================================

// ADead-BIB compila DIRECTAMENTE a código máquina
// Sin intermediarios, sin VM, sin intérprete
// 
// Código fuente (.adB) -> Lexer -> Parser -> AST -> 
// -> CodeGen -> Opcodes x86-64 -> PE/ELF Binario
//
// ¡El binario resultante es NATIVO y PEQUEÑO!

// --- EJEMPLO MÍNIMO ---
// Este código genera un ejecutable de ~1.5KB
print("Hello from native binary!")

// --- TAMAÑOS DE BINARIO ---
// Standard build: ~1.5 KB
// Tiny build:     < 500 bytes
// Nano build:     ~1 KB (mínimo válido x64)
// Micro build:    < 256 bytes (x86 32-bit)

// --- COMANDOS ---
// adeadc build archivo.adB        -> Binario estándar
// adeadc tiny archivo.adB         -> PE ultra-compacto
// adeadc nano output.exe          -> PE más pequeño posible
// adeadc micro output.exe         -> PE32 sub-256 bytes

fn main() {
    print("=== ADead-BIB: Directo al Binario ===")
    print("Sin VM, sin intérprete")
    print("Código nativo x86-64")
    print("Binarios ultra-pequeños")
    
    // Variables van directo a registros/stack
    let x = 42      // -> mov rax, 42
    let y = 10      // -> mov rbx, 10
    let z = x + y   // -> add rax, rbx
    
    print("Resultado:")
    print(z)
    
    print("¡Esto es código NATIVO!")
}
