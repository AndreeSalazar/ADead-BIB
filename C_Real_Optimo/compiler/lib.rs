//! ADead-BIB Compiler - Frontend, Middle, Backend, CLI
//!
//! Compilador C/C++ optimizado con pipeline propio:
//! - Frontend: lexer, parser, preprocessor, AST
//! - Middle: IR, optimizer, UB detector
//! - Backend: codegen x86-64, encoder, PE/ELF
//! - CLI: adB command line interface

pub mod frontend;
pub mod middle;
pub mod backend;
pub mod cli;

// Re-exportar tipos principales
pub use frontend::lexer::Lexer;
pub use frontend::parser::Parser;
pub use middle::ir::{IrModule, IrFunction, IrBuilder};
pub use middle::optimizer::Optimizer;
pub use middle::ub_detector::UbDetector;
pub use backend::codegen::Codegen;
pub use backend::pe::PeBuilder;
pub use backend::elf::ElfBuilder;
