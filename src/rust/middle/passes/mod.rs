// ============================================================
// ADead-BIB Optimization Passes
// ============================================================
// LLVM-style pass infrastructure
// Passes transform IR to optimize code
// ============================================================

pub mod pass_manager;
pub mod transform;

pub use pass_manager::{PassManager, Pass, PassKind, OptLevel};
pub use transform::*;
