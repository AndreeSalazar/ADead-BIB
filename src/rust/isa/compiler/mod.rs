// ============================================================
// ADead-BIB ISA Compiler — Modular Structure
// ============================================================
// Pipeline: AST → ADeadIR (Vec<ADeadOp>) → Encoder → bytes
//
// Sin ASM. Sin NASM. Sin LLVM. Solo ISA puro.
// Inspirado en FASM — encoding compacto y eficiente.
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

mod core;
mod compile;
mod functions;
mod statements;
mod expressions;
mod helpers;
mod control_flow;
mod arrays;

pub use core::*;
