//! ADead-BIB Middle End Library
//! 
//! Representación Intermedia (IR), optimizaciones y análisis.

pub mod ir;
pub mod passes;
pub mod ub_detector;
pub mod strict_type_checker;
pub mod optimizer;

// Re-exports
pub use ir::{Program, Function, BasicBlock, Type, Expr, Stmt, ADeadOp, Reg, Operand};
pub use passes::transform;
pub use ub_detector::{UBDetector, UBReport};
