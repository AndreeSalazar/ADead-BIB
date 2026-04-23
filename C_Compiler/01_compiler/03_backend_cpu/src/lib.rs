// ============================================================
// adeb-backend-x64 — ADead-BIB x86-64 Backend
// ============================================================
//
// Modular structure (v11.0):
//
// lib.rs                    ← Thin facade: pub mod + re-exports
// ├── isa/                  ← ISA Layer (types, encoding, compilation)
// │   ├── arch/             ← CRITICAL: encoder, decoder, VEX, bit_resolver
// │   ├── monolith/         ← GENERIC: isa_compiler, c_isa, cpp_isa, optimizer
// │   └── mod.rs            ← Core types: Reg, ADeadOp, Operand, ADeadIR
// │
// ├── iat_registry.rs       ← IAT v6 (18 DLLs, 340+ slots)
// ├── pe.rs                 ← PE output (.exe generation)
// ├── validate.rs           ← Validation layer (compiler → PE)
// ├── flat_binary.rs        ← Raw binary output (boot sectors)
// ├── elf.rs                ← ELF output (stub)
// └── po.rs                 ← .Po output (FastOS format)
//
// Pipeline:
//   .c/.cpp → Frontend → AST → [isa/monolith] → Vec<ADeadOp>
//           → [isa/arch/encoder] → bytes → [validate] → [pe/flat_binary] → .exe
//
// Both arch/ and monolith/ can independently produce bytes for PE:
//   arch/     : hand-crafted ADeadOp → Encoder → bytes
//   monolith/ : Program AST → IsaCompiler → bytes (full pipeline)
//
// ============================================================

pub mod isa;

pub mod frontend {
    pub mod ast {
        pub use adeb_core::ast::*;
        pub use adeb_core::types::{RegSize, Type};
    }
}

// ── Extracted modules (each in its own file) ──
pub mod iat_registry;
pub mod pe;
pub mod validate;
pub mod flat_binary;
pub mod elf;
pub mod po;

// ── Backward-compatible re-export ──
// Internal code references `crate::backend::cpu::iat_registry`
// This re-export keeps those paths working without changing every caller.
pub mod backend {
    pub mod cpu {
        pub use crate::iat_registry;
    }
}
