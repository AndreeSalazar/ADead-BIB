//! Stub types for optimizer compatibility

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Identifier(String),
    BinaryOp { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
    Comparison { op: String, left: Box<Expr>, right: Box<Expr> },
    Ternary { cond: Box<Expr>, then_branch: Box<Expr>, else_branch: Box<Expr> },
    BitwiseOp { op: BitwiseOp, left: Box<Expr>, right: Box<Expr> },
    BitwiseNot(Box<Expr>),
    Call { name: String, args: Vec<Expr> },
    Index { object: Box<Expr>, index: Box<Expr> },
    Array(Vec<Expr>),
    Cast { target_type: String, expr: Box<Expr> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitwiseOp {
    And, Or, Xor, Shl, Shr,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { name: String, var_type: Option<String>, value: Option<Expr> },
    Assign { name: String, value: Expr },
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    While { condition: Expr, body: Vec<Stmt> },
    Print(Expr),
    Return(Option<Expr>),
}
