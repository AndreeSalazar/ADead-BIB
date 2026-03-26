//! ADead-BIB C Frontend Library
//! 
//! Lexer, parser y conversión a IR para el lenguaje C.

pub mod lexer;
pub mod parser;
pub mod c_to_ir;
pub mod ast;
pub mod compiler_extensions;
pub mod preprocessor;
pub mod stdlib;

// Re-exports
pub use lexer::{CLexer, CToken, CTokenKind};
pub use parser::CParser;
pub use c_to_ir::compile_c_to_program;
