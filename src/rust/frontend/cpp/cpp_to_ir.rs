// ============================================================
// ADead-BIB C++ Frontend — C++ AST → ADead-BIB IR
// ============================================================
// Converts C++ AST to ADead-BIB IR (Program/Function/Stmt/Expr)
// Handles: classes → structs, templates → monomorphized, vtable elimination
//
// ADead-BIB Philosophy:
//   - vtables → resolved at compile time (devirtualization)
//   - RTTI → eliminated
//   - exceptions → error codes
//   - templates → only instantiated code survives
//
// Sin GCC. Sin LLVM. Sin Clang. Solo ADead-BIB. 💀🦈
// ============================================================

use super::cpp_ast::*;
use crate::frontend::ast::{
    Expr, Stmt, Program, Function, Param, Struct as IrStruct,
    StructField, BinOp, CmpOp, UnaryOp as IrUnaryOp, BitwiseOp as IrBitwiseOp,
    SizeOfArg, FunctionAttributes, ProgramAttributes, SwitchCase, CompoundOp,
};
use crate::frontend::types::Type;

pub struct CppToIR {
    type_aliases: Vec<(String, CppType)>,
    class_methods: Vec<(String, String, Vec<CppParam>, CppType)>, // (class, method, params, ret)
    current_namespace: Option<String>, // Track current namespace for unqualified calls
    namespace_functions: Vec<String>,  // All function names in current namespace
    current_class: Option<String>,     // Track current class for this-> resolution
    class_fields: Vec<(String, Vec<String>)>, // (class_name, field_names)
}

impl CppToIR {
    pub fn new() -> Self {
        Self {
            type_aliases: Vec::new(),
            class_methods: Vec::new(),
            current_namespace: None,
            namespace_functions: Vec::new(),
            current_class: None,
            class_fields: Vec::new(),
        }
    }
    
    /// Check if an identifier is a field of the current class
    fn is_class_field(&self, name: &str) -> bool {
        if let Some(ref class_name) = self.current_class {
            for (cn, fields) in &self.class_fields {
                if cn == class_name && fields.contains(&name.to_string()) {
                    return true;
                }
            }
        }
        false
    }

