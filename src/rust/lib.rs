// ADead-BIB - Main Library
// Ruta 2: Directo AST → Opcodes → Binario

pub mod frontend;
pub mod backend;

pub use frontend::parser;
pub use frontend::ast;
pub use backend::pe;
pub use backend::elf;

