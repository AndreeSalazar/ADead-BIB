// ============================================================
// arch/types.rs — Re-export of core ISA types
// ============================================================
// Centralizes all x86-64 architecture types used by the
// encoder, decoder, and bit resolver.
//
// Usage:
//   use crate::isa::arch::types::{Reg, ADeadOp, Operand, Condition};
// ============================================================

pub use super::super::{
    ADeadIR, ADeadOp, CallTarget, Condition, Label, Operand, Reg,
};

// ── Register classification helpers ──

/// Returns the Win64 ABI argument registers in order.
/// RCX, RDX, R8, R9 for integer/pointer args.
pub const WIN64_ARG_REGS: [Reg; 4] = [Reg::RCX, Reg::RDX, Reg::R8, Reg::R9];

/// Returns the Win64 ABI float argument registers.
/// XMM0-XMM3 for float/double args.
pub const WIN64_FLOAT_ARG_REGS: [Reg; 4] = [Reg::XMM0, Reg::XMM1, Reg::XMM2, Reg::XMM3];

/// Callee-saved registers in Win64 ABI.
pub const WIN64_CALLEE_SAVED: [Reg; 7] = [
    Reg::RBX, Reg::RBP, Reg::RDI, Reg::RSI,
    Reg::R12, Reg::R13, Reg::R14,
];

/// Volatile (caller-saved) registers in Win64 ABI.
pub const WIN64_VOLATILE: [Reg; 9] = [
    Reg::RAX, Reg::RCX, Reg::RDX, Reg::R8, Reg::R9,
    Reg::R10, Reg::R11, Reg::R14, Reg::R15,
];

/// Shadow space size in bytes (Win64 ABI: 32 bytes minimum).
pub const WIN64_SHADOW_SPACE: u32 = 32;

/// Stack alignment requirement (16 bytes on x86-64).
pub const STACK_ALIGNMENT: u32 = 16;
