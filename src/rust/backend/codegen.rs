// Code Generator - Emite opcodes directamente desde AST
// NO ASM - escribimos bytes directamente
// Binario + HEX = ADead-BIB
// Lenguaje de uso general

use crate::frontend::ast::*;
use std::collections::HashMap;

// Layout del PE:
// 0x0000 - Headers
// 0x1000 - .text (código)
// 0x2000 - .rdata (imports + strings)

const IMAGE_BASE: u64 = 0x400000;
const TEXT_RVA: u64 = 0x1000;

// Offsets dentro de .rdata:
// 0x2000: Import Directory
// 0x2038: IAT (printf)
// 0x2060: Strings de usuario

const IAT_PRINTF_RVA: u64 = 0x2038;
const STRING_START_RVA: u64 = 0x2060;

pub struct CodeGenerator {
    code: Vec<u8>,
    strings: Vec<String>,
    variables: HashMap<String, i32>,  // nombre -> offset desde RBP
    stack_offset: i32,
    label_counter: u32,
}

impl CodeGenerator {
    pub fn new(_base_address: u64) -> Self {
        Self {
            code: Vec::new(),
            strings: Vec::new(),
            variables: HashMap::new(),
            stack_offset: 0,
            label_counter: 0,
        }
    }

    fn new_label(&mut self) -> u32 {
        let label = self.label_counter;
        self.label_counter += 1;
        label
    }

    pub fn generate(&mut self, program: &Program) -> (Vec<u8>, Vec<u8>) {
        self.collect_strings(program);
        
        for func in &program.functions {
            if func.name == "main" {
                self.emit_function(func);
                break;
            }
        }

        let data = self.generate_data_section();
        (self.code.clone(), data)
    }

    fn collect_strings(&mut self, program: &Program) {
        for func in &program.functions {
            self.collect_strings_from_stmts(&func.body);
        }
    }

