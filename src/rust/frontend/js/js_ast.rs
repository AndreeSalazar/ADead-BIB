// ============================================================
// JsDead-BIB — JavaScript AST
// ============================================================
// JS AST types for the JavaScript frontend
// Implícitamente estricto — "Respetar Bits"
// Sin Node.js. Sin V8. Sin runtime. JS → ASM directo.
// ============================================================

/// JavaScript expression AST node
#[derive(Debug, Clone)]
pub enum JsExpr {
    // Literales
    NumberInt(i64),
    NumberFloat(f64),
    StringLit(String),
    TemplateLit(Vec<TemplateSegment>),
    Bool(bool),
    Null,
    Undefined,

    // Variables
    Identifier(String),
    This,

    // Operaciones
    BinaryOp {
        op: JsBinOp,
        left: Box<JsExpr>,
        right: Box<JsExpr>,
    },
    UnaryOp {
        op: JsUnOp,
        expr: Box<JsExpr>,
    },

    // Acceso
    MemberAccess {
        object: Box<JsExpr>,
        property: String,
        optional: bool,
    },
    ComputedAccess {
        object: Box<JsExpr>,
        index: Box<JsExpr>,
    },

    // Llamadas
    Call {
        callee: Box<JsExpr>,
        args: Vec<JsExpr>,
    },
    New {
        callee: Box<JsExpr>,
        args: Vec<JsExpr>,
    },

    // Funciones
    ArrowFunc {
        params: Vec<JsParam>,
        body: JsArrowBody,
    },
    FuncExpr {
        name: Option<String>,
        params: Vec<JsParam>,
        body: Vec<JsStmt>,
    },

    // Arrays y Objetos
    ArrayLit(Vec<JsExpr>),
    ObjectLit(Vec<(String, JsExpr)>),

    // Spread
    Spread(Box<JsExpr>),

    // Ternario
    Ternary {
        cond: Box<JsExpr>,
        then_expr: Box<JsExpr>,
        else_expr: Box<JsExpr>,
    },

    // Await
    Await(Box<JsExpr>),

    // Assignment
    Assign {
        target: Box<JsExpr>,
        op: JsAssignOp,
        value: Box<JsExpr>,
    },

    // Typeof
    Typeof(Box<JsExpr>),

    // Void
    VoidExpr(Box<JsExpr>),
}

/// Template literal segment
#[derive(Debug, Clone)]
pub enum TemplateSegment {
    Str(String),
    Expr(JsExpr),
}

/// Arrow function body
#[derive(Debug, Clone)]
pub enum JsArrowBody {
    Expr(Box<JsExpr>),
    Block(Vec<JsStmt>),
}

/// JavaScript statement AST node
#[derive(Debug, Clone)]
pub enum JsStmt {
    // Declaraciones
    VarDecl {
        kind: DeclKind,
        name: String,
        type_ann: Option<JsType>,
        init: Option<JsExpr>,
    },

    // Control de flujo
    If {
        cond: JsExpr,
        then_body: Box<JsStmt>,
        else_body: Option<Box<JsStmt>>,
    },
    For {
        init: Option<Box<JsStmt>>,
        cond: Option<JsExpr>,
        update: Option<JsExpr>,
        body: Box<JsStmt>,
    },
    ForOf {
        decl: String,
        iter: JsExpr,
        body: Box<JsStmt>,
    },
    ForIn {
        decl: String,
        iter: JsExpr,
        body: Box<JsStmt>,
    },
    While {
        cond: JsExpr,
        body: Box<JsStmt>,
    },
    DoWhile {
        body: Box<JsStmt>,
        cond: JsExpr,
    },
    Switch {
        expr: JsExpr,
        cases: Vec<JsCase>,
    },

    // Salida
    Return(Option<JsExpr>),
    Break(Option<String>),
    Continue(Option<String>),
    Throw(JsExpr),

