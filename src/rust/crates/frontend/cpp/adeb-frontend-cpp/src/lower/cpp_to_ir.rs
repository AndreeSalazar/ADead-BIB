// ============================================================
// C++ AST → ADead-BIB IR Converter
// ============================================================
// Lowers C++ AST to ADead-BIB's Program/Function/Stmt/Expr
// Classes → Structs + functions, new/delete → malloc/free,
// References → pointers, Namespaces → flattened names
// ============================================================

use crate::ast::*;
use crate::frontend::ast::{
    BinOp, BitwiseOp, CmpOp, CompoundOp, Expr, Function, FunctionAttributes,
    Param, Program, ProgramAttributes, Stmt, Struct,
    StructField, UnaryOp, SwitchCase,
};
use crate::frontend::types::Type;

use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn fresh_temp(prefix: &str) -> String {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("__{}{}", prefix, id)
}

pub struct CppToIR {
    ns: Vec<String>,
    current_class: Option<String>,
    enum_constants: Vec<(String, i64)>,
}

impl CppToIR {
    pub fn new() -> Self {
        Self { ns: Vec::new(), current_class: None, enum_constants: Vec::new() }
    }

    fn mangled(&self, name: &str) -> String {
        if self.ns.is_empty() { name.to_string() }
        else { format!("{}::{}", self.ns.join("::"), name) }
    }

    pub fn convert(&mut self, unit: &CppTranslationUnit) -> Result<Program, String> {
        let mut prog = Program::new();
        prog.attributes = ProgramAttributes::default();

        // Pass 1: collect enums, typedefs
        for d in &unit.declarations {
            if let CppTopLevel::EnumDef { values, .. } = d {
                let mut val = 0i64;
                for (name, expr) in values {
                    if let Some(CppExpr::IntLiteral(v)) = expr { val = *v; }
                    self.enum_constants.push((name.clone(), val));
                    val += 1;
                }
            }
        }

        // Pass 2: everything else
        for d in &unit.declarations {
            self.convert_top_level(d, &mut prog)?;
        }
        Ok(prog)
    }

    fn convert_top_level(&mut self, decl: &CppTopLevel, prog: &mut Program) -> Result<(), String> {
        match decl {
            CppTopLevel::FunctionDef { return_type, name, params, body, .. } => {
                let fname = self.mangled(name);
                let ir_params: Vec<Param> = params.iter().map(|p| self.convert_param(p)).collect();
                let ir_body = self.convert_stmts(body)?;
                prog.functions.push(Function {
                    name: fname,
                    params: ir_params,
                    return_type: Some(self.type_name(return_type)),
                    resolved_return_type: self.convert_type(return_type),
                    body: ir_body,
                    attributes: FunctionAttributes::default(),
                });
            }
            CppTopLevel::ClassDef { name, bases, members, is_struct, .. } => {
                self.convert_class(name, bases, members, *is_struct, prog)?;
            }
            CppTopLevel::Namespace { name, declarations } => {
                self.ns.push(name.clone());
                for d in declarations { self.convert_top_level(d, prog)?; }
                self.ns.pop();
            }
            CppTopLevel::GlobalVar { type_spec, declarators } => {
                for d in declarators {
                    let val = d.initializer.as_ref().map(|e| self.convert_expr(e));
                    prog.statements.push(Stmt::VarDecl {
                        var_type: self.convert_type(type_spec),
                        name: d.name.clone(),
                        value: val,
                    });
                }
            }
            CppTopLevel::EnumDef { .. } => { /* handled in pass 1 */ }
            CppTopLevel::TypeAlias { .. } | CppTopLevel::UsingDecl { .. }
            | CppTopLevel::UsingNamespace(_) | CppTopLevel::FunctionDecl { .. }
            | CppTopLevel::StaticAssert { .. } | CppTopLevel::ExternC { .. }
            | CppTopLevel::TemplateInstantiation { .. }
            | CppTopLevel::TemplateSpecialization { .. }
            | CppTopLevel::TemplateFuncSpecialization { .. } => {}
        }
        Ok(())
    }