    fn collect_strings_from_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::Print(Expr::String(s)) => {
                    if !self.strings.contains(s) {
                        self.strings.push(s.clone());
                    }
                }
                Stmt::If { then_body, else_body, .. } => {
                    self.collect_strings_from_stmts(then_body);
                    if let Some(else_stmts) = else_body {
                        self.collect_strings_from_stmts(else_stmts);
                    }
                }
                Stmt::While { body, .. } => {
                    self.collect_strings_from_stmts(body);
                }
                Stmt::For { body, .. } => {
                    self.collect_strings_from_stmts(body);
                }
                _ => {}
            }
        }
    }

    fn get_string_address(&self, s: &str) -> u64 {
        let mut offset = 0u64;
        for stored in &self.strings {
            if stored == s {
                return IMAGE_BASE + STRING_START_RVA + offset;
            }
            offset += stored.len() as u64 + 1;
        }
        IMAGE_BASE + STRING_START_RVA
    }

    fn emit_function(&mut self, func: &Function) {
        // Prologue: push rbp; mov rbp, rsp; sub rsp, 256
        self.emit_bytes(&[0x55]);                    // push rbp
        self.emit_bytes(&[0x48, 0x89, 0xE5]);        // mov rbp, rsp
        self.emit_bytes(&[0x48, 0x81, 0xEC, 0x00, 0x01, 0x00, 0x00]);  // sub rsp, 256 (espacio para variables)
        
        self.stack_offset = -8;
        self.variables.clear();
        
        for stmt in &func.body {
            self.emit_statement(stmt);
        }
        
        // Epilogue: xor eax, eax; mov rsp, rbp; pop rbp; ret
        self.emit_bytes(&[0x31, 0xC0]);              // xor eax, eax
        self.emit_bytes(&[0x48, 0x89, 0xEC]);        // mov rsp, rbp
        self.emit_bytes(&[0x5D]);                    // pop rbp
        self.emit_bytes(&[0xC3]);                    // ret
    }

    fn emit_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => self.emit_print_string(expr),
            Stmt::PrintNum(expr) => self.emit_print_num(expr),
            Stmt::Assign { name, value } => self.emit_assign(name, value),
            Stmt::IndexAssign { object, index, value } => {
                // Evaluar value
                self.emit_expression(value);
                self.emit_bytes(&[0x50]); // push rax (value)
                
                // Evaluar index
                self.emit_expression(index);
                self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                self.emit_bytes(&[0x50]); // push rbx
                
                // Evaluar object
                self.emit_expression(object);
                
                // pop rbx (index), pop rcx (value)
                self.emit_bytes(&[0x5B]); // pop rbx
                self.emit_bytes(&[0x59]); // pop rcx
                
                // mov [rax + rbx*8 + 8], rcx
                self.emit_bytes(&[0x48, 0x89, 0x4C, 0xD8, 0x08]);
            }
            Stmt::FieldAssign { object, field: _, value } => {
                self.emit_expression(value);
                self.emit_bytes(&[0x50]); // push rax
                self.emit_expression(object);
                self.emit_bytes(&[0x59]); // pop rcx
                // mov [rax], rcx
                self.emit_bytes(&[0x48, 0x89, 0x08]);
            }
            Stmt::If { condition, then_body, else_body } => {
                self.emit_if(condition, then_body, else_body.as_deref());
            }
            Stmt::While { condition, body } => self.emit_while(condition, body),
            Stmt::For { var, start, end, body } => self.emit_for(var, start, end, body),
            Stmt::ForEach { var, iterable, body } => {
                // Simplificado: tratar como for normal de 0 a len
                self.emit_for(var, &Expr::Number(0), iterable, body);
            }
            Stmt::Return(expr) => self.emit_return(expr.as_ref()),
            Stmt::Pass => {} // No-op
            Stmt::Assert { condition, message: _ } => {
                // Evaluar condición, si es 0 terminar
                self.emit_condition(condition);
                self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
                // jnz skip (assertion passed)
                self.emit_bytes(&[0x75, 0x01]); // jnz +1
                self.emit_bytes(&[0xCC]); // int3 (breakpoint/crash)
            }
            Stmt::Expr(expr) => { self.emit_expression(expr); }
            Stmt::Break | Stmt::Continue => {}
        }
    }

    fn emit_assign(&mut self, name: &str, value: &Expr) {
        self.emit_expression(value);
        
        let offset = if let Some(&off) = self.variables.get(name) {
            off
        } else {
            let off = self.stack_offset;
            self.variables.insert(name.to_string(), off);
            self.stack_offset -= 8;
            off
        };
        
        // mov [rbp+offset], rax (using 32-bit displacement)
        self.emit_bytes(&[0x48, 0x89, 0x85]);
        self.emit_i32(offset);
    }

    fn emit_print_string(&mut self, expr: &Expr) {
        if let Expr::String(s) = expr {
            let string_addr = self.get_string_address(s);
            
            // mov rcx, string_addr
            self.emit_bytes(&[0x48, 0xB9]);
            self.emit_u64(string_addr);
            
            // call printf via IAT
            self.emit_call_printf();
        }
    }

    fn emit_print_num(&mut self, expr: &Expr) {
        self.emit_expression(expr);
        
        // Para imprimir número, usamos printf con "%d\n"
        // Agregar format string si no existe
        let fmt = "%d\n".to_string();
        if !self.strings.contains(&fmt) {
            self.strings.push(fmt.clone());
        }
        let fmt_addr = self.get_string_address("%d\n");
        
        // mov rdx, rax (valor a imprimir)
        self.emit_bytes(&[0x48, 0x89, 0xC2]);
        
        // mov rcx, fmt_addr
        self.emit_bytes(&[0x48, 0xB9]);
        self.emit_u64(fmt_addr);
        
        self.emit_call_printf();
    }

    fn emit_call_printf(&mut self) {
        // sub rsp, 32 (shadow space)
        self.emit_bytes(&[0x48, 0x83, 0xEC, 0x20]);
        
        // call [rip+offset]
        let call_end_rva = TEXT_RVA + self.code.len() as u64 + 6;
        let offset = IAT_PRINTF_RVA as i64 - call_end_rva as i64;
        self.emit_bytes(&[0xFF, 0x15]);
        self.emit_i32(offset as i32);
        
        // add rsp, 32
        self.emit_bytes(&[0x48, 0x83, 0xC4, 0x20]);
    }

    fn emit_if(&mut self, condition: &Expr, then_body: &[Stmt], else_body: Option<&[Stmt]>) {
        self.emit_condition(condition);
        
        // test rax, rax
        self.emit_bytes(&[0x48, 0x85, 0xC0]);
        
        // je else_label (placeholder)
        self.emit_bytes(&[0x0F, 0x84]);
        let je_offset_pos = self.code.len();
        self.emit_i32(0); // placeholder
        
        // then body
        for stmt in then_body {
            self.emit_statement(stmt);
        }
        
        if else_body.is_some() {
            // jmp end_label (placeholder)
            self.emit_bytes(&[0xE9]);
            let jmp_offset_pos = self.code.len();
            self.emit_i32(0);
            
            // else_label:
            let else_label = self.code.len();
            let je_offset = (else_label - je_offset_pos - 4) as i32;
            self.code[je_offset_pos..je_offset_pos + 4].copy_from_slice(&je_offset.to_le_bytes());
            
            // else body
            for stmt in else_body.unwrap() {
                self.emit_statement(stmt);
            }
            
            // end_label:
            let end_label = self.code.len();
            let jmp_offset = (end_label - jmp_offset_pos - 4) as i32;
            self.code[jmp_offset_pos..jmp_offset_pos + 4].copy_from_slice(&jmp_offset.to_le_bytes());
        } else {
            // else_label = end_label
            let else_label = self.code.len();
            let je_offset = (else_label - je_offset_pos - 4) as i32;
            self.code[je_offset_pos..je_offset_pos + 4].copy_from_slice(&je_offset.to_le_bytes());
        }
    }

    fn emit_while(&mut self, condition: &Expr, body: &[Stmt]) {
        let loop_start = self.code.len();
        
        self.emit_condition(condition);
        
        // test rax, rax
        self.emit_bytes(&[0x48, 0x85, 0xC0]);
        
        // je end_label
        self.emit_bytes(&[0x0F, 0x84]);
        let je_offset_pos = self.code.len();
        self.emit_i32(0);
        
        // body
        for stmt in body {
            self.emit_statement(stmt);
        }
        
        // jmp loop_start
        self.emit_bytes(&[0xE9]);
        let jmp_back = (loop_start as i64 - self.code.len() as i64 - 4) as i32;
        self.emit_i32(jmp_back);
        
        // end_label:
        let end_label = self.code.len();
        let je_offset = (end_label - je_offset_pos - 4) as i32;
        self.code[je_offset_pos..je_offset_pos + 4].copy_from_slice(&je_offset.to_le_bytes());
    }

    fn emit_for(&mut self, var: &str, start: &Expr, end: &Expr, body: &[Stmt]) {
        // var = start
        self.emit_expression(start);
        let var_offset = self.stack_offset;
        self.variables.insert(var.to_string(), var_offset);
        self.stack_offset -= 8;
        self.emit_bytes(&[0x48, 0x89, 0x45]);
        self.code.push(var_offset as u8);
        
        // Guardar end en stack
        self.emit_expression(end);
        let end_offset = self.stack_offset;
        self.stack_offset -= 8;
        self.emit_bytes(&[0x48, 0x89, 0x45]);
        self.code.push(end_offset as u8);
        
        let loop_start = self.code.len();
        
        // cmp var, end
        self.emit_bytes(&[0x48, 0x8B, 0x45]);
        self.code.push(var_offset as u8);
        self.emit_bytes(&[0x48, 0x3B, 0x45]);
        self.code.push(end_offset as u8);
        
        // jge end_label
        self.emit_bytes(&[0x0F, 0x8D]);
        let jge_offset_pos = self.code.len();
        self.emit_i32(0);
        
        // body
        for stmt in body {
            self.emit_statement(stmt);
        }
        
        // var++
        self.emit_bytes(&[0x48, 0xFF, 0x45]);
        self.code.push(var_offset as u8);
        
        // jmp loop_start
        self.emit_bytes(&[0xE9]);
        let jmp_back = (loop_start as i64 - self.code.len() as i64 - 4) as i32;
        self.emit_i32(jmp_back);
        
        // end_label:
        let end_label = self.code.len();
        let jge_offset = (end_label - jge_offset_pos - 4) as i32;
        self.code[jge_offset_pos..jge_offset_pos + 4].copy_from_slice(&jge_offset.to_le_bytes());
    }

    fn emit_return(&mut self, expr: Option<&Expr>) {
        if let Some(e) = expr {
            self.emit_expression(e);
        } else {
            self.emit_bytes(&[0x31, 0xC0]); // xor eax, eax
        }
        // Epilogue y ret
        self.emit_bytes(&[0x48, 0x89, 0xEC]); // mov rsp, rbp
        self.emit_bytes(&[0x5D]);             // pop rbp
        self.emit_bytes(&[0xC3]);             // ret
    }

    fn emit_condition(&mut self, expr: &Expr) {
        match expr {
            Expr::Comparison { op, left, right } => {
                self.emit_expression(left);
                self.emit_bytes(&[0x50]); // push rax
                self.emit_expression(right);
                self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                self.emit_bytes(&[0x58]); // pop rax
                
                // cmp rax, rbx
                self.emit_bytes(&[0x48, 0x39, 0xD8]);
                
                // setcc al
                match op {
                    CmpOp::Eq => self.emit_bytes(&[0x0F, 0x94, 0xC0]),  // sete al
                    CmpOp::Ne => self.emit_bytes(&[0x0F, 0x95, 0xC0]),  // setne al
                    CmpOp::Lt => self.emit_bytes(&[0x0F, 0x9C, 0xC0]),  // setl al
                    CmpOp::Le => self.emit_bytes(&[0x0F, 0x9E, 0xC0]),  // setle al
                    CmpOp::Gt => self.emit_bytes(&[0x0F, 0x9F, 0xC0]),  // setg al
                    CmpOp::Ge => self.emit_bytes(&[0x0F, 0x9D, 0xC0]),  // setge al
                }
                
                // movzx rax, al
                self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]);
            }
            Expr::Bool(b) => {
                let val = if *b { 1u32 } else { 0u32 };
                self.emit_bytes(&[0xB8]);
                self.emit_u32(val);
            }
            _ => {
                self.emit_expression(expr);
            }
        }
    }

    fn emit_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                // mov rax, imm64
                self.emit_bytes(&[0x48, 0xB8]);
                self.emit_u64(*n as u64);
            }
            Expr::Bool(b) => {
                let val = if *b { 1u64 } else { 0u64 };
                self.emit_bytes(&[0x48, 0xB8]);
                self.emit_u64(val);
            }
            Expr::String(_) => {}
            Expr::Variable(name) => {
                if let Some(&offset) = self.variables.get(name) {
                    // mov rax, [rbp+offset] (32-bit displacement)
                    self.emit_bytes(&[0x48, 0x8B, 0x85]);
                    self.emit_i32(offset);
                } else {
                    // Variable no encontrada, usar 0
                    self.emit_bytes(&[0x31, 0xC0]);
                }
            }
            Expr::BinaryOp { op, left, right } => {
                self.emit_expression(left);
                self.emit_bytes(&[0x50]); // push rax
                self.emit_expression(right);
                self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                self.emit_bytes(&[0x58]); // pop rax
                
                match op {
                    BinOp::Add => self.emit_bytes(&[0x48, 0x01, 0xD8]),
                    BinOp::Sub => self.emit_bytes(&[0x48, 0x29, 0xD8]),
                    BinOp::Mul => self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]),
                    BinOp::Div => {
                        self.emit_bytes(&[0x48, 0x99]); // cqo
                        self.emit_bytes(&[0x48, 0xF7, 0xFB]); // idiv rbx
                    }
                    BinOp::Mod => {
                        self.emit_bytes(&[0x48, 0x99]); // cqo
                        self.emit_bytes(&[0x48, 0xF7, 0xFB]); // idiv rbx
                        self.emit_bytes(&[0x48, 0x89, 0xD0]); // mov rax, rdx
                    }
                    BinOp::And => self.emit_bytes(&[0x48, 0x21, 0xD8]),
                    BinOp::Or => self.emit_bytes(&[0x48, 0x09, 0xD8]),
                }
            }
            Expr::UnaryOp { op, expr: inner } => {
                self.emit_expression(inner);
                match op {
                    UnaryOp::Neg => self.emit_bytes(&[0x48, 0xF7, 0xD8]), // neg rax
                    UnaryOp::Not => {
                        self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
                        self.emit_bytes(&[0x0F, 0x94, 0xC0]); // sete al
                        self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                    }
                }
            }
            Expr::Comparison { .. } => {
                self.emit_condition(expr);
            }
            Expr::Call { name, args } => {
                match name.as_str() {
                    "len" => {
                        // len(array) - retorna el primer elemento (length)
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            // mov rax, [rax] (load length from first position)
                            self.emit_bytes(&[0x48, 0x8B, 0x00]);
                        }
                    }
                    "abs" => {
                        // abs(x) - valor absoluto
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            // test rax, rax
                            self.emit_bytes(&[0x48, 0x85, 0xC0]);
                            // jns skip (if positive)
                            self.emit_bytes(&[0x79, 0x03]);
                            // neg rax
                            self.emit_bytes(&[0x48, 0xF7, 0xD8]);
                        }
                    }
                    "min" => {
                        // min(a, b)
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]); // push rax
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                            self.emit_bytes(&[0x58]); // pop rax
                            // cmp rax, rbx
                            self.emit_bytes(&[0x48, 0x39, 0xD8]);
                            // cmovg rax, rbx (if rax > rbx, rax = rbx)
                            self.emit_bytes(&[0x48, 0x0F, 0x4F, 0xC3]);
                        }
                    }
                    "max" => {
                        // max(a, b)
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]); // push rax
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                            self.emit_bytes(&[0x58]); // pop rax
                            // cmp rax, rbx
                            self.emit_bytes(&[0x48, 0x39, 0xD8]);
                            // cmovl rax, rbx (if rax < rbx, rax = rbx)
                            self.emit_bytes(&[0x48, 0x0F, 0x4C, 0xC3]);
                        }
                    }
                    "pow" => {
                        // pow(base, exp) - potencia simple
                        if args.len() >= 2 {
                            self.emit_expression(&args[1]); // exp
                            self.emit_bytes(&[0x50]); // push rax (exp)
                            self.emit_expression(&args[0]); // base
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (base)
                            self.emit_bytes(&[0x5A]); // pop rdx (exp)
                            
                            // result = 1
                            self.emit_bytes(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                            
                            // loop: while rdx > 0
                            let loop_start = self.code.len();
                            self.emit_bytes(&[0x48, 0x85, 0xD2]); // test rdx, rdx
                            self.emit_bytes(&[0x74, 0x08]); // jz end (skip 8 bytes)
                            
                            // rax = rax * rbx
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]); // imul rax, rbx
                            // rdx--
                            self.emit_bytes(&[0x48, 0xFF, 0xCA]); // dec rdx
                            // jmp loop
                            let jmp_offset = loop_start as i32 - self.code.len() as i32 - 2;
                            self.emit_bytes(&[0xEB, jmp_offset as u8]);
                        }
                    }
                    "sqrt" => {
                        // sqrt(x) - raíz cuadrada entera (aproximación)
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            // Aproximación: buscar n donde n*n <= x
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (x)
                            self.emit_bytes(&[0x31, 0xC0]); // xor eax, eax (n = 0)
                            
                            // loop
                            let loop_start = self.code.len();
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC9]); // imul rcx, rcx (n*n)
                            self.emit_bytes(&[0x48, 0x39, 0xD9]); // cmp rcx, rbx
                            self.emit_bytes(&[0x77, 0x05]); // ja end
                            self.emit_bytes(&[0x48, 0xFF, 0xC0]); // inc rax
                            let jmp_offset = loop_start as i32 - self.code.len() as i32 - 2;
                            self.emit_bytes(&[0xEB, jmp_offset as u8]);
                            // Ajustar resultado
                            self.emit_bytes(&[0x48, 0xFF, 0xC8]); // dec rax
                        }
                    }
                    "sum" => {
                        // sum(a, b, c, ...) - suma de todos los argumentos
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            for arg in args.iter().skip(1) {
                                self.emit_bytes(&[0x50]); // push rax
                                self.emit_expression(arg);
                                self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                                self.emit_bytes(&[0x58]); // pop rax
                                self.emit_bytes(&[0x48, 0x01, 0xD8]); // add rax, rbx
                            }
                        }
                    }
                    "input" => {
                        // input() - placeholder, retorna 0
                        self.emit_bytes(&[0x31, 0xC0]); // xor eax, eax
                    }
                    "type" => {
                        // type(x) - retorna tipo (placeholder: 1=int, 2=str, etc)
                        self.emit_bytes(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                    }
                    "clamp" => {
                        // clamp(value, min, max) - limitar valor entre min y max
                        if args.len() >= 3 {
                            self.emit_expression(&args[0]); // value
                            self.emit_bytes(&[0x50]); // push rax
                            self.emit_expression(&args[1]); // min
                            self.emit_bytes(&[0x50]); // push rax
                            self.emit_expression(&args[2]); // max
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax (max)
                            self.emit_bytes(&[0x58]); // pop rax (min)
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (min)
                            self.emit_bytes(&[0x58]); // pop rax (value)
                            // if value < min: value = min
                            self.emit_bytes(&[0x48, 0x39, 0xD8]); // cmp rax, rbx
                            self.emit_bytes(&[0x48, 0x0F, 0x4C, 0xC3]); // cmovl rax, rbx
                            // if value > max: value = max
                            self.emit_bytes(&[0x48, 0x39, 0xC8]); // cmp rax, rcx
                            self.emit_bytes(&[0x48, 0x0F, 0x4F, 0xC1]); // cmovg rax, rcx
                        }
                    }
                    "sign" => {
                        // sign(x) - retorna -1, 0, o 1
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
                            self.emit_bytes(&[0x48, 0xC7, 0xC3, 0x00, 0x00, 0x00, 0x00]); // mov rbx, 0
                            self.emit_bytes(&[0x48, 0xC7, 0xC1, 0x01, 0x00, 0x00, 0x00]); // mov rcx, 1
                            self.emit_bytes(&[0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF]); // mov rdx, -1
                            self.emit_bytes(&[0x48, 0x0F, 0x4F, 0xC1]); // cmovg rax, rcx (if > 0)
                            self.emit_bytes(&[0x48, 0x0F, 0x4C, 0xC2]); // cmovl rax, rdx (if < 0)
                            self.emit_bytes(&[0x48, 0x0F, 0x44, 0xC3]); // cmove rax, rbx (if == 0)
                        }
                    }
                    "even" => {
                        // even(x) - retorna 1 si es par, 0 si impar
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x83, 0xE0, 0x01]); // and rax, 1
                            self.emit_bytes(&[0x48, 0x83, 0xF0, 0x01]); // xor rax, 1
                        }
                    }
                    "odd" => {
                        // odd(x) - retorna 1 si es impar, 0 si par
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x83, 0xE0, 0x01]); // and rax, 1
                        }
                    }
                    "sqr" => {
                        // sqr(x) - cuadrado de x
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC0]); // imul rax, rax
                        }
                    }
                    "cube" => {
                        // cube(x) - cubo de x
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC0]); // imul rax, rax
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]); // imul rax, rbx
                        }
                    }
                    "neg" => {
                        // neg(x) - negativo de x
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0xF7, 0xD8]); // neg rax
                        }
                    }
                    "inc" => {
                        // inc(x) - x + 1
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0xFF, 0xC0]); // inc rax
                        }
                    }
                    "dec" => {
                        // dec(x) - x - 1
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0xFF, 0xC8]); // dec rax
                        }
                    }
                    "double" => {
                        // double(x) - x * 2
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0xD1, 0xE0]); // shl rax, 1
                        }
                    }
                    "half" => {
                        // half(x) - x / 2
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0xD1, 0xF8]); // sar rax, 1
                        }
                    }
                    "mod" => {
                        // mod(a, b) - a % b
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]); // push rax
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                            self.emit_bytes(&[0x58]); // pop rax
                            self.emit_bytes(&[0x48, 0x99]); // cqo
                            self.emit_bytes(&[0x48, 0xF7, 0xF9]); // idiv rcx
                            self.emit_bytes(&[0x48, 0x89, 0xD0]); // mov rax, rdx
                        }
                    }
                    "div" => {
                        // div(a, b) - a / b
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]); // push rax
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                            self.emit_bytes(&[0x58]); // pop rax
                            self.emit_bytes(&[0x48, 0x99]); // cqo
                            self.emit_bytes(&[0x48, 0xF7, 0xF9]); // idiv rcx
                        }
                    }
                    "avg" => {
                        // avg(a, b) - promedio de dos valores
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]); // push rax
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                            self.emit_bytes(&[0x58]); // pop rax
                            self.emit_bytes(&[0x48, 0x01, 0xD8]); // add rax, rbx
                            self.emit_bytes(&[0x48, 0xD1, 0xF8]); // sar rax, 1
                        }
                    }
                    "diff" => {
                        // diff(a, b) - diferencia absoluta
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]); // push rax
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                            self.emit_bytes(&[0x58]); // pop rax
                            self.emit_bytes(&[0x48, 0x29, 0xD8]); // sub rax, rbx
                            // abs
                            self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
                            self.emit_bytes(&[0x79, 0x03]); // jns +3
                            self.emit_bytes(&[0x48, 0xF7, 0xD8]); // neg rax
                        }
                    }
                    "is_positive" => {
                        // is_positive(x) - 1 si x > 0
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
                            self.emit_bytes(&[0x0F, 0x9F, 0xC0]); // setg al
                            self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                        }
                    }
                    "is_negative" => {
                        // is_negative(x) - 1 si x < 0
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
                            self.emit_bytes(&[0x0F, 0x9C, 0xC0]); // setl al
                            self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                        }
                    }
                    "is_zero" => {
                        // is_zero(x) - 1 si x == 0
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
                            self.emit_bytes(&[0x0F, 0x94, 0xC0]); // sete al
                            self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                        }
                    }
                    // Operadores bit a bit
                    "bit_and" => {
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x21, 0xD8]); // and rax, rbx
                        }
                    }
                    "bit_or" => {
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x09, 0xD8]); // or rax, rbx
                        }
                    }
                    "bit_xor" => {
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x31, 0xD8]); // xor rax, rbx
                        }
                    }
                    "bit_not" => {
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0xF7, 0xD0]); // not rax
                        }
                    }
                    "shl" => {
                        // shl(x, n) - shift left
                        if args.len() >= 2 {
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0xD3, 0xE0]); // shl rax, cl
                        }
                    }
                    "shr" => {
                        // shr(x, n) - shift right
                        if args.len() >= 2 {
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0xD3, 0xF8]); // sar rax, cl
                        }
                    }
                    // Conversiones
                    "to_int" | "int" => {
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            // Ya es int, no hacer nada
                        }
                    }
                    "to_bool" | "bool" => {
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
                            self.emit_bytes(&[0x0F, 0x95, 0xC0]); // setne al
                            self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                        }
                    }
                    // Constantes
                    "PI" | "pi" => {
                        self.emit_bytes(&[0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00]); // mov rax, 3
                    }
                    "E" | "e" => {
                        self.emit_bytes(&[0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00]); // mov rax, 2
                    }
                    "TRUE" | "true" => {
                        self.emit_bytes(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                    }
                    "FALSE" | "false" => {
                        self.emit_bytes(&[0x31, 0xC0]); // xor eax, eax
                    }
                    "NULL" | "null" => {
                        self.emit_bytes(&[0x31, 0xC0]); // xor eax, eax
                    }
                    // Utilidades adicionales
                    "swap" => {
                        // swap no tiene sentido como función, retorna 0
                        self.emit_bytes(&[0x31, 0xC0]);
                    }
                    "identity" => {
                        // identity(x) - retorna x
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                        }
                    }
                    "always" => {
                        // always(x) - siempre retorna x (para closures)
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                        }
                    }
                    "never" => {
                        // never() - retorna 0
                        self.emit_bytes(&[0x31, 0xC0]);
                    }
                    // Funciones de rango y secuencias
                    "range_sum" => {
                        // range_sum(start, end) - suma de start a end
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]); // start
                            self.emit_bytes(&[0x50]); // push start
                            self.emit_expression(&args[1]); // end
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (end)
                            self.emit_bytes(&[0x58]); // pop rax (start)
                            // Formula: (end - start + 1) * (start + end) / 2
                            self.emit_bytes(&[0x48, 0x01, 0xD8]); // add rax, rbx (start + end)
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                            self.emit_bytes(&[0x48, 0x89, 0xD8]); // mov rax, rbx
                            self.emit_bytes(&[0x48, 0x29, 0xC0]); // Placeholder - simplified
                            self.emit_bytes(&[0x48, 0x89, 0xC8]); // mov rax, rcx
                            self.emit_bytes(&[0x48, 0xD1, 0xF8]); // sar rax, 1 (divide by 2 approx)
                        }
                    }
                    "factorial" => {
                        // factorial(n) - n!
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax (n)
                            self.emit_bytes(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                            // Loop: while rcx > 1: rax *= rcx; rcx--
                            let loop_start = self.code.len();
                            self.emit_bytes(&[0x48, 0x83, 0xF9, 0x01]); // cmp rcx, 1
                            self.emit_bytes(&[0x7E, 0x08]); // jle end
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC1]); // imul rax, rcx
                            self.emit_bytes(&[0x48, 0xFF, 0xC9]); // dec rcx
                            let offset = (self.code.len() - loop_start) as i8;
                            self.emit_bytes(&[0xEB, (-(offset as i8) - 2) as u8]); // jmp loop_start
                        }
                    }
                    "fib" => {
                        // fib(n) - n-ésimo número de Fibonacci
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax (n)
                            self.emit_bytes(&[0x48, 0x31, 0xC0]); // xor rax, rax (a = 0)
                            self.emit_bytes(&[0x48, 0xC7, 0xC3, 0x01, 0x00, 0x00, 0x00]); // mov rbx, 1 (b = 1)
                            self.emit_bytes(&[0x48, 0x83, 0xF9, 0x00]); // cmp rcx, 0
                            self.emit_bytes(&[0x74, 0x10]); // je end
                            // Loop
                            self.emit_bytes(&[0x48, 0x89, 0xC2]); // mov rdx, rax
                            self.emit_bytes(&[0x48, 0x89, 0xD8]); // mov rax, rbx
                            self.emit_bytes(&[0x48, 0x01, 0xD3]); // add rbx, rdx
                            self.emit_bytes(&[0x48, 0xFF, 0xC9]); // dec rcx
                            self.emit_bytes(&[0x75, 0xF3]); // jne loop
                        }
                    }
                    "gcd" => {
                        // gcd(a, b) - máximo común divisor
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]); // push rax
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (b)
                            self.emit_bytes(&[0x58]); // pop rax (a)
                            // while b != 0: a, b = b, a % b
                            self.emit_bytes(&[0x48, 0x85, 0xDB]); // test rbx, rbx
                            self.emit_bytes(&[0x74, 0x0E]); // jz end
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                            self.emit_bytes(&[0x48, 0x99]); // cqo
                            self.emit_bytes(&[0x48, 0xF7, 0xFB]); // idiv rbx
                            self.emit_bytes(&[0x48, 0x89, 0xD8]); // mov rax, rbx
                            self.emit_bytes(&[0x48, 0x89, 0xD3]); // mov rbx, rdx
                            self.emit_bytes(&[0xEB, 0xED]); // jmp loop
                        }
                    }
                    "lcm" => {
                        // lcm(a, b) - mínimo común múltiplo = a * b / gcd(a, b)
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]); // push a
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x50]); // push b
                            // Calcular a * b
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                            self.emit_bytes(&[0x48, 0x8B, 0x44, 0x24, 0x08]); // mov rax, [rsp+8]
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]); // imul rax, rbx
                            self.emit_bytes(&[0x48, 0x83, 0xC4, 0x10]); // add rsp, 16
                        }
                    }
                    "is_prime" => {
                        // is_prime(n) - 1 si es primo, 0 si no
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x83, 0xF8, 0x02]); // cmp rax, 2
                            self.emit_bytes(&[0x7C, 0x1A]); // jl not_prime
                            self.emit_bytes(&[0x74, 0x15]); // je is_prime
                            self.emit_bytes(&[0xA8, 0x01]); // test al, 1
                            self.emit_bytes(&[0x74, 0x14]); // jz not_prime
                            // Simplified: just check if odd and > 2
                            self.emit_bytes(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                            self.emit_bytes(&[0xEB, 0x0C]); // jmp end
                            // is_prime:
                            self.emit_bytes(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                            self.emit_bytes(&[0xEB, 0x05]); // jmp end
                            // not_prime:
                            self.emit_bytes(&[0x48, 0x31, 0xC0]); // xor rax, rax
                        }
                    }
                    // Funciones de comparación múltiple
                    "min3" => {
                        // min3(a, b, c) - mínimo de tres valores
                        if args.len() >= 3 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[2]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax (c)
                            self.emit_bytes(&[0x58]); // pop rax (b)
                            self.emit_bytes(&[0x48, 0x39, 0xC8]); // cmp rax, rcx
                            self.emit_bytes(&[0x48, 0x0F, 0x4F, 0xC1]); // cmovg rax, rcx
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                            self.emit_bytes(&[0x58]); // pop rax (a)
                            self.emit_bytes(&[0x48, 0x39, 0xC8]); // cmp rax, rcx
                            self.emit_bytes(&[0x48, 0x0F, 0x4F, 0xC1]); // cmovg rax, rcx
                        }
                    }
                    "max3" => {
                        // max3(a, b, c) - máximo de tres valores
                        if args.len() >= 3 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[2]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax (c)
                            self.emit_bytes(&[0x58]); // pop rax (b)
                            self.emit_bytes(&[0x48, 0x39, 0xC8]); // cmp rax, rcx
                            self.emit_bytes(&[0x48, 0x0F, 0x4C, 0xC1]); // cmovl rax, rcx
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                            self.emit_bytes(&[0x58]); // pop rax (a)
                            self.emit_bytes(&[0x48, 0x39, 0xC8]); // cmp rax, rcx
                            self.emit_bytes(&[0x48, 0x0F, 0x4C, 0xC1]); // cmovl rax, rcx
                        }
                    }
                    "between" => {
                        // between(x, min, max) - 1 si min <= x <= max
                        if args.len() >= 3 {
                            self.emit_expression(&args[0]); // x
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]); // min
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[2]); // max
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax (max)
                            self.emit_bytes(&[0x58]); // pop rax (min)
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (min)
                            self.emit_bytes(&[0x58]); // pop rax (x)
                            self.emit_bytes(&[0x48, 0x39, 0xD8]); // cmp rax, rbx
                            self.emit_bytes(&[0x0F, 0x9D, 0xC2]); // setge dl
                            self.emit_bytes(&[0x48, 0x39, 0xC8]); // cmp rax, rcx
                            self.emit_bytes(&[0x0F, 0x9E, 0xC0]); // setle al
                            self.emit_bytes(&[0x20, 0xD0]); // and al, dl
                            self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                        }
                    }
                    "equals" => {
                        // equals(a, b) - 1 si a == b
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x39, 0xD8]); // cmp rax, rbx
                            self.emit_bytes(&[0x0F, 0x94, 0xC0]); // sete al
                            self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                        }
                    }
                    "not_equals" => {
                        // not_equals(a, b) - 1 si a != b
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x39, 0xD8]); // cmp rax, rbx
                            self.emit_bytes(&[0x0F, 0x95, 0xC0]); // setne al
                            self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                        }
                    }
                    "less" => {
                        // less(a, b) - 1 si a < b
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x39, 0xD8]); // cmp rax, rbx
                            self.emit_bytes(&[0x0F, 0x9C, 0xC0]); // setl al
                            self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                        }
                    }
                    "greater" => {
                        // greater(a, b) - 1 si a > b
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x39, 0xD8]); // cmp rax, rbx
                            self.emit_bytes(&[0x0F, 0x9F, 0xC0]); // setg al
                            self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
                        }
                    }
                    "print" => {
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                        }
                        self.emit_call_printf();
                    }
                    // ========================================
                    // FUNCIONES MATRICIALES PARA IA
                    // ========================================
                    "dot" => {
                        // dot(a1, a2, a3, ..., b1, b2, b3, ...) - producto punto
                        // Simplificado: dot(a, b, c, d) = a*b + c*d
                        if args.len() >= 4 {
                            // a * b
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]); // push a
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (b)
                            self.emit_bytes(&[0x58]); // pop rax (a)
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]); // imul rax, rbx
                            self.emit_bytes(&[0x50]); // push a*b
                            // c * d
                            self.emit_expression(&args[2]);
                            self.emit_bytes(&[0x50]); // push c
                            self.emit_expression(&args[3]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (d)
                            self.emit_bytes(&[0x58]); // pop rax (c)
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]); // imul rax, rbx
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (c*d)
                            self.emit_bytes(&[0x58]); // pop rax (a*b)
                            self.emit_bytes(&[0x48, 0x01, 0xD8]); // add rax, rbx
                        } else if args.len() >= 2 {
                            // dot(a, b) = a * b
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]);
                        }
                    }
                    "dot6" => {
                        // dot6(a1,b1, a2,b2, a3,b3) = a1*b1 + a2*b2 + a3*b3
                        if args.len() >= 6 {
                            // Acumulador en stack
                            self.emit_bytes(&[0x48, 0x31, 0xC0]); // xor rax, rax
                            self.emit_bytes(&[0x50]); // push 0 (acumulador)
                            
                            // Par 1: a1 * b1
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]);
                            self.emit_bytes(&[0x48, 0x01, 0x04, 0x24]); // add [rsp], rax
                            
                            // Par 2: a2 * b2
                            self.emit_expression(&args[2]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[3]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]);
                            self.emit_bytes(&[0x48, 0x01, 0x04, 0x24]); // add [rsp], rax
                            
                            // Par 3: a3 * b3
                            self.emit_expression(&args[4]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[5]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]);
                            self.emit_bytes(&[0x48, 0x01, 0x04, 0x24]); // add [rsp], rax
                            
                            self.emit_bytes(&[0x58]); // pop rax (resultado)
                        }
                    }
                    "sum_sq" => {
                        // sum_sq(a, b, c, ...) = a² + b² + c² + ...
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC0]); // imul rax, rax
                            for arg in args.iter().skip(1) {
                                self.emit_bytes(&[0x50]); // push acumulador
                                self.emit_expression(arg);
                                self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC0]); // imul rax, rax
                                self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                                self.emit_bytes(&[0x58]); // pop rax
                                self.emit_bytes(&[0x48, 0x01, 0xD8]); // add rax, rbx
                            }
                        }
                    }
                    "norm_sq" => {
                        // norm_sq(a, b, c) = a² + b² + c² (norma al cuadrado)
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC0]); // a²
                            for arg in args.iter().skip(1) {
                                self.emit_bytes(&[0x50]);
                                self.emit_expression(arg);
                                self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC0]); // x²
                                self.emit_bytes(&[0x48, 0x89, 0xC3]);
                                self.emit_bytes(&[0x58]);
                                self.emit_bytes(&[0x48, 0x01, 0xD8]);
                            }
                        }
                    }
                    "weighted_sum" => {
                        // weighted_sum(v1, w1, v2, w2, ...) = v1*w1 + v2*w2 + ...
                        if args.len() >= 2 {
                            // Primer par
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]);
                            
                            // Pares adicionales
                            let mut i = 2;
                            while i + 1 < args.len() {
                                self.emit_bytes(&[0x50]); // push acumulador
                                self.emit_expression(&args[i]);
                                self.emit_bytes(&[0x50]);
                                self.emit_expression(&args[i + 1]);
                                self.emit_bytes(&[0x48, 0x89, 0xC3]);
                                self.emit_bytes(&[0x58]);
                                self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]);
                                self.emit_bytes(&[0x48, 0x89, 0xC3]);
                                self.emit_bytes(&[0x58]);
                                self.emit_bytes(&[0x48, 0x01, 0xD8]);
                                i += 2;
                            }
                        }
                    }
                    "relu" => {
                        // relu(x) = max(0, x) - activación ReLU
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x31, 0xDB]); // xor rbx, rbx (0)
                            self.emit_bytes(&[0x48, 0x39, 0xD8]); // cmp rax, rbx
                            self.emit_bytes(&[0x48, 0x0F, 0x4C, 0xC3]); // cmovl rax, rbx
                        }
                    }
                    "sigmoid_approx" => {
                        // sigmoid_approx(x) - aproximación: x < -4 ? 0 : x > 4 ? 100 : 50 + x*12
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            // Simplificado: clamp entre 0 y 100 basado en signo
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                            self.emit_bytes(&[0x48, 0x6B, 0xC0, 0x0C]); // imul rax, 12
                            self.emit_bytes(&[0x48, 0x83, 0xC0, 0x32]); // add rax, 50
                            // Clamp 0-100
                            self.emit_bytes(&[0x48, 0x83, 0xF8, 0x00]); // cmp rax, 0
                            self.emit_bytes(&[0x48, 0xC7, 0xC1, 0x00, 0x00, 0x00, 0x00]); // mov rcx, 0
                            self.emit_bytes(&[0x48, 0x0F, 0x4C, 0xC1]); // cmovl rax, rcx
                            self.emit_bytes(&[0x48, 0x83, 0xF8, 0x64]); // cmp rax, 100
                            self.emit_bytes(&[0x48, 0xC7, 0xC1, 0x64, 0x00, 0x00, 0x00]); // mov rcx, 100
                            self.emit_bytes(&[0x48, 0x0F, 0x4F, 0xC1]); // cmovg rax, rcx
                        }
                    }
                    "softmax_max" => {
                        // softmax_max(a, b, c, ...) - retorna el máximo (para softmax)
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            for arg in args.iter().skip(1) {
                                self.emit_bytes(&[0x50]);
                                self.emit_expression(arg);
                                self.emit_bytes(&[0x48, 0x89, 0xC3]);
                                self.emit_bytes(&[0x58]);
                                self.emit_bytes(&[0x48, 0x39, 0xD8]);
                                self.emit_bytes(&[0x48, 0x0F, 0x4C, 0xC3]); // cmovl rax, rbx
                            }
                        }
                    }
                    "scale" => {
                        // scale(x, factor) = x * factor / 100
                        if args.len() >= 2 {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC3]);
                            self.emit_bytes(&[0x58]);
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]); // imul rax, rbx
                            self.emit_bytes(&[0x48, 0xC7, 0xC1, 0x64, 0x00, 0x00, 0x00]); // mov rcx, 100
                            self.emit_bytes(&[0x48, 0x99]); // cqo
                            self.emit_bytes(&[0x48, 0xF7, 0xF9]); // idiv rcx
                        }
                    }
                    "lerp" => {
                        // lerp(a, b, t) = a + (b - a) * t / 100 - interpolación lineal
                        if args.len() >= 3 {
                            self.emit_expression(&args[0]); // a
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[1]); // b
                            self.emit_bytes(&[0x50]);
                            self.emit_expression(&args[2]); // t
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax (t)
                            self.emit_bytes(&[0x58]); // pop rax (b)
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (b)
                            self.emit_bytes(&[0x58]); // pop rax (a)
                            self.emit_bytes(&[0x50]); // push a
                            self.emit_bytes(&[0x48, 0x29, 0xC3]); // sub rbx, rax (b - a)
                            self.emit_bytes(&[0x48, 0x89, 0xD8]); // mov rax, rbx
                            self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC1]); // imul rax, rcx
                            self.emit_bytes(&[0x48, 0xC7, 0xC1, 0x64, 0x00, 0x00, 0x00]); // mov rcx, 100
                            self.emit_bytes(&[0x48, 0x99]); // cqo
                            self.emit_bytes(&[0x48, 0xF7, 0xF9]); // idiv rcx
                            self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
                            self.emit_bytes(&[0x58]); // pop rax (a)
                            self.emit_bytes(&[0x48, 0x01, 0xD8]); // add rax, rbx
                        }
                    }
                    _ => {
                        // Función genérica
                        if !args.is_empty() {
                            self.emit_expression(&args[0]);
                            self.emit_bytes(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                        }
                        if args.len() > 1 {
                            self.emit_expression(&args[1]);
                            self.emit_bytes(&[0x48, 0x89, 0xC2]); // mov rdx, rax
                        }
                    }
                }
            }
            // OOP expressions - simplified implementation
            Expr::New { class_name: _, args } => {
                // Allocate object on stack (simplified: just store fields)
                // For now, return a pointer to stack space
                let obj_offset = self.stack_offset;
                self.stack_offset -= 64; // Reserve 64 bytes for object
                
                // Initialize fields from constructor args
                for (i, arg) in args.iter().enumerate() {
                    self.emit_expression(arg);
                    let field_offset = obj_offset - (i as i32 * 8);
                    self.emit_bytes(&[0x48, 0x89, 0x45]);
                    self.code.push(field_offset as u8);
                }
                
                // Return pointer to object (lea rax, [rbp+offset])
                self.emit_bytes(&[0x48, 0x8D, 0x45]);
                self.code.push(obj_offset as u8);
            }
            Expr::MethodCall { object, method: _, args } => {
                // Simplified: evaluate object, then call method
                self.emit_expression(object);
                self.emit_bytes(&[0x50]); // push rax (save object pointer)
                
                // Evaluate args
                for arg in args {
                    self.emit_expression(arg);
                }
                
                self.emit_bytes(&[0x58]); // pop rax (restore object pointer)
            }
            Expr::FieldAccess { object, field: _ } => {
                // Simplified: load object, access first field
                self.emit_expression(object);
                // mov rax, [rax] (load first field)
                self.emit_bytes(&[0x48, 0x8B, 0x00]);
            }
            Expr::This => {
                // Load 'this' pointer (first local variable)
                if let Some(&offset) = self.variables.get("this") {
                    self.emit_bytes(&[0x48, 0x8B, 0x45]);
                    self.code.push(offset as u8);
                } else {
                    self.emit_bytes(&[0x31, 0xC0]); // xor eax, eax
                }
            }
            Expr::Super => {
                if let Some(&offset) = self.variables.get("this") {
                    self.emit_bytes(&[0x48, 0x8B, 0x45]);
                    self.code.push(offset as u8);
                } else {
                    self.emit_bytes(&[0x31, 0xC0]);
                }
            }
            // Nuevas expresiones
            Expr::Float(f) => {
                // Convertir float a bits y cargar
                let bits = (*f).to_bits();
                self.emit_bytes(&[0x48, 0xB8]);
                self.emit_u64(bits);
            }
            Expr::Null => {
                self.emit_bytes(&[0x31, 0xC0]); // xor eax, eax (null = 0)
            }
            Expr::Array(elements) => {
                // Allocar espacio en stack para array
                let arr_offset = self.stack_offset;
                self.stack_offset -= (elements.len() as i32 * 8) + 8; // +8 para length
                
                // Guardar length
                self.emit_bytes(&[0x48, 0xB8]);
                self.emit_u64(elements.len() as u64);
                self.emit_bytes(&[0x48, 0x89, 0x45]);
                self.code.push(arr_offset as u8);
                
                // Guardar elementos
                for (i, elem) in elements.iter().enumerate() {
                    self.emit_expression(elem);
                    let elem_offset = arr_offset - 8 - (i as i32 * 8);
                    self.emit_bytes(&[0x48, 0x89, 0x45]);
                    self.code.push(elem_offset as u8);
                }
                
                // Retornar puntero al array
                self.emit_bytes(&[0x48, 0x8D, 0x45]);
                self.code.push(arr_offset as u8);
            }
            Expr::Index { object, index } => {
                // Evaluar objeto (puntero al array)
                self.emit_expression(object);
                self.emit_bytes(&[0x50]); // push rax
                
                // Evaluar índice
                self.emit_expression(index);
                self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax (index)
                
                self.emit_bytes(&[0x58]); // pop rax (array ptr)
                
                // Calcular offset: (index + 1) * 8 (skip length)
                // mov rax, [rax + rbx*8 + 8]
                self.emit_bytes(&[0x48, 0x8B, 0x44, 0xD8, 0x08]);
            }
            Expr::Slice { object, start: _, end: _ } => {
                // Simplificado: retornar el objeto
                self.emit_expression(object);
            }
            Expr::Lambda { params: _, body } => {
                // Simplificado: evaluar body
                self.emit_expression(body);
            }
            Expr::Ternary { condition, then_expr, else_expr } => {
                self.emit_condition(condition);
                self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
                
                // je else
                self.emit_bytes(&[0x0F, 0x84]);
                let je_pos = self.code.len();
                self.emit_i32(0);
                
                // then
                self.emit_expression(then_expr);
                
                // jmp end
                self.emit_bytes(&[0xE9]);
                let jmp_pos = self.code.len();
                self.emit_i32(0);
                
                // else:
                let else_label = self.code.len();
                let je_offset = (else_label - je_pos - 4) as i32;
                self.code[je_pos..je_pos + 4].copy_from_slice(&je_offset.to_le_bytes());
                
                self.emit_expression(else_expr);
                
                // end:
                let end_label = self.code.len();
                let jmp_offset = (end_label - jmp_pos - 4) as i32;
                self.code[jmp_pos..jmp_pos + 4].copy_from_slice(&jmp_offset.to_le_bytes());
            }
        }
    }

    fn generate_data_section(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for s in &self.strings {
            data.extend_from_slice(s.as_bytes());
            data.push(0);
        }
        data
    }

    fn emit_bytes(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    fn emit_u32(&mut self, value: u32) {
        self.code.extend_from_slice(&value.to_le_bytes());
    }

    fn emit_u64(&mut self, value: u64) {
        self.code.extend_from_slice(&value.to_le_bytes());
    }

    fn emit_i32(&mut self, value: i32) {
        self.code.extend_from_slice(&value.to_le_bytes());
    }
}

