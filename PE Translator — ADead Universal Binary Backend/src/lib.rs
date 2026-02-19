// ============================================================
// PE Translator — ADead Universal Binary Backend
// ============================================================
// Converts ADead-BIB intermediate format into native executables:
//   - PE  (Windows .exe/.dll)
//   - ELF (Linux)
//   - FsOS (FastOS native)
//
// Architecture:
//   ADead-BIB → Backend Trait → Target-specific emitter → Binary
//
// Philosophy: "One format in, any platform out."
// ============================================================

pub mod bib;
pub mod targets;
pub mod runtime;
