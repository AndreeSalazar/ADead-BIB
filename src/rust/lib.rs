// ============================================================
// ADead-BIB v5.0 - Main Library
// ============================================================
// ADead = ASM Dead | BIB = Binary Is Binary
//
// Architecture inspired by:
// - LLVM: IR and optimization passes
// - GCC: Aggressive optimizations
// - MSVC: Windows ABI integration
// - FASM: Direct byte generation
// - Rust: Type safety and modules
//
// Pipeline: Source → AST → IR → Optimization → x86-64 → PE/ELF
// ============================================================

// Core modules
pub mod backend;
pub mod bg;
pub mod builder;
pub mod cache;
pub mod frontend;
pub mod isa;
pub mod optimizer;
pub mod output;
pub mod preprocessor;
pub mod runtime;

// Middle-end (LLVM-style IR and passes)
pub mod middle;

// ── NEW: Toolchain Heritage ──────────────────────────────────────────────────
// Explicit inheritance from LLVM, GCC, and MSVC.
// Contains: attributes, builtins, calling conventions, name mangling.
pub mod toolchain;

// Backend re-exports
pub use backend::cpu::flat_binary::FlatBinaryGenerator;
pub use backend::elf;
pub use backend::pe;

// Security module
pub use bg::{BinaryGuardian, SecurityLevel, SecurityPolicy, Verdict};

// Frontend re-exports
pub use frontend::ast;
pub use frontend::c;
pub use frontend::cpp;
pub use frontend::type_checker;

// ISA layer re-exports
pub use isa::codegen;
pub use isa::isa_compiler::IsaCompiler;

// Runtime re-exports
pub use runtime::{CPUFeatures, ComputeBackend};

// Middle-end re-exports
pub use middle::ir::{Function as IRFunction, Module as IRModule, Type as IRType};
pub use middle::lowering::lower_to_ir;
pub use middle::passes::{OptLevel, PassManager};
pub use middle::ub_detector::{UBDetector, UBKind, UBReport};

// Preprocessor re-exports (Sin CMake, Sin Linker)
pub use preprocessor::{HeaderResolver, MacroExpander, SymbolDedup};

// Cache re-exports (fastos.bib system)
pub use cache::ADeadCache;

// Output re-exports
pub use output::OutputFormat;

// ── Toolchain Heritage re-exports ────────────────────────────────────────────
// LLVM: attributes, intrinsics, calling conventions
pub use toolchain::llvm_attrs::{LlvmAttribute, LlvmCallingConv, LlvmIntrinsic};
// GCC: __attribute__(()), __builtin_*
pub use toolchain::gcc_builtins::{GccAttribute, GccBuiltin};
// MSVC: __declspec(), calling conventions, extensions
pub use toolchain::msvc_compat::{MsvcCallingConv, MsvcDeclspec, MsvcExtension, MsvcPragma};
// Unified calling convention table
pub use toolchain::calling_conventions::{
    detect_convention, shadow_space, CallFrame, CallingConvention,
};
// C++ name mangling
pub use toolchain::cpp_name_mangler::{ManglerContext, ManglingStyle, NameMangler};
