// Code Generator - Emite opcodes directamente desde AST
// NO ASM - escribimos bytes directamente
// Binario + HEX = ADead-BIB

use crate::frontend::ast::*;

// Layout del PE:
// 0x0000 - Headers
// 0x1000 - .text (código)
// 0x2000 - .rdata (imports + strings)

const IMAGE_BASE: u64 = 0x400000;
const TEXT_RVA: u64 = 0x1000;
const RDATA_RVA: u64 = 0x2000;

// Offsets dentro de .rdata:
// 0x2000: Import Directory (20 bytes + 20 null terminator)
// 0x2028: ILT (8 bytes + 8 null)
// 0x2038: IAT (8 bytes + 8 null) 
// 0x2048: DLL name "msvcrt.dll\0"
// 0x2054: Hint/Name (2 bytes hint + "printf\0")
// 0x2060: Strings de usuario

const IAT_RVA: u64 = 0x2038;  // Donde está la dirección de printf
const STRING_START_RVA: u64 = 0x2060;  // Donde empiezan los strings

pub struct CodeGenerator {
    code: Vec<u8>,
    strings: Vec<String>,
}

impl CodeGenerator {
    pub fn new(_base_address: u64) -> Self {
        Self {
            code: Vec::new(),
            strings: Vec::new(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> (Vec<u8>, Vec<u8>) {
        // Primero recolectar strings para calcular direcciones
        self.collect_strings(program);
        
        // Buscar función main y generar código
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

    fn collect_strings(&mut self, program: &Program) {
        for func in &program.functions {
            for stmt in &func.body {
                if let Stmt::Print(Expr::String(s)) = stmt {
                    if !self.strings.contains(s) {
                        self.strings.push(s.clone());
                    }
                }
            }
        }
    }

    fn get_string_address(&self, s: &str) -> u64 {
        let mut offset = 0u64;
        for stored in &self.strings {
            if stored == s {
                return IMAGE_BASE + STRING_START_RVA + offset;
            }
            offset += stored.len() as u64 + 1; // +1 null terminator
        }
        IMAGE_BASE + STRING_START_RVA // fallback
    }

    fn emit_function(&mut self, func: &Function) {
        // Windows x64: sub rsp, 40 (shadow space 32 + align 8)
        self.emit_bytes(&[0x48, 0x83, 0xEC, 0x28]);
        
        // Emitir statements
        for stmt in &func.body {
            self.emit_statement(stmt);
        }
        
        // xor eax, eax (return 0)
        self.emit_bytes(&[0x31, 0xC0]);
        
        // add rsp, 40
        self.emit_bytes(&[0x48, 0x83, 0xC4, 0x28]);
        
        // ret
        self.code.push(0xC3);
    }

    fn emit_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                self.emit_print(expr);
            }
            Stmt::Assign { name: _, value } => {
                self.emit_expression(value);
            }
            Stmt::Expr(expr) => {
                self.emit_expression(expr);
            }
        }
    }

    fn emit_print(&mut self, expr: &Expr) {
        match expr {
            Expr::String(s) => {
                let string_addr = self.get_string_address(s);
                
                // mov rcx, string_addr (dirección absoluta del string)
                self.code.push(0x48); // REX.W
                self.code.push(0xB9); // MOV rcx, imm64
                self.emit_u64(string_addr);
                
                // call [rip+offset] -> call printf via IAT
                // RIP apunta a la instrucción DESPUÉS del call (6 bytes: FF 15 + 4 bytes offset)
                // Offset desde el final de esta instrucción hasta IAT
                let call_end_rva = TEXT_RVA + self.code.len() as u64 + 6; // +6 para FF 15 xx xx xx xx
                let offset = IAT_RVA as i64 - call_end_rva as i64;
                
                self.code.push(0xFF); // CALL
                self.code.push(0x15); // [rip+disp32]
                self.emit_i32(offset as i32);
            }
            _ => {
                eprintln!("⚠️  Print solo soporta strings por ahora");
            }
        }
    }

    fn emit_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                // mov eax, value (32-bit es suficiente para números pequeños)
                self.code.push(0xB8);
                self.emit_u32(*n as u32);
            }
            Expr::String(_) => {}
            Expr::Variable(_) => {}
            Expr::BinaryOp { op, left, right } => {
                self.emit_binary_op(op, left, right);
            }
            Expr::Call { .. } => {}
        }
    }

    fn emit_binary_op(&mut self, op: &BinOp, left: &Expr, right: &Expr) {
        self.emit_expression(left);
        self.code.push(0x50); // push rax
        self.emit_expression(right);
        self.emit_bytes(&[0x48, 0x89, 0xC3]); // mov rbx, rax
        self.code.push(0x58); // pop rax
        
        match op {
            BinOp::Add => self.emit_bytes(&[0x48, 0x01, 0xD8]),
            BinOp::Sub => self.emit_bytes(&[0x48, 0x29, 0xD8]),
            BinOp::Mul => self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]),
            BinOp::Div => {
                self.emit_bytes(&[0x48, 0x99]); // cqo
                self.emit_bytes(&[0x48, 0xF7, 0xFB]); // idiv rbx
            }
        }
    }

    fn generate_data_section(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for s in &self.strings {
            data.extend_from_slice(s.as_bytes());
            data.push(0); // Null terminator
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

