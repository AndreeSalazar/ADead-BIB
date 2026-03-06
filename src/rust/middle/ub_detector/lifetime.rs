// ============================================================
// Lifetime Analysis — Use-After-Free Detection
// ============================================================

use super::report::{UBKind, UBReport, UBSeverity};
use crate::ast::{Expr, Program, Stmt};
use std::collections::HashSet;

pub fn analyze_lifetimes(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    for func in &program.functions {
        let mut analyzer = LifetimeAnalyzer::new(&func.name);
        for stmt in &func.body {
            analyzer.check_stmt(stmt);
        }
        reports.extend(analyzer.reports);
    }

    for stmt in &program.statements {
        let mut analyzer = LifetimeAnalyzer::new("main");
        analyzer.check_stmt(stmt);
        reports.extend(analyzer.reports);
    }

    reports
}

struct LifetimeAnalyzer {
    func_name: String,
    freed_vars: HashSet<String>,
    reports: Vec<UBReport>,
}

impl LifetimeAnalyzer {
    fn new(func_name: &str) -> Self {
        Self {
            func_name: func_name.to_string(),
            freed_vars: HashSet::new(),
            reports: Vec::new(),
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Free(expr) => {
                if let Expr::Variable(name) = expr {
                    if self.freed_vars.contains(name) {
                        self.reports.push(
                            UBReport::new(
                                UBSeverity::Error,
                                UBKind::DoubleFree,
                                format!("Double free of variable '{}'", name),
                            )
                            .with_location(self.func_name.clone(), 0)
                            .with_suggestion("Remove duplicate free() call".to_string()),
                        );
                    } else {
                        self.freed_vars.insert(name.clone());
                    }
                }
            }
            Stmt::Assign { name, value } => {
                if self.freed_vars.contains(name) {
                    self.reports.push(
                        UBReport::new(
                            UBSeverity::Error,
                            UBKind::UseAfterFree,
                            format!("Use of freed variable '{}'", name),
                        )
                        .with_location(self.func_name.clone(), 0)
                        .with_suggestion("Do not use variable after free()".to_string()),
                    );
                }
                self.check_expr_use(value);
            }
            Stmt::DerefAssign { pointer, value } => {
                self.check_expr_use(pointer);
                self.check_expr_use(value);
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
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
            _ => {}
        }
    }

    fn check_expr_use(&mut self, expr: &Expr) {
        match expr {
            Expr::Variable(name) => {
                if self.freed_vars.contains(name) {
                    self.reports.push(
                        UBReport::new(
                            UBSeverity::Error,
                            UBKind::UseAfterFree,
                            format!("Use of freed variable '{}'", name),
                        )
                        .with_location(self.func_name.clone(), 0),
                    );
                }
            }
            Expr::Deref(inner) => {
                self.check_expr_use(inner);
            }
            Expr::BinaryOp { left, right, .. } => {
                self.check_expr_use(left);
                self.check_expr_use(right);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_analysis() {
        let program = Program::new();
        let reports = analyze_lifetimes(&program);
        assert_eq!(reports.len(), 0);
    }
}
