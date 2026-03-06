// ============================================================
// Array Bounds Check Detection
// ============================================================

use crate::ast::{Program, Stmt, Expr};
use super::report::{UBReport, UBSeverity, UBKind};

pub fn analyze_bounds(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    for func in &program.functions {
        for stmt in &func.body {
            check_stmt_bounds(stmt, &func.name, &mut reports);
        }
    }

    for stmt in &program.statements {
        check_stmt_bounds(stmt, "main", &mut reports);
    }

    reports
}

fn check_stmt_bounds(stmt: &Stmt, func_name: &str, reports: &mut Vec<UBReport>) {
    match stmt {
        Stmt::IndexAssign { object, index, .. } => {
            if let Some(size) = get_array_size(object) {
                if let Some(idx) = get_constant_index(index) {
                    if idx < 0 || idx >= size {
                        reports.push(
                            UBReport::new(
                                UBSeverity::Error,
                                UBKind::ArrayOutOfBounds,
                                format!("Array index {} out of bounds [0..{})", idx, size),
                            )
                            .with_location(func_name.to_string(), 0)
                            .with_suggestion(format!("Index must be in range [0..{})", size))
                        );
                    }
                }
            }
        }
        Stmt::If { then_body, else_body, .. } => {
            for s in then_body {
                check_stmt_bounds(s, func_name, reports);
            }
            if let Some(eb) = else_body {
                for s in eb {
                    check_stmt_bounds(s, func_name, reports);
                }
            }
        }
        Stmt::While { body, .. } => {
            for s in body {
                check_stmt_bounds(s, func_name, reports);
            }
        }
        _ => {}
    }
}

fn get_array_size(_expr: &Expr) -> Option<i64> {
    // TODO: Implementar análisis de tipo para obtener tamaño de array
    // Por ahora retornamos None (análisis conservador)
    None
}

fn get_constant_index(expr: &Expr) -> Option<i64> {
    match expr {
        Expr::Number(n) => Some(*n),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_detection() {
        let program = Program::new();
        let reports = analyze_bounds(&program);
        assert_eq!(reports.len(), 0);
    }
}
