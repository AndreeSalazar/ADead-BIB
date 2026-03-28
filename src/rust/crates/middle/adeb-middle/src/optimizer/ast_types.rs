//! Stub types for optimizer compatibility

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Identifier(String),
    Variable(String),
    BinaryOp { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
    UnaryOp { op: UnaryOp, expr: Box<Expr> },
    Comparison { op: CmpOp, left: Box<Expr>, right: Box<Expr> },
    Ternary { condition: Box<Expr>, then_expr: Box<Expr>, else_expr: Box<Expr> },
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

pub type BinOp = BinaryOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitwiseOp {
    And, Or, Xor, LeftShift, RightShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Eq, Ne, Lt, Le, Gt, Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg, Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { name: String, var_type: Option<String>, value: Option<Expr> },
    Assign { name: String, value: Expr },
    If { condition: Expr, then_body: Vec<Stmt>, else_body: Option<Vec<Stmt>> },
    While { condition: Expr, body: Vec<Stmt> },
    DoWhile { body: Vec<Stmt>, condition: Expr },
    For { var: String, start: Expr, end: Expr, body: Vec<Stmt> },
    Print(Expr),
    Println(Expr),
    PrintNum(Expr),
    Expr(Expr),
    Return(Option<Expr>),
    Pass,
}
