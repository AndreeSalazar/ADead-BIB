// ============================================================
// ADead-BIB IR (Intermediate Representation)
// ============================================================
// Inspired by LLVM IR - SSA form with typed values
//
// Key concepts:
// - Module: Top-level container (like LLVM Module)
// - Function: Contains basic blocks
// - BasicBlock: Sequence of instructions ending in terminator
// - Instruction: SSA operation producing a Value
// - Value: Typed result of an instruction
// ============================================================

mod module;
mod function;
pub mod basicblock;
mod instruction;
mod types;
mod value;
mod builder;

pub use module::{Module, GlobalVariable};
pub use function::Function;
pub use basicblock::BasicBlock;
pub use instruction::{Instruction, Opcode, BinaryOp, CompareOp, CastOp};
pub use types::Type;
pub use value::{Value, ValueId, Constant};
pub use builder::IRBuilder;