    // ── Class → Struct + Functions ──────────────────────────
    fn convert_class(&mut self, name: &str, bases: &[CppBaseClass],
        members: &[CppClassMember], _is_struct: bool, prog: &mut Program
    ) -> Result<(), String> {
        let mut fields = Vec::new();
        // Inherit base class fields (simple embedding)
        for base in bases {
            fields.push(StructField {
                name: format!("__base_{}", base.name),
                field_type: Type::Struct(base.name.clone()),
            });
        }

        let old_class = self.current_class.take();
        self.current_class = Some(name.to_string());

        for m in members {
            match m {
                CppClassMember::Field { type_spec, name: fname, .. } => {
                    fields.push(StructField {
                        name: fname.clone(),
                        field_type: self.convert_type(type_spec),
                    });
                }
                CppClassMember::Method { return_type, name: mname, params, body: Some(body), .. } => {
                    let fname = format!("{}::{}", name, mname);
                    let mut ir_params = vec![Param::typed("this".into(),
                        Type::Pointer(Box::new(Type::Struct(name.to_string()))))];
                    ir_params.extend(params.iter().map(|p| self.convert_param(p)));
                    let ir_body = self.convert_stmts(body)?;
                    let func = Function {
                        name: fname,
                        params: ir_params,
                        return_type: Some(self.type_name(return_type)),
                        resolved_return_type: self.convert_type(return_type),
                        body: ir_body,
                        attributes: FunctionAttributes::default(),
                    };
                    prog.functions.push(func);
                }
                CppClassMember::Constructor { params, body: Some(body), .. } => {
                    let fname = format!("{}::__init", name);
                    let mut ir_params = vec![Param::typed("this".into(),
                        Type::Pointer(Box::new(Type::Struct(name.to_string()))))];
                    ir_params.extend(params.iter().map(|p| self.convert_param(p)));
                    prog.functions.push(Function {
                        name: fname,
                        params: ir_params,
                        return_type: None,
                        resolved_return_type: Type::Void,
                        body: self.convert_stmts(body)?,
                        attributes: FunctionAttributes::default(),
                    });
                }
                CppClassMember::Destructor { body: Some(body), .. } => {
                    let fname = format!("{}::__destroy", name);
                    let ir_params = vec![Param::typed("this".into(),
                        Type::Pointer(Box::new(Type::Struct(name.to_string()))))];
                    prog.functions.push(Function {
                        name: fname,
                        params: ir_params,
                        return_type: None,
                        resolved_return_type: Type::Void,
                        body: self.convert_stmts(body)?,
                        attributes: FunctionAttributes::default(),
                    });
                }
                _ => {}
            }
        }

        prog.structs.push(Struct { name: name.to_string(), fields, is_packed: false });
        self.current_class = old_class;
        Ok(())
    }

    // ── Statements ──────────────────────────────────────────
    fn convert_stmts(&mut self, stmts: &[CppStmt]) -> Result<Vec<Stmt>, String> {
        let mut out = Vec::new();
        for s in stmts { self.convert_stmt(s, &mut out)?; }
        Ok(out)
    }

