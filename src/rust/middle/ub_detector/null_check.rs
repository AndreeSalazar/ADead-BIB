// ============================================================
// Null Pointer Dereference Detection
// ============================================================

use super::report::{UBKind, UBReport, UBSeverity};
use crate::ast::{Expr, Program, Stmt};
use std::collections::HashMap;

pub fn analyze_null_safety(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    // Analizar funciones
    for func in &program.functions {
        let mut null_vars = HashMap::new();
        let mut current_line = 0;
        for stmt in &func.body {
            check_stmt_null(
                stmt,
                &func.name,
                &mut reports,
                &mut null_vars,
                &mut current_line,
            );
        }
    }

    // Analizar top-level statements
    let mut null_vars = HashMap::new();
    let mut current_line = 0;
    for stmt in &program.statements {
        check_stmt_null(
            stmt,
            "main",
            &mut reports,
            &mut null_vars,
            &mut current_line,
        );
    }

    reports
}

fn check_stmt_null(
    stmt: &Stmt,
    func_name: &str,
    reports: &mut Vec<UBReport>,
    null_vars: &mut HashMap<String, bool>,
    current_line: &mut usize,
) {
    match stmt {
        Stmt::LineMarker(l) => {
            *current_line = *l;
        }
        Stmt::VarDecl { name, value, .. } => {
            if let Some(val) = value {
                let is_null = is_potentially_null(val, null_vars);
                null_vars.insert(name.clone(), is_null);
            }
        }
        Stmt::Assign { name, value } => {
            let is_null = is_potentially_null(value, null_vars);
            null_vars.insert(name.clone(), is_null);
        }
        Stmt::DerefAssign { pointer, .. } => {
            if is_potentially_null(pointer, null_vars) {
                reports.push(
                    UBReport::new(
                        UBSeverity::Warning,
                        UBKind::NullPointerDereference,
                        format!("Dereferencing potentially null pointer"),
                    )
                    .with_location(func_name.to_string(), *current_line)
                    .with_suggestion("Add null check before dereference".to_string()),
                );
            }
        }
        Stmt::ArrowAssign { pointer, .. } => {
            if is_potentially_null(pointer, null_vars) {
                reports.push(
                    UBReport::new(
                        UBSeverity::Warning,
                        UBKind::NullPointerDereference,
                        format!("Arrow access on potentially null pointer"),
                    )
                    .with_location(func_name.to_string(), *current_line),
                );
            }
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            check_expr_null(condition, func_name, reports, null_vars, current_line);
            for s in then_body {
                check_stmt_null(s, func_name, reports, null_vars, current_line);
            }
            if let Some(eb) = else_body {
                for s in eb {
                    check_stmt_null(s, func_name, reports, null_vars, current_line);
                }
            }
        }
        Stmt::While { condition, body } => {
            check_expr_null(condition, func_name, reports, null_vars, current_line);
            for s in body {
                check_stmt_null(s, func_name, reports, null_vars, current_line);
            }
        }
        _ => {}
    }
}

fn check_expr_null(
    expr: &Expr,
    func_name: &str,
    reports: &mut Vec<UBReport>,
    null_vars: &HashMap<String, bool>,
    current_line: &mut usize,
) {
    match expr {
        Expr::Deref(inner) => {
            if is_potentially_null(inner, null_vars) {
                reports.push(
                    UBReport::new(
                        UBSeverity::Warning,
                        UBKind::NullPointerDereference,
                        format!("Dereferencing potentially null expression"),
                    )
                    .with_location(func_name.to_string(), *current_line),
                );
            }
        }
        Expr::ArrowAccess { pointer, .. } => {
            if is_potentially_null(pointer, null_vars) {
                reports.push(
                    UBReport::new(
                        UBSeverity::Warning,
                        UBKind::NullPointerDereference,
                        format!("Arrow access on potentially null pointer"),
                    )
                    .with_location(func_name.to_string(), *current_line),
                );
            }
        }
        _ => {}
    }
}

fn is_potentially_null(expr: &Expr, null_vars: &HashMap<String, bool>) -> bool {
    match expr {
        Expr::Nullptr | Expr::Number(0) => true,
        Expr::Variable(name) => *null_vars.get(name).unwrap_or(&false),
        Expr::Cast { expr: inner, .. } => is_potentially_null(inner, null_vars),
        _ => false,
    }
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
