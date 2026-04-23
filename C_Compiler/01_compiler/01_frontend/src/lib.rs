//! ADead-BIB C Frontend Library
//! 
//! Lexer, parser y conversión a IR para el lenguaje C.

pub mod ast;
pub mod compiler_extensions;
pub mod preprocessor;
pub mod stdlib;

pub mod parse {
    pub mod lexer;
    pub mod parser;
}

pub mod lower {
    pub mod to_ir;
}

pub mod c_ast {
    pub use crate::ast::*;
}

pub mod c_lexer {
    pub use crate::parse::lexer::*;
}

pub mod c_parser {
    pub use crate::parse::parser::*;
}

pub mod c_preprocessor {
    pub use crate::preprocessor::*;
}

pub mod c_stdlib {
    pub use crate::stdlib::*;
}

pub mod c_to_ir {
    pub use crate::lower::to_ir::*;
}

pub mod frontend {
    pub mod ast {
        pub use adeb_core::ast::*;
    }
    pub mod types {
        pub use adeb_core::types::*;
    }
}

pub use parse::lexer::{CLexer, CToken};
pub use parse::parser::CParser;
pub use lower::to_ir::compile_c_to_program;
