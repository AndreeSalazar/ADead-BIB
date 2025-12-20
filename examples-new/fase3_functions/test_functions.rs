// Test Fase 3: Múltiples Funciones
// Verifica que la tabla de funciones y llamadas son deterministas
//
// Autor: Eddi Andreé Salazar Matos

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Información de una función compilada
#[derive(Clone, Debug)]
struct CompiledFunction {
    name: String,
    offset: usize,
    size: usize,
}

/// Generador de código con soporte multi-función
struct MultiFunctionCodeGen {
    code: Vec<u8>,
    functions: HashMap<String, CompiledFunction>,
    pending_calls: Vec<(usize, String)>,  // (offset, nombre)
}

impl MultiFunctionCodeGen {
    fn new() -> Self {
        Self {
            code: Vec::new(),
            functions: HashMap::new(),
            pending_calls: Vec::new(),
        }
    }
    
    /// Emite una función simple que retorna un valor
    fn emit_function(&mut self, name: &str, return_value: i64) {
        let func_offset = self.code.len();
        
        // push rbp
        self.code.push(0x55);
        // mov rbp, rsp
        self.code.extend_from_slice(&[0x48, 0x89, 0xE5]);
        
        // mov rax, return_value
        self.code.extend_from_slice(&[0x48, 0xB8]);
        self.code.extend_from_slice(&return_value.to_le_bytes());
        
        // pop rbp
        self.code.push(0x5D);
        // ret
        self.code.push(0xC3);
        
        let func_size = self.code.len() - func_offset;
        self.functions.insert(name.to_string(), CompiledFunction {
            name: name.to_string(),
            offset: func_offset,
            size: func_size,
        });
    }
    
    /// Emite una función que llama a otra
    fn emit_function_with_call(&mut self, name: &str, callee: &str) {
        let func_offset = self.code.len();
        
        // push rbp
        self.code.push(0x55);
        // mov rbp, rsp
        self.code.extend_from_slice(&[0x48, 0x89, 0xE5]);
        // sub rsp, 32 (shadow space)
        self.code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x20]);
        
        // call callee (rel32 placeholder)
        self.code.push(0xE8);
        let call_offset = self.code.len();
        self.pending_calls.push((call_offset, callee.to_string()));
        self.code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // add rsp, 32
        self.code.extend_from_slice(&[0x48, 0x83, 0xC4, 0x20]);
        // pop rbp
        self.code.push(0x5D);
        // ret
        self.code.push(0xC3);
        
        let func_size = self.code.len() - func_offset;
        self.functions.insert(name.to_string(), CompiledFunction {
            name: name.to_string(),
            offset: func_offset,
            size: func_size,
        });
    }
    
    /// Emite función recursiva (factorial simplificado)
    fn emit_recursive_function(&mut self, name: &str) {
        let func_offset = self.code.len();
        
        // push rbp
        self.code.push(0x55);
        // mov rbp, rsp
        self.code.extend_from_slice(&[0x48, 0x89, 0xE5]);
        // sub rsp, 32
        self.code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x20]);
        
        // mov [rbp-8], rcx (guardar parámetro)
        self.code.extend_from_slice(&[0x48, 0x89, 0x4D, 0xF8]);
        
        // cmp rcx, 1
        self.code.extend_from_slice(&[0x48, 0x83, 0xF9, 0x01]);
        // jle base_case
        self.code.extend_from_slice(&[0x7E, 0x15]);
        
        // Caso recursivo: n * factorial(n-1)
        // dec rcx
        self.code.extend_from_slice(&[0x48, 0xFF, 0xC9]);
        // call self (placeholder)
        self.code.push(0xE8);
        let call_offset = self.code.len();
        self.pending_calls.push((call_offset, name.to_string()));
        self.code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // imul rax, [rbp-8]
        self.code.extend_from_slice(&[0x48, 0x0F, 0xAF, 0x45, 0xF8]);
        // jmp end
        self.code.extend_from_slice(&[0xEB, 0x07]);
        
        // base_case: mov rax, 1
        self.code.extend_from_slice(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]);
        
        // end:
        // add rsp, 32
        self.code.extend_from_slice(&[0x48, 0x83, 0xC4, 0x20]);
        // pop rbp
        self.code.push(0x5D);
        // ret
        self.code.push(0xC3);
        
        let func_size = self.code.len() - func_offset;
        self.functions.insert(name.to_string(), CompiledFunction {
            name: name.to_string(),
            offset: func_offset,
            size: func_size,
        });
    }
    
    /// Resuelve todas las llamadas pendientes
    fn resolve_calls(&mut self) {
        for (call_offset, callee_name) in &self.pending_calls {
            if let Some(func) = self.functions.get(callee_name) {
                let rel_offset = func.offset as i32 - (*call_offset as i32 + 4);
                self.code[*call_offset..*call_offset + 4]
                    .copy_from_slice(&rel_offset.to_le_bytes());
            }
        }
    }
    
    /// Genera programa completo
    fn generate(&mut self) -> Vec<u8> {
        self.resolve_calls();
        self.code.clone()
    }
}

/// Calcula hash de bytes
fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

