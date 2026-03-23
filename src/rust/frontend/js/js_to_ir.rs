// ============================================================
// JsDead-BIB — JavaScript to IR Converter
// ============================================================
// Converts JS AST → ADead-BIB IR (Program)
// Implícitamente estricto — "Respetar Bits"
// Sin Node.js. Sin V8. Sin runtime. JS → ASM directo.
//
// Mapeo:
//   let/const/var   → VarDecl
//   function        → Function
//   class           → Struct + methods
//   console.log     → Print/Println
//   for/while/if    → same IR constructs
//   === / !==       → Comparison (strict only)
//   == / !=         → ERROR (blocked)
// ============================================================

use super::js_ast::*;
use super::js_lexer::JsLexer;
use super::js_parser::JsParser;
use crate::frontend::ast::{
    BinOp, BitwiseOp, CmpOp, Expr, Function, FunctionAttributes,
    Param, Program, Stmt, Struct, StructField, SwitchCase, UnaryOp,
};
use crate::frontend::types::Type;

/// JS to IR converter state
pub struct JsToIR {
    /// Variable types for strict checking: name → JsType
    var_types: Vec<(String, JsType)>,
    /// Class field info: (class_name, field_name, type)
    class_fields: Vec<(String, String, JsType)>,
}

impl JsToIR {
    pub fn new() -> Self {
        Self {
            var_types: Vec::new(),
            class_fields: Vec::new(),
        }
    }

    /// Convert a JsProgram into an ADead-BIB Program
    pub fn convert(&mut self, js: &JsProgram) -> Result<Program, String> {
        let mut program = Program::new();

        // First pass: collect class declarations for type info
        for stmt in &js.stmts {
            if let JsStmt::ClassDecl { name, body, .. } = stmt {
                for member in body {
                    if let JsClassMember::Property {
                        name: fname,
                        type_ann,
                        ..
                    } = member
                    {
                        self.class_fields.push((
                            name.clone(),
                            fname.clone(),
                            type_ann.clone().unwrap_or(JsType::Inferred),
                        ));
                    }
                }
            }
        }

        // Second pass: convert statements
        // Collect top-level statements, functions, and classes
        let mut top_stmts = Vec::new();
        let mut has_main = false;

        for stmt in &js.stmts {
            // Unwrap export to get inner statement
            let inner = match stmt {
                JsStmt::Export { item } => item.as_ref(),
                other => other,
            };

            match inner {
                JsStmt::FuncDecl {
                    name,
                    params,
                    return_type,
                    body,
                    ..
                } => {
                    if name == "main" {
                        has_main = true;
                    }
                    let func = self.convert_func_decl(name, params, return_type, body)?;
                    program.add_function(func);
                }
                JsStmt::ClassDecl {
                    name,
                    super_class,
                    body,
                } => {
                    let (strct, methods) =
                        self.convert_class_decl(name, super_class, body)?;
                    program.add_struct(strct);
                    for method in methods {
                        program.add_function(method);
                    }
                }
                _ => {
                    let ir_stmts = self.convert_stmt(stmt)?;
                    top_stmts.extend(ir_stmts);
                }
            }
        }

        // If there are top-level statements and no main(), wrap them in main()
        if !top_stmts.is_empty() && !has_main {
            let main_fn = Function {
                name: "main".into(),
                params: Vec::new(),
                return_type: Some("int".into()),
                resolved_return_type: Type::I32,
                body: top_stmts,
                attributes: FunctionAttributes::default(),
            };
            program.add_function(main_fn);
        } else if !top_stmts.is_empty() {
            // Has main + top-level: append top-level to beginning of main
            // Actually, just add as statements
            for s in top_stmts {
                program.add_statement(s);
            }
        }

        Ok(program)
    }

    // ── Function conversion ─────────────────────────────────

