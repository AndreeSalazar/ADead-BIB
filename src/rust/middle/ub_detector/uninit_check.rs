// ============================================================
// Uninitialized Variable Detection
// ============================================================
// Detecta variables usadas antes de ser inicializadas.
// UBKind::UninitializedVariable
// ============================================================

use crate::ast::{Program, Stmt, Expr};
use super::report::{UBReport, UBSeverity, UBKind};
use std::collections::HashSet;

pub fn analyze_uninitialized(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    for func in &program.functions {
        let mut analyzer = UninitAnalyzer::new(&func.name);
        for stmt in &func.body {
            analyzer.check_stmt(stmt);
        }
        reports.extend(analyzer.reports);
    }

    for stmt in &program.statements {
        let mut analyzer = UninitAnalyzer::new("main");
        analyzer.check_stmt(stmt);
        reports.extend(analyzer.reports);
    }

    reports
}

struct UninitAnalyzer {
    func_name: String,
    /// Variables que han sido inicializadas
    initialized: HashSet<String>,
    /// Variables declaradas sin valor inicial
    declared_uninit: HashSet<String>,
    reports: Vec<UBReport>,
}

impl UninitAnalyzer {
    fn new(func_name: &str) -> Self {
        Self {
            func_name: func_name.to_string(),
            initialized: HashSet::new(),
            declared_uninit: HashSet::new(),
            reports: Vec::new(),
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            // Variable declarada con valor → inicializada
            Stmt::VarDecl { name, value: Some(_), .. } => {
                self.initialized.insert(name.clone());
            }
            // Variable declarada SIN valor → no inicializada
            Stmt::VarDecl { name, value: None, .. } => {
                self.declared_uninit.insert(name.clone());
            }
            // Asignacion → marca como inicializada
            Stmt::Assign { name, value } => {
                self.check_expr_use(value);
                self.initialized.insert(name.clone());
                self.declared_uninit.remove(name);
            }
            // Return con valor → verificar uso
            Stmt::Return(Some(expr)) => {
                self.check_expr_use(expr);
            }
            Stmt::If { condition, then_body, else_body, .. } => {
                self.check_expr_use(condition);
                for s in then_body {
                    self.check_stmt(s);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        self.check_stmt(s);
                    }
                }
            }
            Stmt::While { condition, body } => {
                self.check_expr_use(condition);
                for s in body {
                    self.check_stmt(s);
                }
            }
            Stmt::Expr(expr) => {
                self.check_expr_use(expr);
            }
            _ => {}
        }
    }

    fn check_expr_use(&mut self, expr: &Expr) {
        match expr {
            Expr::Variable(name) => {
                if self.declared_uninit.contains(name) && !self.initialized.contains(name) {
                    self.reports.push(
                        UBReport::new(
                            UBSeverity::Error,
                            UBKind::UninitializedVariable,
                            format!("Variable '{}' used before initialization", name),
                        )
                        .with_location(self.func_name.clone(), 0)
                        .with_suggestion(format!("Initialize '{}' before use", name))
                    );
                }
            }
            Expr::BinaryOp { left, right, .. } => {
                self.check_expr_use(left);
                self.check_expr_use(right);
            }
            Expr::Deref(inner) => {
                self.check_expr_use(inner);
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.check_expr_use(arg);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uninit_detection() {
        let program = Program::new();
        let reports = analyze_uninitialized(&program);
        assert_eq!(reports.len(), 0);
    }
}
