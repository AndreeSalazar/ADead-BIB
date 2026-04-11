// ============================================================
// ISA Optimization Layer — Peephole, SoA, register allocation
// ============================================================
// All optimization passes that transform ADeadOp sequences
// before final encoding to machine code.
// ============================================================

#[path = "../optimizer.rs"]
pub mod optimizer;

#[path = "../soa_optimizer.rs"]
pub mod soa_optimizer;

#[path = "../reg_alloc.rs"]
pub mod reg_alloc;

#[path = "../ymm_allocator.rs"]
pub mod ymm_allocator;
