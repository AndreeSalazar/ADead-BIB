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

pub mod backend;
pub mod bg;
pub mod builder;
pub mod frontend;
pub mod isa;
pub mod optimizer;
pub mod runtime;

pub use backend::cpu::flat_binary::FlatBinaryGenerator;
pub use backend::elf;
pub use backend::pe;
pub use bg::{BinaryGuardian, SecurityPolicy, SecurityLevel, Verdict};
pub use frontend::ast;
pub use frontend::c;
pub use frontend::lexer;
pub use frontend::parser;
pub use frontend::type_checker;
pub use isa::isa_compiler::IsaCompiler;
pub use runtime::{CPUFeatures, ComputeBackend};
