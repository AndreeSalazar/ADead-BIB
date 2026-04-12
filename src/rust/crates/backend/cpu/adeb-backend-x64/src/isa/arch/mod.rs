// ============================================================
// ISA Architecture Layer — CRITICAL (non-generic)
// ============================================================
// Partes CRÍTICAS de la arquitectura x86-64:
//   - Types: Reg, ADeadOp, Operand, Condition, Label, ADeadIR
//   - Encoder: ADeadOp → bytes (multi-pass FASM-style)
//   - Decoder: bytes → disassembly
//   - VEX: AVX/VEX prefix emission
//   - BitResolver: label/jump resolution
//
// Esta carpeta es RESPONSABILIDAD — ignora lo genérico.
// Contiene el núcleo que NO se puede simplificar:
//   cada instrucción x86-64 tiene encoding único.
//
// Uso independiente:
//   use crate::isa::arch::{Reg, ADeadOp, Encoder};
//   let mut ir = ADeadIR::new();
//   ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RBP) });
//   let result = Encoder::new().encode_all(ir.ops());
//   // result.code → bytes listos para PE output
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

// Re-export canonical modules from parent (isa/) for clean access.
// The actual source files live at isa/*.rs — this module provides
// the organized "arch" grouping as a facade.

// ── x86-64 Instruction Encoder (FASM-inspired multi-pass) ──
pub use super::encoder;

// ── x86-64 Instruction Decoder (80+ patterns) ──
pub use super::decoder;

// ── AVX/VEX Prefix Emitter ──
pub use super::vex_emitter;

// ── Label & Jump Resolution (multi-pass convergence) ──
pub use super::bit_resolver;

// ── Core types re-exported for convenience ──
pub use super::{ADeadIR, ADeadOp, CallTarget, Condition, Label, Operand, Reg};
