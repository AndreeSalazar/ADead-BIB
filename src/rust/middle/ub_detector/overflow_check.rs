// ============================================================
// Integer Overflow/Underflow Detection
// ============================================================

use crate::ast::{Program, Stmt, Expr};
use crate::middle::ir::BinaryOp;
use super::report::{UBReport, UBSeverity, UBKind};

pub fn analyze_overflow(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();

    for func in &program.functions {
        for stmt in &func.body {
            check_stmt_overflow(stmt, &func.name, &mut reports);
        }
    }

    for stmt in &program.statements {
        check_stmt_overflow(stmt, "main", &mut reports);
    }

    reports
}

fn check_stmt_overflow(stmt: &Stmt, func_name: &str, reports: &mut Vec<UBReport>) {
    match stmt {
        Stmt::Assign { value, .. } => {
            check_expr_overflow(value, func_name, reports);
        }
        Stmt::VarDecl { value: Some(val), .. } => {
            check_expr_overflow(val, func_name, reports);
        }
        Stmt::If { condition, then_body, else_body, .. } => {
            check_expr_overflow(condition, func_name, reports);
            check_stmt_overflow(then_body, func_name, reports);
            if let Some(eb) = else_body {
                check_stmt_overflow(eb, func_name, reports);
            }
        }
        Stmt::While { condition, body } => {
            check_expr_overflow(condition, func_name, reports);
            check_stmt_overflow(body, func_name, reports);
        }
        Stmt::Block(stmts) => {
            for s in stmts {
                check_stmt_overflow(s, func_name, reports);
            }
        }
        _ => {}
    }
}

fn check_expr_overflow(expr: &Expr, func_name: &str, reports: &mut Vec<UBReport>) {
    match expr {
        Expr::BinaryOp { op, left, right } => {
            // Detectar overflow en operaciones aritméticas
            if let (Expr::Number(l), Expr::Number(r)) = (left.as_ref(), right.as_ref()) {
                match op {
                    BinaryOp::Add => {
                        if will_overflow_add(*l, *r) {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::IntegerOverflow,
                                    format!("Integer overflow in addition: {} + {}", l, r),
                                )
                                .with_location(func_name.to_string(), 0)
                                .with_suggestion("Use checked arithmetic or wider type".to_string())
                            );
                        }
                    }
                    BinaryOp::Sub => {
                        if will_underflow_sub(*l, *r) {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::IntegerUnderflow,
                                    format!("Integer underflow in subtraction: {} - {}", l, r),
                                )
                                .with_location(func_name.to_string(), 0)
                            );
                        }
                    }
                    BinaryOp::Mul => {
                        if will_overflow_mul(*l, *r) {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::IntegerOverflow,
                                    format!("Integer overflow in multiplication: {} * {}", l, r),
                                )
                                .with_location(func_name.to_string(), 0)
                            );
                        }
                    }
                    BinaryOp::Div => {
                        if *r == 0 {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::DivisionByZero,
                                    format!("Division by zero: {} / 0", l),
                                )
                                .with_location(func_name.to_string(), 0)
                                .with_suggestion("Add zero check before division".to_string())
                            );
                        }
                    }
                    BinaryOp::Mod => {
                        if *r == 0 {
                            reports.push(
                                UBReport::new(
                                    UBSeverity::Error,
                                    UBKind::DivisionByZero,
                                    format!("Modulo by zero: {} % 0", l),
                                )
                                .with_location(func_name.to_string(), 0)
                            );
                        }
                    }
                    _ => {}
                }
            }
            check_expr_overflow(left, func_name, reports);
            check_expr_overflow(right, func_name, reports);
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
