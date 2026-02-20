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

pub mod codegen;       // DEPRECATED: use isa_compiler (via isa module)
pub mod codegen_v2;    // DEPRECATED: use isa_compiler (via isa module)
pub mod microvm;
pub mod pe;
pub mod pe_compact;
pub mod pe_isa; // 🔥 PE ISA Direct - Compacto con imports
pub mod pe_minimal;
pub mod pe_tiny;
pub mod pe_ultra; // 🔥 PE Ultra-Compacto v2.0 - Más pequeño que ASM
pub mod syscalls; // 🔥 PE Compact - SectionAlign = FileAlign = 0x200
                  // pub mod pe_simple; // Deshabilitado: tiene errores de API
pub mod binary_raw; // 🔥 Generador de binario CRUDO - bytes directos
pub mod elf;
pub mod flat_binary;
pub mod pe_valid;
pub mod os_codegen; // 🔥 OS-Level Codegen - Phase 6: multi-mode, GDT/IDT, paging, Rust bridge
pub mod fastos_format; // 🔥 FastOS Format - Alternativa a PE/ELF para FastOS (magic: "FsOS")
pub mod win32_resolver; // 🔥 Flat Binary Generator - boot sectors & bare-metal
