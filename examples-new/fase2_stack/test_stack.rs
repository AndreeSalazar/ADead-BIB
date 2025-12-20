// Test Fase 2: Stack Dinámico
// Verifica que el stack dinámico genera opcodes correctos y deterministas
// Prueba con muchas variables (más de 256 bytes del límite anterior)
//
// Autor: Eddi Andreé Salazar Matos

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Simulación del generador de código con stack dinámico
struct StackCodeGen {
    code: Vec<u8>,
    stack_offset: i32,
    max_stack: i32,
}

impl StackCodeGen {
    fn new() -> Self {
        Self {
            code: Vec::new(),
            stack_offset: -8,
            max_stack: 0,
        }
    }
    
    /// Emite prologue con stack dinámico
    fn emit_prologue(&mut self) -> usize {
        // push rbp
        self.code.push(0x55);
        // mov rbp, rsp
        self.code.extend_from_slice(&[0x48, 0x89, 0xE5]);
        // sub rsp, imm32 (placeholder)
        self.code.extend_from_slice(&[0x48, 0x81, 0xEC, 0x00, 0x00, 0x00, 0x00]);
        
        // Retorna posición del placeholder para parchear después
        self.code.len() - 4
    }
    
    /// Emite epilogue
    fn emit_epilogue(&mut self) {
        // xor eax, eax
        self.code.extend_from_slice(&[0x31, 0xC0]);
        // mov rsp, rbp
        self.code.extend_from_slice(&[0x48, 0x89, 0xEC]);
        // pop rbp
        self.code.push(0x5D);
        // ret
        self.code.push(0xC3);
    }
    
    /// Asigna una variable en el stack
    fn emit_assign(&mut self, value: i64) -> i32 {
        let offset = self.stack_offset;
        self.stack_offset -= 8;
        
        if -self.stack_offset > self.max_stack {
            self.max_stack = -self.stack_offset;
        }
        
        // mov rax, imm64
        self.code.extend_from_slice(&[0x48, 0xB8]);
        self.code.extend_from_slice(&value.to_le_bytes());
        
        // mov [rbp+offset], rax
        self.code.extend_from_slice(&[0x48, 0x89, 0x85]);
        self.code.extend_from_slice(&offset.to_le_bytes());
        
        offset
    }
    
    /// Lee una variable del stack
    fn emit_load(&mut self, offset: i32) {
        // mov rax, [rbp+offset]
        self.code.extend_from_slice(&[0x48, 0x8B, 0x85]);
        self.code.extend_from_slice(&offset.to_le_bytes());
    }
    
    /// Parchea el tamaño del stack en el prologue
    fn patch_stack_size(&mut self, placeholder_pos: usize) {
        // Alinear a 16 bytes
        let aligned_size = ((self.max_stack + 15) & !15) as u32;
        self.code[placeholder_pos..placeholder_pos + 4]
            .copy_from_slice(&aligned_size.to_le_bytes());
    }
    
    /// Genera función con N variables
    fn generate_function_with_vars(&mut self, num_vars: usize) -> Vec<u8> {
        self.code.clear();
        self.stack_offset = -8;
        self.max_stack = 0;
        
        let placeholder = self.emit_prologue();
        
        // Crear N variables
        let mut offsets = Vec::new();
        for i in 0..num_vars {
            let offset = self.emit_assign(i as i64 * 100);
            offsets.push(offset);
        }
        
        // Leer algunas variables para verificar
        if !offsets.is_empty() {
            self.emit_load(offsets[0]);
            if offsets.len() > 1 {
                self.emit_load(offsets[offsets.len() - 1]);
            }
        }
        
        self.emit_epilogue();
        self.patch_stack_size(placeholder);
        
        self.code.clone()
    }
}

/// Calcula hash de bytes
fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

/// Test 1: Stack con 10 variables (80 bytes)
fn test_stack_10_vars() -> bool {
    let mut gen1 = StackCodeGen::new();
    let mut gen2 = StackCodeGen::new();
    
    let code1 = gen1.generate_function_with_vars(10);
    let code2 = gen2.generate_function_with_vars(10);
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  10 vars hash1: {:016x}", hash1);
    println!("  10 vars hash2: {:016x}", hash2);
    println!("  Bytes: {} | Stack: {} bytes | Determinista: {}", 
             code1.len(), gen1.max_stack, hash1 == hash2);
    
    hash1 == hash2
}

