// ============================================================
// ADead-BIB Backend - Binary Is Binary
// ============================================================
// Emitimos BYTES directamente, sin ASM intermedio.
//
// Estructura:
// - cpu/     : Bytes x86-64 directos → PE/ELF/RAW
// - gpu/     : Bytes GPU directos → SPIR-V/CUDA/HEX
//
// Filosofía: "Código → Bytes → Binario. Sin intermediarios."
// ============================================================

pub mod cpu;
pub mod gpu;

// Re-exports para compatibilidad
pub use cpu::pe;
pub use cpu::elf;
// DEPRECATED re-exports (use adead_bib::isa::isa_compiler instead)
pub use cpu::codegen;
pub use cpu::codegen_v2;
pub use cpu::syscalls;
pub use cpu::win32_resolver;
pub use cpu::pe_minimal;
pub use cpu::pe_tiny;
pub use cpu::microvm;
