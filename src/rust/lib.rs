// ============================================================
// ADead-BIB - Main Library
// ============================================================
// ADead = ASM Dead | BIB = Binary Is Binary
//
// Filosofía: Código → AST → BYTES DIRECTOS → Binario/HEX
// Sin ASM intermedio. Sin LLVM. Sin linker externo.
//
// El compilador emite bytes x86-64/GPU directamente.
// ============================================================

pub mod frontend;
pub mod backend;
pub mod runtime;
pub mod optimizer;
pub mod builder;
pub mod isa;

pub use frontend::parser;
pub use frontend::ast;
pub use frontend::lexer;
pub use frontend::type_checker;
pub use backend::pe;
pub use backend::elf;
pub use runtime::{CPUFeatures, ComputeBackend};
pub use isa::isa_compiler::IsaCompiler;

