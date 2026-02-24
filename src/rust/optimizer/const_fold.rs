// Constant folding pass for ADead-BIB
// Evaluates constant expressions at compile time
// Example: 2 + 3 * 4 → 14

use crate::frontend::ast::*;

pub struct ConstFolder;

impl ConstFolder {
    pub fn new() -> Self {
        Self
    }

    /// Fold constant expressions in a program
    pub fn fold_program(&self, program: &mut Program) {
        for func in &mut program.functions {
            self.fold_stmts(&mut func.body);
        }
        self.fold_stmts(&mut program.statements);
    }

    fn fold_stmts(&self, stmts: &mut Vec<Stmt>) {
        for stmt in stmts.iter_mut() {
            self.fold_stmt(stmt);
        }
    }

    fn fold_stmt(&self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Assign { value, .. } => {
                *value = self.fold_expr(value.clone());
            }
            Stmt::Print(expr) | Stmt::Println(expr) | Stmt::PrintNum(expr) => {
                *expr = self.fold_expr(expr.clone());
            }
            Stmt::If { condition, then_body, else_body } => {
                *condition = self.fold_expr(condition.clone());
                self.fold_stmts(then_body);
                if let Some(body) = else_body {
                    self.fold_stmts(body);
                }
            }
            Stmt::While { condition, body } => {
                *condition = self.fold_expr(condition.clone());
                self.fold_stmts(body);
            }
            Stmt::DoWhile { body, condition } => {
                self.fold_stmts(body);
                *condition = self.fold_expr(condition.clone());
            }
            Stmt::For { start, end, body, .. } => {
                *start = self.fold_expr(start.clone());
                *end = self.fold_expr(end.clone());
                self.fold_stmts(body);
            }
            Stmt::Return(Some(expr)) => {
                *expr = self.fold_expr(expr.clone());
            }
            Stmt::Expr(expr) => {
                *expr = self.fold_expr(expr.clone());
            }
            _ => {}
        }
    }

    pub fn fold_expr(&self, expr: Expr) -> Expr {
        match expr {
            Expr::BinaryOp { op, left, right } => {
                let left = self.fold_expr(*left);
                let right = self.fold_expr(*right);

                // If both sides are constants, compute result
                if let (Expr::Number(l), Expr::Number(r)) = (&left, &right) {
                    match op {
                        BinOp::Add => return Expr::Number(l.wrapping_add(*r)),
                        BinOp::Sub => return Expr::Number(l.wrapping_sub(*r)),
                        BinOp::Mul => return Expr::Number(l.wrapping_mul(*r)),
                        BinOp::Div if *r != 0 => return Expr::Number(l.wrapping_div(*r)),
                        BinOp::Mod if *r != 0 => return Expr::Number(l.wrapping_rem(*r)),
                        _ => {}
                    }
                }

                // Strength reduction / identity simplifications
                // x + 0 → x, x - 0 → x, x * 0 → 0, x * 1 → x, x / 1 → x
                if let Expr::Number(r) = &right {
                    match (op, *r) {
                        (BinOp::Add, 0) | (BinOp::Sub, 0) => return left,
                        (BinOp::Mul, 0) => return Expr::Number(0),
                        (BinOp::Mul, 1) | (BinOp::Div, 1) => return left,
                        _ => {}
                    }
                }
                if let Expr::Number(l) = &left {
                    match (op, *l) {
                        (BinOp::Add, 0) => return right,
                        (BinOp::Mul, 0) => return Expr::Number(0),
                        (BinOp::Mul, 1) => return right,
                        _ => {}
                    }
                }

                Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Expr::UnaryOp { op, expr: inner } => {
                let inner = self.fold_expr(*inner);
                if let Expr::Number(n) = &inner {
                    match op {
                        UnaryOp::Neg => return Expr::Number(-n),
                        UnaryOp::Not => return Expr::Number(if *n == 0 { 1 } else { 0 }),
                    }
                }
                Expr::UnaryOp { op, expr: Box::new(inner) }
            }
            Expr::Comparison { op, left, right } => {
                let left = self.fold_expr(*left);
                let right = self.fold_expr(*right);

                if let (Expr::Number(l), Expr::Number(r)) = (&left, &right) {
                    let result = match op {
                        CmpOp::Eq => l == r,
                        CmpOp::Ne => l != r,
                        CmpOp::Lt => l < r,
                        CmpOp::Le => l <= r,
                        CmpOp::Gt => l > r,
                        CmpOp::Ge => l >= r,
                    };
                    return Expr::Number(if result { 1 } else { 0 });
                }

                Expr::Comparison {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Expr::Call { name, args } => {
                let args = args.into_iter().map(|a| self.fold_expr(a)).collect();
                Expr::Call { name, args }
            }
            _ => expr,
        }
    }
}
