// ============================================================
// ISA Optimization Layer — DEPRECATED (use monolith/ instead)
// ============================================================
// Alias kept for backward compatibility.
// New code should use: crate::isa::monolith::optimizer / reg_alloc / etc.
// ============================================================

pub use super::optimizer;
pub use super::soa_optimizer;
pub use super::reg_alloc;
pub use super::ymm_allocator;
