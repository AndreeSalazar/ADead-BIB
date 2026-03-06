pub mod branch_detector;
pub mod branchless;
pub mod simd;
pub mod binary_optimizer;
pub mod const_fold;
pub mod dead_code;
pub mod const_prop;
pub mod redundant;
pub mod inline_exp;

pub use binary_optimizer::{BinaryOptimizer, OptLevel, OptimizationStats, PESizeOptimizer};
pub use const_fold::ConstFolder;
pub use dead_code::DeadCodeEliminator;
pub use const_prop::ConstPropagator;
pub use redundant::RedundantEliminator;
pub use inline_exp::InlineExpander;