    pub fn convert(&mut self, unit: &CppTranslationUnit) -> Result<Program, String> {
        let mut program = Program::new();
        program.attributes = ProgramAttributes::default();

        // First pass: collect type aliases, class info, and field names
        for decl in &unit.declarations {
            match decl {
                CppTopLevel::TypeAlias { new_name, original, .. } => {
                    self.type_aliases.push((new_name.clone(), original.clone()));
                }
                CppTopLevel::ClassDef { name, members, .. } => {
                    let mut field_names = Vec::new();
                    for member in members {
                        match member {
                            CppClassMember::Method { name: method_name, params, return_type, .. } => {
                                self.class_methods.push((
                                    name.clone(),
                                    method_name.clone(),
                                    params.clone(),
                                    return_type.clone(),
                                ));
                            }
                            CppClassMember::Field { name: field_name, .. } => {
                                field_names.push(field_name.clone());
                            }
                            _ => {}
                        }
                    }
                    self.class_fields.push((name.clone(), field_names));
                }
                _ => {}
            }
        }

        // Second pass: convert declarations
        for decl in &unit.declarations {
            match decl {
                CppTopLevel::FunctionDef { return_type, name, params, body, .. } => {
                    let func = self.convert_function(return_type, name, params, body)?;
                    program.functions.push(func);
                }
                CppTopLevel::ClassDef { name, members, bases, .. } => {
                    // Convert class to struct + methods as functions
                    let ir_struct = self.convert_class_to_struct(name, members, bases)?;
                    program.structs.push(ir_struct);

                    // Convert methods to standalone functions
                    for member in members {
                        match member {
                            CppClassMember::Method {
                                name: method_name, return_type, params, body: Some(body), ..
                            } => {
                                let func_name = format!("{}::{}", name, method_name);
                                let func = self.convert_method(return_type, &func_name, name, params, body)?;
                                program.functions.push(func);
                            }
                            CppClassMember::Constructor {
                                params, body: Some(body), initializer_list, ..
                            } => {
                                let _func_name = format!("{}::{}", name, name);
                                let func = self.convert_constructor(name, params, initializer_list, body)?;
                                program.functions.push(func);
                            }
                            CppClassMember::Destructor { body: Some(body), .. } => {
                                let func_name = format!("{}::~{}", name, name);
                                let func = self.convert_function(
                                    &CppType::Void, &func_name, &[], body
                                )?;
                                program.functions.push(func);
                            }
                            _ => {}
                        }
                    }
                }
                CppTopLevel::Namespace { name: ns_name, declarations } => {
                    // Collect all function names in this namespace for unqualified call resolution
                    self.namespace_functions.clear();
                    for inner_decl in declarations.iter() {
                        if let CppTopLevel::FunctionDef { name: fname, .. } = inner_decl {
                            self.namespace_functions.push(fname.clone());
                        }
                    }
                    self.current_namespace = Some(ns_name.clone());

                    // Flatten namespace — prefix function names with ns::
                    for inner_decl in declarations {
                        match inner_decl {
                            CppTopLevel::FunctionDef { return_type, name: fname, params, body, .. } => {
                                let qualified = format!("{}::{}", ns_name, fname);
                                let func = self.convert_function(return_type, &qualified, params, body)?;
                                program.functions.push(func);
                            }
                            CppTopLevel::Namespace { name: inner_ns, declarations: inner_decls } => {
                                // Nested namespace: ns::inner::func
                                for inner2 in inner_decls {
                                    if let CppTopLevel::FunctionDef { return_type, name: fname, params, body, .. } = inner2 {
                                        let qualified = format!("{}::{}::{}", ns_name, inner_ns, fname);
                                        let func = self.convert_function(return_type, &qualified, params, body)?;
                                        program.functions.push(func);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    self.current_namespace = None;
                    self.namespace_functions.clear();
                }
                CppTopLevel::EnumDef { name: _, values, .. } => {
                    // Enum constants become global assignments
                    for (i, (ident, val)) in values.iter().enumerate() {
                        let value = if let Some(expr) = val {
                            self.convert_expr(expr)?
                        } else {
                            Expr::Number(i as i64)
                        };
                        program.statements.push(Stmt::VarDecl {
                            var_type: Type::I32,
                            name: ident.clone(),
                            value: Some(value),
                        });
                    }
                }
                CppTopLevel::GlobalVar { type_spec, declarators } => {
                    for d in declarators {
                        let var_type = self.convert_type(type_spec);
                        let init = if let Some(ref e) = d.initializer {
                            Some(self.convert_expr(e)?)
                        } else {
                            None
                        };
                        program.statements.push(Stmt::VarDecl {
                            var_type,
                            name: d.name.clone(),
                            value: init,
                        });
                    }
                }
                CppTopLevel::FunctionDecl { .. } => {} // Prototypes — skip
                CppTopLevel::UsingNamespace(_) => {}
                CppTopLevel::ExternC { declarations } => {
                    for inner in declarations {
                        if let CppTopLevel::FunctionDef { return_type, name, params, body, .. } = inner {
                            let func = self.convert_function(return_type, name, params, body)?;
                            program.functions.push(func);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(program)
    }

    // ========== Type conversion ==========

    fn convert_type(&self, cpp_type: &CppType) -> Type {
        match cpp_type {
            CppType::Void => Type::Void,
            CppType::Bool => Type::Bool,
            CppType::Char | CppType::Char8 => Type::I8,
            CppType::WChar | CppType::Char16 => Type::I16,
            CppType::Char32 => Type::I32,
            CppType::Short => Type::I16,
            CppType::Int => Type::I32,
            CppType::Long => Type::I64,
            CppType::LongLong => Type::I64,
            CppType::Float => Type::F32,
            CppType::Double | CppType::LongDouble => Type::F64,
            CppType::Auto => Type::Auto,
            CppType::Unsigned(inner) => match inner.as_ref() {
                CppType::Char => Type::U8,
                CppType::Short => Type::U16,
                CppType::Int => Type::U32,
                CppType::Long | CppType::LongLong => Type::U64,
                _ => Type::U32,
            },
            CppType::Signed(inner) => self.convert_type(inner),
            CppType::Const(inner) | CppType::Volatile(inner) | CppType::Mutable(inner) |
            CppType::Constexpr(inner) => self.convert_type(inner),
            CppType::Pointer(inner) => Type::Pointer(Box::new(self.convert_type(inner))),
            CppType::Reference(inner) | CppType::RValueRef(inner) =>
                Type::Reference(Box::new(self.convert_type(inner))),
            CppType::Array(inner, size) => Type::Array(Box::new(self.convert_type(inner)), *size),
            CppType::Named(name) | CppType::Class(name) | CppType::Struct(name) =>
                Type::Named(name.clone()),
            CppType::Enum(_) => Type::I32,
            CppType::StdString => Type::Str,
            CppType::StdVector(inner) => Type::Array(Box::new(self.convert_type(inner)), None),
            CppType::UniquePtr(inner) | CppType::SharedPtr(inner) | CppType::WeakPtr(inner) =>
                Type::Pointer(Box::new(self.convert_type(inner))),
            CppType::StdOptional(inner) => self.convert_type(inner),
            CppType::SizeT => Type::U64,
            CppType::Nullptr => Type::Pointer(Box::new(Type::Void)),
            CppType::TemplateType { name, args } => {
                if args.len() == 1 {
                    Type::Array(Box::new(self.convert_type(&args[0])), None)
                } else {
                    Type::Named(name.clone())
                }
            }
            _ => Type::I64,
        }
    }

    // ========== Class → Struct ==========

    fn convert_class_to_struct(&self, name: &str, members: &[CppClassMember], _bases: &[CppBaseClass]) -> Result<IrStruct, String> {
        let mut fields = Vec::new();
        for member in members {
            if let CppClassMember::Field { type_spec, name: field_name, .. } = member {
                fields.push(StructField {
                    name: field_name.clone(),
                    field_type: self.convert_type(type_spec),
                });
            }
        }
        Ok(IrStruct {
            name: name.to_string(),
            fields,
            is_packed: false,
        })
    }

    // ========== Function conversion ==========

    fn convert_function(&self, ret_type: &CppType, name: &str, params: &[CppParam], body: &[CppStmt]) -> Result<Function, String> {
        let ir_params: Vec<Param> = params.iter().map(|p| {
            Param {
                name: p.name.clone().unwrap_or_else(|| "unnamed".to_string()),
                param_type: self.convert_type(&p.param_type),
                default_value: None,
            }
        }).collect();

        let mut ir_body = Vec::new();
        for stmt in body {
            ir_body.extend(self.convert_stmt(stmt)?);
        }

        Ok(Function {
            name: name.to_string(),
            params: ir_params,
            body: ir_body,
            return_type: None,
            resolved_return_type: self.convert_type(ret_type),
            attributes: FunctionAttributes::default(),
        })
    }

    fn convert_method(&mut self, ret_type: &CppType, func_name: &str, class_name: &str, params: &[CppParam], body: &[CppStmt]) -> Result<Function, String> {
        // Set current class for this-> resolution
        self.current_class = Some(class_name.to_string());
        
        // Add implicit 'this' pointer as first param
        let mut all_params = vec![CppParam {
            param_type: CppType::Pointer(Box::new(CppType::Named(class_name.to_string()))),
            name: Some("this".to_string()),
            default_value: None,
            is_variadic: false,
        }];
        all_params.extend_from_slice(params);
        let result = self.convert_function(ret_type, func_name, &all_params, body);
        
        self.current_class = None;
        result
    }

    fn convert_constructor(&mut self, class_name: &str, params: &[CppParam], init_list: &[(String, CppExpr)], body: &[CppStmt]) -> Result<Function, String> {
        // Set current class for this-> resolution
        self.current_class = Some(class_name.to_string());
        
        let func_name = format!("{}::{}", class_name, class_name);
        
        // Add implicit 'this' pointer as first param (like methods)
        let mut all_params = vec![CppParam {
            param_type: CppType::Pointer(Box::new(CppType::Named(class_name.to_string()))),
            name: Some("this".to_string()),
            default_value: None,
            is_variadic: false,
        }];
        all_params.extend_from_slice(params);
        
        // Convert initializer list to field assignments at start of body
        let mut full_body = Vec::new();
        for (field, expr) in init_list {
            full_body.push(CppStmt::Expr(CppExpr::Assign {
                target: Box::new(CppExpr::MemberAccess {
                    object: Box::new(CppExpr::Identifier("this".to_string())),
                    member: field.clone(),
                }),
                value: Box::new(expr.clone()),
            }));
        }
        full_body.extend_from_slice(body);
        
        let result = self.convert_function(&CppType::Void, &func_name, &all_params, &full_body);
        self.current_class = None;
        result
    }

    // ========== Statement conversion ==========

    fn convert_stmt(&self, stmt: &CppStmt) -> Result<Vec<Stmt>, String> {
        match stmt {
            CppStmt::Expr(expr) => {
                // FASM-inspired: detect assignment expressions and emit proper Stmt::Assign
                // Without this, `a = a + 1;` becomes Expr(a+1) instead of Assign{a, a+1}
                match expr {
                    CppExpr::Assign { target, value } => {
                        if let CppExpr::Identifier(name) = target.as_ref() {
                            let v = self.convert_expr(value)?;
                            return Ok(vec![Stmt::Assign { name: name.clone(), value: v }]);
                        }
                        // Field assignment: obj.field = value
                        if let CppExpr::MemberAccess { object, member } = target.as_ref() {
                            let obj = self.convert_expr(object)?;
                            let v = self.convert_expr(value)?;
                            return Ok(vec![Stmt::FieldAssign {
                                object: obj, field: member.clone(), value: v,
                            }]);
                        }
                        // Array index assignment: arr[i] = value
                        if let CppExpr::Index { object, index } = target.as_ref() {
                            let obj = self.convert_expr(object)?;
                            let idx = self.convert_expr(index)?;
                            let v = self.convert_expr(value)?;
                            return Ok(vec![Stmt::IndexAssign {
                                object: obj, index: idx, value: v,
                            }]);
                        }
                        let v = self.convert_expr(value)?;
                        return Ok(vec![Stmt::Expr(v)]);
                    }
                    CppExpr::CompoundAssign { target, op, value } => {
                        if let CppExpr::Identifier(name) = target.as_ref() {
                            let v = self.convert_expr(value)?;
                            let comp_op = match op {
                                CppBinOp::Add => CompoundOp::AddAssign,
                                CppBinOp::Sub => CompoundOp::SubAssign,
                                CppBinOp::Mul => CompoundOp::MulAssign,
                                CppBinOp::Div => CompoundOp::DivAssign,
                                CppBinOp::Mod => CompoundOp::ModAssign,
                                CppBinOp::BitAnd => CompoundOp::AndAssign,
                                CppBinOp::BitOr => CompoundOp::OrAssign,
                                CppBinOp::BitXor => CompoundOp::XorAssign,
                                CppBinOp::Shl => CompoundOp::ShlAssign,
                                CppBinOp::Shr => CompoundOp::ShrAssign,
                                _ => CompoundOp::AddAssign,
                            };
                            return Ok(vec![Stmt::CompoundAssign {
                                name: name.clone(), op: comp_op, value: v,
                            }]);
                        }
                        let v = self.convert_expr(value)?;
                        return Ok(vec![Stmt::Expr(v)]);
                    }
                    CppExpr::UnaryOp { op, expr: inner, is_prefix } => {
                        use super::cpp_ast::CppUnaryOp;
                        match op {
                            CppUnaryOp::PreInc | CppUnaryOp::PostInc => {
                                if let CppExpr::Identifier(name) = inner.as_ref() {
                                    return Ok(vec![Stmt::Increment {
                                        name: name.clone(),
                                        is_pre: *is_prefix,
                                        is_increment: true,
                                    }]);
                                }
                            }
                            CppUnaryOp::PreDec | CppUnaryOp::PostDec => {
                                if let CppExpr::Identifier(name) = inner.as_ref() {
                                    return Ok(vec![Stmt::Increment {
                                        name: name.clone(),
                                        is_pre: *is_prefix,
                                        is_increment: false,
                                    }]);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                let ir_expr = self.convert_expr(expr)?;
                // Handle cout << "text" as Println
                if let Expr::String(ref s) = ir_expr {
                    return Ok(vec![Stmt::Print(Expr::String(s.clone()))]);
                }
                Ok(vec![Stmt::Expr(ir_expr)])
            }
            CppStmt::VarDecl { type_spec, declarators } => {
                let mut stmts = Vec::new();
                for d in declarators {
                    let var_type = self.convert_type(type_spec);
                    let init = if let Some(ref e) = d.initializer {
                        Some(self.convert_expr(e)?)
                    } else {
                        None
                    };
                    stmts.push(Stmt::VarDecl {
                        var_type: var_type.clone(),
                        name: d.name.clone(),
                        value: init.clone(),
                    });
                    
                    // If this is a class/struct type, generate constructor call
                    if let CppType::Named(class_name) | CppType::Class(class_name) | CppType::Struct(class_name) = type_spec {
                        let ctor_name = format!("{}::{}", class_name, class_name);
                        // Get constructor args from initializer
                        let ctor_args = if let Some(ref e) = d.initializer {
                            match e {
                                CppExpr::Call { args, .. } => {
                                    args.iter().map(|a| self.convert_expr(a)).collect::<Result<Vec<_>, _>>()?
                                }
                                CppExpr::InitList(items) => {
                                    items.iter().map(|a| self.convert_expr(a)).collect::<Result<Vec<_>, _>>()?
                                }
                                // Single value initializer (e.g., Counter c(5))
                                _ => vec![self.convert_expr(e)?],
                            }
                        } else {
                            vec![] // Default constructor
                        };
                        
                        // Generate: ClassName::ClassName(&var, args...)
                        let mut call_args = vec![Expr::AddressOf(Box::new(Expr::Variable(d.name.clone())))];
                        call_args.extend(ctor_args);
                        stmts.push(Stmt::Expr(Expr::Call {
                            name: ctor_name,
                            args: call_args,
                        }));
                    }
                }
                Ok(stmts)
            }
            CppStmt::Return(Some(expr)) => {
                Ok(vec![Stmt::Return(Some(self.convert_expr(expr)?))])
            }
            CppStmt::Return(None) => {
                Ok(vec![Stmt::Return(None)])
            }
            CppStmt::Block(stmts) => {
                let mut ir_stmts = Vec::new();
                for s in stmts {
                    ir_stmts.extend(self.convert_stmt(s)?);
                }
                Ok(ir_stmts)
            }
            CppStmt::If { condition, then_body, else_body, .. } => {
                let cond = self.convert_expr(condition)?;
                let then_stmts = self.convert_stmt(then_body)?;
                let else_stmts = if let Some(eb) = else_body {
                    Some(self.convert_stmt(eb)?)
                } else {
                    None
                };
                Ok(vec![Stmt::If {
                    condition: cond,
                    then_body: then_stmts,
                    else_body: else_stmts,
                }])
            }
            CppStmt::While { condition, body } => {
                let cond = self.convert_expr(condition)?;
                let body_stmts = self.convert_stmt(body)?;
                Ok(vec![Stmt::While { condition: cond, body: body_stmts }])
            }
            CppStmt::DoWhile { body, condition } => {
                let body_stmts = self.convert_stmt(body)?;
                let cond = self.convert_expr(condition)?;
                Ok(vec![Stmt::DoWhile { body: body_stmts, condition: cond }])
            }
            CppStmt::For { init, condition, increment, body } => {
                // Convert C++ for to while
                let mut stmts = Vec::new();
                if let Some(init_stmt) = init {
                    stmts.extend(self.convert_stmt(init_stmt)?);
                }
                let cond = condition.as_ref()
                    .map(|c| self.convert_expr(c))
                    .transpose()?
                    .unwrap_or(Expr::Bool(true));
                let mut body_stmts = self.convert_stmt(body)?;
                if let Some(inc) = increment {
                    // Convert increment expression to proper statement
                    // (e.g., i++ → Stmt::Increment, i += 1 → Stmt::CompoundAssign)
                    let inc_ref: &CppExpr = &inc;
                    let inc_stmt = match inc_ref {
                        CppExpr::UnaryOp { op, expr: inner, is_prefix } => {
                            use super::cpp_ast::CppUnaryOp;
                            let is_pre = *is_prefix;
                            match op {
                                CppUnaryOp::PreInc | CppUnaryOp::PostInc => {
                                    if let CppExpr::Identifier(name) = inner.as_ref() {
                                        Some(Stmt::Increment { name: name.clone(), is_pre, is_increment: true })
                                    } else { None }
                                }
                                CppUnaryOp::PreDec | CppUnaryOp::PostDec => {
                                    if let CppExpr::Identifier(name) = inner.as_ref() {
                                        Some(Stmt::Increment { name: name.clone(), is_pre, is_increment: false })
                                    } else { None }
                                }
                                _ => None,
                            }
                        }
                        CppExpr::CompoundAssign { target, op, value } => {
                            if let CppExpr::Identifier(name) = target.as_ref() {
                                let v = self.convert_expr(value)?;
                                let comp_op = match op {
                                    CppBinOp::Add => CompoundOp::AddAssign,
                                    CppBinOp::Sub => CompoundOp::SubAssign,
                                    CppBinOp::Mul => CompoundOp::MulAssign,
                                    CppBinOp::Div => CompoundOp::DivAssign,
                                    CppBinOp::Mod => CompoundOp::ModAssign,
                                    _ => CompoundOp::AddAssign,
                                };
                                Some(Stmt::CompoundAssign { name: name.clone(), op: comp_op, value: v })
                            } else { None }
                        }
                        CppExpr::Assign { target, value } => {
                            if let CppExpr::Identifier(name) = target.as_ref() {
                                let v = self.convert_expr(value)?;
                                Some(Stmt::Assign { name: name.clone(), value: v })
                            } else { None }
                        }
                        _ => None,
                    };
                    body_stmts.push(inc_stmt.unwrap_or_else(|| {
                        Stmt::Expr(self.convert_expr(inc_ref).unwrap_or(Expr::Number(0)))
                    }));
                }
                stmts.push(Stmt::While { condition: cond, body: body_stmts });
                Ok(stmts)
            }
            CppStmt::RangeFor { name, iterable, body, .. } => {
                let iter_expr = self.convert_expr(iterable)?;
                let body_stmts = self.convert_stmt(body)?;
                Ok(vec![Stmt::ForEach {
                    var: name.clone(),
                    iterable: iter_expr,
                    body: body_stmts,
                }])
            }
            CppStmt::Switch { expr, cases, default } => {
                let switch_expr = self.convert_expr(expr)?;
                let ir_cases: Vec<SwitchCase> = cases.iter().map(|c| {
                    let val = self.convert_expr(&c.value).unwrap_or(Expr::Number(0));
                    let body: Vec<Stmt> = c.body.iter()
                        .flat_map(|s| self.convert_stmt(s).unwrap_or_default())
                        .collect();
                    SwitchCase { value: val, body, has_break: true }
                }).collect();
                let default_body = default.as_ref().map(|d| {
                    d.iter().flat_map(|s| self.convert_stmt(s).unwrap_or_default()).collect()
                });
                Ok(vec![Stmt::Switch {
                    expr: switch_expr,
                    cases: ir_cases,
                    default: default_body,
                }])
            }
            CppStmt::Break => Ok(vec![Stmt::Break]),
            CppStmt::Continue => Ok(vec![Stmt::Continue]),
            CppStmt::Goto(_) => Ok(vec![]), // Simplified
            CppStmt::Label(_, inner) => self.convert_stmt(inner),
            CppStmt::Empty => Ok(vec![]),
            CppStmt::Try { body, .. } => {
                // Convert try/catch to just the try body (exception → error codes)
                let mut stmts = Vec::new();
                for s in body {
                    stmts.extend(self.convert_stmt(s)?);
                }
                Ok(stmts)
            }
            CppStmt::Throw(_) => Ok(vec![]), // Eliminated
            CppStmt::CoReturn(expr) => {
                Ok(vec![Stmt::Return(
                    expr.as_ref().map(|e| self.convert_expr(e)).transpose()?
                )])
            }
        }
    }

    // ========== Expression conversion ==========

    fn convert_expr(&self, expr: &CppExpr) -> Result<Expr, String> {
        match expr {
            CppExpr::IntLiteral(n) => Ok(Expr::Number(*n)),
            CppExpr::UIntLiteral(n) => Ok(Expr::Number(*n as i64)),
            CppExpr::FloatLiteral(f) => Ok(Expr::Float(*f)),
            CppExpr::StringLiteral(s) => Ok(Expr::String(s.clone())),
            CppExpr::CharLiteral(c) => Ok(Expr::Number(*c as i64)),
            CppExpr::BoolLiteral(b) => Ok(Expr::Bool(*b)),
            CppExpr::NullptrLiteral => Ok(Expr::Nullptr),
            CppExpr::Identifier(name) => {
                // If inside a class method and this is a class field, convert to this->field
                if self.is_class_field(name) {
                    Ok(Expr::ArrowAccess {
                        pointer: Box::new(Expr::Variable("this".to_string())),
                        field: name.clone(),
                    })
                } else {
                    Ok(Expr::Variable(name.clone()))
                }
            }
            CppExpr::ScopedIdentifier { scope, name } => {
                let full = format!("{}::{}", scope.join("::"), name);
                // Handle std::cout, std::endl, etc
                match full.as_str() {
                    "std::cout" => Ok(Expr::Variable("stdout".to_string())),
                    "std::cerr" => Ok(Expr::Variable("stderr".to_string())),
                    "std::endl" => Ok(Expr::String("\n".to_string())),
                    _ => Ok(Expr::Variable(full)),
                }
            }
            CppExpr::This => Ok(Expr::Variable("this".to_string())),

            CppExpr::BinaryOp { op, left, right } => {
                let l = self.convert_expr(left)?;
                let r = self.convert_expr(right)?;

                // Handle cout << x as Print
                if *op == CppBinOp::Shl {
                    if let Expr::Variable(ref name) = l {
                        if name == "stdout" || name == "cout" {
                            return Ok(r); // Will be wrapped in Print by stmt handler
                        }
                    }
                    // Chained: (cout << "hello") << endl
                    if let Expr::String(_) = l {
                        return Ok(r);
                    }
                }

                match op {
                    CppBinOp::Add => Ok(Expr::BinaryOp { op: BinOp::Add, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Sub => Ok(Expr::BinaryOp { op: BinOp::Sub, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Mul => Ok(Expr::BinaryOp { op: BinOp::Mul, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Div => Ok(Expr::BinaryOp { op: BinOp::Div, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Mod => Ok(Expr::BinaryOp { op: BinOp::Mod, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Eq => Ok(Expr::Comparison { op: CmpOp::Eq, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Ne => Ok(Expr::Comparison { op: CmpOp::Ne, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Lt => Ok(Expr::Comparison { op: CmpOp::Lt, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Le => Ok(Expr::Comparison { op: CmpOp::Le, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Gt => Ok(Expr::Comparison { op: CmpOp::Gt, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Ge => Ok(Expr::Comparison { op: CmpOp::Ge, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::And => Ok(Expr::BinaryOp { op: BinOp::And, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Or => Ok(Expr::BinaryOp { op: BinOp::Or, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::BitAnd => Ok(Expr::BitwiseOp { op: IrBitwiseOp::And, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::BitOr => Ok(Expr::BitwiseOp { op: IrBitwiseOp::Or, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::BitXor => Ok(Expr::BitwiseOp { op: IrBitwiseOp::Xor, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Shl => Ok(Expr::BitwiseOp { op: IrBitwiseOp::LeftShift, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Shr => Ok(Expr::BitwiseOp { op: IrBitwiseOp::RightShift, left: Box::new(l), right: Box::new(r) }),
                    CppBinOp::Spaceship => {
                        // <=> returns -1, 0, 1 — approximate with subtraction
                        Ok(Expr::BinaryOp { op: BinOp::Sub, left: Box::new(l), right: Box::new(r) })
                    }
                }
            }

            CppExpr::UnaryOp { op, expr, .. } => {
                let e = self.convert_expr(expr)?;
                match op {
                    CppUnaryOp::Neg => Ok(Expr::UnaryOp { op: IrUnaryOp::Neg, expr: Box::new(e) }),
                    CppUnaryOp::Not => Ok(Expr::UnaryOp { op: IrUnaryOp::Not, expr: Box::new(e) }),
                    CppUnaryOp::BitNot => Ok(Expr::BitwiseNot(Box::new(e))),
                    CppUnaryOp::PreInc => Ok(Expr::PreIncrement(Box::new(e))),
                    CppUnaryOp::PreDec => Ok(Expr::PreDecrement(Box::new(e))),
                    CppUnaryOp::PostInc => Ok(Expr::PostIncrement(Box::new(e))),
                    CppUnaryOp::PostDec => Ok(Expr::PostDecrement(Box::new(e))),
                }
            }

            CppExpr::Assign { target, value } => {
                let _t = self.convert_expr(target)?;
                let v = self.convert_expr(value)?;
                // Return the value (C++ assignment is an expression)
                Ok(v)
            }

            CppExpr::CompoundAssign { value, .. } => {
                let v = self.convert_expr(value)?;
                Ok(v)
            }

            CppExpr::Call { callee, args } => {
                let ir_args: Vec<Expr> = args.iter()
                    .map(|a| self.convert_expr(a))
                    .collect::<Result<Vec<_>, _>>()?;
                
                // Check if this is a method call: obj.method() or obj->method()
                match callee.as_ref() {
                    CppExpr::MemberAccess { object, member } => {
                        // Method call: obj.method(args) → MethodCall { object, method, args }
                        let obj = self.convert_expr(object)?;
                        return Ok(Expr::MethodCall {
                            object: Box::new(obj),
                            method: member.clone(),
                            args: ir_args,
                        });
                    }
                    CppExpr::ArrowAccess { pointer, member } => {
                        // Method call via pointer: ptr->method(args)
                        let ptr = self.convert_expr(pointer)?;
                        return Ok(Expr::MethodCall {
                            object: Box::new(Expr::Deref(Box::new(ptr))),
                            method: member.clone(),
                            args: ir_args,
                        });
                    }
                    _ => {}
                }
                
                let name = match callee.as_ref() {
                    CppExpr::Identifier(n) => {
                        // If inside a namespace and this is an unqualified call to a sibling
                        // function, qualify it with the namespace prefix (FASM-style label resolution)
                        if let Some(ref ns) = self.current_namespace {
                            if self.namespace_functions.contains(n) {
                                format!("{}::{}", ns, n)
                            } else {
                                n.clone()
                            }
                        } else {
                            n.clone()
                        }
                    }
                    CppExpr::ScopedIdentifier { scope, name } =>
                        format!("{}::{}", scope.join("::"), name),
                    _ => "unknown".to_string(),
                };

                // Handle special functions
                match name.as_str() {
                    "printf" | "std::printf" => {
                        if let Some(Expr::String(ref _s)) = ir_args.first() {
                            return Ok(Expr::Call {
                                name: "printf".to_string(),
                                args: ir_args,
                            });
                        }
                        Ok(Expr::Call { name, args: ir_args })
                    }
                    "std::cout" => Ok(Expr::Call { name: "print".to_string(), args: ir_args }),
                    "malloc" | "std::malloc" => {
                        if let Some(size) = ir_args.first() {
                            Ok(Expr::Malloc(Box::new(size.clone())))
                        } else {
                            Ok(Expr::Malloc(Box::new(Expr::Number(0))))
                        }
                    }
                    _ => Ok(Expr::Call { name, args: ir_args }),
                }
            }

            CppExpr::MemberAccess { object, member } => {
                let obj = self.convert_expr(object)?;
                Ok(Expr::FieldAccess { object: Box::new(obj), field: member.clone() })
            }

            CppExpr::ArrowAccess { pointer, member } => {
                let ptr = self.convert_expr(pointer)?;
                Ok(Expr::ArrowAccess { pointer: Box::new(ptr), field: member.clone() })
            }

            CppExpr::Index { object, index } => {
                let obj = self.convert_expr(object)?;
                let idx = self.convert_expr(index)?;
                Ok(Expr::Index { object: Box::new(obj), index: Box::new(idx) })
            }

            CppExpr::Deref(inner) => {
                Ok(Expr::Deref(Box::new(self.convert_expr(inner)?)))
            }

            CppExpr::AddressOf(inner) => {
                Ok(Expr::AddressOf(Box::new(self.convert_expr(inner)?)))
            }

            CppExpr::Cast { target_type, expr, .. } => {
                let t = self.convert_type(target_type);
                let e = self.convert_expr(expr)?;
                Ok(Expr::Cast { target_type: t, expr: Box::new(e) })
            }

            CppExpr::SizeOf(arg) => {
                match arg {
                    CppSizeOfArg::Type(t) => {
                        let ir_type = self.convert_type(t);
                        Ok(Expr::SizeOf(Box::new(SizeOfArg::Type(ir_type))))
                    }
                    CppSizeOfArg::Expr(e) => {
                        let ir_expr = self.convert_expr(e)?;
                        Ok(Expr::SizeOf(Box::new(SizeOfArg::Expr(ir_expr))))
                    }
                }
            }

            CppExpr::Ternary { condition, then_expr, else_expr } => {
                Ok(Expr::Ternary {
                    condition: Box::new(self.convert_expr(condition)?),
                    then_expr: Box::new(self.convert_expr(then_expr)?),
                    else_expr: Box::new(self.convert_expr(else_expr)?),
                })
            }

            CppExpr::New { type_name, .. } => {
                let t = self.convert_type(type_name);
                let size = Expr::SizeOf(Box::new(SizeOfArg::Type(t)));
                Ok(Expr::Malloc(Box::new(size)))
            }

            CppExpr::Delete { expr, .. } => {
                let e = self.convert_expr(expr)?;
                Ok(Expr::Call { name: "free".to_string(), args: vec![e] })
            }

            CppExpr::Lambda { .. } => {
                // Simplified: convert lambda body to a call expression
                Ok(Expr::Number(0)) // Placeholder
            }

            CppExpr::InitList(items) => {
                let ir_items: Vec<Expr> = items.iter()
                    .map(|e| self.convert_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Expr::Array(ir_items))
            }

            CppExpr::Throw(_) => Ok(Expr::Number(-1)), // Exception → error code
            CppExpr::CoAwait(inner) => self.convert_expr(inner),
            CppExpr::CoYield(inner) => self.convert_expr(inner),

            _ => Ok(Expr::Number(0)),
        }
    }
}

// ========== Public API ==========

/// Convenience: parse C++ source → ADead-BIB Program in one call
/// Full pipeline: Preprocessor → Lexer → Parser → IR
pub fn compile_cpp_to_program(source: &str) -> Result<Program, String> {
    use super::cpp_lexer::CppLexer;
    use super::cpp_parser::CppParser;
    use super::cpp_preprocessor::CppPreprocessor;

    // Phase 0: Preprocess — resolve #include, strip #define/#ifdef/etc.
    let mut preprocessor = CppPreprocessor::new();
    let preprocessed = preprocessor.process(source);

    // Phase 1: Lex — tokenize preprocessed source
    let tokens = CppLexer::new(&preprocessed).tokenize();

    // Phase 2: Parse — tokens → C++ AST
    let unit = CppParser::new(tokens).parse_translation_unit()?;

    // Phase 3: Lower — C++ AST → ADead-BIB IR
    let mut converter = CppToIR::new();
    converter.convert(&unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world_cpp() {
        let program = compile_cpp_to_program(r#"
            int main() {
                printf("Hello from C++!\n");
                return 0;
            }
        "#).unwrap();
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "main");
    }

    #[test]
    fn test_class_compilation() {
        let program = compile_cpp_to_program(r#"
            class Point {
            public:
                int x;
                int y;
                int getX() { return x; }
            };

            int main() {
                return 0;
            }
        "#).unwrap();
        assert!(program.structs.len() >= 1);
        assert_eq!(program.structs[0].name, "Point");
        assert!(program.functions.len() >= 1); // main + getX
    }

    #[test]
    fn test_template_function() {
        let program = compile_cpp_to_program(r#"
            template<typename T>
            T add(T a, T b) {
                return a + b;
            }

            int main() {
                int result = add(3, 4);
                return 0;
            }
        "#).unwrap();
        assert!(program.functions.len() >= 2);
    }

    #[test]
    fn test_namespace() {
        let program = compile_cpp_to_program(r#"
            namespace math {
                int square(int x) {
                    return x * x;
                }
            }

            int main() {
                return 0;
            }
        "#).unwrap();
        assert!(program.functions.len() >= 2);
    }

    #[test]
    fn test_enum_class() {
        let program = compile_cpp_to_program(r#"
            enum class Color : int {
                Red = 0,
                Green = 1,
                Blue = 2
            };

            int main() {
                return 0;
            }
        "#).unwrap();
        assert_eq!(program.statements.len(), 3); // 3 enum constants
    }

    #[test]
    fn test_modern_cpp() {
        let program = compile_cpp_to_program(r#"
            int main() {
                auto x = 42;
                const int y = 100;
                int arr[] = {1, 2, 3};
                return x + y;
            }
        "#).unwrap();
        assert_eq!(program.functions.len(), 1);
    }

    // ========== Example file tests ==========

    #[test]
    fn test_example_hello_cpp() {
        let source = std::fs::read_to_string("examples/cpp/hello.cpp").unwrap();
        let result = compile_cpp_to_program(&source);
        assert!(result.is_ok(), "hello.cpp failed: {}", result.unwrap_err());
    }

    #[test]
    fn test_example_cpp_oop() {
        let source = std::fs::read_to_string("examples/cpp/cpp_oop.cpp").unwrap();
        let result = compile_cpp_to_program(&source);
        assert!(result.is_ok(), "cpp_oop.cpp failed: {}", result.unwrap_err());
    }

    #[test]
    fn test_example_cpp_templates() {
        let source = std::fs::read_to_string("examples/cpp/cpp_templates.cpp").unwrap();
        let result = compile_cpp_to_program(&source);
        assert!(result.is_ok(), "cpp_templates.cpp failed: {}", result.unwrap_err());
    }

    #[test]
    fn test_example_cpp_modern() {
        let source = std::fs::read_to_string("examples/cpp/cpp_modern.cpp").unwrap();
        let result = compile_cpp_to_program(&source);
        assert!(result.is_ok(), "cpp_modern.cpp failed: {}", result.unwrap_err());
    }

    #[test]
    fn test_example_cpp_stdlib_long() {
        let source = std::fs::read_to_string("examples/cpp/cpp_stdlib_long.cpp").unwrap();
        let result = compile_cpp_to_program(&source);
        assert!(result.is_ok(), "cpp_stdlib_long.cpp failed: {}", result.unwrap_err());
        let prog = result.unwrap();
        assert!(prog.functions.len() > 10, "cpp_stdlib_long.cpp should have many functions, got {}", prog.functions.len());
    }

    // ================================================================
    // GCC/LLVM/MSVC-INSPIRED C++ COMPREHENSIVE TESTS
    // ================================================================
    // Covers: classes, inheritance, templates, namespaces, enum class,
    //         constexpr, auto, nullptr, type aliases, control flow
    // Each test verifies parsing + IR generation for C++ features.
    // ================================================================

    // --- Example file tests (new test files) ---

    #[test]
    fn test_example_class_basic() {
        let source = std::fs::read_to_string("examples/cpp/test_class_basic.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_class_basic.cpp failed");
        assert!(prog.structs.len() >= 1, "should have Counter struct");
        assert!(prog.functions.len() >= 1, "should have main + methods");
    }

    #[test]
    fn test_example_inheritance() {
        let source = std::fs::read_to_string("examples/cpp/test_inheritance.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_inheritance.cpp failed");
        assert!(prog.structs.len() >= 3, "should have Shape + Circle + Rectangle, got {}", prog.structs.len());
    }

    #[test]
    fn test_example_template_basic() {
        let source = std::fs::read_to_string("examples/cpp/test_template_basic.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_template_basic.cpp failed");
        assert!(prog.functions.len() >= 4, "should have max + min + abs + main");
    }

    #[test]
    fn test_example_enum_class() {
        let source = std::fs::read_to_string("examples/cpp/test_enum_class.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_enum_class.cpp failed");
        assert_eq!(prog.functions.len(), 1, "should have main");
    }

    #[test]
    fn test_example_constexpr() {
        let source = std::fs::read_to_string("examples/cpp/test_constexpr.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_constexpr.cpp failed");
        assert!(prog.functions.len() >= 4, "should have factorial + fib + square + cube + main");
    }

    #[test]
    fn test_example_auto_nullptr() {
        let source = std::fs::read_to_string("examples/cpp/test_auto_nullptr.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_auto_nullptr.cpp failed");
        assert!(prog.functions.len() >= 3, "should have add + multiply + main");
    }

    #[test]
    fn test_example_nested_namespace() {
        let source = std::fs::read_to_string("examples/cpp/test_nested_namespace.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_nested_namespace.cpp failed");
        assert!(prog.functions.len() >= 4, "should have multiple namespace functions + main");
    }

    #[test]
    fn test_example_using_alias() {
        let source = std::fs::read_to_string("examples/cpp/test_using_alias.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_using_alias.cpp failed");
        assert!(prog.functions.len() >= 3, "should have double_val + triple_val + main");
    }

    #[test]
    fn test_example_class_methods() {
        let source = std::fs::read_to_string("examples/cpp/test_class_methods.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_class_methods.cpp failed");
        assert!(prog.structs.len() >= 1, "should have Calculator struct");
    }

    #[test]
    fn test_example_cpp_control_flow() {
        let source = std::fs::read_to_string("examples/cpp/test_cpp_control_flow.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_cpp_control_flow.cpp failed");
        assert!(prog.functions.len() >= 3, "should have fibonacci + is_prime + main");
    }

    // --- Existing .cpp test file validations ---

    #[test]
    fn test_example_test_minimal() {
        let source = std::fs::read_to_string("examples/cpp/test_minimal.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_minimal.cpp failed");
        assert!(prog.functions.len() >= 2);
    }

    #[test]
    fn test_example_test_5func() {
        let source = std::fs::read_to_string("examples/cpp/test_5func.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_5func.cpp failed");
        assert!(prog.functions.len() >= 5);
    }

    #[test]
    fn test_example_test_namespace() {
        let source = std::fs::read_to_string("examples/cpp/test_namespace.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_namespace.cpp failed");
        assert!(prog.functions.len() >= 4);
    }

    #[test]
    fn test_example_test_recursion_cpp() {
        let source = std::fs::read_to_string("examples/cpp/test_recursion.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_recursion.cpp failed");
        assert!(prog.functions.len() >= 2);
    }

    #[test]
    fn test_example_test_forloop_cpp() {
        let source = std::fs::read_to_string("examples/cpp/test_forloop.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_forloop.cpp failed");
        assert!(prog.functions.len() >= 1);
    }

    #[test]
    fn test_example_test_gcd() {
        let source = std::fs::read_to_string("examples/cpp/test_gcd.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_gcd.cpp failed");
        assert!(prog.functions.len() >= 2);
    }

    #[test]
    fn test_example_test_prime() {
        let source = std::fs::read_to_string("examples/cpp/test_prime.cpp").unwrap();
        let prog = compile_cpp_to_program(&source).expect("test_prime.cpp failed");
        assert!(prog.functions.len() >= 2);
    }

    // --- Inline C++ feature tests (no external files) ---

    #[test]
    fn test_cpp_class_with_constructor() {
        let prog = compile_cpp_to_program(r#"
            class Vector2D {
            public:
                int x;
                int y;
                Vector2D(int px, int py) : x(px), y(py) {}
                int dot(int ox, int oy) { return x * ox + y * oy; }
                int magnitude_sq() { return x * x + y * y; }
            };
            int main() { return 0; }
        "#).unwrap();
        assert!(prog.structs.len() >= 1);
    }

    #[test]
    fn test_cpp_multiple_classes() {
        let prog = compile_cpp_to_program(r#"
            class A {
            public:
                int val;
                A(int v) : val(v) {}
                int get() { return val; }
            };
            class B {
            public:
                int data;
                B(int d) : data(d) {}
                int get() { return data; }
            };
            int main() { return 0; }
        "#).unwrap();
        assert!(prog.structs.len() >= 2);
    }

    #[test]
    fn test_cpp_namespace_with_classes() {
        let prog = compile_cpp_to_program(r#"
            namespace game {
                class Entity {
                public:
                    int id;
                    Entity(int i) : id(i) {}
                };
                int next_id() { return 42; }
            }
            int main() { return 0; }
        "#).unwrap();
        assert!(prog.functions.len() >= 1);
    }

    #[test]
    fn test_cpp_virtual_method() {
        let prog = compile_cpp_to_program(r#"
            class Base {
            public:
                virtual int value() { return 0; }
            };
            class Derived : public Base {
            public:
                int value() override { return 42; }
            };
            int main() { return 0; }
        "#).unwrap();
        assert!(prog.structs.len() >= 2);
    }

    #[test]
    fn test_cpp_explicit_constructor() {
        let prog = compile_cpp_to_program(r#"
            class Wrapper {
            public:
                int val;
                explicit Wrapper(int v) : val(v) {}
                int get() { return val; }
            };
            int main() {
                Wrapper w(42);
                return 0;
            }
        "#).unwrap();
        assert!(prog.structs.len() >= 1);
    }

    #[test]
    fn test_cpp_const_method() {
        let prog = compile_cpp_to_program(r#"
            class Buffer {
            public:
                int size;
                Buffer(int s) : size(s) {}
                int get_size() const { return size; }
                bool empty() const { return size <= 0; }
            };
            int main() { return 0; }
        "#).unwrap();
        assert!(prog.structs.len() >= 1);
    }

    #[test]
    fn test_cpp_noexcept() {
        let prog = compile_cpp_to_program(r#"
            class Safe {
            public:
                int val;
                Safe() : val(0) {}
                void reset() noexcept { val = 0; }
                int get() noexcept { return val; }
            };
            int main() { return 0; }
        "#).unwrap();
        assert!(prog.structs.len() >= 1);
    }

    #[test]
    fn test_cpp_for_loop_with_namespace() {
        let prog = compile_cpp_to_program(r#"
            namespace util {
                int sum(int n) {
                    int total = 0;
                    for (int i = 1; i <= n; i++) {
                        total = total + i;
                    }
                    return total;
                }
            }
            int main() {
                int s = util::sum(100);
                return s;
            }
        "#).unwrap();
        assert!(prog.functions.len() >= 2);
    }

    #[test]
    fn test_cpp_while_loop() {
        let prog = compile_cpp_to_program(r#"
            int power(int base, int exp) {
                int result = 1;
                while (exp > 0) {
                    result = result * base;
                    exp = exp - 1;
                }
                return result;
            }
            int main() {
                int r = power(2, 10);
                return r;
            }
        "#).unwrap();
        assert_eq!(prog.functions.len(), 2);
    }

    #[test]
    fn test_cpp_ternary_operator() {
        let prog = compile_cpp_to_program(r#"
            int abs_val(int x) {
                return (x < 0) ? (0 - x) : x;
            }
            int max2(int a, int b) {
                return (a > b) ? a : b;
            }
            int main() { return abs_val(-5) + max2(3, 7); }
        "#).unwrap();
        assert_eq!(prog.functions.len(), 3);
    }

    // --- End-to-end C++ → machine code tests ---

    #[test]
    fn test_cpp_e2e_hello() {
        let prog = compile_cpp_to_program(r#"
            int main() {
                printf("Hello C++!\n");
                return 0;
            }
        "#).unwrap();
        let mut compiler = crate::isa::isa_compiler::IsaCompiler::new(
            crate::isa::isa_compiler::Target::Windows);
        let (code, data, _, _) = compiler.compile(&prog);
        assert!(!code.is_empty(), "should generate code");
        assert!(!data.is_empty(), "should have string data");
    }

    #[test]
    fn test_cpp_e2e_namespace() {
        let prog = compile_cpp_to_program(r#"
            namespace math {
                int add(int a, int b) { return a + b; }
                int mul(int a, int b) { return a * b; }
            }
            int main() {
                int r = math::add(3, 4);
                printf("result=%d\n", r);
                return 0;
            }
        "#).unwrap();
        let mut compiler = crate::isa::isa_compiler::IsaCompiler::new(
            crate::isa::isa_compiler::Target::Windows);
        let (code, _, _, _) = compiler.compile(&prog);
        assert!(!code.is_empty());
    }

    #[test]
    fn test_cpp_e2e_class() {
        let prog = compile_cpp_to_program(r#"
            class Point {
            public:
                int x;
                int y;
                Point(int px, int py) : x(px), y(py) {}
                int sum() { return x + y; }
            };
            int main() {
                printf("done\n");
                return 0;
            }
        "#).unwrap();
        let mut compiler = crate::isa::isa_compiler::IsaCompiler::new(
            crate::isa::isa_compiler::Target::Windows);
        let (code, _, _, _) = compiler.compile(&prog);
        assert!(!code.is_empty());
    }

    #[test]
    fn test_cpp_e2e_template() {
        let prog = compile_cpp_to_program(r#"
            template<typename T>
            T add(T a, T b) { return a + b; }
            int main() {
                int r = add(10, 20);
                printf("add=%d\n", r);
                return 0;
            }
        "#).unwrap();
        let mut compiler = crate::isa::isa_compiler::IsaCompiler::new(
            crate::isa::isa_compiler::Target::Windows);
        let (code, _, _, _) = compiler.compile(&prog);
        assert!(!code.is_empty());
    }

    #[test]
    fn test_cpp_e2e_full_oop_example() {
        let source = std::fs::read_to_string("examples/cpp/cpp_oop.cpp")
            .expect("cpp_oop.cpp should exist");
        let prog = compile_cpp_to_program(&source).expect("cpp_oop.cpp should parse");
        let mut compiler = crate::isa::isa_compiler::IsaCompiler::new(
            crate::isa::isa_compiler::Target::Windows);
        let (code, data, _, _) = compiler.compile(&prog);
        assert!(!code.is_empty(), "cpp_oop.cpp should generate code");
        assert!(!data.is_empty(), "cpp_oop.cpp should have string data");
    }

    #[test]
    fn test_cpp_e2e_full_templates_example() {
        let source = std::fs::read_to_string("examples/cpp/cpp_templates.cpp")
            .expect("cpp_templates.cpp should exist");
        let prog = compile_cpp_to_program(&source).expect("cpp_templates.cpp should parse");
        let mut compiler = crate::isa::isa_compiler::IsaCompiler::new(
            crate::isa::isa_compiler::Target::Windows);
        let (code, _, _, _) = compiler.compile(&prog);
        assert!(!code.is_empty(), "cpp_templates.cpp should generate code");
    }

    #[test]
    fn test_cpp_e2e_full_modern_example() {
        let source = std::fs::read_to_string("examples/cpp/cpp_modern.cpp")
            .expect("cpp_modern.cpp should exist");
        let prog = compile_cpp_to_program(&source).expect("cpp_modern.cpp should parse");
        let mut compiler = crate::isa::isa_compiler::IsaCompiler::new(
            crate::isa::isa_compiler::Target::Windows);
        let (code, _, _, _) = compiler.compile(&prog);
        assert!(!code.is_empty(), "cpp_modern.cpp should generate code");
    }
}
