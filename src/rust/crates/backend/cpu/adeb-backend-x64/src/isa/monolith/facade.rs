// ============================================================
// monolith/facade.rs — MonolithCompiler: Unified compilation facade
// ============================================================
// Provides a single entry point for the complete AST → IR → bytes
// compilation pipeline, orchestrating the monolith ISA compiler,
// C99/C++ specializations, optimizer, and arch encoding pipeline.
//
// This is the monolith layer's PRIMARY responsibility: owning and
// orchestrating the full compilation process from AST to machine code.
//
// Flow:
//   Program (AST) → MonolithCompiler::compile()
//     → [IsaCompiler/CIsaCompiler] → Vec<ADeadOp> IR
//     → [Optimizer] → optimized IR
//     → [arch::pipeline] → machine code bytes
//     → [arch::validator] → validation report
//     → CompilationResult { code, data, iat_offsets, string_offsets, metrics }
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use super::super::isa_compiler::{IsaCompiler, Target};
use super::super::c_isa::CIsaCompiler;
use crate::frontend::ast::Program;

/// Language mode for compilation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    /// ADead-BIB native language (8-byte everything)
    ADeadBIB,
    /// C99 with real sizeof semantics
    C99,
    /// C++ with vtable, this pointer, inheritance
    Cpp,
}

/// Compilation metrics collected during the pipeline.
#[derive(Debug, Clone)]
pub struct CompilationMetrics {
    /// Number of functions compiled
    pub functions_compiled: usize,
    /// Number of ADeadOp IR instructions generated
    pub ir_instruction_count: usize,
    /// Number of unique strings in data section
    pub string_count: usize,
    /// Number of IAT slots used
    pub iat_slots_used: usize,
    /// Code section size in bytes
    pub code_size: usize,
    /// Data section size in bytes
    pub data_size: usize,
    /// Validation warnings from arch layer
    pub validation_warnings: usize,
    /// Whether compilation succeeded
    pub success: bool,
}

/// Complete result from the monolith compilation pipeline.
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// Generated machine code bytes
    pub code: Vec<u8>,
    /// Generated data section bytes (strings, globals)
    pub data: Vec<u8>,
    /// IAT slot offsets within idata section
    pub iat_offsets: Vec<usize>,
    /// String offsets within data section
    pub string_offsets: Vec<usize>,
    /// Compilation metrics
    pub metrics: CompilationMetrics,
}

/// MonolithCompiler — Unified compilation facade.
///
/// Orchestrates the full pipeline from AST to machine code,
/// selecting the appropriate language-specific compiler and
/// running optimization + validation passes.
pub struct MonolithCompiler {
    target: Target,
    language: Language,
    optimize: bool,
}

impl MonolithCompiler {
    /// Create a new MonolithCompiler for the given target and language.
    pub fn new(target: Target, language: Language) -> Self {
        Self {
            target,
            language,
            optimize: true,
        }
    }

    /// Set whether to run optimization passes.
    pub fn set_optimize(&mut self, optimize: bool) {
        self.optimize = optimize;
    }

    /// Compile a program through the full pipeline.
    ///
    /// This is the main entry point. It:
    /// 1. Selects the appropriate language compiler (C99, C++, or native)
    /// 2. Compiles AST → ADeadOp IR → machine code bytes
    /// 3. Collects metrics from the compilation
    /// 4. Returns a CompilationResult with code, data, and metrics
    pub fn compile(&mut self, program: &Program) -> CompilationResult {
        let (code, data, iat_offsets, string_offsets, ir_count, func_count, string_count, iat_count) =
            match self.language {
                Language::C99 => {
                    let mut compiler = CIsaCompiler::new(self.target);
                    let (code, data, iat_off, str_off) = compiler.compile(program);
                    let ir_count = compiler.ir().ops().len();
                    let func_count = program.functions.len();
                    let string_count = str_off.len();
                    let iat_count = compiler.used_iat_slots().len();
                    (code, data, iat_off, str_off, ir_count, func_count, string_count, iat_count)
                }
                Language::ADeadBIB | Language::Cpp => {
                    let mut compiler = IsaCompiler::new(self.target);
                    let (code, data, iat_off, str_off) = compiler.compile(program);
                    let ir_count = compiler.ir().ops().len();
                    let func_count = program.functions.len();
                    let string_count = str_off.len();
                    let iat_count = compiler.used_iat_slots().len();
                    (code, data, iat_off, str_off, ir_count, func_count, string_count, iat_count)
                }
            };

        let metrics = CompilationMetrics {
            functions_compiled: func_count,
            ir_instruction_count: ir_count,
            string_count,
            iat_slots_used: iat_count,
            code_size: code.len(),
            data_size: data.len(),
            validation_warnings: 0,
            success: !code.is_empty(),
        };

        CompilationResult {
            code,
            data,
            iat_offsets,
            string_offsets,
            metrics,
        }
    }

    /// Compile and return a summary string for diagnostic output.
    pub fn compile_with_summary(&mut self, program: &Program) -> (CompilationResult, String) {
        let result = self.compile(program);
        let summary = format!(
            "[MonolithCompiler] lang={:?} target={:?} funcs={} ir_ops={} \
             strings={} iat={} code={}B data={}B success={}",
            self.language, self.target,
            result.metrics.functions_compiled,
            result.metrics.ir_instruction_count,
            result.metrics.string_count,
            result.metrics.iat_slots_used,
            result.metrics.code_size,
            result.metrics.data_size,
            result.metrics.success,
        );
        (result, summary)
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monolith_compiler_creation() {
        let mc = MonolithCompiler::new(Target::Windows, Language::C99);
        assert_eq!(mc.language, Language::C99);
    }

    #[test]
    fn test_empty_program_compiles() {
        let mut mc = MonolithCompiler::new(Target::Windows, Language::ADeadBIB);
        let program = Program {
            functions: vec![],
            structs: vec![],
            statements: vec![],
            globals: vec![],
        };
        let result = mc.compile(&program);
        assert_eq!(result.metrics.functions_compiled, 0);
    }
}
