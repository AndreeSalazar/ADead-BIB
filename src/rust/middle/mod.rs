// ============================================================
// ADead-BIB Middle-End v4.0
// ============================================================
// Inspired by LLVM IR - The heart of the compiler
//
// Pipeline: AST → IR → Optimization Passes → Backend
// ============================================================

pub mod ir;
pub mod lowering;
pub mod analysis;
pub mod passes;

pub use ir::{Module, Function, BasicBlock, Instruction, Value, Type as IRType};
pub use lowering::lower_to_ir;
pub use passes::PassManager;