    // Bloques
    Block(Vec<JsStmt>),

    // Expresión como statement
    Expr(JsExpr),

    // Funciones y clases
    FuncDecl {
        name: String,
        params: Vec<JsParam>,
        return_type: Option<JsType>,
        body: Vec<JsStmt>,
        is_async: bool,
    },
    ClassDecl {
        name: String,
        super_class: Option<String>,
        body: Vec<JsClassMember>,
    },

    // Try/Catch
    TryCatch {
        try_body: Vec<JsStmt>,
        catch_param: Option<String>,
        catch_body: Option<Vec<JsStmt>>,
        finally_body: Option<Vec<JsStmt>>,
    },

    // Labels
    Labeled {
        label: String,
        stmt: Box<JsStmt>,
    },

    // Modules
    Import {
        items: Vec<JsImportItem>,
        from: String,
    },
    Export {
        item: Box<JsStmt>,
    },
    ExportDefault(JsExpr),

    // Line marker for error reporting
    LineMarker(usize),

    // Empty statement (;)
    Empty,
}

/// Import item specification
#[derive(Debug, Clone)]
pub enum JsImportItem {
    /// import { name } or import { name as alias }
    Named { name: String, alias: Option<String> },
    /// import defaultName from "module"
    Default(String),
    /// import * as name from "module"
    Namespace(String),
}

/// Switch case
#[derive(Debug, Clone)]
pub struct JsCase {
    pub test: Option<JsExpr>, // None = default
    pub body: Vec<JsStmt>,
}

/// Class member
#[derive(Debug, Clone)]
pub enum JsClassMember {
    Constructor {
        params: Vec<JsParam>,
        body: Vec<JsStmt>,
    },
    Method {
        name: String,
        params: Vec<JsParam>,
        return_type: Option<JsType>,
        body: Vec<JsStmt>,
        is_static: bool,
        is_async: bool,
    },
    Property {
        name: String,
        type_ann: Option<JsType>,
        init: Option<JsExpr>,
        is_static: bool,
    },
    Getter {
        name: String,
        body: Vec<JsStmt>,
    },
    Setter {
        name: String,
        param: String,
        body: Vec<JsStmt>,
    },
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct JsParam {
    pub name: String,
    pub type_ann: Option<JsType>,
    pub default: Option<JsExpr>,
    pub is_rest: bool,
}

/// JsDead-BIB type annotations (implícitamente estricto)
#[derive(Debug, Clone, PartialEq)]
pub enum JsType {
    // Primitivos
    Number,  // int (i64)
    Int,     // explicit int
    Float,   // explicit float (f64)
    String,
    Boolean,
    Null,
    Undefined,
    Void,

    // Compuestos
    Array(Box<JsType>),
    Object(Vec<(std::string::String, JsType)>),

    // Función
    Function {
        params: Vec<JsType>,
        ret: Box<JsType>,
    },

    // Clase / Named
    Named(std::string::String),

    // Inferencia
    Inferred,
}

/// Declaration kind
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeclKind {
    Var,
    Let,
    Const,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JsBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    // Equality
    EqStrict,   // ===
    NeStrict,   // !==
    Eq,         // == (blocked in JsDead-BIB)
    Ne,         // != (blocked in JsDead-BIB)
    // Comparison
    Lt,
    Gt,
    Le,
    Ge,
    // Logical
    And,
    Or,
    Nullish,    // ??
    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    UShr,       // >>>
    // Instanceof / in
    Instanceof,
    In,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JsUnOp {
    Neg,
    Not,
    BitNot,
    Typeof,
    Void,
    Delete,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}

/// Assignment operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JsAssignOp {
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

/// A complete JS translation unit
#[derive(Debug, Clone)]
pub struct JsProgram {
    pub stmts: Vec<JsStmt>,
}

impl JsProgram {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }
}

impl Default for JsProgram {
    fn default() -> Self {
        Self::new()
    }
}