/// Test 1: Dos funciones simples
fn test_two_functions() -> bool {
    let mut gen1 = MultiFunctionCodeGen::new();
    let mut gen2 = MultiFunctionCodeGen::new();
    
    gen1.emit_function("foo", 42);
    gen1.emit_function("bar", 100);
    let code1 = gen1.generate();
    
    gen2.emit_function("foo", 42);
    gen2.emit_function("bar", 100);
    let code2 = gen2.generate();
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  two_funcs hash1: {:016x}", hash1);
    println!("  two_funcs hash2: {:016x}", hash2);
    println!("  Bytes: {} | Funciones: {} | Determinista: {}", 
             code1.len(), gen1.functions.len(), hash1 == hash2);
    
    hash1 == hash2
}

/// Test 2: Función que llama a otra
fn test_function_call() -> bool {
    let mut gen1 = MultiFunctionCodeGen::new();
    let mut gen2 = MultiFunctionCodeGen::new();
    
    gen1.emit_function("helper", 42);
    gen1.emit_function_with_call("main", "helper");
    let code1 = gen1.generate();
    
    gen2.emit_function("helper", 42);
    gen2.emit_function_with_call("main", "helper");
    let code2 = gen2.generate();
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  func_call hash1: {:016x}", hash1);
    println!("  func_call hash2: {:016x}", hash2);
    println!("  Bytes: {} | Determinista: {}", code1.len(), hash1 == hash2);
    
    // Verificar que el call fue resuelto (no es 00 00 00 00)
    let call_resolved = code1.windows(5).any(|w| w[0] == 0xE8 && w[1..5] != [0, 0, 0, 0]);
    println!("  Call resuelto: {}", call_resolved);
    
    hash1 == hash2 && call_resolved
}

/// Test 3: Cadena de llamadas (A -> B -> C)
fn test_call_chain() -> bool {
    let mut gen1 = MultiFunctionCodeGen::new();
    let mut gen2 = MultiFunctionCodeGen::new();
    
    gen1.emit_function("func_c", 1);
    gen1.emit_function_with_call("func_b", "func_c");
    gen1.emit_function_with_call("func_a", "func_b");
    let code1 = gen1.generate();
    
    gen2.emit_function("func_c", 1);
    gen2.emit_function_with_call("func_b", "func_c");
    gen2.emit_function_with_call("func_a", "func_b");
    let code2 = gen2.generate();
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  call_chain hash1: {:016x}", hash1);
    println!("  call_chain hash2: {:016x}", hash2);
    println!("  Bytes: {} | Funciones: {} | Determinista: {}", 
             code1.len(), gen1.functions.len(), hash1 == hash2);
    
    hash1 == hash2
}

/// Test 4: Función recursiva
fn test_recursive_function() -> bool {
    let mut gen1 = MultiFunctionCodeGen::new();
    let mut gen2 = MultiFunctionCodeGen::new();
    
    gen1.emit_recursive_function("factorial");
    let code1 = gen1.generate();
    
    gen2.emit_recursive_function("factorial");
    let code2 = gen2.generate();
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  recursive hash1: {:016x}", hash1);
    println!("  recursive hash2: {:016x}", hash2);
    println!("  Bytes: {} | Determinista: {}", code1.len(), hash1 == hash2);
    
    // Mostrar opcodes
    println!("\n  Opcodes factorial:");
    print!("  ");
    for (i, byte) in code1.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 {
            println!();
            print!("  ");
        }
    }
    println!();
    
    hash1 == hash2
}

/// Test 5: Muchas funciones (stress test)
fn test_many_functions() -> bool {
    let mut gen1 = MultiFunctionCodeGen::new();
    let mut gen2 = MultiFunctionCodeGen::new();
    
    // Crear 50 funciones
    for i in 0..50 {
        gen1.emit_function(&format!("func_{}", i), i as i64);
        gen2.emit_function(&format!("func_{}", i), i as i64);
    }
    
    let code1 = gen1.generate();
    let code2 = gen2.generate();
    
    let hash1 = hash_bytes(&code1);
    let hash2 = hash_bytes(&code2);
    
    println!("  50_funcs hash1: {:016x}", hash1);
    println!("  50_funcs hash2: {:016x}", hash2);
    println!("  Bytes: {} | Funciones: {} | Determinista: {}", 
             code1.len(), gen1.functions.len(), hash1 == hash2);
    
    hash1 == hash2
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     ADead-BIB - Fase 3: Test Múltiples Funciones           ║");
    println!("║     Autor: Eddi Andreé Salazar Matos                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    
    println!("🧪 Test 1: Dos funciones simples");
    if test_two_functions() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 2: Función que llama a otra");
    if test_function_call() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 3: Cadena de llamadas (A -> B -> C)");
    if test_call_chain() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 4: Función recursiva");
    if test_recursive_function() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 5: 50 funciones (stress test)");
    if test_many_functions() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("════════════════════════════════════════════════════════════");
    println!("📊 Resultados: {} passed, {} failed", passed, failed);
    
    if failed == 0 {
        println!("✅ FASE 3 COMPLETADA - Múltiples funciones son DETERMINISTAS");
    } else {
        println!("❌ FASE 3 FALLIDA - Revisar implementación");
    }
}
