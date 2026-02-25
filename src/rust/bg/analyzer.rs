// ============================================================
// BG — Binary Guardian: Full Binary Analyzer
// ============================================================
// Complete pipeline: raw bytes → Architecture Map → Verdict
//
// Pipeline:
//   External Binary
//       ↓
//   ISA Decoder (bytes → ADeadOp)
//       ↓
//   Capability Mapper (ADeadOp → ArchitectureMap)
//       ↓
//   Policy Engine (ArchitectureMap × Policy → Verdict)
//       ↓
//   APPROVE / DENY
//
// Pre-execution, single-pass, deterministic.
// Designed for FastOS loader integration.
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use crate::isa::decoder::Decoder;
use crate::isa::ADeadOp;
use super::arch_map::ArchitectureMap;
use super::capability::CapabilityMapper;
use super::policy::{PolicyEngine, SecurityPolicy, SecurityLevel, Verdict};

/// Result of analyzing a binary.
#[derive(Debug)]
pub struct AnalysisResult {
    /// The complete architecture map of the binary
    pub map: ArchitectureMap,
    /// The security verdict (APPROVED/DENIED)
    pub verdict: Verdict,
    /// The inferred minimum security level required
    pub minimum_level: SecurityLevel,
    /// Number of instructions decoded
    pub instruction_count: usize,
}

/// Binary Guardian — Deterministic ISA-Level Capability Guardian.
///
/// Analyzes binaries before execution. Not an antivirus.
/// Not a sandbox. A structural control architecture.
///
/// # Usage
/// ```rust,no_run
/// use adead_bib::bg::{BinaryGuardian, SecurityPolicy};
///
/// let binary_code: Vec<u8> = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
/// let policy = SecurityPolicy::user();
/// let result = BinaryGuardian::analyze_bytes(&binary_code, &policy);
///
/// if result.verdict.is_approved() {
///     // Safe to execute
/// } else {
///     // Block execution
/// }
/// ```
pub struct BinaryGuardian;

impl BinaryGuardian {
    /// Analyze raw x86-64 bytes against a security policy.
    ///
    /// Full pipeline: Decode → Map → Evaluate
    pub fn analyze_bytes(code: &[u8], policy: &SecurityPolicy) -> AnalysisResult {
        // Step 1: Decode bytes → ADeadOp
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(code);

        // Step 2-4: Analyze the decoded instructions
        Self::analyze_ops(&ops, policy)
    }

    /// Analyze already-decoded ADeadOp instructions against a security policy.
    ///
    /// Use this when you already have the IR (e.g., from the compiler pipeline).
    pub fn analyze_ops(ops: &[ADeadOp], policy: &SecurityPolicy) -> AnalysisResult {
        // Step 2: Capability Mapper → ArchitectureMap
        let map = CapabilityMapper::analyze(ops);

        // Step 3: Infer minimum level
        let minimum_level = PolicyEngine::infer_minimum_level(&map);

        // Step 4: Policy Engine → Verdict
        let verdict = PolicyEngine::evaluate(&map, policy);

        AnalysisResult {
            instruction_count: map.instruction_map.total,
            map,
            verdict,
            minimum_level,
        }
    }

    /// Quick check: can this binary run at the given security level?
    /// Returns true if approved, false if denied.
    pub fn can_execute(code: &[u8], level: SecurityLevel) -> bool {
        let policy = match level {
            SecurityLevel::Kernel => SecurityPolicy::kernel(),
            SecurityLevel::Driver => SecurityPolicy::driver(),
            SecurityLevel::Service => SecurityPolicy::service(),
            SecurityLevel::User => SecurityPolicy::user(),
        };
        let result = Self::analyze_bytes(code, &policy);
        result.verdict.is_approved()
    }

    /// Analyze and return only the architecture map (no policy check).
    /// Useful for inspection and debugging.
    pub fn inspect_bytes(code: &[u8]) -> ArchitectureMap {
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(code);
        CapabilityMapper::analyze(&ops)
    }

    /// Analyze compiler output (ADeadOp IR) and return the architecture map.
    /// This is the zero-cost path when BG is integrated in the compiler pipeline.
    pub fn inspect_ir(ops: &[ADeadOp]) -> ArchitectureMap {
        CapabilityMapper::analyze(ops)
    }
}

impl std::fmt::Display for AnalysisResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.map)?;
        writeln!(f, "  Minimum level:  {}", self.minimum_level)?;
        writeln!(f, "  Verdict:        {}", self.verdict)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_safe_prologue() {
        // push rbp; mov rbp, rsp; pop rbp; ret
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0x5D, 0xC3];
        let result = BinaryGuardian::analyze_bytes(&code, &SecurityPolicy::user());

        assert!(result.verdict.is_approved());
        assert_eq!(result.minimum_level, SecurityLevel::User);
        assert_eq!(result.map.instruction_map.privileged_count, 0);
    }

    #[test]
    fn test_quick_check_safe() {
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
        assert!(BinaryGuardian::can_execute(&code, SecurityLevel::User));
    }

    #[test]
    fn test_inspect_bytes() {
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
        let map = BinaryGuardian::inspect_bytes(&code);
        assert!(map.capabilities.is_pure_userspace());
        assert!(map.instruction_map.is_unprivileged());
    }

    #[test]
    fn test_compiler_integration() {
        use crate::isa::*;

        // Simulate compiler output
        let ops = vec![
            ADeadOp::Push { src: Operand::Reg(Reg::RBP) },
            ADeadOp::Mov {
                dst: Operand::Reg(Reg::RBP),
                src: Operand::Reg(Reg::RSP),
            },
            ADeadOp::Mov {
                dst: Operand::Reg(Reg::RAX),
                src: Operand::Imm64(42),
            },
            ADeadOp::Pop { dst: Reg::RBP },
            ADeadOp::Ret,
        ];

        let result = BinaryGuardian::analyze_ops(&ops, &SecurityPolicy::user());
        assert!(result.verdict.is_approved());
        assert_eq!(result.instruction_count, 5);

        // Also works via inspect
        let map = BinaryGuardian::inspect_ir(&ops);
        assert!(map.capabilities.is_pure_userspace());
    }

    #[test]
    fn test_display_result() {
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
        let result = BinaryGuardian::analyze_bytes(&code, &SecurityPolicy::user());
        let display = format!("{}", result);
        assert!(display.contains("APPROVED"));
        assert!(display.contains("Binary Architecture Map"));
    }
}
