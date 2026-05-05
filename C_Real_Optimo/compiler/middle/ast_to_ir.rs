//! AST → IR Converter
//! Transforma el AST de C a representación intermedia
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::frontend::ast::*;
use super::ir::*;
use std::collections::HashMap;

pub struct AstToIr {
    builder: IrBuilder,
    vars: HashMap<String, (IrReg, IrType)>,  // name -> (ptr, type)
    funcs: HashMap<String, IrType>,           // func name -> return type
}

impl AstToIr {
    pub fn new(module_name: &str) -> Self {
        Self {
            builder: IrBuilder::new(module_name),
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }
    
    pub fn convert(mut self, unit: &TranslationUnit) -> IrModule {
        // First pass: register all functions
        for item in &unit.items {
            if let TopLevel::Func(f) = item {
                let ret_ty = self.convert_type(&f.ret_type);
                self.funcs.insert(f.name.clone(), ret_ty);
            }
        }
        
        // Second pass: generate IR
        for item in &unit.items {
            match item {
                TopLevel::Func(f) => self.convert_function(f),
                TopLevel::Var(v) => self.convert_global_var(v),
                _ => {}
            }
        }
        
        self.builder.finish()
    }
    
    fn convert_function(&mut self, func: &FuncDecl) {
        self.vars.clear();
        
        let ret_ty = self.convert_type(&func.ret_type);
        let params: Vec<(String, IrType)> = func.params.iter()
            .filter_map(|p| p.name.clone().map(|n| (n, self.convert_type(&p.ty))))
            .collect();
        
        self.builder.begin_function(&func.name, params.clone(), ret_ty);
        
        // Allocate params as local vars
        for (i, (name, ty)) in params.iter().enumerate() {
            let ptr = self.builder.alloca(*ty);
            self.builder.store(IrValue::Reg(ptr), IrValue::Param(i as u32));
            self.vars.insert(name.clone(), (ptr, *ty));
        }
        
        // Convert body
        if let Some(body) = &func.body {
            for stmt in body {
                self.convert_stmt(stmt);
            }
        }
        
        // Add implicit return if needed
        if ret_ty == IrType::Void {
            self.builder.ret(None);
        }
        
        self.builder.end_function();
    }
    
    fn convert_global_var(&mut self, decl: &Decl) {
        // Global variables - stored in data section
        let ty = self.convert_type(&decl.ty);
        self.builder.add_global(&decl.name, ty);
    }
    
    fn convert_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(e) => { self.convert_expr(e); }
            
            Stmt::Return(val) => {
                let v = val.as_ref().map(|e| self.convert_expr(e));
                self.builder.ret(v);
            }
            
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.convert_stmt(s);
                }
            }
            
            Stmt::Decl(decl) => {
                let ty = self.convert_type(&decl.ty);
                let ptr = self.builder.alloca(ty);
                
                if let Some(init) = &decl.init {
                    let val = self.convert_expr(init);
                    self.builder.store(IrValue::Reg(ptr), val);
                }
                
                self.vars.insert(decl.name.clone(), (ptr, ty));
            }
            
            Stmt::If(cond, then_s, else_s) => {
                let cond_val = self.convert_expr(cond);
                let then_bb = self.builder.new_block("then");
                let else_bb = self.builder.new_block("else");
                let merge_bb = self.builder.new_block("merge");
                
                self.builder.jmp_if(cond_val, then_bb, else_bb);
                
                self.builder.set_block(then_bb);
                self.convert_stmt(then_s);
                self.builder.jmp(merge_bb);
                
                self.builder.set_block(else_bb);
                if let Some(e) = else_s {
                    self.convert_stmt(e);
                }
                self.builder.jmp(merge_bb);
                
                self.builder.set_block(merge_bb);
            }
            
            Stmt::While(cond, body) => {
                let cond_bb = self.builder.new_block("while_cond");
                let body_bb = self.builder.new_block("while_body");
                let exit_bb = self.builder.new_block("while_exit");
                
                self.builder.jmp(cond_bb);
                
                self.builder.set_block(cond_bb);
                let cond_val = self.convert_expr(cond);
                self.builder.jmp_if(cond_val, body_bb, exit_bb);
                
                self.builder.set_block(body_bb);
                self.convert_stmt(body);
                self.builder.jmp(cond_bb);
                
                self.builder.set_block(exit_bb);
            }
            
            Stmt::For(init, cond, inc, body) => {
                if let Some(i) = init { self.convert_stmt(i); }
                
                let cond_bb = self.builder.new_block("for_cond");
                let body_bb = self.builder.new_block("for_body");
                let exit_bb = self.builder.new_block("for_exit");
                
                self.builder.jmp(cond_bb);
                
                self.builder.set_block(cond_bb);
                if let Some(c) = cond {
                    let cond_val = self.convert_expr(c);
                    self.builder.jmp_if(cond_val, body_bb, exit_bb);
                } else {
                    self.builder.jmp(body_bb);
                }
                
                self.builder.set_block(body_bb);
                self.convert_stmt(body);
                if let Some(i) = inc { self.convert_expr(i); }
                self.builder.jmp(cond_bb);
                
                self.builder.set_block(exit_bb);
            }
            
            _ => {}
        }
    }
    
    fn convert_expr(&mut self, expr: &Expr) -> IrValue {
        match expr {
            Expr::IntLit(n) => IrValue::Const(IrConst::I32(*n as i32)),
            Expr::FloatLit(f) => IrValue::Const(IrConst::F64(*f)),
            Expr::CharLit(c) => IrValue::Const(IrConst::I8(*c as i8)),
            Expr::StringLit(s) => {
                let idx = self.builder.add_string(s);
                IrValue::Global(format!("__str_{}", idx))
            }
            
            Expr::Ident(name) => {
                if let Some((ptr, ty)) = self.vars.get(name) {
                    let reg = self.builder.load(*ty, IrValue::Reg(*ptr));
                    IrValue::Reg(reg)
                } else {
                    IrValue::Global(name.clone())
                }
            }
            
            Expr::Binary(op, lhs, rhs) => {
                let l = self.convert_expr(lhs);
                let r = self.convert_expr(rhs);
                let ty = IrType::I32; // simplified
                
                let reg = match op {
                    BinOp::Add    => self.builder.add(ty, l, r),
                    BinOp::Sub    => self.builder.sub(ty, l, r),
                    BinOp::Mul    => self.builder.mul(ty, l, r),
                    BinOp::Div    => self.builder.div(ty, l, r),
                    BinOp::Mod    => self.builder.rem(ty, l, r),
                    BinOp::BitAnd => self.builder.bit_and(ty, l, r),
                    BinOp::BitOr  => self.builder.bit_or(ty, l, r),
                    BinOp::BitXor => self.builder.bit_xor(ty, l, r),
                    BinOp::Shl    => self.builder.shl(ty, l, r),
                    BinOp::Shr    => self.builder.shr(ty, l, r),
                    BinOp::Eq     => self.builder.cmp_eq(l, r),
                    BinOp::Ne     => self.builder.cmp_ne(l, r),
                    BinOp::Lt     => self.builder.cmp_lt(l, r),
                    BinOp::Le     => self.builder.cmp_le(l, r),
                    BinOp::Gt     => self.builder.cmp_gt(l, r),
                    BinOp::Ge     => self.builder.cmp_ge(l, r),
                    BinOp::LogAnd => self.builder.bit_and(ty, l, r),
                    BinOp::LogOr  => self.builder.bit_or(ty, l, r),
                };
                IrValue::Reg(reg)
            }
            
            Expr::Unary(op, inner) => {
                let v = self.convert_expr(inner);
                let ty = IrType::I32;
                let reg = match op {
                    UnaryOp::Neg    => self.builder.neg(ty, v),
                    UnaryOp::BitNot => self.builder.bit_not(ty, v),
                    UnaryOp::Not    => self.builder.cmp_eq(v, IrValue::Const(IrConst::I32(0))),
                };
                IrValue::Reg(reg)
            }
            
            Expr::CompoundAssign(op, lhs, rhs) => {
                // x += rhs  →  x = x + rhs
                let r = self.convert_expr(rhs);
                if let Expr::Ident(name) = lhs.as_ref() {
                    if let Some(&(ptr, ty_var)) = self.vars.get(name) {
                        let ir_ty = ty_var;
                        let cur = self.builder.load(ir_ty, IrValue::Reg(ptr));
                        let new_reg = match op {
                            BinOp::Add => self.builder.add(ir_ty, IrValue::Reg(cur), r),
                            BinOp::Sub => self.builder.sub(ir_ty, IrValue::Reg(cur), r),
                            BinOp::Mul => self.builder.mul(ir_ty, IrValue::Reg(cur), r),
                            BinOp::Div => self.builder.div(ir_ty, IrValue::Reg(cur), r),
                            BinOp::Mod => self.builder.rem(ir_ty, IrValue::Reg(cur), r),
                            BinOp::BitAnd => self.builder.bit_and(ir_ty, IrValue::Reg(cur), r),
                            BinOp::BitOr  => self.builder.bit_or(ir_ty, IrValue::Reg(cur), r),
                            BinOp::BitXor => self.builder.bit_xor(ir_ty, IrValue::Reg(cur), r),
                            BinOp::Shl    => self.builder.shl(ir_ty, IrValue::Reg(cur), r),
                            BinOp::Shr    => self.builder.shr(ir_ty, IrValue::Reg(cur), r),
                            _ => self.builder.add(ir_ty, IrValue::Reg(cur), r),
                        };
                        self.builder.store(IrValue::Reg(ptr), IrValue::Reg(new_reg));
                        return IrValue::Reg(new_reg);
                    }
                }
                IrValue::Const(IrConst::I32(0))
            }
            
            Expr::PreInc(inner) | Expr::PostInc(inner) => {
                if let Expr::Ident(name) = inner.as_ref() {
                    if let Some(&(ptr, ty_var)) = self.vars.get(name) {
                        let cur = self.builder.load(ty_var, IrValue::Reg(ptr));
                        let new_reg = self.builder.add(ty_var, IrValue::Reg(cur), IrValue::Const(IrConst::I32(1)));
                        self.builder.store(IrValue::Reg(ptr), IrValue::Reg(new_reg));
                        return match expr { Expr::PreInc(_) => IrValue::Reg(new_reg), _ => IrValue::Reg(cur) };
                    }
                }
                IrValue::Const(IrConst::I32(0))
            }
            
            Expr::PreDec(inner) | Expr::PostDec(inner) => {
                if let Expr::Ident(name) = inner.as_ref() {
                    if let Some(&(ptr, ty_var)) = self.vars.get(name) {
                        let cur = self.builder.load(ty_var, IrValue::Reg(ptr));
                        let new_reg = self.builder.sub(ty_var, IrValue::Reg(cur), IrValue::Const(IrConst::I32(1)));
                        self.builder.store(IrValue::Reg(ptr), IrValue::Reg(new_reg));
                        return match expr { Expr::PreDec(_) => IrValue::Reg(new_reg), _ => IrValue::Reg(cur) };
                    }
                }
                IrValue::Const(IrConst::I32(0))
            }
            
            Expr::Call(func_expr, args) => {
                let args_ir: Vec<IrValue> = args.iter()
                    .map(|a| self.convert_expr(a))
                    .collect();
                
                let func_name = match func_expr.as_ref() {
                    Expr::Ident(name) => name.clone(),
                    _ => "unknown".to_string(),
                };
                
                let ret_ty = self.funcs.get(&func_name)
                    .copied()
                    .unwrap_or(IrType::I32);
                
                if let Some(reg) = self.builder.call(ret_ty, func_name, args_ir) {
                    IrValue::Reg(reg)
                } else {
                    IrValue::Const(IrConst::I32(0))
                }
            }
            
            Expr::Assign(lhs, rhs) => {
                let val = self.convert_expr(rhs);
                if let Expr::Ident(name) = lhs.as_ref() {
                    if let Some((ptr, _)) = self.vars.get(name) {
                        self.builder.store(IrValue::Reg(*ptr), val.clone());
                    }
                }
                val
            }
            
            // ============================================================
            // B-02 (FASE T2): Ternary lowering — `cond ? a : b`
            // ============================================================
            // Estrategia: alloca + if/else + store en ambos brazos +
            // load del slot temporal en el bloque merge.
            //
            //   %tmp = alloca i32
            //   if cond { jmp then_bb } else { jmp else_bb }
            // then_bb:
            //   store %tmp, then_val
            //   jmp merge_bb
            // else_bb:
            //   store %tmp, else_val
            //   jmp merge_bb
            // merge_bb:
            //   %result = load %tmp
            // ============================================================
            Expr::Ternary(cond, then_e, else_e) => {
                // 1. Reservar slot temporal
                let tmp = self.builder.alloca(IrType::I32);

                // 2. Evaluar condición ANTES de crear los bloques
                let cond_val = self.convert_expr(cond);

                // 3. Crear bloques then / else / merge
                let then_bb  = self.builder.new_block("ter_then");
                let else_bb  = self.builder.new_block("ter_else");
                let merge_bb = self.builder.new_block("ter_merge");

                // 4. Branch condicional
                self.builder.jmp_if(cond_val, then_bb, else_bb);

                // 5. Brazo THEN
                self.builder.set_block(then_bb);
                let then_val = self.convert_expr(then_e);
                self.builder.store(IrValue::Reg(tmp), then_val);
                self.builder.jmp(merge_bb);

                // 6. Brazo ELSE
                self.builder.set_block(else_bb);
                let else_val = self.convert_expr(else_e);
                self.builder.store(IrValue::Reg(tmp), else_val);
                self.builder.jmp(merge_bb);

                // 7. Bloque merge: cargar el resultado
                self.builder.set_block(merge_bb);
                let result = self.builder.load(IrType::I32, IrValue::Reg(tmp));
                IrValue::Reg(result)
            }
            
            _ => IrValue::Const(IrConst::I32(0)),
        }
    }
    
    fn convert_type(&self, ty: &Type) -> IrType {
        match ty {
            Type::Void => IrType::Void,
            Type::Bool | Type::Char => IrType::I8,
            Type::Short => IrType::I16,
            Type::Int => IrType::I32,
            Type::Long | Type::LongLong => IrType::I64,
            Type::Float => IrType::F32,
            Type::Double => IrType::F64,
            Type::Pointer(_) => IrType::Ptr(IrPtrType::Void),
            Type::Signed(t) | Type::Unsigned(t) => self.convert_type(t),
            _ => IrType::I32,
        }
    }
}

pub fn ast_to_ir(unit: &TranslationUnit) -> IrModule {
    AstToIr::new("main").convert(unit)
}
