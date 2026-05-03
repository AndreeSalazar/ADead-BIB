//! Middle tier: IR, Optimizer, UB Detector, AST→IR
pub mod ir;
pub mod optimizer;
pub mod ub_detector;
pub mod ast_to_ir;

pub use ir::*;
pub use optimizer::Optimizer;
pub use ub_detector::UbDetector;
pub use ast_to_ir::ast_to_ir;
