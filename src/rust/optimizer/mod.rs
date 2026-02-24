pub mod branch_detector;
pub mod branchless;
pub mod simd;
pub mod binary_optimizer;
pub mod const_fold;

pub use binary_optimizer::{BinaryOptimizer, OptLevel, OptimizationStats, PESizeOptimizer};
pub use const_fold::ConstFolder;

