// ============================================================
// ISA Compiler — Main compile() and string collection
// ============================================================

use crate::frontend::ast::*;
use crate::isa::ADeadOp;
use crate::isa::encoder::Encoder;
use super::core::{IsaCompiler, CompiledFunction};

impl IsaCompiler {
    /// Compila un programa completo
    pub fn compile(&mut self, program: &Program) -> (Vec<u8>, Vec<u8>, Vec<usize>, Vec<usize>) {
        // Fase 1: Recolectar strings
        self.collect_all_strings(program);
        self.collect_strings_from_stmts(&program.statements);

        // Fase 2: Registrar labels de funciones
        let has_main = program.functions.iter().any(|f| f.name == "main");
        for func in &program.functions {
            let label = self.ir.new_label();
            self.functions.insert(func.name.clone(), CompiledFunction {
                name: func.name.clone(),
                label,
                params: func.params.iter().map(|p| p.name.clone()).collect(),
            });
        }

        // Fase 3: Si hay main, saltar a main
        let main_label = self.functions.get("main").map(|f| f.label);
        let needs_jmp = has_main && (program.functions.len() > 1 || !program.statements.is_empty());
        if needs_jmp {
            if let Some(lbl) = main_label {
                self.ir.emit(ADeadOp::Jmp { target: lbl });
            }
        }

        // Fase 4: Compilar funciones auxiliares
        for func in &program.functions {
            if func.name != "main" {
                self.compile_function(func);
            }
        }

        // Fase 5: Compilar top-level statements
        if !has_main && !program.statements.is_empty() {
            self.compile_top_level(&program.statements);
        }

        // Fase 6: Compilar main
        for func in &program.functions {
            if func.name == "main" {
                self.compile_function(func);
            }
        }

        // Fase 7: Encode ADeadIR → bytes
        let mut encoder = Encoder::new();
        let result = encoder.encode_all(self.ir.ops());

        // Fase 8: Resolver llamadas por nombre
        let code = result.code;
        for (offset, name) in &result.unresolved_calls {
            if let Some(func) = self.functions.get(name) {
                let _ = (offset, func);
            }
        }

        // Fase 9: Generar sección de datos
        let data = self.generate_data_section();

        (code, data, result.iat_call_offsets, result.string_imm64_offsets)
    }

    pub(crate) fn collect_all_strings(&mut self, program: &Program) {
        self.strings.push("%d".to_string());
        self.strings.push("%s".to_string());
        self.strings.push("%.2f".to_string());
        self.strings.push("\n".to_string());

        for func in &program.functions {
            self.collect_strings_from_stmts(&func.body);
        }

        let mut offset = 0u64;
        for s in &self.strings {
            self.string_offsets.insert(s.clone(), offset);
            offset += s.len() as u64 + 1;
        }
    }

    pub(crate) fn collect_strings_from_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::String(s) => {
                let processed = s.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
                if !self.strings.contains(&processed) {
                    self.strings.push(processed);
                }
            }
            Expr::BinaryOp { left, right, .. } => {
                self.collect_strings_from_expr(left);
                self.collect_strings_from_expr(right);
            }
            Expr::UnaryOp { expr: inner, .. } => {
                self.collect_strings_from_expr(inner);
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.collect_strings_from_expr(arg);
                }
            }
            Expr::Comparison { left, right, .. } => {
                self.collect_strings_from_expr(left);
                self.collect_strings_from_expr(right);
            }
            Expr::Ternary { condition, then_expr, else_expr } => {
                self.collect_strings_from_expr(condition);
                self.collect_strings_from_expr(then_expr);
                self.collect_strings_from_expr(else_expr);
            }
            Expr::MethodCall { object, args, .. } => {
                self.collect_strings_from_expr(object);
                for arg in args {
                    self.collect_strings_from_expr(arg);
                }
            }
            Expr::Index { object, index } => {
                self.collect_strings_from_expr(object);
                self.collect_strings_from_expr(index);
            }
            Expr::FieldAccess { object, .. } => {
                self.collect_strings_from_expr(object);
            }
            Expr::Array(elems) => {
                for e in elems {
                    self.collect_strings_from_expr(e);
                }
            }
            Expr::New { args, .. } => {
                for arg in args {
                    self.collect_strings_from_expr(arg);
                }
            }
            _ => {}
        }
    }

    pub(crate) fn collect_strings_from_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::Print(expr) | Stmt::Println(expr) | Stmt::PrintNum(expr) => {
                    self.collect_strings_from_expr(expr);
                }
                Stmt::Assign { value, .. } => {
                    self.collect_strings_from_expr(value);
                }
                Stmt::VarDecl { value, .. } => {
                    if let Some(val) = value {
                        self.collect_strings_from_expr(val);
                    }
                }
                Stmt::If { condition, then_body, else_body } => {
                    self.collect_strings_from_expr(condition);
                    self.collect_strings_from_stmts(then_body);
                    if let Some(else_stmts) = else_body {
                        self.collect_strings_from_stmts(else_stmts);
                    }
                }
                Stmt::While { condition, body } => {
                    self.collect_strings_from_expr(condition);
                    self.collect_strings_from_stmts(body);
                }
                Stmt::DoWhile { body, condition } => {
                    self.collect_strings_from_stmts(body);
                    self.collect_strings_from_expr(condition);
                }
                Stmt::For { start, end, body, .. } => {
                    self.collect_strings_from_expr(start);
                    self.collect_strings_from_expr(end);
                    self.collect_strings_from_stmts(body);
                }
                Stmt::ForEach { iterable, body, .. } => {
                    self.collect_strings_from_expr(iterable);
                    self.collect_strings_from_stmts(body);
                }
                Stmt::Return(Some(expr)) => {
                    self.collect_strings_from_expr(expr);
                }
                Stmt::Expr(expr) => {
                    self.collect_strings_from_expr(expr);
                }
                Stmt::CompoundAssign { value, .. } => {
                    self.collect_strings_from_expr(value);
                }
                Stmt::IndexAssign { object, index, value } => {
                    self.collect_strings_from_expr(object);
                    self.collect_strings_from_expr(index);
                    self.collect_strings_from_expr(value);
                }
                Stmt::FieldAssign { object, value, .. } => {
                    self.collect_strings_from_expr(object);
                    self.collect_strings_from_expr(value);
                }
                _ => {}
            }
        }
    }

    pub(crate) fn generate_data_section(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for s in &self.strings {
            data.extend_from_slice(s.as_bytes());
            data.push(0);
        }
        data
    }
}
