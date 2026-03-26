//! ADead-BIB Middle End Library
//! 
//! Representación Intermedia (IR), optimizaciones y análisis.

pub mod ir;
pub mod passes;
pub mod ub_detector;
pub mod strict_type_checker;
pub mod optimizer;

// Re-exports — SSA IR types
pub use ir::{Module, Function, BasicBlock, Type, Value, ValueId, Constant};
pub use ir::{Instruction, Opcode, BinaryOp, CastOp, CompareOp};
pub use ir::{IRBuilder, GlobalVariable};
pub use ub_detector::{UBDetector, UBReport};
