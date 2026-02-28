// ============================================================
// C AST → ADead-BIB IR Converter
// ============================================================
// Lowers C99 AST to ADead-BIB's Program/Function/Stmt/Expr
// This is the bridge: C enters here, ADead-BIB IR exits.
//
// Pipeline: C Source → CLexer → CParser → CTranslationUnit
//           → CToIR → Program → ISA Compiler → PE/ELF
// ============================================================

use super::c_ast::*;
use crate::frontend::ast::{
    self, BinOp, BitwiseOp, CmpOp, Expr, Function, FunctionAttributes, Import,
    Param, Program, ProgramAttributes, Stmt, Struct, StructField, UnaryOp,
};
use crate::frontend::types::Type;

pub struct CToIR {
    /// Accumulated enum constants as global assigns
    enum_constants: Vec<(String, i64)>,
    /// Typedef aliases (new_name → original CType)
    typedefs: Vec<(String, CType)>,
}

impl CToIR {
    pub fn new() -> Self {
        Self {
            enum_constants: Vec::new(),
            typedefs: Vec::new(),
        }
    }

    /// Main entry: Convert entire C translation unit → ADead-BIB Program
    pub fn convert(&mut self, unit: &CTranslationUnit) -> Result<Program, String> {
        let mut program = Program::new();
        program.attributes = ProgramAttributes::default();

        // First pass: collect structs, enums, typedefs
        for decl in &unit.declarations {
            match decl {
                CTopLevel::StructDef { name, fields } => {
                    program.structs.push(self.convert_struct(name, fields));
                }
                CTopLevel::EnumDef { name: _, values } => {
                    self.collect_enum_constants(values);
                }
                CTopLevel::TypedefDecl { original, new_name } => {
                    self.typedefs.push((new_name.clone(), original.clone()));
                }
                _ => {}
            }
        }

        // Second pass: functions and global vars
        for decl in &unit.declarations {
            match decl {
                CTopLevel::FunctionDef { return_type, name, params, body } => {
                    let func = self.convert_function(return_type, name, params, body)?;
                    program.functions.push(func);
                }
                CTopLevel::FunctionDecl { .. } => {
                    // Prototypes — skip (resolved at link time)
                }
                CTopLevel::GlobalVar { type_spec, declarators } => {
                    // Global variables → top-level statements
                    for decl in declarators {
                        if let Some(ref init) = decl.initializer {
                            let val = self.convert_expr(init)?;
                            program.statements.push(Stmt::Assign {
                                name: decl.name.clone(),
                                value: val,
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        // Inject enum constants as assigns in main if they exist
        // (they'll be available as compile-time constants)

        Ok(program)
    }

    // ========== Type conversion ==========

    fn convert_type(&self, cty: &CType) -> Type {
        match cty {
            CType::Void => Type::Void,
            CType::Char => Type::I8,
            CType::Short => Type::I16,
            CType::Int => Type::I32,
            CType::Long => Type::I64,
            CType::LongLong => Type::I64,
            CType::Float => Type::F32,
            CType::Double => Type::F64,
            CType::Bool => Type::Bool,
            CType::Unsigned(inner) => match inner.as_ref() {
                CType::Char => Type::U8,
                CType::Short => Type::U16,
                CType::Int => Type::U32,
                CType::Long | CType::LongLong => Type::U64,
                _ => Type::U32,
            },
            CType::Signed(inner) => self.convert_type(inner),
            CType::Pointer(inner) => Type::Pointer(Box::new(self.convert_type(inner))),
            CType::Array(inner, size) => Type::Array(Box::new(self.convert_type(inner)), *size),
            CType::Struct(name) => Type::Struct(name.clone()),
            CType::Enum(_) => Type::I32, // enums are ints in C
            CType::Typedef(name) => {
                // Resolve typedef
                if let Some((_, original)) = self.typedefs.iter().find(|(n, _)| n == name) {
                    self.convert_type(original)
                } else {
                    Type::Named(name.clone())
                }
            }
            CType::Function { return_type, params } => {
                let ret = self.convert_type(return_type);
                let args: Vec<Type> = params.iter().map(|p| self.convert_type(p)).collect();
                Type::Function(args, Box::new(ret))
            }
            CType::Const(inner) | CType::Volatile(inner) => self.convert_type(inner),
        }
    }

    fn type_name(cty: &CType) -> Option<String> {
        match cty {
            CType::Void => Some("void".to_string()),
            CType::Char => Some("char".to_string()),
            CType::Short => Some("short".to_string()),
            CType::Int => Some("int".to_string()),
            CType::Long => Some("long".to_string()),
            CType::Float => Some("float".to_string()),
            CType::Double => Some("double".to_string()),
            CType::Bool => Some("bool".to_string()),
            _ => None,
        }
    }

    // ========== Struct conversion ==========

    fn convert_struct(&self, name: &str, fields: &[CStructField]) -> Struct {
        Struct {
            name: name.to_string(),
            fields: fields
                .iter()
                .map(|f| StructField {
                    name: f.name.clone(),
                    field_type: self.convert_type(&f.field_type),
                })
                .collect(),
            is_packed: false,
        }
    }

    // ========== Enum constants ==========

    fn collect_enum_constants(&mut self, values: &[(String, Option<i64>)]) {
        let mut next_val: i64 = 0;
        for (name, explicit_val) in values {
            let val = explicit_val.unwrap_or(next_val);
            self.enum_constants.push((name.clone(), val));
            next_val = val + 1;
        }
    }

    // ========== Function conversion ==========

    fn convert_function(
        &self,
        return_type: &CType,
        name: &str,
        params: &[CParam],
        body: &[CStmt],
    ) -> Result<Function, String> {
        let adead_params: Vec<Param> = params
            .iter()
            .map(|p| {
                let pname = p.name.clone().unwrap_or_else(|| "_".to_string());
                Param {
                    name: pname,
                    param_type: self.convert_type(&p.param_type),
                    default_value: None,
                }
            })
            .collect();

        let mut adead_body = Vec::new();
        for stmt in body {
            let converted = self.convert_stmt(stmt)?;
            adead_body.extend(converted);
        }

        Ok(Function {
            name: name.to_string(),
            params: adead_params,
            return_type: Self::type_name(return_type),
            resolved_return_type: self.convert_type(return_type),
            body: adead_body,
            attributes: FunctionAttributes::default(),
        })
    }

    // ========== Statement conversion ==========

    fn convert_stmt(&self, stmt: &CStmt) -> Result<Vec<Stmt>, String> {
        match stmt {
            CStmt::Expr(expr) => {
                let converted = self.convert_expr_to_stmt(expr)?;
                Ok(converted)
            }

            CStmt::Return(None) => Ok(vec![Stmt::Return(None)]),
            CStmt::Return(Some(expr)) => {
                let val = self.convert_expr(expr)?;
                Ok(vec![Stmt::Return(Some(val))])
            }

            CStmt::VarDecl { type_spec, declarators } => {
                let mut stmts = Vec::new();
                for decl in declarators {
                    let value = if let Some(ref init) = decl.initializer {
                        self.convert_expr(init)?
                    } else {
                        // Default zero initialization
                        self.default_value(type_spec)
                    };

                    // Handle array declarators
                    let final_value = if let Some(CDerivedType::Array(Some(size), _)) = &decl.derived_type {
                        if matches!(value, Expr::Number(0)) {
                            // Zero-initialized array
                            Expr::Array(vec![Expr::Number(0); *size])
                        } else {
                            value
                        }
                    } else {
                        value
                    };

                    stmts.push(Stmt::Assign {
                        name: decl.name.clone(),
                        value: final_value,
                    });
                }
                Ok(stmts)
            }

            CStmt::Block(stmts) => {
                let mut result = Vec::new();
                for s in stmts {
                    result.extend(self.convert_stmt(s)?);
                }
                Ok(result)
            }

            CStmt::If { condition, then_body, else_body } => {
                let cond = self.convert_expr(condition)?;
                let then_stmts = self.convert_stmt(then_body)?;
                let else_stmts = if let Some(else_b) = else_body {
                    Some(self.convert_stmt(else_b)?)
                } else {
                    None
                };

                Ok(vec![Stmt::If {
                    condition: cond,
                    then_body: then_stmts,
                    else_body: else_stmts,
                }])
            }

            CStmt::While { condition, body } => {
                let cond = self.convert_expr(condition)?;
                let body_stmts = self.convert_stmt(body)?;
                Ok(vec![Stmt::While {
                    condition: cond,
                    body: body_stmts,
                }])
            }

            CStmt::DoWhile { body, condition } => {
                // do { body } while (cond) →
                // loop { body; if (!cond) break; }
                let cond = self.convert_expr(condition)?;
                let mut body_stmts = self.convert_stmt(body)?;
                // Add: if (!condition) break
                body_stmts.push(Stmt::If {
                    condition: Expr::UnaryOp {
                        op: UnaryOp::Not,
                        expr: Box::new(cond),
                    },
                    then_body: vec![Stmt::Break],
                    else_body: None,
                });
                Ok(vec![Stmt::While {
                    condition: Expr::Bool(true),
                    body: body_stmts,
                }])
            }

            CStmt::For { init, condition, update, body } => {
                let mut result = Vec::new();

                // Init statement
                if let Some(init_stmt) = init {
                    result.extend(self.convert_stmt(init_stmt)?);
                }

                // Build loop body
                let mut loop_body = self.convert_stmt(body)?;

                // Add update at end of loop body
                if let Some(upd) = update {
                    let upd_stmts = self.convert_expr_to_stmt(upd)?;
                    loop_body.extend(upd_stmts);
                }

                // Condition (default true if missing)
                let cond = if let Some(c) = condition {
                    self.convert_expr(c)?
                } else {
                    Expr::Bool(true)
                };

                result.push(Stmt::While {
                    condition: cond,
                    body: loop_body,
                });

                Ok(result)
            }

            CStmt::Switch { expr, cases } => {
                // switch → chain of if/else
                let switch_val = self.convert_expr(expr)?;
                // Create temp var for switch value
                let switch_var = "__switch_val".to_string();
                let mut result = vec![Stmt::Assign {
                    name: switch_var.clone(),
                    value: switch_val,
                }];

                let mut last_else: Option<Vec<Stmt>> = None;

                // Process cases in reverse to build if/else chain
                for case in cases.iter().rev() {
                    let mut case_body: Vec<Stmt> = Vec::new();
                    for s in &case.body {
                        let stmts = self.convert_stmt(s)?;
                        for st in stmts {
                            if matches!(st, Stmt::Break) {
                                break;
                            }
                            case_body.push(st);
                        }
                    }

                    if let Some(ref val) = case.value {
                        // case N:
                        let cond = Expr::Comparison {
                            op: CmpOp::Eq,
                            left: Box::new(Expr::Variable(switch_var.clone())),
                            right: Box::new(self.convert_expr(val)?),
                        };
                        let if_stmt = Stmt::If {
                            condition: cond,
                            then_body: case_body,
                            else_body: last_else.take(),
                        };
                        last_else = Some(vec![if_stmt]);
                    } else {
                        // default:
                        last_else = Some(case_body);
                    }
                }

                if let Some(stmts) = last_else {
                    result.extend(stmts);
                }

                Ok(result)
            }

            CStmt::Break => Ok(vec![Stmt::Break]),
            CStmt::Continue => Ok(vec![Stmt::Continue]),
            CStmt::Goto(_label) => Ok(vec![]), // Skip goto for now
            CStmt::Label(_name, inner) => self.convert_stmt(inner),
            CStmt::Empty => Ok(vec![]),
        }
    }

    /// Convert a C expression used as a statement (handles printf, assignments, etc.)
    fn convert_expr_to_stmt(&self, expr: &CExpr) -> Result<Vec<Stmt>, String> {
        match expr {
            // printf("...") → Print/Println
            CExpr::Call { func, args } => {
                if let CExpr::Identifier(name) = func.as_ref() {
                    match name.as_str() {
                        "printf" => return self.convert_printf(args),
                        "puts" => {
                            if let Some(arg) = args.first() {
                                let val = self.convert_expr(arg)?;
                                return Ok(vec![Stmt::Println(val)]);
                            }
                            return Ok(vec![]);
                        }
                        "putchar" => {
                            if let Some(arg) = args.first() {
                                let val = self.convert_expr(arg)?;
                                return Ok(vec![Stmt::Print(val)]);
                            }
                            return Ok(vec![]);
                        }
                        "free" => {
                            if let Some(arg) = args.first() {
                                let val = self.convert_expr(arg)?;
                                return Ok(vec![Stmt::Free(val)]);
                            }
                            return Ok(vec![]);
                        }
                        _ => {}
                    }
                }
                // General function call as statement
                let call_expr = self.convert_expr(expr)?;
                Ok(vec![Stmt::Expr(call_expr)])
            }

            // x = 5 → Assign
            CExpr::Assign { op, target, value } => {
                self.convert_assignment(op, target, value)
            }

            // x++ / ++x as statement
            CExpr::UnaryOp { op, expr: inner, .. } => {
                match op {
                    CUnaryOp::PreInc | CUnaryOp::PostInc => {
                        if let CExpr::Identifier(name) = inner.as_ref() {
                            Ok(vec![Stmt::Assign {
                                name: name.clone(),
                                value: Expr::BinaryOp {
                                    op: BinOp::Add,
                                    left: Box::new(Expr::Variable(name.clone())),
                                    right: Box::new(Expr::Number(1)),
                                },
                            }])
                        } else {
                            let e = self.convert_expr(expr)?;
                            Ok(vec![Stmt::Expr(e)])
                        }
                    }
                    CUnaryOp::PreDec | CUnaryOp::PostDec => {
                        if let CExpr::Identifier(name) = inner.as_ref() {
                            Ok(vec![Stmt::Assign {
                                name: name.clone(),
                                value: Expr::BinaryOp {
                                    op: BinOp::Sub,
                                    left: Box::new(Expr::Variable(name.clone())),
                                    right: Box::new(Expr::Number(1)),
                                },
                            }])
                        } else {
                            let e = self.convert_expr(expr)?;
                            Ok(vec![Stmt::Expr(e)])
                        }
                    }
                    _ => {
                        let e = self.convert_expr(expr)?;
                        Ok(vec![Stmt::Expr(e)])
                    }
                }
            }

            // Comma expression as statement → multiple statements
            CExpr::Comma(exprs) => {
                let mut stmts = Vec::new();
                for e in exprs {
                    stmts.extend(self.convert_expr_to_stmt(e)?);
                }
                Ok(stmts)
            }

            _ => {
                let e = self.convert_expr(expr)?;
                Ok(vec![Stmt::Expr(e)])
            }
        }
    }

    fn convert_assignment(
        &self,
        op: &CAssignOp,
        target: &CExpr,
        value: &CExpr,
    ) -> Result<Vec<Stmt>, String> {
        let rhs = self.convert_expr(value)?;

        match target {
            CExpr::Identifier(name) => {
                let final_value = match op {
                    CAssignOp::Assign => rhs,
                    CAssignOp::AddAssign => Expr::BinaryOp {
                        op: BinOp::Add,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                    CAssignOp::SubAssign => Expr::BinaryOp {
                        op: BinOp::Sub,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                    CAssignOp::MulAssign => Expr::BinaryOp {
                        op: BinOp::Mul,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                    CAssignOp::DivAssign => Expr::BinaryOp {
                        op: BinOp::Div,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                    CAssignOp::ModAssign => Expr::BinaryOp {
                        op: BinOp::Mod,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                    CAssignOp::AndAssign => Expr::BitwiseOp {
                        op: BitwiseOp::And,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                    CAssignOp::OrAssign => Expr::BitwiseOp {
                        op: BitwiseOp::Or,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                    CAssignOp::XorAssign => Expr::BitwiseOp {
                        op: BitwiseOp::Xor,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                    CAssignOp::ShlAssign => Expr::BitwiseOp {
                        op: BitwiseOp::LeftShift,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                    CAssignOp::ShrAssign => Expr::BitwiseOp {
                        op: BitwiseOp::RightShift,
                        left: Box::new(Expr::Variable(name.clone())),
                        right: Box::new(rhs),
                    },
                };
                Ok(vec![Stmt::Assign { name: name.clone(), value: final_value }])
            }
            CExpr::Index { array, index } => {
                let obj = self.convert_expr(array)?;
                let idx = self.convert_expr(index)?;
                // For compound assignment on array index, we need the simple value
                let final_rhs = match op {
                    CAssignOp::Assign => rhs,
                    _ => rhs, // simplified — compound array assign treated as direct
                };
                Ok(vec![Stmt::IndexAssign {
                    object: obj,
                    index: idx,
                    value: final_rhs,
                }])
            }
            CExpr::Member { object, field } => {
                let obj = self.convert_expr(object)?;
                Ok(vec![Stmt::FieldAssign {
                    object: obj,
                    field: field.clone(),
                    value: rhs,
                }])
            }
            CExpr::ArrowMember { pointer, field } => {
                let ptr = self.convert_expr(pointer)?;
                let derefed = Expr::Deref(Box::new(ptr));
                Ok(vec![Stmt::FieldAssign {
                    object: derefed,
                    field: field.clone(),
                    value: rhs,
                }])
            }
            CExpr::Deref(inner) => {
                // *ptr = val → DerefAssign
                let ptr = self.convert_expr(inner)?;
                Ok(vec![Stmt::DerefAssign {
                    pointer: ptr,
                    value: rhs,
                }])
            }
            _ => {
                // Fallback: convert as expression statement
                Ok(vec![Stmt::Expr(rhs)])
            }
        }
    }

    /// Convert printf format string to Print/Println statements
    fn convert_printf(&self, args: &[CExpr]) -> Result<Vec<Stmt>, String> {
        if args.is_empty() {
            return Ok(vec![]);
        }

        // First arg should be format string
        let fmt = match &args[0] {
            CExpr::StringLiteral(s) => s.clone(),
            other => {
                // Non-string first arg: just print it
                let val = self.convert_expr(other)?;
                return Ok(vec![Stmt::Print(val)]);
            }
        };

        let mut stmts = Vec::new();
        let mut arg_idx = 1;
        let mut current_str = String::new();
        let chars: Vec<char> = fmt.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '%' && i + 1 < chars.len() {
                // Flush current string
                if !current_str.is_empty() {
                    stmts.push(Stmt::Print(Expr::String(current_str.clone())));
                    current_str.clear();
                }

                i += 1;
                // Skip width/precision modifiers
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == '-' || chars[i] == 'l' || chars[i] == 'h') {
                    i += 1;
                }

                if i < chars.len() {
                    match chars[i] {
                        'd' | 'i' | 'u' | 'x' | 'X' | 'o' | 'f' | 'e' | 'g' | 'c' => {
                            if arg_idx < args.len() {
                                let val = self.convert_expr(&args[arg_idx])?;
                                stmts.push(Stmt::Print(val));
                                arg_idx += 1;
                            }
                        }
                        's' => {
                            if arg_idx < args.len() {
                                let val = self.convert_expr(&args[arg_idx])?;
                                stmts.push(Stmt::Print(val));
                                arg_idx += 1;
                            }
                        }
                        'p' => {
                            if arg_idx < args.len() {
                                let val = self.convert_expr(&args[arg_idx])?;
                                stmts.push(Stmt::Print(val));
                                arg_idx += 1;
                            }
                        }
                        '%' => {
                            current_str.push('%');
                        }
                        _ => {}
                    }
                    i += 1;
                }
            } else if chars[i] == '\n' {
                // Newline
                if !current_str.is_empty() {
                    stmts.push(Stmt::Println(Expr::String(current_str.clone())));
                    current_str.clear();
                } else {
                    stmts.push(Stmt::Println(Expr::String(String::new())));
                }
                i += 1;
            } else {
                current_str.push(chars[i]);
                i += 1;
            }
        }

        // Flush remaining string
        if !current_str.is_empty() {
            stmts.push(Stmt::Print(Expr::String(current_str)));
        }

        if stmts.is_empty() {
            stmts.push(Stmt::Print(Expr::String(fmt)));
        }

        Ok(stmts)
    }

    // ========== Expression conversion ==========

    fn convert_expr(&self, expr: &CExpr) -> Result<Expr, String> {
        match expr {
            CExpr::IntLiteral(n) => Ok(Expr::Number(*n)),
            CExpr::FloatLiteral(f) => Ok(Expr::Float(*f)),
            CExpr::StringLiteral(s) => Ok(Expr::String(s.clone())),
            CExpr::CharLiteral(c) => Ok(Expr::Number(*c as i64)),
            CExpr::Null => Ok(Expr::Nullptr),

            CExpr::Identifier(name) => {
                // Check if it's an enum constant
                if let Some((_, val)) = self.enum_constants.iter().find(|(n, _)| n == name) {
                    Ok(Expr::Number(*val))
                } else {
                    Ok(Expr::Variable(name.clone()))
                }
            }

            CExpr::BinaryOp { op, left, right } => {
                let l = self.convert_expr(left)?;
                let r = self.convert_expr(right)?;

                match op {
                    CBinOp::Add => Ok(Expr::BinaryOp { op: BinOp::Add, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Sub => Ok(Expr::BinaryOp { op: BinOp::Sub, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Mul => Ok(Expr::BinaryOp { op: BinOp::Mul, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Div => Ok(Expr::BinaryOp { op: BinOp::Div, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Mod => Ok(Expr::BinaryOp { op: BinOp::Mod, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Eq => Ok(Expr::Comparison { op: CmpOp::Eq, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Ne => Ok(Expr::Comparison { op: CmpOp::Ne, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Lt => Ok(Expr::Comparison { op: CmpOp::Lt, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Gt => Ok(Expr::Comparison { op: CmpOp::Gt, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Le => Ok(Expr::Comparison { op: CmpOp::Le, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Ge => Ok(Expr::Comparison { op: CmpOp::Ge, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::LogAnd => Ok(Expr::BinaryOp { op: BinOp::And, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::LogOr => Ok(Expr::BinaryOp { op: BinOp::Or, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::BitAnd => Ok(Expr::BitwiseOp { op: BitwiseOp::And, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::BitOr => Ok(Expr::BitwiseOp { op: BitwiseOp::Or, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::BitXor => Ok(Expr::BitwiseOp { op: BitwiseOp::Xor, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Shl => Ok(Expr::BitwiseOp { op: BitwiseOp::LeftShift, left: Box::new(l), right: Box::new(r) }),
                    CBinOp::Shr => Ok(Expr::BitwiseOp { op: BitwiseOp::RightShift, left: Box::new(l), right: Box::new(r) }),
                }
            }

            CExpr::UnaryOp { op, expr: inner, prefix } => {
                let e = self.convert_expr(inner)?;
                match op {
                    CUnaryOp::Neg => Ok(Expr::UnaryOp { op: UnaryOp::Neg, expr: Box::new(e) }),
                    CUnaryOp::LogNot => Ok(Expr::UnaryOp { op: UnaryOp::Not, expr: Box::new(e) }),
                    CUnaryOp::BitNot => Ok(Expr::BitwiseNot(Box::new(e))),
                    CUnaryOp::PreInc => Ok(Expr::PreIncrement(Box::new(e))),
                    CUnaryOp::PreDec => Ok(Expr::PreDecrement(Box::new(e))),
                    CUnaryOp::PostInc => Ok(Expr::PostIncrement(Box::new(e))),
                    CUnaryOp::PostDec => Ok(Expr::PostDecrement(Box::new(e))),
                }
            }

            CExpr::Call { func, args } => {
                let name = match func.as_ref() {
                    CExpr::Identifier(n) => n.clone(),
                    _ => {
                        // Function pointer call — use as expression
                        let f = self.convert_expr(func)?;
                        let a: Result<Vec<Expr>, String> = args.iter().map(|a| self.convert_expr(a)).collect();
                        return Ok(Expr::Call {
                            name: format!("{:?}", f),
                            args: a?,
                        });
                    }
                };

                // Map C stdlib functions to ADead-BIB
                match name.as_str() {
                    "malloc" => {
                        if let Some(size_arg) = args.first() {
                            let size = self.convert_expr(size_arg)?;
                            return Ok(Expr::Malloc(Box::new(size)));
                        }
                        Ok(Expr::Nullptr)
                    }
                    "sizeof" => {
                        if let Some(arg) = args.first() {
                            let e = self.convert_expr(arg)?;
                            return Ok(Expr::SizeOf(Box::new(ast::SizeOfArg::Expr(e))));
                        }
                        Ok(Expr::Number(0))
                    }
                    "realloc" => {
                        if args.len() >= 2 {
                            let ptr = self.convert_expr(&args[0])?;
                            let size = self.convert_expr(&args[1])?;
                            return Ok(Expr::Realloc {
                                ptr: Box::new(ptr),
                                new_size: Box::new(size),
                            });
                        }
                        Ok(Expr::Nullptr)
                    }
                    _ => {
                        let a: Result<Vec<Expr>, String> = args.iter().map(|a| self.convert_expr(a)).collect();
                        Ok(Expr::Call { name, args: a? })
                    }
                }
            }

            CExpr::Index { array, index } => {
                let obj = self.convert_expr(array)?;
                let idx = self.convert_expr(index)?;
                Ok(Expr::Index {
                    object: Box::new(obj),
                    index: Box::new(idx),
                })
            }

            CExpr::Member { object, field } => {
                let obj = self.convert_expr(object)?;
                Ok(Expr::FieldAccess {
                    object: Box::new(obj),
                    field: field.clone(),
                })
            }

            CExpr::ArrowMember { pointer, field } => {
                let ptr = self.convert_expr(pointer)?;
                Ok(Expr::ArrowAccess {
                    pointer: Box::new(ptr),
                    field: field.clone(),
                })
            }

            CExpr::Cast { target_type, expr: inner } => {
                let e = self.convert_expr(inner)?;
                let ty = self.convert_type(target_type);
                Ok(Expr::Cast { target_type: ty, expr: Box::new(e) })
            }

            CExpr::SizeofType(ty) => {
                let aty = self.convert_type(ty);
                Ok(Expr::SizeOf(Box::new(ast::SizeOfArg::Type(aty))))
            }
            CExpr::SizeofExpr(inner) => {
                let e = self.convert_expr(inner)?;
                Ok(Expr::SizeOf(Box::new(ast::SizeOfArg::Expr(e))))
            }

            CExpr::Ternary { condition, then_expr, else_expr } => {
                let c = self.convert_expr(condition)?;
                let t = self.convert_expr(then_expr)?;
                let e = self.convert_expr(else_expr)?;
                Ok(Expr::Ternary {
                    condition: Box::new(c),
                    then_expr: Box::new(t),
                    else_expr: Box::new(e),
                })
            }

            CExpr::AddressOf(inner) => {
                let e = self.convert_expr(inner)?;
                Ok(Expr::AddressOf(Box::new(e)))
            }

            CExpr::Deref(inner) => {
                let e = self.convert_expr(inner)?;
                Ok(Expr::Deref(Box::new(e)))
            }

            CExpr::Assign { op, target, value } => {
                // Assignment as expression — convert to the value side
                let rhs = self.convert_expr(value)?;
                match op {
                    CAssignOp::Assign => Ok(rhs),
                    _ => {
                        let lhs = self.convert_expr(target)?;
                        let bin_op = match op {
                            CAssignOp::AddAssign => BinOp::Add,
                            CAssignOp::SubAssign => BinOp::Sub,
                            CAssignOp::MulAssign => BinOp::Mul,
                            CAssignOp::DivAssign => BinOp::Div,
                            CAssignOp::ModAssign => BinOp::Mod,
                            _ => BinOp::Add, // fallback
                        };
                        Ok(Expr::BinaryOp {
                            op: bin_op,
                            left: Box::new(lhs),
                            right: Box::new(rhs),
                        })
                    }
                }
            }

            CExpr::Comma(exprs) => {
                // Comma expression → return last value
                if let Some(last) = exprs.last() {
                    self.convert_expr(last)
                } else {
                    Ok(Expr::Number(0))
                }
            }
        }
    }

    fn default_value(&self, cty: &CType) -> Expr {
        match cty {
            CType::Float | CType::Double => Expr::Float(0.0),
            CType::Pointer(_) => Expr::Nullptr,
            CType::Bool => Expr::Bool(false),
            CType::Char => Expr::Number(0),
            _ => Expr::Number(0),
        }
    }
}

/// Convenience: parse C source → ADead-BIB Program in one call
pub fn compile_c_to_program(source: &str) -> Result<Program, String> {
    use super::c_lexer::CLexer;
    use super::c_parser::CParser;

    let tokens = CLexer::new(source).tokenize();
    let unit = CParser::new(tokens).parse_translation_unit()?;
    let mut converter = CToIR::new();
    converter.convert(&unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let program = compile_c_to_program(r#"
            #include <stdio.h>
            int main() {
                printf("Hello from C!\n");
                return 0;
            }
        "#).unwrap();

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "main");
        assert!(!program.functions[0].body.is_empty());
    }

    #[test]
    fn test_variables_and_math() {
        let program = compile_c_to_program(r#"
            int add(int a, int b) {
                return a + b;
            }
            int main() {
                int x = 10;
                int y = 20;
                int z = add(x, y);
                return z;
            }
        "#).unwrap();

        assert_eq!(program.functions.len(), 2);
        assert_eq!(program.functions[0].name, "add");
        assert_eq!(program.functions[1].name, "main");
    }

    #[test]
    fn test_control_flow() {
        let program = compile_c_to_program(r#"
            int main() {
                int sum = 0;
                for (int i = 0; i < 10; i++) {
                    sum += i;
                }
                if (sum > 30) {
                    return 1;
                } else {
                    return 0;
                }
            }
        "#).unwrap();

        assert_eq!(program.functions.len(), 1);
    }

    #[test]
    fn test_struct() {
        let program = compile_c_to_program(r#"
            struct Point {
                int x;
                int y;
            };
            int main() {
                return 0;
            }
        "#).unwrap();

        assert_eq!(program.structs.len(), 1);
        assert_eq!(program.structs[0].name, "Point");
        assert_eq!(program.structs[0].fields.len(), 2);
    }
}