    fn convert_func_decl(
        &mut self,
        name: &str,
        params: &[JsParam],
        return_type: &Option<JsType>,
        body: &[JsStmt],
    ) -> Result<Function, String> {
        let ir_params: Vec<Param> = params
            .iter()
            .map(|p| {
                let ty = self.js_type_to_ir(&p.type_ann.clone().unwrap_or(JsType::Inferred));
                Param::typed(p.name.clone(), ty)
            })
            .collect();

        let ret_type = return_type
            .as_ref()
            .map(|t| self.js_type_to_ir(t))
            .unwrap_or(Type::Void);

        // Register param types
        for p in params {
            self.var_types.push((
                p.name.clone(),
                p.type_ann.clone().unwrap_or(JsType::Inferred),
            ));
        }

        let mut ir_body = Vec::new();
        for stmt in body {
            ir_body.extend(self.convert_stmt(stmt)?);
        }

        // If function is main and doesn't end with return, add return 0
        if name == "main" {
            let has_return = ir_body.iter().any(|s| matches!(s, Stmt::Return(_)));
            if !has_return {
                ir_body.push(Stmt::Return(Some(Expr::Number(0))));
            }
        }

        Ok(Function {
            name: name.to_string(),
            params: ir_params,
            return_type: return_type.as_ref().map(|t| format!("{:?}", t)),
            resolved_return_type: ret_type,
            body: ir_body,
            attributes: FunctionAttributes::default(),
        })
    }

    // ── Class conversion ────────────────────────────────────

    fn convert_class_decl(
        &mut self,
        name: &str,
        _super_class: &Option<String>,
        body: &[JsClassMember],
    ) -> Result<(Struct, Vec<Function>), String> {
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        for member in body {
            match member {
                JsClassMember::Property {
                    name: fname,
                    type_ann,
                    ..
                } => {
                    let ty = self.js_type_to_ir(&type_ann.clone().unwrap_or(JsType::Inferred));
                    fields.push(StructField {
                        name: fname.clone(),
                        field_type: ty,
                    });
                }
                JsClassMember::Constructor { params, body: cbody } => {
                    // Convert constructor to ClassName::constructor method
                    let mut ir_params = vec![Param::typed(
                        "this".into(),
                        Type::Pointer(Box::new(Type::Struct(name.to_string()))),
                    )];
                    for p in params {
                        let ty =
                            self.js_type_to_ir(&p.type_ann.clone().unwrap_or(JsType::Inferred));
                        ir_params.push(Param::typed(p.name.clone(), ty));
                    }

                    let mut ir_body = Vec::new();
                    for stmt in cbody {
                        ir_body.extend(self.convert_stmt(stmt)?);
                    }

                    methods.push(Function {
                        name: format!("{}::constructor", name),
                        params: ir_params,
                        return_type: Some("void".into()),
                        resolved_return_type: Type::Void,
                        body: ir_body,
                        attributes: FunctionAttributes::default(),
                    });
                }
                JsClassMember::Method {
                    name: mname,
                    params,
                    return_type,
                    body: mbody,
                    ..
                } => {
                    let mut ir_params = vec![Param::typed(
                        "this".into(),
                        Type::Pointer(Box::new(Type::Struct(name.to_string()))),
                    )];
                    for p in params {
                        let ty =
                            self.js_type_to_ir(&p.type_ann.clone().unwrap_or(JsType::Inferred));
                        ir_params.push(Param::typed(p.name.clone(), ty));
                    }

                    let ret_type = return_type
                        .as_ref()
                        .map(|t| self.js_type_to_ir(t))
                        .unwrap_or(Type::Void);

                    let mut ir_body = Vec::new();
                    for stmt in mbody {
                        ir_body.extend(self.convert_stmt(stmt)?);
                    }

                    methods.push(Function {
                        name: format!("{}::{}", name, mname),
                        params: ir_params,
                        return_type: return_type.as_ref().map(|t| format!("{:?}", t)),
                        resolved_return_type: ret_type,
                        body: ir_body,
                        attributes: FunctionAttributes::default(),
                    });
                }
                _ => {} // Getter/Setter can be added later
            }
        }

        let strct = Struct {
            name: name.to_string(),
            fields,
            is_packed: false,
        };

        Ok((strct, methods))
    }

