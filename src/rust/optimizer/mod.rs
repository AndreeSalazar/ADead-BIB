pub mod branch_detector;
pub mod branchless;
pub mod simd;
pub mod binary_optimizer;

pub use binary_optimizer::{BinaryOptimizer, OptLevel, OptimizationStats, PESizeOptimizer};

