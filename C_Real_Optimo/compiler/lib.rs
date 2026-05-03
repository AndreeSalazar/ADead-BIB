//! ADead-BIB Compiler - Frontend, Middle, Backend
//!
//! Compilador C/C++ optimizado con pipeline propio:
//! - Frontend: lexer, parser, preprocessor, AST
//! - Middle: IR, optimizer, UB detector
//! - Backend: codegen x86-64, encoder, PE/ELF

pub mod frontend;
pub mod middle;
pub mod backend;

// Re-exportar tipos principales
pub use frontend::lexer::Lexer;
pub use frontend::parser::Parser;
pub use middle::ir::IR;
pub use backend::codegen::Codegen;
