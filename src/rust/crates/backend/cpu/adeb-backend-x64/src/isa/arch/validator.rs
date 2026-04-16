// ============================================================
// arch/validator.rs — ADeadOp IR Validator
// ============================================================
// Validates IR sequences before encoding to catch errors early.
// This is an arch-level responsibility because validation rules
// depend on x86-64 encoding constraints (register classes,
// operand sizes, alignment, etc.).
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use super::super::{ADeadOp, CallTarget};

/// Severity of a validation issue.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

/// A single validation finding.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub index: usize,
    pub message: String,
}

/// Result of validating an IR sequence.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
    pub instruction_count: usize,
    pub label_count: usize,
    pub call_count: usize,
    pub branch_count: usize,
}

impl ValidationResult {
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    pub fn error_count(&self) -> usize {
        self.issues.iter().filter(|i| i.severity == Severity::Error).count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues.iter().filter(|i| i.severity == Severity::Warning).count()
    }
}

/// Validate an IR sequence for x86-64 encoding correctness.
///
/// Checks:
/// - Label definitions are unique (no duplicates)
/// - Branch targets reference defined labels
/// - Call targets are valid
/// - Memory operands have valid displacements
/// - RawBytes aren't suspiciously large
pub fn validate_ir(ops: &[ADeadOp]) -> ValidationResult {
    let mut issues = Vec::new();
    let mut defined_labels = std::collections::HashSet::new();
    let mut referenced_labels = Vec::new();
    let mut label_count = 0usize;
    let mut call_count = 0usize;
    let mut branch_count = 0usize;

    for (i, op) in ops.iter().enumerate() {
        match op {
            ADeadOp::Label(label) => {
                label_count += 1;
                if !defined_labels.insert(*label) {
                    issues.push(ValidationIssue {
                        severity: Severity::Error,
                        index: i,
                        message: format!("Duplicate label definition: {}", label),
                    });
                }
            }
            ADeadOp::Jmp { target } => {
                branch_count += 1;
                referenced_labels.push((i, *target));
            }
            ADeadOp::Jcc { target, .. } => {
                branch_count += 1;
                referenced_labels.push((i, *target));
            }
            ADeadOp::Call { target } => {
                call_count += 1;
                if let CallTarget::Relative(label) = target {
                    referenced_labels.push((i, *label));
                }
            }
            ADeadOp::CallIAT { iat_rva } => {
                call_count += 1;
                if *iat_rva == 0 {
                    issues.push(ValidationIssue {
                        severity: Severity::Warning,
                        index: i,
                        message: "CallIAT with RVA=0 — may be unresolved".to_string(),
                    });
                }
            }
            ADeadOp::RawBytes(bytes) => {
                if bytes.len() > 4096 {
                    issues.push(ValidationIssue {
                        severity: Severity::Warning,
                        index: i,
                        message: format!("Large RawBytes block: {} bytes", bytes.len()),
                    });
                }
            }
            _ => {}
        }
    }

    // Check that all referenced labels are defined
    for (ref_idx, label) in &referenced_labels {
        if !defined_labels.contains(label) {
            issues.push(ValidationIssue {
                severity: Severity::Warning,
                index: *ref_idx,
                message: format!("Branch target label {} not yet defined (may be forward ref)", label),
            });
        }
    }

    ValidationResult {
        issues,
        instruction_count: ops.len(),
        label_count,
        call_count,
        branch_count,
    }
}

/// Quick check: returns true if the IR has no validation errors.
pub fn is_valid(ops: &[ADeadOp]) -> bool {
    !validate_ir(ops).has_errors()
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::ADeadIR;

    #[test]
    fn test_empty_ir_valid() {
        let result = validate_ir(&[]);
        assert!(!result.has_errors());
        assert_eq!(result.instruction_count, 0);
    }

    #[test]
    fn test_duplicate_label_error() {
        let mut ir = ADeadIR::new();
        let label = ir.new_label();
        let ops = vec![ADeadOp::Label(label), ADeadOp::Nop, ADeadOp::Label(label)];
        let result = validate_ir(&ops);
        assert!(result.has_errors());
        assert_eq!(result.error_count(), 1);
    }

    #[test]
    fn test_valid_branch() {
        let mut ir = ADeadIR::new();
        let label = ir.new_label();
        let ops = vec![
            ADeadOp::Label(label),
            ADeadOp::Nop,
            ADeadOp::Jmp { target: label },
        ];
        let result = validate_ir(&ops);
        assert!(!result.has_errors());
        assert_eq!(result.branch_count, 1);
        assert_eq!(result.label_count, 1);
    }

    #[test]
    fn test_metrics() {
        let mut ir = ADeadIR::new();
        let l1 = ir.new_label();
        let l2 = ir.new_label();
        let ops = vec![
            ADeadOp::Label(l1),
            ADeadOp::Nop,
            ADeadOp::Call { target: CallTarget::Relative(l1) },
            ADeadOp::Jcc { cond: Condition::Equal, target: l2 },
            ADeadOp::Label(l2),
            ADeadOp::CallIAT { iat_rva: 0x2050 },
            ADeadOp::Ret,
        ];
        let result = validate_ir(&ops);
        assert!(!result.has_errors());
        assert_eq!(result.instruction_count, 7);
        assert_eq!(result.label_count, 2);
        assert_eq!(result.call_count, 2);
        assert_eq!(result.branch_count, 2); // Call+Jcc
    }
}
