// ADead-BIB Backend - Organizado por Target
//
// Estructura:
// - cpu/     : Generación de código x86-64, PE, ELF, syscalls
// - gpu/     : Detección GPU, Vulkan, HEX binario
//
// Filosofía: "Buildeas, optimizas y programas: Las 3 combinadas"

pub mod cpu;
pub mod gpu;

// Re-exports para compatibilidad
pub use cpu::pe;
pub use cpu::elf;
pub use cpu::codegen;
pub use cpu::codegen_v2;
pub use cpu::syscalls;
pub use cpu::win32_resolver;
pub use cpu::pe_minimal;
pub use cpu::pe_tiny;
pub use cpu::microvm;