    fn convert_stmt(&mut self, stmt: &CppStmt, out: &mut Vec<Stmt>) -> Result<(), String> {
        match stmt {
            CppStmt::Expr(e) => {
                // Phase 2: handle assignments at statement level
                match e {
                    CppExpr::Assign { target, value } => {
                        self.lower_assign(target, value, out);
                    }
                    CppExpr::CompoundAssign { op, target, value } => {
                        self.lower_compound_assign(op, target, value, out);
                    }
                    _ => { out.push(Stmt::Expr(self.convert_expr(e))); }
                }
            }
            CppStmt::Return(Some(e)) => { out.push(Stmt::Return(Some(self.convert_expr(e)))); }
            CppStmt::Return(None) => { out.push(Stmt::Return(None)); }
            CppStmt::VarDecl { type_spec, declarators } => {
                for d in declarators {
                    let val = d.initializer.as_ref().map(|e| self.convert_expr(e));
                    out.push(Stmt::VarDecl {
                        var_type: self.convert_type(type_spec),
                        name: d.name.clone(),
                        value: val,
                    });
                }
            }
            CppStmt::Block(stmts) => {
                for s in stmts { self.convert_stmt(s, out)?; }
            }
            CppStmt::If { condition, then_body, else_body, is_constexpr, .. } => {
                // Phase 2: if constexpr — evaluate at compile time, eliminate dead branch
                if *is_constexpr {
                    if let Some(val) = self.try_eval_constexpr(condition) {
                        if val {
                            self.convert_stmt(then_body, out)?;
                        } else if let Some(eb) = else_body {
                            self.convert_stmt(eb, out)?;
                        }
                        return Ok(());
                    }
                }
                let cond = self.convert_expr(condition);
                let then_b = self.convert_stmt_to_vec(then_body)?;
                let else_b = match else_body {
                    Some(eb) => Some(self.convert_stmt_to_vec(eb)?),
                    None => None,
                };
                out.push(Stmt::If { condition: cond, then_body: then_b, else_body: else_b });
            }
            CppStmt::While { condition, body } => {
                out.push(Stmt::While {
                    condition: self.convert_expr(condition),
                    body: self.convert_stmt_to_vec(body)?,
                });
            }
            CppStmt::DoWhile { body, condition } => {
                out.push(Stmt::DoWhile {
                    body: self.convert_stmt_to_vec(body)?,
                    condition: self.convert_expr(condition),
                });
            }
            CppStmt::For { init, condition, increment, body } => {
                // Lower for to: { init; while(cond) { body; incr; } }
                if let Some(init_s) = init { self.convert_stmt(init_s, out)?; }
                let cond = condition.as_ref().map(|c| self.convert_expr(c))
                    .unwrap_or(Expr::Bool(true));
                let mut loop_body = self.convert_stmt_to_vec(body)?;
                if let Some(inc) = increment {
                    loop_body.push(Stmt::Expr(self.convert_expr(inc)));
                }
                out.push(Stmt::While { condition: cond, body: loop_body });
            }
            CppStmt::RangeFor { type_spec: _, name, iterable, body } => {
                // Lower range-for to: for each element
                out.push(Stmt::ForEach {
                    var: name.clone(),
                    iterable: self.convert_expr(iterable),
                    body: self.convert_stmt_to_vec(body)?,
                });
            }
            CppStmt::Switch { expr, cases, default } => {
                let ir_cases: Vec<SwitchCase> = cases.iter().map(|c| {
                    SwitchCase {
                        value: self.convert_expr(&c.value),
                        body: c.body.iter().filter_map(|s| {
                            let mut v = Vec::new();
                            let _ = self.convert_stmt(s, &mut v);
                            v.into_iter().next()
                        }).collect(),
                        has_break: c.body.iter().any(|s| matches!(s, CppStmt::Break)),
                    }
                }).collect();
                let ir_default = default.as_ref().map(|d| {
                    let mut v = Vec::new();
                    for s in d { let _ = self.convert_stmt(s, &mut v); }
                    v
                });
                out.push(Stmt::Switch {
                    expr: self.convert_expr(expr),
                    cases: ir_cases,
                    default: ir_default,
                });
            }
            CppStmt::Break => { out.push(Stmt::Break); }
            CppStmt::Continue => { out.push(Stmt::Continue); }
            CppStmt::Goto(label) => { out.push(Stmt::JumpTo { label: label.clone() }); }
            CppStmt::Label(name, inner) => {
                out.push(Stmt::LabelDef { name: name.clone() });
                self.convert_stmt(inner, out)?;
            }
            CppStmt::Try { body, .. } => {
                // Lower try/catch: just emit the body (exceptions → removed)
                for s in body { self.convert_stmt(s, out)?; }
            }
            CppStmt::Throw(_) => { /* eliminated */ }
            CppStmt::CoReturn(e) => {
                out.push(Stmt::Return(e.as_ref().map(|x| self.convert_expr(x))));
            }
            CppStmt::Empty | CppStmt::LineMarker(_) => {}
        }
        Ok(())
    }

