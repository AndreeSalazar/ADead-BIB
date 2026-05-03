//! AST C99 - Generado automáticamente
#![allow(dead_code)]

pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

// ============== TYPES ==============
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Bool,
    Char,
    Short,
    Int,
    Long,
    LongLong,
    Float,
    Double,
    Signed(Box<Type>),
    Unsigned(Box<Type>),
    Pointer(Box<Type>),
    Array(Box<Type>, Option<usize>),
    Function(Box<Type>, Vec<Type>),
    Struct(String),
    Union(String),
    Enum(String),
    Typedef(String),
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Type::Void => 0,
            Type::Bool | Type::Char => 1,
            Type::Short => 2,
            Type::Int | Type::Float => 4,
            Type::Long | Type::LongLong | Type::Double | Type::Pointer(_) => 8,
            Type::Signed(t) | Type::Unsigned(t) => t.size(),
            Type::Array(t, Some(n)) => t.size() * n,
            _ => 8,
        }
    }
}

// ============== EXPRESSIONS ==============
#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    IntLit(i64),
    FloatLit(f64),
    CharLit(char),
    StringLit(String),
    
    // Identifiers
    Ident(String),
    
    // Operators
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    
    // Access
    Index(Box<Expr>, Box<Expr>),
    Member(Box<Expr>, String),
    Arrow(Box<Expr>, String),
    
    // Calls
    Call(Box<Expr>, Vec<Expr>),
    
    // Casts
    Cast(Type, Box<Expr>),
    
    // Sizeof
    SizeofExpr(Box<Expr>),
    SizeofType(Type),
    
    // Compound
    Comma(Vec<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    CompoundAssign(BinOp, Box<Expr>, Box<Expr>),
    
    // Address
    AddrOf(Box<Expr>),
    Deref(Box<Expr>),
    
    // Increment
    PreInc(Box<Expr>),
    PreDec(Box<Expr>),
    PostInc(Box<Expr>),
    PostDec(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    BitAnd, BitOr, BitXor, Shl, Shr,
    Eq, Ne, Lt, Gt, Le, Ge,
    LogAnd, LogOr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg, Not, BitNot,
}

// ============== STATEMENTS ==============
#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Return(Option<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    DoWhile(Box<Stmt>, Expr),
    For(Option<Box<Stmt>>, Option<Expr>, Option<Expr>, Box<Stmt>),
    Switch(Expr, Vec<SwitchCase>),
    Break,
    Continue,
    Goto(String),
    Label(String, Box<Stmt>),
    Block(Vec<Stmt>),
    Decl(Decl),
    Empty,
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub value: Option<Expr>, // None = default
    pub stmts: Vec<Stmt>,
}

// ============== DECLARATIONS ==============
#[derive(Debug, Clone)]
pub struct Decl {
    pub ty: Type,
    pub name: String,
    pub init: Option<Expr>,
    pub is_static: bool,
    pub is_extern: bool,
    pub is_const: bool,
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub ret_type: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Option<Vec<Stmt>>,
    pub is_inline: bool,
    pub is_static: bool,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Type,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: Option<String>,
    pub fields: Vec<Decl>,
    pub is_union: bool,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: Option<String>,
    pub variants: Vec<(String, Option<i64>)>,
}

#[derive(Debug, Clone)]
pub struct TypedefDecl {
    pub ty: Type,
    pub name: String,
}

// ============== TRANSLATION UNIT ==============
#[derive(Debug, Clone)]
pub enum TopLevel {
    Func(FuncDecl),
    Var(Decl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Typedef(TypedefDecl),
}

#[derive(Debug, Default)]
pub struct TranslationUnit {
    pub items: Vec<TopLevel>,
}

impl TranslationUnit {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}
