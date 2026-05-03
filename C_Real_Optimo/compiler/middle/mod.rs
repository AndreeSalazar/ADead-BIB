//! ADead-BIB Compiler Middle Tier
//! - IR: Intermediate Representation
//! - Optimizer: DCE, ConstantFolding, Inlining, Peephole
//! - UB Detector: Division by zero, Null deref, OOB
//! Generado automáticamente

pub mod ir;
pub mod optimizer;
pub mod ub_detector;

pub use ir::{IrModule, IrFunction, IrBlock, IrInstr, IrType, IrValue, IrConst, IrReg, IrBuilder};
pub use optimizer::{Optimizer, OptimizationPass};
pub use ub_detector::{UbDetector, UbKind, UbReport, detect_ub};