    // ── Statement conversion ────────────────────────────────

    fn convert_stmt(&mut self, stmt: &JsStmt) -> Result<Vec<Stmt>, String> {
        match stmt {
            JsStmt::VarDecl {
                kind: _,
                name,
                type_ann,
                init,
            } => {
                let ty = type_ann
                    .as_ref()
                    .map(|t| self.js_type_to_ir(t))
                    .unwrap_or_else(|| {
                        // Infer type from initializer
                        if let Some(init_expr) = init {
                            self.infer_ir_type(init_expr)
                        } else {
                            Type::I64
                        }
                    });

                // Register variable type
                if let Some(ta) = type_ann {
                    self.var_types.push((name.clone(), ta.clone()));
                }

                let value = init.as_ref().map(|e| self.convert_expr(e)).transpose()?;

                Ok(vec![Stmt::VarDecl {
                    var_type: ty,
                    name: name.clone(),
                    value,
                }])
            }

            JsStmt::Expr(expr) => {
                // Check for console.log special case
                if let Some(print_stmts) = self.try_convert_console_log(expr) {
                    return Ok(print_stmts);
                }
                let ir_expr = self.convert_expr(expr)?;
                Ok(vec![Stmt::Expr(ir_expr)])
            }

            JsStmt::If {
                cond,
                then_body,
                else_body,
            } => {
                let ir_cond = self.convert_expr_to_condition(cond)?;
                let then_stmts = self.convert_stmt(then_body)?;
                let else_stmts = if let Some(eb) = else_body {
                    Some(self.convert_stmt(eb)?)
                } else {
                    None
                };
                Ok(vec![Stmt::If {
                    condition: ir_cond,
                    then_body: then_stmts,
                    else_body: else_stmts,
                }])
            }

            JsStmt::While { cond, body } => {
                let ir_cond = self.convert_expr_to_condition(cond)?;
                let ir_body = self.convert_stmt(body)?;
                Ok(vec![Stmt::While {
                    condition: ir_cond,
                    body: ir_body,
                }])
            }

            JsStmt::DoWhile { body, cond } => {
                let ir_body = self.convert_stmt(body)?;
                let ir_cond = self.convert_expr_to_condition(cond)?;
                Ok(vec![Stmt::DoWhile {
                    body: ir_body,
                    condition: ir_cond,
                }])
            }

            JsStmt::For {
                init,
                cond,
                update,
                body,
            } => {
                // Convert JS for to C-style for using IR While
                let mut result = Vec::new();

                // Init
                if let Some(init_stmt) = init {
                    result.extend(self.convert_stmt(init_stmt)?);
                }

                // While loop with update at end
                let ir_cond = if let Some(c) = cond {
                    self.convert_expr_to_condition(c)?
                } else {
                    Expr::Bool(true)
                };

                let mut loop_body = self.convert_stmt(body)?;

                // Update (as expression statement)
                if let Some(upd) = update {
                    let ir_upd = self.convert_expr(upd)?;
                    loop_body.push(Stmt::Expr(ir_upd));
                }

                result.push(Stmt::While {
                    condition: ir_cond,
                    body: loop_body,
                });

                Ok(result)
            }

            JsStmt::ForOf { decl, iter, body } => {
                // for-of → forEach pattern using While + index
                let ir_iter = self.convert_expr(iter)?;
                let ir_body = self.convert_stmt(body)?;
                Ok(vec![Stmt::ForEach {
                    var: decl.clone(),
                    iterable: ir_iter,
                    body: ir_body,
                }])
            }

            JsStmt::Return(value) => {
                let ir_val = value.as_ref().map(|e| self.convert_expr(e)).transpose()?;
                Ok(vec![Stmt::Return(ir_val)])
            }

            JsStmt::Break(_) => Ok(vec![Stmt::Break]),
            JsStmt::Continue(_) => Ok(vec![Stmt::Continue]),

            JsStmt::Block(stmts) => {
                let mut result = Vec::new();
                for s in stmts {
                    result.extend(self.convert_stmt(s)?);
                }
                Ok(result)
            }

            JsStmt::Switch { expr, cases } => {
                let ir_expr = self.convert_expr(expr)?;
                let mut ir_cases = Vec::new();
                let mut ir_default = None;

                for case in cases {
                    let mut body = Vec::new();
                    for s in &case.body {
                        body.extend(self.convert_stmt(s)?);
                    }
                    let has_break = body.iter().any(|s| matches!(s, Stmt::Break));

                    if let Some(test) = &case.test {
                        let ir_test = self.convert_expr(test)?;
                        ir_cases.push(SwitchCase {
                            value: ir_test,
                            body,
                            has_break,
                        });
                    } else {
                        ir_default = Some(body);
                    }
                }

                Ok(vec![Stmt::Switch {
                    expr: ir_expr,
                    cases: ir_cases,
                    default: ir_default,
                }])
            }

            JsStmt::Throw(expr) => {
                // Convert throw to print error + return
                let ir_expr = self.convert_expr(expr)?;
                Ok(vec![
                    Stmt::Println(ir_expr),
                    Stmt::Return(Some(Expr::Number(1))),
                ])
            }

            JsStmt::TryCatch {
                try_body,
                catch_body,
                ..
            } => {
                // MVP: just emit try body (no actual exception handling)
                let mut result = Vec::new();
                for s in try_body {
                    result.extend(self.convert_stmt(s)?);
                }
                // Catch body as dead code comment (for future)
                let _ = catch_body;
                Ok(result)
            }

            JsStmt::Empty => Ok(Vec::new()),
            JsStmt::LineMarker(line) => Ok(vec![Stmt::LineMarker(*line)]),

            JsStmt::Labeled { stmt, .. } => self.convert_stmt(stmt),

            // Function/class declarations handled at top level
            JsStmt::FuncDecl { .. } | JsStmt::ClassDecl { .. } => Ok(Vec::new()),

            // Module statements — pass through inner stmt for exports
            JsStmt::Import { .. } => Ok(Vec::new()),
            JsStmt::Export { item } => self.convert_stmt(item),
            JsStmt::ExportDefault(_) => Ok(Vec::new()),

            JsStmt::ForIn { decl, iter, body } => {
                let ir_iter = self.convert_expr(iter)?;
                let ir_body = self.convert_stmt(body)?;
                Ok(vec![Stmt::ForEach {
                    var: decl.clone(),
                    iterable: ir_iter,
                    body: ir_body,
                }])
            }
        }
    }

