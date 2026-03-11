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

// ============================================================
// FFI Integration for FastOS (C wrapper)
// ============================================================
#[no_mangle]
pub extern "C" fn bg_rust_can_execute(bytes: *const u8, len: usize, level: i32) -> i32 {
    if bytes.is_null() || len == 0 {
        return 0;
    }
    let slice = unsafe { std::slice::from_raw_parts(bytes, len) };
    let sec_level = match level {
        0 => SecurityLevel::Kernel,
        1 => SecurityLevel::Driver,
        2 => SecurityLevel::Service,
        _ => SecurityLevel::User,
    };
    if BinaryGuardian::can_execute(slice, sec_level) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn bg_rust_version() -> *const u8 {
    b"2.0.0\0".as_ptr()
}