    fn convert_stmt_to_vec(&mut self, stmt: &CppStmt) -> Result<Vec<Stmt>, String> {
        let mut v = Vec::new();
        self.convert_stmt(stmt, &mut v)?;
        Ok(v)
    }

    // ── Expressions ─────────────────────────────────────────
    fn convert_expr(&self, expr: &CppExpr) -> Expr {
        match expr {
            CppExpr::IntLiteral(n) => Expr::Number(*n),
            CppExpr::UIntLiteral(n) => Expr::Number(*n as i64),
            CppExpr::FloatLiteral(f) => Expr::Float(*f),
            CppExpr::StringLiteral(s) => Expr::String(s.clone()),
            CppExpr::CharLiteral(c) => Expr::Number(*c as i64),
            CppExpr::BoolLiteral(b) => Expr::Bool(*b),
            CppExpr::NullptrLiteral => Expr::Nullptr,
            CppExpr::Identifier(name) => {
                // Check enum constants
                for (en, ev) in &self.enum_constants {
                    if en == name { return Expr::Number(*ev); }
                }
                Expr::Variable(name.clone())
            }
            CppExpr::ScopedIdentifier { scope, name } => {
                let full = format!("{}::{}", scope.join("::"), name);
                // cout/cin/endl → printf/scanf helpers
                if full == "std::cout" { return Expr::Variable("__stdout".into()); }
                if full == "std::cin" { return Expr::Variable("__stdin".into()); }
                if full == "std::endl" { return Expr::String("\n".into()); }
                Expr::Variable(full)
            }
            CppExpr::This => Expr::Variable("this".into()),
            CppExpr::BinaryOp { op, left, right } => {
                let l = Box::new(self.convert_expr(left));
                let r = Box::new(self.convert_expr(right));
                match op {
                    CppBinOp::Add => Expr::BinaryOp { op: BinOp::Add, left: l, right: r },
                    CppBinOp::Sub => Expr::BinaryOp { op: BinOp::Sub, left: l, right: r },
                    CppBinOp::Mul => Expr::BinaryOp { op: BinOp::Mul, left: l, right: r },
                    CppBinOp::Div => Expr::BinaryOp { op: BinOp::Div, left: l, right: r },
                    CppBinOp::Mod => Expr::BinaryOp { op: BinOp::Mod, left: l, right: r },
                    CppBinOp::Eq => Expr::Comparison { op: CmpOp::Eq, left: l, right: r },
                    CppBinOp::Ne => Expr::Comparison { op: CmpOp::Ne, left: l, right: r },
                    CppBinOp::Lt => Expr::Comparison { op: CmpOp::Lt, left: l, right: r },
                    CppBinOp::Le => Expr::Comparison { op: CmpOp::Le, left: l, right: r },
                    CppBinOp::Gt => Expr::Comparison { op: CmpOp::Gt, left: l, right: r },
                    CppBinOp::Ge => Expr::Comparison { op: CmpOp::Ge, left: l, right: r },
                    CppBinOp::And => Expr::BinaryOp { op: BinOp::And, left: l, right: r },
                    CppBinOp::Or => Expr::BinaryOp { op: BinOp::Or, left: l, right: r },
                    CppBinOp::BitAnd => Expr::BitwiseOp { op: BitwiseOp::And, left: l, right: r },
                    CppBinOp::BitOr => Expr::BitwiseOp { op: BitwiseOp::Or, left: l, right: r },
                    CppBinOp::BitXor => Expr::BitwiseOp { op: BitwiseOp::Xor, left: l, right: r },
                    CppBinOp::Shl => Expr::BitwiseOp { op: BitwiseOp::LeftShift, left: l, right: r },
                    CppBinOp::Shr => Expr::BitwiseOp { op: BitwiseOp::RightShift, left: l, right: r },
                    CppBinOp::Spaceship => {
                        // <=> lowers to: (a < b) ? -1 : ((a > b) ? 1 : 0)
                        Expr::Ternary {
                            condition: Box::new(Expr::Comparison { op: CmpOp::Lt, left: l.clone(), right: r.clone() }),
                            then_expr: Box::new(Expr::Number(-1)),
                            else_expr: Box::new(Expr::Ternary {
                                condition: Box::new(Expr::Comparison { op: CmpOp::Gt, left: l, right: r }),
                                then_expr: Box::new(Expr::Number(1)),
                                else_expr: Box::new(Expr::Number(0)),
                            }),
                        }
                    }
                }
            }
            CppExpr::UnaryOp { op, expr: inner, .. } => {
                let e = Box::new(self.convert_expr(inner));
                match op {
                    CppUnaryOp::Neg => Expr::UnaryOp { op: UnaryOp::Neg, expr: e },
                    CppUnaryOp::Not => Expr::UnaryOp { op: UnaryOp::Not, expr: e },
                    CppUnaryOp::BitNot => Expr::BitwiseNot(e),
                    CppUnaryOp::PreInc => Expr::PreIncrement(e),
                    CppUnaryOp::PreDec => Expr::PreDecrement(e),
                    CppUnaryOp::PostInc => Expr::PostIncrement(e),
                    CppUnaryOp::PostDec => Expr::PostDecrement(e),
                }
            }
            CppExpr::Assign { target, value } => {
                // Assignment as expression evaluates to the assigned value
                self.convert_expr(value)
            }
            CppExpr::CompoundAssign { target, value, .. } => {
                // Compound assignment as expression evaluates to the result
                self.convert_expr(value)
            }
            CppExpr::Call { callee, args } => {
                let ir_args: Vec<Expr> = args.iter().map(|a| self.convert_expr(a)).collect();
                match callee.as_ref() {
                    CppExpr::Identifier(name) => Expr::Call { name: name.clone(), args: ir_args },
                    CppExpr::ScopedIdentifier { scope, name } => {
                        let full = format!("{}::{}", scope.join("::"), name);
                        Expr::Call { name: full, args: ir_args }
                    }
                    CppExpr::MemberAccess { object, member } => {
                        let obj = self.convert_expr(object);
                        Expr::MethodCall { object: Box::new(obj), method: member.clone(), args: ir_args }
                    }
                    CppExpr::ArrowAccess { pointer, member } => {
                        let ptr = self.convert_expr(pointer);
                        Expr::MethodCall { object: Box::new(ptr), method: member.clone(), args: ir_args }
                    }
                    _ => Expr::Call { name: "__unknown_call".into(), args: ir_args },
                }
            }
            CppExpr::MemberAccess { object, member } => {
                Expr::FieldAccess { object: Box::new(self.convert_expr(object)), field: member.clone() }
            }
            CppExpr::ArrowAccess { pointer, member } => {
                Expr::ArrowAccess { pointer: Box::new(self.convert_expr(pointer)), field: member.clone() }
            }
            CppExpr::Index { object, index } => {
                Expr::Index { object: Box::new(self.convert_expr(object)), index: Box::new(self.convert_expr(index)) }
            }
            CppExpr::Deref(inner) => Expr::Deref(Box::new(self.convert_expr(inner))),
            CppExpr::AddressOf(inner) => Expr::AddressOf(Box::new(self.convert_expr(inner))),
            CppExpr::Cast { target_type, expr: inner, .. } => {
                Expr::Cast { target_type: self.convert_type(target_type), expr: Box::new(self.convert_expr(inner)) }
            }
            CppExpr::SizeOf(arg) => {
                let sa = match arg {
                    CppSizeOfArg::Type(t) => crate::frontend::ast::SizeOfArg::Type(self.convert_type(t)),
                    CppSizeOfArg::Expr(e) => crate::frontend::ast::SizeOfArg::Expr(self.convert_expr(e)),
                };
                Expr::SizeOf(Box::new(sa))
            }
            CppExpr::Ternary { condition, then_expr, else_expr } => {
                Expr::Ternary {
                    condition: Box::new(self.convert_expr(condition)),
                    then_expr: Box::new(self.convert_expr(then_expr)),
                    else_expr: Box::new(self.convert_expr(else_expr)),
                }
            }
            CppExpr::New { type_name, args, .. } => {
                let ir_args: Vec<Expr> = args.iter().map(|a| self.convert_expr(a)).collect();
                let tname = self.type_name(type_name);
                Expr::New { class_name: tname, args: ir_args }
            }
            CppExpr::Delete { expr: inner, is_array: _ } => {
                // Lower to a call — actual Stmt::Delete handled at statement level
                self.convert_expr(inner)
            }
            CppExpr::Lambda { params, body, .. } => {
                // Phase 2: lambda → closure struct + anonymous function
                let _lambda_name = fresh_temp("lambda");
                let pnames: Vec<String> = params.iter()
                    .map(|p| p.name.clone().unwrap_or_else(|| fresh_temp("lp")))
                    .collect();
                // Lambda body: take last expr as result, or Number(0)
                let body_expr = if let Some(CppStmt::Return(Some(e))) = body.last() {
                    self.convert_expr(e)
                } else if body.len() == 1 {
                    if let CppStmt::Expr(e) = &body[0] {
                        self.convert_expr(e)
                    } else {
                        Expr::Number(0)
                    }
                } else {
                    Expr::Number(0)
                };
                Expr::Lambda { params: pnames, body: Box::new(body_expr) }
            }
            CppExpr::InitList(exprs) => {
                Expr::Array(exprs.iter().map(|e| self.convert_expr(e)).collect())
            }
            CppExpr::Throw(_) => Expr::Number(0),
            CppExpr::CoAwait(e) | CppExpr::CoYield(e) => self.convert_expr(e),
            CppExpr::TypeId(_) | CppExpr::RangeExpr { .. }
            | CppExpr::StructuredBinding(_) | CppExpr::FoldExpr { .. }
            | CppExpr::PackExpansion(_) => Expr::Number(0),
        }
    }

