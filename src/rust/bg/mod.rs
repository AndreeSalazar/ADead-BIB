// ============================================================
// BG — Binary Guardian
// ============================================================
// Deterministic ISA-Level Capability Guardian
//
// Not an antivirus. Not a sandbox. Not heuristic.
// A structural control architecture.
//
// Pipeline:
//   Binary → ISA Decoder → ABIB IR → Capability Mapper
//       → Architecture Map → Policy Engine → APPROVE/DENY
//
// Pre-execution analysis. O(n) build, O(1) query.
// Designed for FastOS loader integration.
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com
// ============================================================

pub mod arch_map;
pub mod capability;
pub mod policy;
pub mod analyzer;

// Re-exports for ergonomic API
pub use analyzer::{BinaryGuardian, AnalysisResult};
pub use arch_map::{ArchitectureMap, InstructionClass, Capabilities};
pub use capability::CapabilityMapper;
pub use policy::{PolicyEngine, SecurityPolicy, SecurityLevel, Verdict, Violation, ViolationType};
