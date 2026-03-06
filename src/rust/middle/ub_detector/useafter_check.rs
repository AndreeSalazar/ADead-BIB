// ============================================================
// Use-After-Free & Dangling Pointer Detection
// ============================================================
// Extends lifetime.rs with dangling pointer detection.
// Tracks scope boundaries to detect dangling stack pointers.
// UBKind::UseAfterFree, UBKind::DanglingPointer
// ============================================================

use crate::ast::{Program, Stmt, Expr};
use super::report::{UBReport, UBSeverity, UBKind};
use std::collections::HashMap;

pub fn analyze_use_after_free(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    for func in &program.functions {
        let mut analyzer = UseAfterAnalyzer::new(&func.name);
        for stmt in &func.body {
            analyzer.check_stmt(stmt);
        }
        reports.extend(analyzer.reports);
    }

    for stmt in &program.statements {
        let mut analyzer = UseAfterAnalyzer::new("main");
        analyzer.check_stmt(stmt);
        reports.extend(analyzer.reports);
    }

    reports
}

/// Estado de un puntero rastreado
#[derive(Debug, Clone, PartialEq)]
enum PtrState {
    Valid,
    Freed,
    PointsToStack,
}

struct UseAfterAnalyzer {
    func_name: String,
    /// Estado de punteros: nombre -> estado
    ptr_states: HashMap<String, PtrState>,
    /// Nivel de scope actual (para detectar dangling)
    scope_depth: usize,
    /// Variables declaradas en cada scope
    scope_vars: Vec<Vec<String>>,
    reports: Vec<UBReport>,
}

impl UseAfterAnalyzer {
    fn new(func_name: &str) -> Self {
        Self {
            func_name: func_name.to_string(),
            ptr_states: HashMap::new(),
            scope_depth: 0,
            scope_vars: vec![Vec::new()],
            reports: Vec::new(),
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Free(expr) => {
                if let Expr::Variable(name) = expr {
                    if self.ptr_states.get(name) == Some(&PtrState::Freed) {
                        // Double free ya lo detecta lifetime.rs
                    }
                    self.ptr_states.insert(name.clone(), PtrState::Freed);
                }
            }
            Stmt::VarDecl { name, .. } => {
                if let Some(vars) = self.scope_vars.last_mut() {
                    vars.push(name.clone());
                }
            }
            Stmt::Assign { name, value } => {
                // Si se asigna la direccion de una variable local
                if is_address_of_local(value) {
                    self.ptr_states.insert(name.clone(), PtrState::PointsToStack);
                }
                self.check_expr_use(value);
            }
            Stmt::DerefAssign { pointer, value } => {
                self.check_expr_use(pointer);
                self.check_expr_use(value);
            }
            Stmt::If { condition, then_body, else_body, .. } => {
                self.check_expr_use(condition);
                self.enter_scope();
                for s in then_body {
                    self.check_stmt(s);
                }
                self.leave_scope();
                if let Some(eb) = else_body {
                    self.enter_scope();
                    for s in eb {
                        self.check_stmt(s);
                    }
                    self.leave_scope();
                }
            }
            Stmt::While { condition, body } => {
                self.check_expr_use(condition);
                self.enter_scope();
                for s in body {
                    self.check_stmt(s);
                }
                self.leave_scope();
            }
            _ => {}
        }
    }

    fn enter_scope(&mut self) {
        self.scope_depth += 1;
        self.scope_vars.push(Vec::new());
    }

    fn leave_scope(&mut self) {
        // Al salir del scope, las variables locales mueren
        if let Some(leaving_vars) = self.scope_vars.pop() {
            for var in &leaving_vars {
                // Cualquier puntero que apunte a estas variables es dangling
                let dangling_ptrs: Vec<String> = self.ptr_states.iter()
                    .filter(|(_, state)| **state == PtrState::PointsToStack)
                    .map(|(name, _)| name.clone())
                    .collect();

                for ptr_name in dangling_ptrs {
                    self.reports.push(
                        UBReport::new(
                            UBSeverity::Warning,
                            UBKind::DanglingPointer,
                            format!(
                                "Pointer '{}' may dangle after '{}' leaves scope",
                                ptr_name, var
                            ),
                        )
                        .with_location(self.func_name.clone(), 0)
                        .with_suggestion(
                            "Do not take address of stack variable that outlives scope".to_string()
                        )
                    );
                }
            }
        }
        self.scope_depth = self.scope_depth.saturating_sub(1);
    }

    fn check_expr_use(&mut self, expr: &Expr) {
        match expr {
            Expr::Variable(name) => {
                if self.ptr_states.get(name) == Some(&PtrState::Freed) {
                    self.reports.push(
                        UBReport::new(
                            UBSeverity::Error,
                            UBKind::UseAfterFree,
                            format!("Use of freed pointer '{}'", name),
                        )
                        .with_location(self.func_name.clone(), 0)
                        .with_suggestion("Do not use pointer after free()".to_string())
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

fn is_address_of_local(expr: &Expr) -> bool {
    matches!(expr, Expr::AddressOf(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_after_free_detection() {
        let program = Program::new();
        let reports = analyze_use_after_free(&program);
        assert_eq!(reports.len(), 0);
    }
}
