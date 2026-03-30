//! ADead-BIB C++ Frontend Library
//! 
//! Lexer, parser, semántica OOP y conversión a IR para C++.

pub mod ast;
pub mod compiler_extensions;
pub mod preprocessor;
pub mod stdlib;
pub mod ad_bindgen;

pub mod parse {
    pub mod lexer;
    pub mod parser;
}

pub mod lower {
    pub mod cpp_to_ir;
}

// Compatibility aliases matching cpp_mod.rs convention
pub mod cpp_ast {
    pub use crate::ast::*;
}

pub mod cpp_lexer {
    pub use crate::parse::lexer::*;
}

pub mod cpp_parser {
    pub use crate::parse::parser::*;
}

pub mod cpp_preprocessor {
    pub use crate::preprocessor::*;
}

pub mod cpp_stdlib {
    pub use crate::stdlib::*;
}

pub mod cpp_to_ir {
    pub use crate::lower::cpp_to_ir::*;
}

// Access to adeb-core IR types
pub mod frontend {
    pub mod ast {
        pub use adeb_core::ast::*;
    }
    pub mod types {
        pub use adeb_core::types::*;
    }
}

// Re-exports
pub use parse::lexer::{CppLexer, CppToken};
pub use parse::parser::CppParser;
pub use lower::cpp_to_ir::compile_cpp_to_program;