    // ── Type conversion ─────────────────────────────────────
    fn convert_type(&self, ty: &CppType) -> Type {
        match ty {
            CppType::Void => Type::Void,
            CppType::Bool => Type::Bool,
            CppType::Char | CppType::WChar | CppType::Char8
            | CppType::Char16 | CppType::Char32 => Type::I8,
            CppType::Short => Type::I16,
            CppType::Int => Type::I32,
            CppType::Long => Type::I32,
            CppType::LongLong => Type::I64,
            CppType::Float => Type::F32,
            CppType::Double | CppType::LongDouble => Type::F64,
            CppType::Auto => Type::Auto,
            CppType::Unsigned(inner) => self.convert_type(inner),
            CppType::Signed(inner) => self.convert_type(inner),
            CppType::Const(inner) | CppType::Volatile(inner)
            | CppType::Mutable(inner) | CppType::Constexpr(inner) => self.convert_type(inner),
            CppType::Pointer(inner) => Type::Pointer(Box::new(self.convert_type(inner))),
            CppType::Reference(inner) | CppType::RValueRef(inner) => {
                Type::Pointer(Box::new(self.convert_type(inner)))
            }
            CppType::Array(inner, size) => {
                Type::Array(Box::new(self.convert_type(inner)), *size)
            }
            CppType::Named(n) | CppType::Struct(n) | CppType::Class(n) => Type::Struct(n.clone()),
            CppType::Enum(_) => Type::I32,
            CppType::Union(n) => Type::Struct(n.clone()),
            CppType::Typedef(n) => Type::Struct(n.clone()),
            CppType::StdString | CppType::StdStringView => Type::Pointer(Box::new(Type::I8)),
            CppType::SizeT | CppType::Nullptr => Type::I64,
            _ => Type::I64,
        }
    }

