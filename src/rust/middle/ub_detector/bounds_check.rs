// ============================================================
// Array Bounds Check Detection
// ============================================================

use super::report::{UBKind, UBReport, UBSeverity};
use crate::ast::{Expr, Program, Stmt, Type};
use std::collections::HashMap;

pub fn analyze_bounds(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    for func in &program.functions {
        let mut arrays = HashMap::new();
        let mut current_line = 0;
        for stmt in &func.body {
            check_stmt_bounds(
                stmt,
                &func.name,
                &mut reports,
                &mut arrays,
                &mut current_line,
            );
        }
    }

    let mut arrays = HashMap::new();
    let mut current_line = 0;
    for stmt in &program.statements {
        check_stmt_bounds(stmt, "main", &mut reports, &mut arrays, &mut current_line);
    }

    reports
}

fn check_stmt_bounds(
    stmt: &Stmt,
    func_name: &str,
    reports: &mut Vec<UBReport>,
    arrays: &mut HashMap<String, i64>,
    current_line: &mut usize,
) {
    match stmt {
        Stmt::LineMarker(l) => {
            *current_line = *l;
        }
        Stmt::VarDecl { name, var_type, .. } => {
            if let Type::Array(_, Some(size)) = var_type {
                arrays.insert(name.clone(), *size as i64);
            }
        }
        Stmt::IndexAssign { object, index, .. } => {
            if let Some(size) = get_array_size(object, arrays) {
                if let Some(idx) = get_constant_index(index) {
                    if idx < 0 || idx >= size {
                        reports.push(
                            UBReport::new(
                                UBSeverity::Error,
                                UBKind::ArrayOutOfBounds,
                                format!("Array index {} out of bounds [0..{})", idx, size),
                            )
                            .with_location(func_name.to_string(), *current_line)
                            .with_suggestion(format!("Index must be in range [0..{})", size)),
                        );
                    }
                }
            }
        }
        Stmt::If {
            then_body,
            else_body,
            ..
        } => {
            for s in then_body {
                check_stmt_bounds(s, func_name, reports, arrays, current_line);
            }
            if let Some(eb) = else_body {
                for s in eb {
                    check_stmt_bounds(s, func_name, reports, arrays, current_line);
                }
            }
        }
        Stmt::While { body, .. } => {
            for s in body {
                check_stmt_bounds(s, func_name, reports, arrays, current_line);
            }
        }
        _ => {}
    }
}

fn get_array_size(expr: &Expr, arrays: &HashMap<String, i64>) -> Option<i64> {
    if let Expr::Variable(name) = expr {
        return arrays.get(name).copied();
    }
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
