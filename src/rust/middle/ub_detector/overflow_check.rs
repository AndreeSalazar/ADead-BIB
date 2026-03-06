// ============================================================
// Integer Overflow/Underflow Detection
// ============================================================

use super::report::{UBKind, UBReport, UBSeverity};
use crate::ast::{BinOp, Expr, Program, Stmt};

pub fn analyze_overflow(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    for func in &program.functions {
        let mut current_line = 0;
        for stmt in &func.body {
            check_stmt_overflow(stmt, &func.name, &mut reports, &mut current_line);
        }
    }

    let mut current_line = 0;
    for stmt in &program.statements {
        check_stmt_overflow(stmt, "main", &mut reports, &mut current_line);
    }

    reports
}

fn check_stmt_overflow(
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
            check_expr_overflow(value, func_name, reports, current_line);
        }
        Stmt::VarDecl {
            value: Some(val), ..
        } => {
            check_expr_overflow(val, func_name, reports, current_line);
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            check_expr_overflow(condition, func_name, reports, current_line);
            for s in then_body {
                check_stmt_overflow(s, func_name, reports, current_line);
            }
            if let Some(eb) = else_body {
                for s in eb {
                    check_stmt_overflow(s, func_name, reports, current_line);
                }
            }
        }
        Stmt::While { condition, body } => {
            check_expr_overflow(condition, func_name, reports, current_line);
            for s in body {
                check_stmt_overflow(s, func_name, reports, current_line);
            }
        }
        _ => {}
    }
}

fn check_expr_overflow(
    expr: &Expr,
    func_name: &str,
    reports: &mut Vec<UBReport>,
    current_line: &mut usize,
) {
    match expr {
        Expr::BinaryOp { op, left, right } => {
            // Detectar overflow en operaciones aritméticas
            if let (Expr::Number(l), Expr::Number(r)) = (left.as_ref(), right.as_ref()) {
                match op {
                    BinOp::Add => {
                        if will_overflow_add(*l, *r) {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::IntegerOverflow,
                                    format!("Integer overflow in addition: {} + {}", l, r),
                                )
                                .with_location(func_name.to_string(), *current_line)
                                .with_suggestion(
                                    "Use checked arithmetic or wider type".to_string(),
                                ),
                            );
                        }
                    }
                    BinOp::Sub => {
                        if will_underflow_sub(*l, *r) {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::IntegerUnderflow,
                                    format!("Integer underflow in subtraction: {} - {}", l, r),
                                )
                                .with_location(func_name.to_string(), *current_line),
                            );
                        }
                    }
                    BinOp::Mul => {
                        if will_overflow_mul(*l, *r) {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::IntegerOverflow,
                                    format!("Integer overflow in multiplication: {} * {}", l, r),
                                )
                                .with_location(func_name.to_string(), *current_line),
                            );
                        }
                    }
                    BinOp::Div => {
                        if *r == 0 {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::DivisionByZero,
                                    format!("Division by zero: {} / 0", l),
                                )
                                .with_location(func_name.to_string(), *current_line)
                                .with_suggestion("Add zero check before division".to_string()),
                            );
                        }
                    }
                    BinOp::Mod => {
                        if *r == 0 {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::DivisionByZero,
                                    format!("Modulo by zero: {} % 0", l),
                                )
                                .with_location(func_name.to_string(), *current_line),
                            );
                        }
                    }
                    _ => {}
                }
            }
            check_expr_overflow(left, func_name, reports, current_line);
            check_expr_overflow(right, func_name, reports, current_line);
        }
        _ => {}
    }
}

fn will_overflow_add(a: i64, b: i64) -> bool {
    a.checked_add(b).is_none()
}

fn will_underflow_sub(a: i64, b: i64) -> bool {
    a.checked_sub(b).is_none()
}

fn will_overflow_mul(a: i64, b: i64) -> bool {
    a.checked_mul(b).is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_detection() {
        assert!(will_overflow_add(i64::MAX, 1));
        assert!(!will_overflow_add(100, 200));
    }

    #[test]
    fn test_division_by_zero() {
        let program = Program::new();
        let reports = analyze_overflow(&program);
        assert_eq!(reports.len(), 0);
    }
}
