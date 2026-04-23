// ============================================================
// ISA Architecture Layer — CRITICAL (non-generic)
// ============================================================
// Partes CRÍTICAS de la arquitectura x86-64:
//   - Types: Reg, ADeadOp, Operand, Condition, Label, ADeadIR
//   - Encoder: ADeadOp → bytes (multi-pass FASM-style)
//   - Decoder: bytes → disassembly
//   - VEX: AVX/VEX prefix emission
//   - BitResolver: label/jump resolution
//   - Pipeline: orchestrated encoding → PE output
//   - Types module: Win64 ABI constants, register classifications
//
// Estructura:
//   arch/
//   ├── mod.rs          ← THIS FILE: re-exports + organization
//   ├── types.rs        ← Win64 ABI constants, register helpers
//   └── pipeline.rs     ← ADeadOp IR → Encoder → bytes pipeline
//
// Uso independiente:
//   use crate::isa::arch::{Reg, ADeadOp, Encoder};
//   let mut ir = ADeadIR::new();
//   ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RBP) });
//   let code = arch::pipeline::encode(&ir);
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

// ── New dedicated modules ──
pub mod types;
pub mod pipeline;
pub mod validator;

// ── Re-export canonical modules from parent (isa/) ──

// x86-64 Instruction Encoder (FASM-inspired multi-pass)
pub use super::encoder;

// x86-64 Instruction Decoder (80+ patterns)
pub use super::decoder;

// AVX/VEX Prefix Emitter
pub use super::vex_emitter;

// Label & Jump Resolution (multi-pass convergence)
pub use super::bit_resolver;

// Core types re-exported for convenience
pub use super::{ADeadIR, ADeadOp, CallTarget, Condition, Label, Operand, Reg};