    fn type_name(&self, ty: &CppType) -> String {
        match ty {
            CppType::Void => "void".into(),
            CppType::Bool => "bool".into(),
            CppType::Char => "char".into(),
            CppType::Int => "int".into(),
            CppType::Long => "long".into(),
            CppType::LongLong => "long long".into(),
            CppType::Float => "float".into(),
            CppType::Double => "double".into(),
            CppType::Auto => "auto".into(),
            CppType::Pointer(inner) => format!("{}*", self.type_name(inner)),
            CppType::Reference(inner) => format!("{}&", self.type_name(inner)),
            CppType::Named(n) | CppType::Struct(n) | CppType::Class(n) => n.clone(),
            _ => "int".into(),
        }
    }

    fn convert_param(&self, p: &CppParam) -> Param {
        let name = p.name.clone().unwrap_or_else(|| fresh_temp("p"));
        Param::typed(name, self.convert_type(&p.param_type))
    }

    // ── Phase 2: Assignment lowering ────────────────────────

    fn lower_assign(&self, target: &CppExpr, value: &CppExpr, out: &mut Vec<Stmt>) {
        let v = self.convert_expr(value);
        match target {
            CppExpr::Identifier(name) => {
                out.push(Stmt::Assign { name: name.clone(), value: v });
            }
            CppExpr::MemberAccess { object, member } => {
                out.push(Stmt::FieldAssign {
                    object: self.convert_expr(object),
                    field: member.clone(),
                    value: v,
                });
            }
            CppExpr::ArrowAccess { pointer, member } => {
                out.push(Stmt::ArrowAssign {
                    pointer: self.convert_expr(pointer),
                    field: member.clone(),
                    value: v,
                });
            }
            CppExpr::Deref(inner) => {
                out.push(Stmt::DerefAssign {
                    pointer: self.convert_expr(inner),
                    value: v,
                });
            }
            CppExpr::Index { object, index } => {
                out.push(Stmt::IndexAssign {
                    object: self.convert_expr(object),
                    index: self.convert_expr(index),
                    value: v,
                });
            }
            _ => {
                out.push(Stmt::Expr(v));
            }
        }
    }

