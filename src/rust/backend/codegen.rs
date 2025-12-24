// Code Generator - Emite opcodes directamente desde AST
// NO ASM - escribimos bytes directamente

use crate::frontend::ast::*;

const BASE_ADDRESS: u64 = 0x400000;

pub struct CodeGenerator {
    code: Vec<u8>,
    strings: Vec<String>,
    string_addresses: Vec<u64>,
    base_address: u64,
}

impl CodeGenerator {
    pub fn new(base_address: u64) -> Self {
        Self {
            code: Vec::new(),
            strings: Vec::new(),
            string_addresses: Vec::new(),
            base_address,
        }
    }

    pub fn generate(&mut self, program: &Program) -> (Vec<u8>, Vec<u8>) {
        // Buscar función main
        for func in &program.functions {
            if func.name == "main" {
                self.emit_function(func);
                break;
            }
        }

        // Generar sección de datos (strings)
        let data = self.generate_data_section();

        (self.code.clone(), data)
    }

    fn emit_function(&mut self, func: &Function) {
        // Para Windows x64, necesitamos un entry point válido
        // Por ahora, código mínimo que simplemente retorna
        
        // Emitir statements
        for stmt in &func.body {
            self.emit_statement(stmt);
        }
        
        // Retornar (código mínimo)
        // ret
        self.code.push(0xC3);
    }

    fn emit_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                self.emit_print(expr);
            }
            Stmt::Assign { name: _, value } => {
                // Por ahora solo evaluamos, no guardamos
                self.emit_expression(value);
            }
            Stmt::Expr(expr) => {
                self.emit_expression(expr);
            }
            _ => {
                // Not implemented yet for legacy codegen
            }
        }
    }

    fn emit_print(&mut self, expr: &Expr) {
        match expr {
            Expr::String(s) => {
                // Versión simplificada: por ahora solo retornamos
                // TODO: Agregar llamada a printf cuando Import Table esté lista
                
                // Por ahora, emitimos código que simplemente retorna
                // Esto permite que el PE sea válido y ejecutable
                // Luego agregamos printf
                
                eprintln!("⚠️  Print implementación pendiente - usando ret por ahora");
                
                // Guardamos el string para referencia futura
                let _string_idx = self.add_string(s.clone());
            }
            _ => {
                eprintln!("⚠️  Print solo soporta strings por ahora");
            }
        }
    }

    fn emit_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                // mov rax, value
                self.code.push(0x48); // REX.W
                self.code.push(0xC7); // MOV
                self.code.push(0xC0); // ModR/M: rax
                self.emit_u32(*n as u32);
            }
            Expr::String(_) => {
                // Ya manejado en emit_print
            }
            Expr::Variable(_) => {
                // TODO: Implementar variables
            }
            Expr::BinaryOp { op, left, right } => {
                self.emit_binary_op(op, left, right);
            }
            Expr::Call { .. } => {
                // TODO: Implementar llamadas
            }
            _ => {
                // Not implemented yet for legacy codegen
            }
        }
    }

    fn emit_binary_op(&mut self, op: &BinOp, left: &Expr, right: &Expr) {
        // Evaluar left
        self.emit_expression(left);
        // push rax
        self.code.push(0x50);
        
        // Evaluar right
        self.emit_expression(right);
        // mov rbx, rax
        self.emit_bytes(&[0x48, 0x89, 0xC3]);
        
        // pop rax
        self.code.push(0x58);
        
        match op {
            BinOp::Add => {
                // add rax, rbx
                self.emit_bytes(&[0x48, 0x01, 0xD8]);
            }
            BinOp::Sub => {
                // sub rax, rbx
                self.emit_bytes(&[0x48, 0x29, 0xD8]);
            }
            BinOp::Mul => {
                // imul rax, rbx
                self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]);
            }
            BinOp::Div => {
                // cqo (sign extend rax to rdx:rax)
                self.code.push(0x48);
                self.code.push(0x99);
                // idiv rbx
                self.emit_bytes(&[0x48, 0xF7, 0xFB]);
            }
            _ => {
                // Not implemented yet for legacy codegen
            }
        }
    }

    fn add_string(&mut self, s: String) -> usize {
        let idx = self.strings.len();
        self.strings.push(s.clone());
        
        // Calcular dirección (data section empieza después del código)
        let data_offset = self.base_address + 0x1000; // .data empieza en +0x1000
        let string_addr = data_offset + (idx as u64 * 0x100); // Cada string en offset diferente
        self.string_addresses.push(string_addr);
        
        idx
    }

    fn generate_data_section(&self) -> Vec<u8> {
        let mut data = Vec::new();
        
        for s in &self.strings {
            data.extend_from_slice(s.as_bytes());
            data.push(0); // Null terminator
            // Alinear a 16 bytes
            while data.len() % 16 != 0 {
                data.push(0);
            }
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
}

