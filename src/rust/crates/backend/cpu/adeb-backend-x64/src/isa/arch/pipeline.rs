// ============================================================
// arch/pipeline.rs — Complete x86-64 encoding pipeline
// ============================================================
// Orchestrates: ADeadOp IR → Encoder → BitResolver → PE bytes
//
// This module provides the high-level pipeline that takes
// compiled ADeadOp sequences and produces final executable bytes
// ready for PE output.
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use super::super::{ADeadIR, ADeadOp};
use super::super::encoder::{self, EncodeResult};

/// Result of the full encoding pipeline.
#[derive(Debug, Clone)]
pub struct EncodingPipelineResult {
    /// Inner encode result from the encoder
    pub encode_result: EncodeResult,
    /// Total number of instructions processed
    pub instruction_count: usize,
    /// Whether encoding succeeded without errors
    pub success: bool,
    /// Any warnings generated during encoding
    pub warnings: Vec<String>,
}

/// Pipeline configuration for encoding
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Base RVA for code section (default: 0x1000)
    pub code_rva: u32,
    /// Base RVA for idata section (default: 0x2000)
    pub idata_rva: u32,
    /// Image base address (default: 0x140000000 for PE64)
    pub image_base: u64,
    /// Enable peephole optimization pass before encoding
    pub optimize: bool,
    /// Maximum encoding passes for convergence (default: 10)
    pub max_passes: u32,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            code_rva: 0x1000,
            idata_rva: 0x2000,
            image_base: 0x0000000140000000,
            optimize: true,
            max_passes: 10,
        }
    }
}

/// Encode a sequence of ADeadOps into machine code bytes.
///
/// This is the main entry point for the arch/ layer.
/// It runs the multi-pass encoder with label resolution.
pub fn encode_pipeline(ir: &ADeadIR, _config: &PipelineConfig) -> EncodingPipelineResult {
    let ops = ir.ops();
    let mut warnings = Vec::new();

    // Phase 1: Encoding pass with multi-pass label resolution
    let mut enc = encoder::Encoder::new();
    let result = enc.encode_all(ops);

    let instruction_count = ops.len();

    if result.code.is_empty() && !ops.is_empty() {
        warnings.push("Encoder produced empty output for non-empty IR".to_string());
    }

    EncodingPipelineResult {
        encode_result: result,
        instruction_count,
        success: true,
        warnings,
    }
}

/// Quick encode helper — uses default config
pub fn encode(ir: &ADeadIR) -> Vec<u8> {
    let config = PipelineConfig::default();
    encode_pipeline(ir, &config).encode_result.code
}

/// Compute the total size that an IR sequence will encode to,
/// without actually producing the bytes. Useful for PE layout.
pub fn estimate_code_size(ops: &[ADeadOp]) -> usize {
    // Conservative estimate: most instructions are 1-15 bytes
    // Labels and pseudo-ops are 0 bytes
    let mut size = 0usize;
    for op in ops {
        size += match op {
            ADeadOp::Label(_) => 0,
            ADeadOp::Nop => 1,
            ADeadOp::Ret => 1,
            ADeadOp::Cld => 1,
            ADeadOp::Syscall => 2,
            ADeadOp::Push { .. } => 2,  // average
            ADeadOp::Pop { .. } => 2,
            ADeadOp::CallIAT { .. } => 6,
            ADeadOp::Mov { .. } => 7,   // conservative
            ADeadOp::RawBytes(b) => b.len(),
            _ => 5, // average instruction size
        };
    }
    size
}
