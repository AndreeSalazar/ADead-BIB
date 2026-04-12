// ============================================================
// ISA Monolith Layer — GENERIC (classic compilation)
// ============================================================
// Partes GENÉRICAS del compilador — lo clásico que ya sabemos:
//   - ISA Compiler: AST → ADeadOp IR (el monolito principal)
//   - C ISA: C99 sizeof, alignment, struct layouts
//   - C++ ISA: vtable, this pointer, inheritance
//   - Modular Compiler: split version (compile, expressions, etc.)
//   - Optimizer: peephole optimization on ADeadOp sequences
//   - Register Allocator: GPR allocation
//   - SoA Optimizer: struct-of-arrays vectorization
//   - YMM Allocator: AVX2 256-bit register management
//
// Esta carpeta es lo GENÉRICO — no depende de encodings
// específicos de cada instrucción x86-64. Trabaja con
// ADeadOp como abstracción, delegando al arch/ layer
// para la codificación final.
//
// Flujo:
//   Program (AST) → [monolith] → Vec<ADeadOp> → [arch/encoder] → bytes
//
// Uso:
//   use crate::isa::monolith::isa_compiler::{IsaCompiler, Target};
//   use crate::isa::monolith::c_isa::CIsaCompiler;
//   let mut compiler = CIsaCompiler::new(Target::Windows);
//   let (code, data, iat_off, str_off) = compiler.compile(&program);
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

// Re-export canonical modules from parent (isa/) for clean access.
// The actual source files live at isa/*.rs — this module provides
// the organized "monolith" grouping as a facade.

pub use super::isa_compiler;
pub use super::c_isa;
pub use super::cpp_isa;
pub use super::compiler;
pub use super::optimizer;
pub use super::reg_alloc;
pub use super::soa_optimizer;
pub use super::ymm_allocator;
pub use super::codegen;
