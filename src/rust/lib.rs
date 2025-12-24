// ADead-BIB - Main Library
// Ruta 2: Directo AST → Opcodes → Binario
// Runtime determinista para exprimir CPU y GPU

pub mod frontend;
pub mod backend;
pub mod runtime;
pub mod optimizer;

pub use frontend::parser;
pub use frontend::ast;
pub use backend::pe;
pub use backend::elf;
pub use runtime::{CPUFeatures, ComputeBackend};

