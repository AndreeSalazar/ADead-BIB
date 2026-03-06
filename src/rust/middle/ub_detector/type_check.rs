// ============================================================
// Type Confusion & Invalid Cast Detection
// ============================================================
// Detecta casts invalidos y confusion de tipos.
// UBKind::TypeConfusion, UBKind::InvalidCast
// ============================================================

use super::report::{UBKind, UBReport, UBSeverity};
use crate::ast::{Expr, Program, Stmt, Type};

pub fn analyze_type_safety(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    for func in &program.functions {
        let mut current_line = 0;
        for stmt in &func.body {
            check_stmt_types(stmt, &func.name, &mut reports, &mut current_line);
        }
    }

    let mut current_line = 0;
    for stmt in &program.statements {
        check_stmt_types(stmt, "main", &mut reports, &mut current_line);
    }

    reports
}

fn check_stmt_types(
    stmt: &Stmt,
    func_name: &str,
    reports: &mut Vec<UBReport>,
    current_line: &mut usize,
) {
    match stmt {
        Stmt::LineMarker(l) => {
            *current_line = *l;
        }
        Stmt::Assign { value, .. } => {
            check_expr_types(value, func_name, reports, current_line);
        }
        Stmt::VarDecl {
            value: Some(val), ..
        } => {
            check_expr_types(val, func_name, reports, current_line);
        }
        Stmt::DerefAssign { pointer, value } => {
            check_expr_types(pointer, func_name, reports, current_line);
            check_expr_types(value, func_name, reports, current_line);
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            check_expr_types(condition, func_name, reports, current_line);
            for s in then_body {
                check_stmt_types(s, func_name, reports, current_line);
            }
            if let Some(eb) = else_body {
                for s in eb {
                    check_stmt_types(s, func_name, reports, current_line);
                }
            }
        }
        Stmt::While { condition, body } => {
            check_expr_types(condition, func_name, reports, current_line);
            for s in body {
                check_stmt_types(s, func_name, reports, current_line);
            }
        }
        Stmt::Expr(expr) => {
            check_expr_types(expr, func_name, reports, current_line);
        }
        _ => {}
    }
}

fn check_expr_types(
    expr: &Expr,
    func_name: &str,
    reports: &mut Vec<UBReport>,
    current_line: &mut usize,
) {
    match expr {
        Expr::Cast {
            target_type,
            expr: inner,
        } => {
            // Detectar casts potencialmente peligrosos
            if is_dangerous_cast(target_type) {
                reports.push(
                    UBReport::new(
                        UBSeverity::Warning,
                        UBKind::InvalidCast,
                        format!("Potentially unsafe cast to '{:?}'", target_type),
                    )
                    .with_location(func_name.to_string(), *current_line)
                    .with_suggestion("Verify cast is valid at runtime".to_string()),
                );
            }
            check_expr_types(inner, func_name, reports, current_line);
        }
        Expr::BinaryOp { left, right, .. } => {
            check_expr_types(left, func_name, reports, current_line);
            check_expr_types(right, func_name, reports, current_line);
        }
        Expr::Deref(inner) => {
            check_expr_types(inner, func_name, reports, current_line);
        }
        Expr::Call { args, .. } => {
            for arg in args {
                check_expr_types(arg, func_name, reports, current_line);
            }
        }
        _ => {}
    }
}

fn is_dangerous_cast(target_type: &Type) -> bool {
    // Casts a puntero void o puntero crudo son peligrosos
    matches!(target_type, Type::Pointer(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_safety_clean() {
        let program = Program::new();
        let reports = analyze_type_safety(&program);
        assert_eq!(reports.len(), 0);
    }
}
