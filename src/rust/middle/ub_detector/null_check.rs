// ============================================================
// Null Pointer Dereference Detection
// ============================================================

use crate::ast::{Program, Stmt, Expr};
use super::report::{UBReport, UBSeverity, UBKind};

pub fn analyze_null_safety(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    // Analizar funciones
    for func in &program.functions {
        for stmt in &func.body {
            check_stmt_null(stmt, &func.name, &mut reports);
        }
    }

    // Analizar top-level statements
    for stmt in &program.statements {
        check_stmt_null(stmt, "main", &mut reports);
    }

    reports
}

fn check_stmt_null(stmt: &Stmt, func_name: &str, reports: &mut Vec<UBReport>) {
    match stmt {
        Stmt::DerefAssign { pointer, .. } => {
            if is_potentially_null(pointer) {
                reports.push(
                    UBReport::new(
                        UBSeverity::Warning,
                        UBKind::NullPointerDereference,
                        format!("Dereferencing potentially null pointer"),
                    )
                    .with_location(func_name.to_string(), 0)
                    .with_suggestion("Add null check before dereference".to_string())
                );
            }
        }
        Stmt::ArrowAssign { pointer, .. } => {
            if is_potentially_null(pointer) {
                reports.push(
                    UBReport::new(
                        UBSeverity::Warning,
                        UBKind::NullPointerDereference,
                        format!("Arrow access on potentially null pointer"),
                    )
                    .with_location(func_name.to_string(), 0)
                );
            }
        }
        Stmt::If { condition, then_body, else_body, .. } => {
            check_expr_null(condition, func_name, reports);
            for s in then_body {
                check_stmt_null(s, func_name, reports);
            }
            if let Some(eb) = else_body {
                for s in eb {
                    check_stmt_null(s, func_name, reports);
                }
            }
        }
        Stmt::While { condition, body } => {
            check_expr_null(condition, func_name, reports);
            for s in body {
                check_stmt_null(s, func_name, reports);
            }
        }
        _ => {}
    }
}

fn check_expr_null(expr: &Expr, func_name: &str, reports: &mut Vec<UBReport>) {
    match expr {
        Expr::Deref(inner) => {
            if is_potentially_null(inner) {
                reports.push(
                    UBReport::new(
                        UBSeverity::Warning,
                        UBKind::NullPointerDereference,
                        format!("Dereferencing potentially null expression"),
                    )
                    .with_location(func_name.to_string(), 0)
                );
            }
        }
        Expr::ArrowAccess { pointer, .. } => {
            if is_potentially_null(pointer) {
                reports.push(
                    UBReport::new(
                        UBSeverity::Warning,
                        UBKind::NullPointerDereference,
                        format!("Arrow access on potentially null pointer"),
                    )
                    .with_location(func_name.to_string(), 0)
                );
            }
        }
        _ => {}
    }
}

fn is_potentially_null(expr: &Expr) -> bool {
    matches!(expr, Expr::Nullptr | Expr::Number(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_detection() {
        let program = Program::new();
        let reports = analyze_null_safety(&program);
        assert_eq!(reports.len(), 0);
    }
}