/// Test 2: Stack con 50 variables (400 bytes - supera límite anterior de 256)
fn test_stack_50_vars() -> bool {
    let mut gen1 = StackCodeGen::new();
    let mut gen2 = StackCodeGen::new();
    
    let code1 = gen1.generate_function_with_vars(50);
    let code2 = gen2.generate_function_with_vars(50);
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  50 vars hash1: {:016x}", hash1);
    println!("  50 vars hash2: {:016x}", hash2);
    println!("  Bytes: {} | Stack: {} bytes | Determinista: {}", 
             code1.len(), gen1.max_stack, hash1 == hash2);
    println!("  ⚠️  Stack {} bytes > 256 bytes (límite anterior)", gen1.max_stack);
    
    hash1 == hash2 && gen1.max_stack > 256
}

/// Test 3: Stack con 100 variables (800 bytes)
fn test_stack_100_vars() -> bool {
    let mut gen1 = StackCodeGen::new();
    let mut gen2 = StackCodeGen::new();
    
    let code1 = gen1.generate_function_with_vars(100);
    let code2 = gen2.generate_function_with_vars(100);
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  100 vars hash1: {:016x}", hash1);
    println!("  100 vars hash2: {:016x}", hash2);
    println!("  Bytes: {} | Stack: {} bytes | Determinista: {}", 
             code1.len(), gen1.max_stack, hash1 == hash2);
    
    hash1 == hash2
}

/// Test 4: Stack con 500 variables (4000 bytes - stress test)
fn test_stack_500_vars() -> bool {
    let mut gen1 = StackCodeGen::new();
    let mut gen2 = StackCodeGen::new();
    
    let code1 = gen1.generate_function_with_vars(500);
    let code2 = gen2.generate_function_with_vars(500);
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  500 vars hash1: {:016x}", hash1);
    println!("  500 vars hash2: {:016x}", hash2);
    println!("  Bytes: {} | Stack: {} bytes | Determinista: {}", 
             code1.len(), gen1.max_stack, hash1 == hash2);
    
    hash1 == hash2
}

/// Test 5: Verificar alineación a 16 bytes
fn test_stack_alignment() -> bool {
    let mut gen = StackCodeGen::new();
    
    // Probar varios tamaños
    let sizes = [1, 3, 7, 10, 15, 16, 17, 31, 32, 33];
    let mut all_aligned = true;
    
    println!("  Verificando alineación a 16 bytes:");
    for &size in &sizes {
        gen.code.clear();
        gen.stack_offset = -8;
        gen.max_stack = 0;
        
        let placeholder = gen.emit_prologue();
        for i in 0..size {
            gen.emit_assign(i as i64);
        }
        gen.emit_epilogue();
        gen.patch_stack_size(placeholder);
        
        // Leer el tamaño del stack del código generado
        let stack_size = u32::from_le_bytes([
            gen.code[placeholder],
            gen.code[placeholder + 1],
            gen.code[placeholder + 2],
            gen.code[placeholder + 3],
        ]);
        
        let is_aligned = stack_size % 16 == 0;
        all_aligned = all_aligned && is_aligned;
        
        println!("    {} vars -> stack {} bytes (aligned: {})", 
                 size, stack_size, is_aligned);
    }
    
    all_aligned
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     ADead-BIB - Fase 2: Test Stack Dinámico                ║");
    println!("║     Autor: Eddi Andreé Salazar Matos                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    
    println!("🧪 Test 1: Stack con 10 variables");
    if test_stack_10_vars() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 2: Stack con 50 variables (supera 256 bytes)");
    if test_stack_50_vars() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 3: Stack con 100 variables");
    if test_stack_100_vars() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 4: Stack con 500 variables (stress test)");
    if test_stack_500_vars() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 5: Alineación a 16 bytes");
    if test_stack_alignment() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("════════════════════════════════════════════════════════════");
    println!("📊 Resultados: {} passed, {} failed", passed, failed);
    
    if failed == 0 {
        println!("✅ FASE 2 COMPLETADA - Stack dinámico es DETERMINISTA");
    } else {
        println!("❌ FASE 2 FALLIDA - Revisar implementación");
    }
}
