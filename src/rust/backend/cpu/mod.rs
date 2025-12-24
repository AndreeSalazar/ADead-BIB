// ADead-BIB - CPU Backend
// Generación directa de código máquina x86-64
// Sin assembler, sin linker, sin runtime
//
// Estructura:
// - codegen.rs      : Generador de código legacy
// - codegen_v2.rs   : Generador de código avanzado (multi-función, syscalls)
// - syscalls.rs     : Syscalls directos Windows/Linux
// - microvm.rs      : MicroVM bytecode ultra-compacto
// - pe*.rs          : Generadores PE Windows (tiny, minimal, full)
// - elf.rs          : Generador ELF Linux
// - win32_resolver.rs: Resolución de imports Windows

pub mod codegen;
pub mod codegen_v2;
pub mod syscalls;
pub mod microvm;
pub mod pe;
pub mod pe_minimal;
pub mod pe_tiny;
// pub mod pe_simple; // Deshabilitado: tiene errores de API
pub mod pe_valid;
pub mod elf;
pub mod win32_resolver;
