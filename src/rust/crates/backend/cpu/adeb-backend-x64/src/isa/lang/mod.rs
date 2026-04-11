// ============================================================
// ISA Language Layer — Language-specific code generation
// ============================================================
// C99 and C++ specific ISA compiler wrappers.
// ============================================================

#[path = "../c_isa.rs"]
pub mod c_isa;

#[path = "../cpp_isa.rs"]
pub mod cpp_isa;
