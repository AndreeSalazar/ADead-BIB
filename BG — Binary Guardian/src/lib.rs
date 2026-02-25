// ============================================================
// BG — Binary Guardian
// ============================================================
// Deterministic ISA-Level Capability Guardian
//
// No antivirus. No sandbox clásico. No heurísticas.
// Arquitectura de control estructural.
//
//   Binary → Loader → ISA Decoder → ABIB IR → Capability Mapper
//       → Architecture Map → Policy Engine → APPROVE / DENY
//
// ● Pre-execution: analiza una vez, genera mapa compacto.
// ● Deterministic: mismo binario + misma policy = mismo resultado.
// ● O(n) build, O(1) query.
// ● Directo al ISA: no depende de lenguaje, formato, ni alto nivel.
// ● Hardware map: clasifica exactamente qué hardware toca el binario.
//
// Diseñado para FastOS loader integration.
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com
// ============================================================

pub mod arch_map;
pub mod capability;
pub mod policy;
pub mod analyzer;
pub mod binary_loader;

// Re-exports — API ergonómica
pub use analyzer::{BinaryGuardian, AnalysisResult};
pub use arch_map::{
    ArchitectureMap, InstructionClass, Capabilities,
    StructuralIntegrity, ImportExportMap, HardwareAccessMap, HardwareDevice, HardwareAccess,
};
pub use capability::CapabilityMapper;
pub use policy::{
    PolicyEngine, SecurityPolicy, SecurityLevel,
    Verdict, Violation, ViolationType, ViolationSeverity,
};
pub use binary_loader::{BinaryLoader, BinaryInfo, SectionInfo, SectionKind, ImportEntry, ExportEntry};
