// ============================================================
// ADead-BIB Transformation Passes
// ============================================================
// Optimization passes that transform IR
// Inspired by LLVM transform passes
// ============================================================

mod dce;
mod constfold;
mod inline;
mod gvn;
mod licm;
mod unroll;
mod vectorize;
mod simplify_cfg;
mod merge_functions;

pub use dce::DeadCodeElimPass;
pub use constfold::ConstantFoldPass;
pub use inline::InlinePass;
pub use gvn::GVNPass;
pub use licm::LICMPass;
pub use unroll::LoopUnrollPass;
pub use vectorize::VectorizePass;
pub use simplify_cfg::SimplifyCFGPass;
pub use merge_functions::MergeFunctionsPass;
