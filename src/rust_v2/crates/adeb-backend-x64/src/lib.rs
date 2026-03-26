//! ADead-BIB x64 Backend Library
//! 
//! Generación de código x86-64, ISA, encoder y codegen.

pub mod isa;
pub mod codegen;
pub mod pe;

// Re-exports
pub use isa::{IsaCompiler, ADeadOp, Reg, Operand, ADeadIR};
pub use codegen::x64_codegen;
