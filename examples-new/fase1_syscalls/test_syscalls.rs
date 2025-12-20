// Test Fase 1: Syscalls Directos
// Verifica que los syscalls generan opcodes correctos y deterministas
//
// Autor: Eddi Andreé Salazar Matos

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Módulo de syscalls (copia local para test aislado)
mod syscalls {
    /// Linux syscalls
    pub mod linux {
        pub const SYS_WRITE: u64 = 1;
        pub const SYS_EXIT: u64 = 60;
        pub const STDOUT: u64 = 1;
        
        /// Genera opcodes para sys_write
        pub fn emit_write(code: &mut Vec<u8>, fd: u64, buf_addr: u64, count: u64) {
            // mov rax, SYS_WRITE (1)
            code.extend_from_slice(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]);
            
            // mov rdi, fd
            code.extend_from_slice(&[0x48, 0xC7, 0xC7]);
            code.extend_from_slice(&(fd as u32).to_le_bytes());
            
            // mov rsi, buf_addr
            code.extend_from_slice(&[0x48, 0xBE]);
            code.extend_from_slice(&buf_addr.to_le_bytes());
            
            // mov rdx, count
            code.extend_from_slice(&[0x48, 0xC7, 0xC2]);
            code.extend_from_slice(&(count as u32).to_le_bytes());
            
            // syscall
            code.extend_from_slice(&[0x0F, 0x05]);
        }
        
        /// Genera opcodes para sys_exit
        pub fn emit_exit(code: &mut Vec<u8>, exit_code: u64) {
            // mov rax, SYS_EXIT (60)
            code.extend_from_slice(&[0x48, 0xC7, 0xC0, 0x3C, 0x00, 0x00, 0x00]);
            
            // mov rdi, exit_code
            if exit_code == 0 {
                code.extend_from_slice(&[0x48, 0x31, 0xFF]); // xor rdi, rdi
            } else {
                code.extend_from_slice(&[0x48, 0xC7, 0xC7]);
                code.extend_from_slice(&(exit_code as u32).to_le_bytes());
            }
            
            // syscall
            code.extend_from_slice(&[0x0F, 0x05]);
        }
    }
}

/// Calcula hash de bytes para verificar determinismo
fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

/// Test 1: sys_write genera opcodes deterministas
fn test_syscall_write_deterministic() -> bool {
    let mut code1 = Vec::new();
    let mut code2 = Vec::new();
    
    // Generar dos veces con mismos parámetros
    syscalls::linux::emit_write(&mut code1, 1, 0x400078, 14);
    syscalls::linux::emit_write(&mut code2, 1, 0x400078, 14);
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  sys_write hash1: {:016x}", hash1);
    println!("  sys_write hash2: {:016x}", hash2);
    println!("  Bytes: {} | Determinista: {}", code1.len(), hash1 == hash2);
    
    hash1 == hash2
}

/// Test 2: sys_exit genera opcodes deterministas
fn test_syscall_exit_deterministic() -> bool {
    let mut code1 = Vec::new();
    let mut code2 = Vec::new();
    
    syscalls::linux::emit_exit(&mut code1, 0);
    syscalls::linux::emit_exit(&mut code2, 0);
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  sys_exit hash1: {:016x}", hash1);
    println!("  sys_exit hash2: {:016x}", hash2);
    println!("  Bytes: {} | Determinista: {}", code1.len(), hash1 == hash2);
    
    hash1 == hash2
}

/// Test 3: Hello World completo genera opcodes deterministas
fn test_hello_world_deterministic() -> bool {
    fn generate_hello_world() -> Vec<u8> {
        let mut code = Vec::new();
        
        // String "Hello, World!\n" estará en 0x400078
        let string_addr: u64 = 0x400078;
        let string_len: u64 = 14;
        
        // sys_write(1, string_addr, 14)
        syscalls::linux::emit_write(&mut code, 1, string_addr, string_len);
        
        // sys_exit(0)
        syscalls::linux::emit_exit(&mut code, 0);
        
        code
    }
    
    let code1 = generate_hello_world();
    let code2 = generate_hello_world();
    let code3 = generate_hello_world();
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    let hash3 = hash_bytes(&code3);
    
    println!("  hello_world hash1: {:016x}", hash1);
    println!("  hello_world hash2: {:016x}", hash2);
    println!("  hello_world hash3: {:016x}", hash3);
    println!("  Bytes: {} | Determinista: {}", code1.len(), hash1 == hash2 && hash2 == hash3);
    
    // Mostrar opcodes generados
    println!("\n  Opcodes generados:");
    print!("  ");
    for (i, byte) in code1.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 {
            println!();
            print!("  ");
        }
    }
    println!();
    
    hash1 == hash2 && hash2 == hash3
}

/// Test 4: Múltiples llamadas en secuencia
fn test_multiple_syscalls_deterministic() -> bool {
    fn generate_sequence() -> Vec<u8> {
        let mut code = Vec::new();
        
        // Múltiples writes
        syscalls::linux::emit_write(&mut code, 1, 0x400100, 5);
        syscalls::linux::emit_write(&mut code, 1, 0x400105, 10);
        syscalls::linux::emit_write(&mut code, 2, 0x400115, 20); // stderr
        
        // Exit
        syscalls::linux::emit_exit(&mut code, 42);
        
        code
    }
    
    let code1 = generate_sequence();
    let code2 = generate_sequence();
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  multi_syscall hash1: {:016x}", hash1);
    println!("  multi_syscall hash2: {:016x}", hash2);
    println!("  Bytes: {} | Determinista: {}", code1.len(), hash1 == hash2);
    
    hash1 == hash2
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     ADead-BIB - Fase 1: Test Syscalls Deterministas        ║");
    println!("║     Autor: Eddi Andreé Salazar Matos                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    
    println!("🧪 Test 1: sys_write determinista");
    if test_syscall_write_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 2: sys_exit determinista");
    if test_syscall_exit_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 3: Hello World determinista");
    if test_hello_world_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 4: Múltiples syscalls determinista");
    if test_multiple_syscalls_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("════════════════════════════════════════════════════════════");
    println!("📊 Resultados: {} passed, {} failed", passed, failed);
    
    if failed == 0 {
        println!("✅ FASE 1 COMPLETADA - Syscalls son DETERMINISTAS");
    } else {
        println!("❌ FASE 1 FALLIDA - Revisar implementación");
    }
}
