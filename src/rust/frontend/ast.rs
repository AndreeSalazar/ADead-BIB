// AST (Abstract Syntax Tree) para ADead-BIB
// Lenguaje de uso general con OOP - Binario + HEX

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Variable(String),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Comparison {
        op: CmpOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    // Arrays y colecciones
    Array(Vec<Expr>),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Slice {
        object: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
    },
    // OOP
    New {
        class_name: String,
        args: Vec<Expr>,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
    This,
    Super,
    // Input del usuario
    Input,  // input() - lee un número del teclado
    // Funcional
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CmpOp {
    Eq,      // ==
    Ne,      // !=
    Lt,      // <
    Le,      // <=
    Gt,      // >
    Ge,      // >=
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Println(Expr),  // println con \n automático
    PrintNum(Expr),
    Assign {
        name: String,
        value: Expr,
    },
    IndexAssign {
        object: Expr,
        index: Expr,
        value: Expr,
    },
    FieldAssign {
        object: Expr,
        field: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        start: Expr,
        end: Expr,
        body: Vec<Stmt>,
    },
    ForEach {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    Pass,
    Assert {
        condition: Expr,
        message: Option<Expr>,
    },
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
    pub body: Vec<Stmt>,
}

// OOP: Interface/Trait
#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
    pub methods: Vec<MethodSignature>,
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
}

// OOP: Clase con herencia y polimorfismo
#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub parent: Option<String>,        // Herencia
    pub implements: Vec<String>,       // Interfaces implementadas
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub constructor: Option<Method>,   // __init__
    pub destructor: Option<Method>,    // __del__
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub type_name: Option<String>,
    pub default_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
    pub body: Vec<Stmt>,
    pub is_virtual: bool,              // Para polimorfismo
    pub is_override: bool,             // Override de método padre
    pub is_static: bool,               // Método estático
}

// Rust-style struct
#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub type_name: Option<String>,
}

// Rust-style impl block
#[derive(Debug, Clone)]
pub struct Impl {
    pub struct_name: String,
    pub methods: Vec<Function>,
}

// Sistema de imports
#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
    pub items: Vec<String>,      // from module import item1, item2
    pub alias: Option<String>,   // import module as alias
}

#[derive(Debug, Clone)]
pub struct Program {
    pub imports: Vec<Import>,
    pub interfaces: Vec<Interface>,
    pub classes: Vec<Class>,
    pub structs: Vec<Struct>,       // Rust-style structs
    pub impls: Vec<Impl>,           // Rust-style impl blocks
    pub functions: Vec<Function>,
    pub statements: Vec<Stmt>, // Top-level statements (scripts)
}

impl Program {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
            interfaces: Vec::new(),
            classes: Vec::new(),
            structs: Vec::new(),
            impls: Vec::new(),
            functions: Vec::new(),
            statements: Vec::new(),
        }
    }
    
    pub fn add_struct(&mut self, s: Struct) {
        self.structs.push(s);
    }
    
    pub fn add_impl(&mut self, i: Impl) {
        self.impls.push(i);
    }
    
    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }
    
    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func);
    }
    
    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }
    
    pub fn add_interface(&mut self, iface: Interface) {
        self.interfaces.push(iface);
    }

    pub fn add_statement(&mut self, stmt: Stmt) {
        self.statements.push(stmt);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

