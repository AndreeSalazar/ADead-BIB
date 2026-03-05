// ============================================================
// UB_Detector — Undefined Behavior Detection
// ============================================================
// Analiza el IR antes de codegen para detectar:
// - Use-after-free
// - Null pointer dereference
// - Array bounds violations
// - Integer overflow
// - Uninitialized variables
// - Type confusion
// - Stack overflow
// - Data races (future)
// ============================================================

pub mod analyzer;
pub mod null_check;
pub mod bounds_check;
pub mod overflow_check;
pub mod lifetime;
pub mod report;

use crate::ast::Program;
pub use report::{UBReport, UBSeverity, UBKind};

/// UB_Detector principal — analiza un programa IR completo
pub struct UBDetector {
    reports: Vec<UBReport>,
    strict_mode: bool,
}

impl UBDetector {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
            strict_mode: false,
        }
    }

    pub fn with_strict_mode(mut self) -> Self {
        self.strict_mode = true;
        self
    }

    /// Analiza el programa IR y retorna reportes de UB
    pub fn analyze(&mut self, program: &Program) -> Vec<UBReport> {
        self.reports.clear();

        // 1. Análisis de null pointer dereference
        let null_reports = null_check::analyze_null_safety(program);
        self.reports.extend(null_reports);

        // 2. Análisis de array bounds
        let bounds_reports = bounds_check::analyze_bounds(program);
        self.reports.extend(bounds_reports);

        // 3. Análisis de integer overflow
        let overflow_reports = overflow_check::analyze_overflow(program);
        self.reports.extend(overflow_reports);

        // 4. Análisis de lifetime (use-after-free)
        let lifetime_reports = lifetime::analyze_lifetimes(program);
        self.reports.extend(lifetime_reports);

        // Ordenar por severidad (Error > Warning > Info)
        self.reports.sort_by(|a, b| b.severity.cmp(&a.severity));

        self.reports.clone()
    }

    /// Retorna true si hay errores críticos (bloquean compilación)
    pub fn has_errors(&self) -> bool {
        self.reports.iter().any(|r| r.severity == UBSeverity::Error)
    }

    /// Imprime todos los reportes
    pub fn print_reports(&self) {
        if self.reports.is_empty() {
            println!("✓ No undefined behavior detected");
            return;
        }

        println!("\n=== UB_Detector Report ===");
        for report in &self.reports {
            report.print();
        }
        println!("=========================\n");
    }
}

impl Default for UBDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ub_detector_creation() {
        let detector = UBDetector::new();
        assert_eq!(detector.reports.len(), 0);
        assert!(!detector.strict_mode);
    }

    #[test]
    fn test_strict_mode() {
        let detector = UBDetector::new().with_strict_mode();
        assert!(detector.strict_mode);
    }
}