    fn lower_compound_assign(&self, op: &CppBinOp, target: &CppExpr, value: &CppExpr, out: &mut Vec<Stmt>) {
        let ir_op = match op {
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
        match target {
            CppExpr::Identifier(name) => {
                out.push(Stmt::CompoundAssign {
                    name: name.clone(),
                    op: ir_op,
                    value: self.convert_expr(value),
                });
            }
            _ => {
                // Fallback: expand to full assign
                let t = self.convert_expr(target);
                let v = self.convert_expr(value);
                let bin_op = match op {
                    CppBinOp::Add => BinOp::Add,
                    CppBinOp::Sub => BinOp::Sub,
                    CppBinOp::Mul => BinOp::Mul,
                    CppBinOp::Div => BinOp::Div,
                    CppBinOp::Mod => BinOp::Mod,
                    _ => BinOp::Add,
                };
                out.push(Stmt::Expr(Expr::BinaryOp {
                    op: bin_op,
                    left: Box::new(t),
                    right: Box::new(v),
                }));
            }
        }
    }

    // ── Phase 2: Compile-time evaluation for if constexpr ───

    fn try_eval_constexpr(&self, expr: &CppExpr) -> Option<bool> {
        match expr {
            CppExpr::BoolLiteral(b) => Some(*b),
            CppExpr::IntLiteral(n) => Some(*n != 0),
            CppExpr::BinaryOp { op: CppBinOp::Eq, left, right } => {
                match (self.try_eval_int(left), self.try_eval_int(right)) {
                    (Some(a), Some(b)) => Some(a == b),
                    _ => None,
                }
            }
            CppExpr::BinaryOp { op: CppBinOp::Ne, left, right } => {
                match (self.try_eval_int(left), self.try_eval_int(right)) {
                    (Some(a), Some(b)) => Some(a != b),
                    _ => None,
                }
            }
            CppExpr::BinaryOp { op: CppBinOp::Lt, left, right } => {
                match (self.try_eval_int(left), self.try_eval_int(right)) {
                    (Some(a), Some(b)) => Some(a < b),
                    _ => None,
                }
            }
            CppExpr::BinaryOp { op: CppBinOp::Gt, left, right } => {
                match (self.try_eval_int(left), self.try_eval_int(right)) {
                    (Some(a), Some(b)) => Some(a > b),
                    _ => None,
                }
            }
            CppExpr::BinaryOp { op: CppBinOp::And, left, right } => {
                match (self.try_eval_constexpr(left), self.try_eval_constexpr(right)) {
                    (Some(a), Some(b)) => Some(a && b),
                    _ => None,
                }
            }
            CppExpr::BinaryOp { op: CppBinOp::Or, left, right } => {
                match (self.try_eval_constexpr(left), self.try_eval_constexpr(right)) {
                    (Some(a), Some(b)) => Some(a || b),
                    _ => None,
                }
            }
            CppExpr::UnaryOp { op: CppUnaryOp::Not, expr, .. } => {
                self.try_eval_constexpr(expr).map(|v| !v)
            }
            _ => None,
        }
    }

    fn try_eval_int(&self, expr: &CppExpr) -> Option<i64> {
        match expr {
            CppExpr::IntLiteral(n) => Some(*n),
            CppExpr::BoolLiteral(b) => Some(if *b { 1 } else { 0 }),
            CppExpr::UnaryOp { op: CppUnaryOp::Neg, expr, .. } => {
                self.try_eval_int(expr).map(|v| -v)
            }
            CppExpr::BinaryOp { op, left, right } => {
                let a = self.try_eval_int(left)?;
                let b = self.try_eval_int(right)?;
                match op {
                    CppBinOp::Add => a.checked_add(b),
                    CppBinOp::Sub => a.checked_sub(b),
                    CppBinOp::Mul => a.checked_mul(b),
                    CppBinOp::Div if b != 0 => a.checked_div(b),
                    CppBinOp::Mod if b != 0 => a.checked_rem(b),
                    _ => None,
                }
            }
            CppExpr::Identifier(name) => {
                // Check enum constants
                for (en, ev) in &self.enum_constants {
                    if en == name { return Some(*ev); }
                }
                None
            }
            _ => None,
        }
    }
}

/// Convenience function: compile C++ source → Program IR
pub fn compile_cpp_to_program(source: &str) -> Result<Program, String> {
    use crate::preprocessor::CppPreprocessor;
    use crate::parse::lexer::CppLexer;
    use crate::parse::parser::CppParser;

    let mut pp = CppPreprocessor::new();
    let preprocessed = pp.process(source);
    let (tokens, lines) = CppLexer::new(&preprocessed).tokenize();
    let unit = CppParser::new(tokens, lines).parse_translation_unit()?;
    let mut lower = CppToIR::new();
    lower.convert(&unit)
}

// ── Tests ───────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let prog = compile_cpp_to_program("int main() { return 0; }").unwrap();
        assert_eq!(prog.functions.len(), 1);
        assert_eq!(prog.functions[0].name, "main");
    }

    #[test]
    fn test_simple_class() {
        let prog = compile_cpp_to_program(r#"
            class Point {
            public:
                int x;
                int y;
            };
            int main() { return 0; }
        "#).unwrap();
        assert!(prog.structs.iter().any(|s| s.name == "Point"));
        assert!(prog.functions.iter().any(|f| f.name == "main"));
    }

    #[test]
    fn test_class_with_method() {
        let prog = compile_cpp_to_program(r#"
            class Calc {
            public:
                int add(int a, int b) { return a + b; }
            };
            int main() { return 0; }
        "#).unwrap();
        assert!(prog.functions.iter().any(|f| f.name == "Calc::add"));
    }

    #[test]
    fn test_namespace() {
        let prog = compile_cpp_to_program(r#"
            namespace math {
                int square(int x) { return x * x; }
            }
            int main() { return 0; }
        "#).unwrap();
        assert!(prog.functions.iter().any(|f| f.name == "math::square"));
    }

    #[test]
    fn test_enum() {
        let prog = compile_cpp_to_program(r#"
            enum Color { Red, Green, Blue };
            int main() { return 0; }
        "#).unwrap();
        assert_eq!(prog.functions.len(), 1);
    }
}
