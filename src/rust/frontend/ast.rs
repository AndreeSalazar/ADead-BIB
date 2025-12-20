// AST (Abstract Syntax Tree) para ADead-BIB

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    String(String),
    Variable(String),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Assign {
        name: String,
        value: Expr,
    },
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
    
    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func);
    }
}