    // ── Expression conversion ───────────────────────────────

    fn convert_expr(&self, expr: &JsExpr) -> Result<Expr, String> {
        match expr {
            JsExpr::NumberInt(n) => Ok(Expr::Number(*n)),
            JsExpr::NumberFloat(f) => Ok(Expr::Float(*f)),
            JsExpr::StringLit(s) => Ok(Expr::String(s.clone())),
            JsExpr::TemplateLit(segments) => {
                // MVP: concatenate all segments as string
                let mut s = String::new();
                for seg in segments {
                    match seg {
                        TemplateSegment::Str(text) => s.push_str(text),
                        TemplateSegment::Expr(_) => s.push_str("?"),
                    }
                }
                Ok(Expr::String(s))
            }
            JsExpr::Bool(b) => Ok(Expr::Bool(*b)),
            JsExpr::Null => Ok(Expr::Null),
            JsExpr::Undefined => Ok(Expr::Null),
            JsExpr::Identifier(name) => Ok(Expr::Variable(name.clone())),
            JsExpr::This => Ok(Expr::This),

            JsExpr::BinaryOp { op, left, right } => {
                let ir_left = self.convert_expr(left)?;
                let ir_right = self.convert_expr(right)?;

                match op {
                    // Arithmetic → BinaryOp
                    JsBinOp::Add => Ok(Expr::BinaryOp {
                        op: BinOp::Add,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Sub => Ok(Expr::BinaryOp {
                        op: BinOp::Sub,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Mul => Ok(Expr::BinaryOp {
                        op: BinOp::Mul,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Div => Ok(Expr::BinaryOp {
                        op: BinOp::Div,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Mod => Ok(Expr::BinaryOp {
                        op: BinOp::Mod,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),

                    // Comparison → Comparison
                    JsBinOp::EqStrict => Ok(Expr::Comparison {
                        op: CmpOp::Eq,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::NeStrict => Ok(Expr::Comparison {
                        op: CmpOp::Ne,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Lt => Ok(Expr::Comparison {
                        op: CmpOp::Lt,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Gt => Ok(Expr::Comparison {
                        op: CmpOp::Gt,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Le => Ok(Expr::Comparison {
                        op: CmpOp::Le,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Ge => Ok(Expr::Comparison {
                        op: CmpOp::Ge,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),

                    // Logical → BinaryOp (And/Or)
                    JsBinOp::And => Ok(Expr::BinaryOp {
                        op: BinOp::And,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Or => Ok(Expr::BinaryOp {
                        op: BinOp::Or,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),

                    // Bitwise → BitwiseOp
                    JsBinOp::BitAnd => Ok(Expr::BitwiseOp {
                        op: BitwiseOp::And,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::BitOr => Ok(Expr::BitwiseOp {
                        op: BitwiseOp::Or,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::BitXor => Ok(Expr::BitwiseOp {
                        op: BitwiseOp::Xor,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Shl => Ok(Expr::BitwiseOp {
                        op: BitwiseOp::LeftShift,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),
                    JsBinOp::Shr | JsBinOp::UShr => Ok(Expr::BitwiseOp {
                        op: BitwiseOp::RightShift,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),

                    // Blocked: == and != (use === and !==)
                    JsBinOp::Eq => Err(
                        "JsDead-BIB: '==' is blocked — use '===' (strict equality). Respeta los bits."
                            .into(),
                    ),
                    JsBinOp::Ne => Err(
                        "JsDead-BIB: '!=' is blocked — use '!==' (strict inequality). Respeta los bits."
                            .into(),
                    ),

                    // Power → convert to multiplication (MVP)
                    JsBinOp::Pow => Ok(Expr::BinaryOp {
                        op: BinOp::Mul,
                        left: Box::new(ir_left),
                        right: Box::new(ir_right),
                    }),

                    // Nullish → ternary (value ?? default)
                    JsBinOp::Nullish => Ok(Expr::Ternary {
                        condition: Box::new(Expr::Comparison {
                            op: CmpOp::Ne,
                            left: Box::new(ir_left.clone()),
                            right: Box::new(Expr::Null),
                        }),
                        then_expr: Box::new(ir_left),
                        else_expr: Box::new(ir_right),
                    }),

                    // instanceof / in → not supported yet
                    JsBinOp::Instanceof | JsBinOp::In => {
                        Ok(Expr::Bool(false))
                    }
                }
            }

            JsExpr::UnaryOp { op, expr: inner } => {
                let ir_inner = self.convert_expr(inner)?;
                match op {
                    JsUnOp::Neg => Ok(Expr::UnaryOp {
                        op: UnaryOp::Neg,
                        expr: Box::new(ir_inner),
                    }),
                    JsUnOp::Not => Ok(Expr::UnaryOp {
                        op: UnaryOp::Not,
                        expr: Box::new(ir_inner),
                    }),
                    JsUnOp::BitNot => Ok(Expr::BitwiseNot(Box::new(ir_inner))),
                    JsUnOp::PreInc => Ok(Expr::PreIncrement(Box::new(ir_inner))),
                    JsUnOp::PreDec => Ok(Expr::PreDecrement(Box::new(ir_inner))),
                    JsUnOp::PostInc => Ok(Expr::PostIncrement(Box::new(ir_inner))),
                    JsUnOp::PostDec => Ok(Expr::PostDecrement(Box::new(ir_inner))),
                    JsUnOp::Typeof | JsUnOp::Void | JsUnOp::Delete => {
                        Ok(Expr::Number(0)) // MVP placeholder
                    }
                }
            }

            JsExpr::MemberAccess {
                object, property, ..
            } => {
                let ir_obj = self.convert_expr(object)?;
                Ok(Expr::FieldAccess {
                    object: Box::new(ir_obj),
                    field: property.clone(),
                })
            }

            JsExpr::ComputedAccess { object, index } => {
                let ir_obj = self.convert_expr(object)?;
                let ir_idx = self.convert_expr(index)?;
                Ok(Expr::Index {
                    object: Box::new(ir_obj),
                    index: Box::new(ir_idx),
                })
            }

            JsExpr::Call { callee, args } => {
                // Check for console.log
                if let JsExpr::MemberAccess {
                    object, property, ..
                } = callee.as_ref()
                {
                    if let JsExpr::Identifier(obj) = object.as_ref() {
                        if obj == "console" && property == "log" {
                            // Return first arg as a print expression
                            if let Some(first) = args.first() {
                                return self.convert_expr(first);
                            }
                            return Ok(Expr::Number(0));
                        }
                    }
                }

                let name = self.extract_call_name(callee);
                let ir_args: Result<Vec<Expr>, String> =
                    args.iter().map(|a| self.convert_expr(a)).collect();
                Ok(Expr::Call {
                    name,
                    args: ir_args?,
                })
            }

            JsExpr::New { callee, args } => {
                let name = self.extract_call_name(callee);
                let ir_args: Result<Vec<Expr>, String> =
                    args.iter().map(|a| self.convert_expr(a)).collect();
                Ok(Expr::New {
                    class_name: name,
                    args: ir_args?,
                })
            }

            JsExpr::ArrayLit(elements) => {
                let ir_elems: Result<Vec<Expr>, String> =
                    elements.iter().map(|e| self.convert_expr(e)).collect();
                Ok(Expr::Array(ir_elems?))
            }

            JsExpr::ObjectLit(_props) => {
                // MVP: objects not fully supported yet
                Ok(Expr::Number(0))
            }

            JsExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                let ir_cond = self.convert_expr_to_condition(cond)?;
                let ir_then = self.convert_expr(then_expr)?;
                let ir_else = self.convert_expr(else_expr)?;
                Ok(Expr::Ternary {
                    condition: Box::new(ir_cond),
                    then_expr: Box::new(ir_then),
                    else_expr: Box::new(ir_else),
                })
            }

            JsExpr::Assign { target, op, value } => {
                let ir_target = self.convert_expr(target)?;
                let ir_value = self.convert_expr(value)?;

                match op {
                    JsAssignOp::Eq => {
                        // Simple assignment — returned as expression
                        // The caller (convert_stmt) handles this
                        Ok(ir_value)
                    }
                    _ => {
                        // Compound assignment
                        let bin_op = match op {
                            JsAssignOp::Add => BinOp::Add,
                            JsAssignOp::Sub => BinOp::Sub,
                            JsAssignOp::Mul => BinOp::Mul,
                            JsAssignOp::Div => BinOp::Div,
                            JsAssignOp::Mod => BinOp::Mod,
                            JsAssignOp::Eq => unreachable!(),
                        };
                        Ok(Expr::BinaryOp {
                            op: bin_op,
                            left: Box::new(ir_target),
                            right: Box::new(ir_value),
                        })
                    }
                }
            }

            JsExpr::ArrowFunc { params, body } => {
                let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
                match body {
                    JsArrowBody::Expr(e) => {
                        let ir_body = self.convert_expr(e)?;
                        Ok(Expr::Lambda {
                            params: param_names,
                            body: Box::new(ir_body),
                        })
                    }
                    JsArrowBody::Block(_) => {
                        // Block arrow functions: not fully supported as expressions
                        Ok(Expr::Lambda {
                            params: param_names,
                            body: Box::new(Expr::Number(0)),
                        })
                    }
                }
            }

            JsExpr::FuncExpr { .. } => Ok(Expr::Number(0)), // MVP placeholder

            JsExpr::Spread(inner) => self.convert_expr(inner),
            JsExpr::Await(inner) => self.convert_expr(inner),
            JsExpr::Typeof(_) => Ok(Expr::String("unknown".into())),
            JsExpr::VoidExpr(_) => Ok(Expr::Number(0)),
        }
    }

    // ── console.log → Print ─────────────────────────────────

    fn try_convert_console_log(&self, expr: &JsExpr) -> Option<Vec<Stmt>> {
        if let JsExpr::Call { callee, args } = expr {
            if let JsExpr::MemberAccess {
                object, property, ..
            } = callee.as_ref()
            {
                if let JsExpr::Identifier(obj) = object.as_ref() {
                    if obj == "console" && property == "log" {
                        let mut stmts = Vec::new();
                        for (i, arg) in args.iter().enumerate() {
                            if let Ok(ir_expr) = self.convert_expr(arg) {
                                if i == args.len() - 1 {
                                    stmts.push(Stmt::Println(ir_expr));
                                } else {
                                    stmts.push(Stmt::Print(ir_expr));
                                }
                            }
                        }
                        return Some(stmts);
                    }
                }
            }
        }
        None
    }

    // ── Expression to condition ──────────────────────────────

    fn convert_expr_to_condition(&self, expr: &JsExpr) -> Result<Expr, String> {
        match expr {
            JsExpr::BinaryOp { op, left, right } => {
                let is_comparison = matches!(
                    op,
                    JsBinOp::EqStrict
                        | JsBinOp::NeStrict
                        | JsBinOp::Eq
                        | JsBinOp::Ne
                        | JsBinOp::Lt
                        | JsBinOp::Gt
                        | JsBinOp::Le
                        | JsBinOp::Ge
                );

                if is_comparison {
                    self.convert_expr(expr)
                } else {
                    // Logical ops
                    let ir_left = self.convert_expr(left)?;
                    let ir_right = self.convert_expr(right)?;
                    match op {
                        JsBinOp::And => Ok(Expr::BinaryOp {
                            op: BinOp::And,
                            left: Box::new(ir_left),
                            right: Box::new(ir_right),
                        }),
                        JsBinOp::Or => Ok(Expr::BinaryOp {
                            op: BinOp::Or,
                            left: Box::new(ir_left),
                            right: Box::new(ir_right),
                        }),
                        _ => {
                            // Truthy check: expr != 0
                            let ir = self.convert_expr(expr)?;
                            Ok(Expr::Comparison {
                                op: CmpOp::Ne,
                                left: Box::new(ir),
                                right: Box::new(Expr::Number(0)),
                            })
                        }
                    }
                }
            }
            _ => {
                // Convert to truthy: expr != 0
                let ir = self.convert_expr(expr)?;
                Ok(Expr::Comparison {
                    op: CmpOp::Ne,
                    left: Box::new(ir),
                    right: Box::new(Expr::Number(0)),
                })
            }
        }
    }

    // ── Helpers ──────────────────────────────────────────────

    fn extract_call_name(&self, callee: &JsExpr) -> String {
        match callee {
            JsExpr::Identifier(name) => name.clone(),
            JsExpr::MemberAccess {
                object, property, ..
            } => {
                let obj = self.extract_call_name(object);
                format!("{}.{}", obj, property)
            }
            _ => "__unknown_call".into(),
        }
    }

    fn js_type_to_ir(&self, ty: &JsType) -> Type {
        match ty {
            JsType::Int | JsType::Number => Type::I64,
            JsType::Float => Type::F64,
            JsType::String => Type::Str,
            JsType::Boolean => Type::Bool,
            JsType::Void => Type::Void,
            JsType::Null | JsType::Undefined => Type::Void,
            JsType::Array(inner) => {
                let ir_inner = self.js_type_to_ir(inner);
                Type::Array(Box::new(ir_inner), None)
            }
            JsType::Named(name) => Type::Struct(name.clone()),
            JsType::Function { .. } => Type::I64, // function pointer
            JsType::Object(_) => Type::I64,        // object pointer
            JsType::Inferred => Type::I64,          // default to i64
        }
    }

    fn infer_ir_type(&self, expr: &JsExpr) -> Type {
        match expr {
            JsExpr::NumberInt(_) => Type::I64,
            JsExpr::NumberFloat(_) => Type::F64,
            JsExpr::StringLit(_) | JsExpr::TemplateLit(_) => Type::Str,
            JsExpr::Bool(_) => Type::Bool,
            JsExpr::Null | JsExpr::Undefined => Type::I64,
            JsExpr::ArrayLit(_) => Type::Array(Box::new(Type::I64), None),
            JsExpr::Identifier(name) => {
                for (vn, vt) in &self.var_types {
                    if vn == name {
                        return self.js_type_to_ir(vt);
                    }
                }
                Type::I64
            }
            JsExpr::BinaryOp { left, right, .. } => {
                let lt = self.infer_ir_type(left);
                let rt = self.infer_ir_type(right);
                if lt == Type::F64 || rt == Type::F64 {
                    Type::F64
                } else {
                    lt
                }
            }
            JsExpr::Call { .. } => Type::I64,
            _ => Type::I64,
        }
    }
}

// ── Public API ──────────────────────────────────────────────

/// Convenience: parse JS source → ADead-BIB Program in one call
/// Full pipeline: Lexer → Parser → JS AST → IR
pub fn compile_js_to_program(source: &str) -> Result<Program, String> {
    // Phase 1: Lex
    let mut lexer = JsLexer::new(source);
    let (tokens, lines) = lexer.tokenize();

    // Phase 2: Parse
    let mut parser = JsParser::new(tokens, lines);
    let js_program = parser.parse()?;

    // Phase 3: Convert to IR
    let mut converter = JsToIR::new();
    converter.convert(&js_program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world_js() {
        let program = compile_js_to_program(
            r#"
            console.log("Hello from JsDead-BIB");
        "#,
        )
        .unwrap();
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "main");
    }

    #[test]
    fn test_variable_declaration() {
        let program = compile_js_to_program(
            r#"
            let x: int = 5;
            let y: int = 3;
            let z: int = x + y;
            console.log(z);
        "#,
        )
        .unwrap();
        assert_eq!(program.functions.len(), 1);
        assert!(!program.functions[0].body.is_empty());
    }

    #[test]
    fn test_function_declaration() {
        let program = compile_js_to_program(
            r#"
            function add(a: int, b: int): int {
                return a + b;
            }
            function main(): void {
                let result: int = add(5, 3);
                console.log(result);
            }
        "#,
        )
        .unwrap();
        assert_eq!(program.functions.len(), 2);
    }

    #[test]
    fn test_class_declaration() {
        let program = compile_js_to_program(
            r#"
            class Point {
                x: int
                y: int
                constructor(x: int, y: int) {
                    this.x = x;
                    this.y = y;
                }
                distanceSquared(): int {
                    return this.x * this.x + this.y * this.y;
                }
            }
        "#,
        )
        .unwrap();
        assert!(!program.structs.is_empty());
        assert_eq!(program.structs[0].name, "Point");
    }

    #[test]
    fn test_strict_equality_blocked() {
        let result = compile_js_to_program(
            r#"
            let x: int = 5;
            if (x == 5) { console.log("yes"); }
        "#,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("=="));
    }

    #[test]
    fn test_for_loop() {
        let program = compile_js_to_program(
            r#"
            function main(): void {
                for (let i: int = 0; i < 10; i++) {
                    console.log(i);
                }
            }
        "#,
        )
        .unwrap();
        assert_eq!(program.functions.len(), 1);
    }
}
