//! ADead-BIB Core Library
//! 
//! Proporciona tipos fundamentales, diagnósticos, manejo de fuentes y símbolos
//! usados por todos los demás crates del compilador.

pub mod diagnostics;
pub mod source;
pub mod symbols;

// Re-exports comunes
pub use diagnostics::{Diagnostic, DiagnosticLevel, DiagnosticManager};
pub use source::{SourceFile, SourceLocation, SourceMap};
pub use symbols::{Symbol, SymbolTable, SymbolKind};
