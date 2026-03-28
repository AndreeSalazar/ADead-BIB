//! ADead-BIB C++ Frontend Library
//! 
//! Lexer, parser, semántica OOP y conversión a IR para C++.

pub mod lexer;
pub mod parser;
pub mod cpp_to_ir;
pub mod ast;
pub mod compiler_extensions;
pub mod preprocessor;
pub mod stdlib;
pub mod ad_bindgen;

// Re-exports
pub use lexer::{CppLexer, CppToken, CppTokenKind};
pub use parser::CppParser;
pub use cpp_to_ir::compile_cpp_to_program;
