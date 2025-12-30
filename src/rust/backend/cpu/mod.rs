// ============================================================
// ADead-BIB - CPU Backend
// ============================================================
// BINARY IS BINARY - Emitimos bytes x86-64 DIRECTAMENTE
// Sin ASM textual. Sin assembler externo. Sin linker.
//
// Estructura:
// - binary_emitter.rs : 🔥 CORE - Emite bytes x86-64 directos
// - binary_raw.rs     : Generador de binario crudo (técnica prohibida)
// - codegen.rs        : Generador legacy (usa binary_emitter)
// - codegen_v2.rs     : Generador avanzado (multi-función)
// - syscalls.rs       : Syscalls directos Windows/Linux
// - microvm.rs        : MicroVM bytecode ultra-compacto
// - pe*.rs            : Generadores PE Windows (sin linker)
// - elf.rs            : Generador ELF Linux (sin linker)
// - win32_resolver.rs : Resolución de imports Windows
//
// Flujo: AST → binary_emitter → bytes → PE/ELF/RAW
// ============================================================

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
pub mod binary_raw;  // 🔥 Generador de binario CRUDO - bytes directos
