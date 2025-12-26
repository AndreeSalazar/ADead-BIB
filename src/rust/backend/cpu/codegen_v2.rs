// ADead-BIB CodeGen v2.0 - Sin Límites
// Generación de código mejorada con:
// - Múltiples funciones
// - Stack dinámico
// - Syscalls directos (opcional)
// - Multi-target (Windows/Linux)
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com

use crate::frontend::ast::*;
use std::collections::HashMap;

/// Target de compilación
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Target {
    Windows,
    Linux,
    Raw,
}

/// Información de una función compilada
#[derive(Clone, Debug)]
pub struct CompiledFunction {
    pub name: String,
    pub offset: usize,
    pub size: usize,
    pub params: Vec<String>,
    pub locals_size: i32,
}

/// CodeGenerator v2 - Sin límites
pub struct CodeGeneratorV2 {
    // Código generado
    code: Vec<u8>,
    data: Vec<u8>,
    
    // Strings
    strings: Vec<String>,
    string_offsets: HashMap<String, u64>,
    
    // Funciones
    functions: HashMap<String, CompiledFunction>,
    function_calls: Vec<(usize, String)>,  // (offset, nombre) para resolver después
    
    // Estado actual
    current_function: Option<String>,
    variables: HashMap<String, i32>,
    stack_offset: i32,
    max_stack: i32,
    
    // Configuración
    target: Target,
    base_address: u64,
    data_rva: u64,
}

impl CodeGeneratorV2 {
    pub fn new(target: Target) -> Self {
        let (base, data_rva) = match target {
            Target::Windows => (0x0000000140000000, 0x2060),
            Target::Linux => (0x400000, 0x1000),
            Target::Raw => (0x0, 0x1000),
        };
        
        Self {
            code: Vec::new(),
            data: Vec::new(),
            strings: Vec::new(),
            string_offsets: HashMap::new(),
            functions: HashMap::new(),
            function_calls: Vec::new(),
            current_function: None,
            variables: HashMap::new(),
            stack_offset: 0,
            max_stack: 0,
            target,
            base_address: base,
            data_rva,
        }
    }
    
    /// Genera código para todo el programa
    pub fn generate(&mut self, program: &Program) -> (Vec<u8>, Vec<u8>) {
        // Fase 1: Recolectar strings
        self.collect_all_strings(program);
        // Incluir strings de nivel superior
        self.collect_strings_from_stmts(&program.statements);
        
        // Fase 2: Compilar nivel superior como punto de entrada (si existe)
        if !program.statements.is_empty() {
            self.compile_top_level(&program.statements);
        }
        
        // Fase 3: Compilar todas las funciones
        for func in &program.functions {
            self.compile_function(func);
        }
        
        // Fase 4: Resolver llamadas a funciones
        self.resolve_function_calls();
        
        // Fase 5: Generar sección de datos
        self.generate_data_section();
        
        (self.code.clone(), self.data.clone())
    }
    
    /// Recolecta todos los strings del programa
    fn collect_all_strings(&mut self, program: &Program) {
        // Añadir formatos de printf y \n para println
        self.strings.push("%d".to_string());
        self.strings.push("%s".to_string());
        self.strings.push("\n".to_string());
        
        for func in &program.functions {
            self.collect_strings_from_stmts(&func.body);
        }
        
        // Calcular offsets de strings
        let mut offset = 0u64;
        for s in &self.strings {
            self.string_offsets.insert(s.clone(), offset);
            offset += s.len() as u64 + 1;
        }
    }
    
    /// Compila statements de nivel superior como función de entrada
    fn compile_top_level(&mut self, stmts: &[Stmt]) {
        let func_offset = self.code.len();
        self.current_function = Some("__entry".to_string());
        self.variables.clear();
        self.stack_offset = -8;
        self.max_stack = 0;
        
        // Prologue
        self.emit_bytes(&[0x55]);                    // push rbp
        self.emit_bytes(&[0x48, 0x89, 0xE5]);        // mov rbp, rsp
        self.emit_bytes(&[0x48, 0x81, 0xEC, 0x00, 0x00, 0x00, 0x00]);  // sub rsp, imm32
        let stack_size_offset = self.code.len() - 4;
        
        for stmt in stmts {
            self.emit_statement(stmt);
        }
        
        // Epilogue
        self.emit_bytes(&[0x31, 0xC0]);              // xor eax, eax
        self.emit_bytes(&[0x48, 0x89, 0xEC]);        // mov rsp, rbp
        self.emit_bytes(&[0x5D]);                    // pop rbp
        self.emit_bytes(&[0xC3]);                    // ret
        
        let stack_size = ((-self.stack_offset + 15) & !15) as u32; // 16-byte alignment
        self.code[stack_size_offset..stack_size_offset + 4]
            .copy_from_slice(&stack_size.to_le_bytes());
        
        self.functions.insert("__entry".to_string(), CompiledFunction {
            name: "__entry".to_string(),
            offset: func_offset,
            size: self.code.len() - func_offset,
            params: vec![],
            locals_size: -self.stack_offset,
        });
        
        self.current_function = None;
    }
    
    fn collect_strings_from_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::Print(Expr::String(s)) | Stmt::Println(Expr::String(s)) => {
                    // Procesar secuencias de escape (estilo C++)
                    let processed = s.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
                    if !self.strings.contains(&processed) {
                        self.strings.push(processed);
                    }
                }
                Stmt::If { then_body, else_body, .. } => {
                    self.collect_strings_from_stmts(then_body);
                    if let Some(else_stmts) = else_body {
                        self.collect_strings_from_stmts(else_stmts);
                    }
                }
                Stmt::While { body, .. } => self.collect_strings_from_stmts(body),
                Stmt::For { body, .. } => self.collect_strings_from_stmts(body),
                Stmt::ForEach { body, .. } => self.collect_strings_from_stmts(body),
                _ => {}
            }
        }
    }
    
    /// Compila una función
    fn compile_function(&mut self, func: &Function) {
        let func_offset = self.code.len();
        self.current_function = Some(func.name.clone());
        self.variables.clear();
        self.stack_offset = -8;
        self.max_stack = 0;
        
        // Registrar parámetros
        for (i, param) in func.params.iter().enumerate() {
            // Windows x64 calling convention: rcx, rdx, r8, r9, stack
            let param_offset = match i {
                0..=3 => {
                    // Parámetros en registros, guardar en stack
                    let off = self.stack_offset;
                    self.stack_offset -= 8;
                    off
                }
                _ => {
                    // Parámetros en stack (offset positivo desde rbp)
                    16 + ((i - 4) as i32 * 8)
                }
            };
            self.variables.insert(param.name.clone(), param_offset);
        }
        
        // Prologue placeholder (se parchea después)
        let _prologue_start = self.code.len();
        self.emit_bytes(&[0x55]);                    // push rbp
        self.emit_bytes(&[0x48, 0x89, 0xE5]);        // mov rbp, rsp
        self.emit_bytes(&[0x48, 0x81, 0xEC, 0x00, 0x00, 0x00, 0x00]);  // sub rsp, imm32
        let stack_size_offset = self.code.len() - 4;
        
        // Guardar parámetros de registros en stack
        for (i, param) in func.params.iter().enumerate().take(4) {
            if let Some(&offset) = self.variables.get(&param.name) {
                match i {
                    0 => self.emit_bytes(&[0x48, 0x89, 0x4D]), // mov [rbp+off], rcx
                    1 => self.emit_bytes(&[0x48, 0x89, 0x55]), // mov [rbp+off], rdx
                    2 => self.emit_bytes(&[0x4C, 0x89, 0x45]), // mov [rbp+off], r8
                    3 => self.emit_bytes(&[0x4C, 0x89, 0x4D]), // mov [rbp+off], r9
                    _ => {}
                }
                self.code.push(offset as u8);
            }
        }
        
        // Body
        for stmt in &func.body {
            self.emit_statement(stmt);
        }
        
        // Epilogue implícito si no hay return
        self.emit_bytes(&[0x31, 0xC0]);              // xor eax, eax
        self.emit_bytes(&[0x48, 0x89, 0xEC]);        // mov rsp, rbp
        self.emit_bytes(&[0x5D]);                    // pop rbp
        self.emit_bytes(&[0xC3]);                    // ret
        
        // Parchear tamaño del stack
        let stack_size = ((-self.stack_offset + 15) & !15) as u32; // Alinear a 16
        self.code[stack_size_offset..stack_size_offset + 4]
            .copy_from_slice(&stack_size.to_le_bytes());
        
        // Registrar función
        let func_size = self.code.len() - func_offset;
        self.functions.insert(func.name.clone(), CompiledFunction {
            name: func.name.clone(),
            offset: func_offset,
            size: func_size,
            params: func.params.iter().map(|p| p.name.clone()).collect(),
            locals_size: -self.stack_offset,
        });
        
        self.current_function = None;
    }
    
    /// Resuelve llamadas a funciones
    fn resolve_function_calls(&mut self) {
        for (call_offset, func_name) in &self.function_calls {
            if let Some(func) = self.functions.get(func_name) {
                let rel_offset = func.offset as i32 - (*call_offset as i32 + 4);
                self.code[*call_offset..*call_offset + 4]
                    .copy_from_slice(&rel_offset.to_le_bytes());
            }
        }
    }
    
    /// Genera sección de datos
    fn generate_data_section(&mut self) {
        for s in &self.strings.clone() {
            self.data.extend_from_slice(s.as_bytes());
            self.data.push(0);
        }
    }
    
    /// Obtiene dirección de un string
    fn get_string_address(&self, s: &str) -> u64 {
        if let Some(&offset) = self.string_offsets.get(s) {
            self.base_address + self.data_rva + offset
        } else {
            self.base_address + self.data_rva
        }
    }
    
    // ========================================
    // Emisión de statements
    // ========================================
    
    fn emit_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => self.emit_print(expr),
            Stmt::Println(expr) => self.emit_println(expr),
            Stmt::PrintNum(expr) => self.emit_print_num(expr),
            Stmt::Assign { name, value } => self.emit_assign(name, value),
            Stmt::If { condition, then_body, else_body } => {
                self.emit_if(condition, then_body, else_body.as_deref());
            }
            Stmt::While { condition, body } => self.emit_while(condition, body),
            Stmt::For { var, start, end, body } => self.emit_for(var, start, end, body),
            Stmt::Return(expr) => self.emit_return(expr.as_ref()),
            Stmt::Expr(expr) => { self.emit_expression(expr); }
            Stmt::Pass => {}
            _ => {}
        }
    }
    
    fn emit_print(&mut self, expr: &Expr) {
        if let Expr::String(s) = expr {
            // Procesar secuencias de escape como \n, \t, \r (estilo C++)
            let processed = s.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
            
            // NO añadir \n automáticamente - el usuario lo pone manualmente
            // Asegurar que el string procesado esté en la tabla
            if !self.strings.contains(&processed) {
                self.strings.push(processed.clone());
            }
            
            let string_addr = self.get_string_address(&processed);
            
            match self.target {
                Target::Linux => {
                    // sys_write(1, buf, len)
                    self.emit_bytes(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                    self.emit_bytes(&[0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00]); // mov rdi, 1
                    self.emit_bytes(&[0x48, 0xBE]);                               // mov rsi, addr
                    self.emit_u64(string_addr);
                    self.emit_bytes(&[0x48, 0xC7, 0xC2]);                         // mov rdx, len
                    self.emit_u32(processed.len() as u32);
                    self.emit_bytes(&[0x0F, 0x05]);                               // syscall
                }
                Target::Windows | Target::Raw => {
                    // Usar printf via IAT (compatible con codegen actual)
                    self.emit_bytes(&[0x48, 0xB9]);
                    self.emit_u64(string_addr);
                    self.emit_call_printf();
                }
            }
        } else {
            // Evaluar la expresión y determinar si es numérica
            self.emit_expression(expr); // RAX = valor
            
            // Detectar si es una expresión numérica (variable, número, operación)
            let is_numeric = matches!(expr, 
                Expr::Number(_) | 
                Expr::Variable(_) | 
                Expr::BinaryOp { .. } |
                Expr::Bool(_)
            );

            match self.target {
                Target::Windows | Target::Raw => {
                    if is_numeric {
                        // printf("%d", rax) para números (sin \n - el usuario lo pone)
                        let fmt_addr = self.get_string_address("%d");
                        
                        self.emit_bytes(&[0x48, 0x89, 0xC2]); // mov rdx, rax
                        self.emit_bytes(&[0x48, 0xB9]);       // mov rcx, fmt_addr
                        self.emit_u64(fmt_addr);
                        self.emit_call_printf();
                    } else {
                        // printf("%s", rax) para strings (sin \n - el usuario lo pone)
                        let fmt_addr = self.get_string_address("%s");

                        self.emit_bytes(&[0x48, 0x89, 0xC2]); // mov rdx, rax
                        self.emit_bytes(&[0x48, 0xB9]);       // mov rcx, fmt_addr
                        self.emit_u64(fmt_addr);
                        self.emit_call_printf();
                    }
                }
                Target::Linux => {
                    // TODO: Implement for Linux
                }
            }
        }
    }
    
    /// println - igual que print pero añade \n automáticamente
    fn emit_println(&mut self, expr: &Expr) {
        // Primero imprimir la expresión
        self.emit_print(expr);
        
        // Luego imprimir \n
        let newline = "\n".to_string();
        if !self.strings.contains(&newline) {
            self.strings.push(newline.clone());
        }
        let nl_addr = self.get_string_address("\n");
        
        match self.target {
            Target::Windows | Target::Raw => {
                self.emit_bytes(&[0x48, 0xB9]);
                self.emit_u64(nl_addr);
                self.emit_call_printf();
            }
            Target::Linux => {}
        }
    }
    
    fn emit_print_num(&mut self, expr: &Expr) {
        self.emit_expression(expr);
        
        // Usar %d sin \n - el usuario pone \n manualmente
        let fmt_addr = self.get_string_address("%d");
        
        self.emit_bytes(&[0x48, 0x89, 0xC2]);  // mov rdx, rax
        self.emit_bytes(&[0x48, 0xB9]);
        self.emit_u64(fmt_addr);
        self.emit_call_printf();
    }
    
    fn emit_call_printf(&mut self) {
        // Windows x64 calling convention
        self.emit_bytes(&[0x48, 0x83, 0xEC, 0x20]);  // sub rsp, 32
        
        // call [rip+offset] - placeholder para IAT
        let call_end_rva = 0x1000 + self.code.len() as u64 + 6;
        let iat_printf_rva = 0x2038u64;
        let offset = iat_printf_rva as i64 - call_end_rva as i64;
        self.emit_bytes(&[0xFF, 0x15]);
        self.emit_i32(offset as i32);
        
        self.emit_bytes(&[0x48, 0x83, 0xC4, 0x20]);  // add rsp, 32
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
        
        self.emit_bytes(&[0x48, 0x89, 0x85]);
        self.emit_i32(offset);
    }
    
    fn emit_if(&mut self, condition: &Expr, then_body: &[Stmt], else_body: Option<&[Stmt]>) {
        self.emit_condition(condition);
        self.emit_bytes(&[0x48, 0x85, 0xC0]);  // test rax, rax
        
        self.emit_bytes(&[0x0F, 0x84]);
        let je_offset_pos = self.code.len();
        self.emit_i32(0);
        
        for stmt in then_body {
            self.emit_statement(stmt);
        }
        
        if let Some(else_stmts) = else_body {
            self.emit_bytes(&[0xE9]);
            let jmp_offset_pos = self.code.len();
            self.emit_i32(0);
            
            let else_label = self.code.len();
            let je_offset = (else_label - je_offset_pos - 4) as i32;
            self.code[je_offset_pos..je_offset_pos + 4].copy_from_slice(&je_offset.to_le_bytes());
            
            for stmt in else_stmts {
                self.emit_statement(stmt);
            }
            
            let end_label = self.code.len();
            let jmp_offset = (end_label - jmp_offset_pos - 4) as i32;
            self.code[jmp_offset_pos..jmp_offset_pos + 4].copy_from_slice(&jmp_offset.to_le_bytes());
        } else {
            let else_label = self.code.len();
            let je_offset = (else_label - je_offset_pos - 4) as i32;
            self.code[je_offset_pos..je_offset_pos + 4].copy_from_slice(&je_offset.to_le_bytes());
        }
    }
    
    fn emit_while(&mut self, condition: &Expr, body: &[Stmt]) {
        let loop_start = self.code.len();
        
        self.emit_condition(condition);
        self.emit_bytes(&[0x48, 0x85, 0xC0]);
        
        self.emit_bytes(&[0x0F, 0x84]);
        let je_offset_pos = self.code.len();
        self.emit_i32(0);
        
        for stmt in body {
            self.emit_statement(stmt);
        }
        
        self.emit_bytes(&[0xE9]);
        let jmp_back = (loop_start as i64 - self.code.len() as i64 - 4) as i32;
        self.emit_i32(jmp_back);
        
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
        self.emit_bytes(&[0x48, 0x89, 0x85]);
        self.emit_i32(var_offset);
        
        // end en stack
        self.emit_expression(end);
        let end_offset = self.stack_offset;
        self.stack_offset -= 8;
        self.emit_bytes(&[0x48, 0x89, 0x85]);
        self.emit_i32(end_offset);
        
        let loop_start = self.code.len();
        
        // cmp var, end
        self.emit_bytes(&[0x48, 0x8B, 0x85]);
        self.emit_i32(var_offset);
        self.emit_bytes(&[0x48, 0x3B, 0x85]);
        self.emit_i32(end_offset);
        
        // jge end
        self.emit_bytes(&[0x0F, 0x8D]);
        let jge_offset_pos = self.code.len();
        self.emit_i32(0);
        
        for stmt in body {
            self.emit_statement(stmt);
        }
        
        // var++
        self.emit_bytes(&[0x48, 0xFF, 0x85]);
        self.emit_i32(var_offset);
        
        // jmp loop_start
        self.emit_bytes(&[0xE9]);
        let jmp_back = (loop_start as i64 - self.code.len() as i64 - 4) as i32;
        self.emit_i32(jmp_back);
        
        let end_label = self.code.len();
        let jge_offset = (end_label - jge_offset_pos - 4) as i32;
        self.code[jge_offset_pos..jge_offset_pos + 4].copy_from_slice(&jge_offset.to_le_bytes());
    }
    
    fn emit_return(&mut self, expr: Option<&Expr>) {
        if let Some(e) = expr {
            self.emit_expression(e);
        } else {
            self.emit_bytes(&[0x31, 0xC0]);
        }
        self.emit_bytes(&[0x48, 0x89, 0xEC]);
        self.emit_bytes(&[0x5D]);
        self.emit_bytes(&[0xC3]);
    }
    
    fn emit_condition(&mut self, expr: &Expr) {
        match expr {
            Expr::Comparison { op, left, right } => {
                self.emit_expression(left);
                self.emit_bytes(&[0x50]);
                self.emit_expression(right);
                self.emit_bytes(&[0x48, 0x89, 0xC3]);
                self.emit_bytes(&[0x58]);
                self.emit_bytes(&[0x48, 0x39, 0xD8]);
                
                match op {
                    CmpOp::Eq => self.emit_bytes(&[0x0F, 0x94, 0xC0]),
                    CmpOp::Ne => self.emit_bytes(&[0x0F, 0x95, 0xC0]),
                    CmpOp::Lt => self.emit_bytes(&[0x0F, 0x9C, 0xC0]),
                    CmpOp::Le => self.emit_bytes(&[0x0F, 0x9E, 0xC0]),
                    CmpOp::Gt => self.emit_bytes(&[0x0F, 0x9F, 0xC0]),
                    CmpOp::Ge => self.emit_bytes(&[0x0F, 0x9D, 0xC0]),
                }
                
                self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]);
            }
            Expr::Bool(b) => {
                let val = if *b { 1u32 } else { 0u32 };
                self.emit_bytes(&[0xB8]);
                self.emit_u32(val);
            }
            _ => self.emit_expression(expr),
        }
    }
    
    fn emit_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                self.emit_bytes(&[0x48, 0xB8]);
                self.emit_u64(*n as u64);
            }
            Expr::Bool(b) => {
                let val = if *b { 1u64 } else { 0u64 };
                self.emit_bytes(&[0x48, 0xB8]);
                self.emit_u64(val);
            }
            Expr::Variable(name) => {
                if let Some(&offset) = self.variables.get(name) {
                    self.emit_bytes(&[0x48, 0x8B, 0x85]);
                    self.emit_i32(offset);
                } else {
                    self.emit_bytes(&[0x31, 0xC0]);
                }
            }
            Expr::BinaryOp { op, left, right } => {
                self.emit_expression(left);
                self.emit_bytes(&[0x50]);
                self.emit_expression(right);
                self.emit_bytes(&[0x48, 0x89, 0xC3]);
                self.emit_bytes(&[0x58]);
                
                match op {
                    BinOp::Add => self.emit_bytes(&[0x48, 0x01, 0xD8]),
                    BinOp::Sub => self.emit_bytes(&[0x48, 0x29, 0xD8]),
                    BinOp::Mul => self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC3]),
                    BinOp::Div => {
                        self.emit_bytes(&[0x48, 0x99]);
                        self.emit_bytes(&[0x48, 0xF7, 0xFB]);
                    }
                    BinOp::Mod => {
                        self.emit_bytes(&[0x48, 0x99]);
                        self.emit_bytes(&[0x48, 0xF7, 0xFB]);
                        self.emit_bytes(&[0x48, 0x89, 0xD0]);
                    }
                    BinOp::And => self.emit_bytes(&[0x48, 0x21, 0xD8]),
                    BinOp::Or => self.emit_bytes(&[0x48, 0x09, 0xD8]),
                }
            }
            Expr::UnaryOp { op, expr: inner } => {
                self.emit_expression(inner);
                match op {
                    UnaryOp::Neg => self.emit_bytes(&[0x48, 0xF7, 0xD8]),
                    UnaryOp::Not => {
                        self.emit_bytes(&[0x48, 0x85, 0xC0]);
                        self.emit_bytes(&[0x0F, 0x94, 0xC0]);
                        self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]);
                    }
                }
            }
            Expr::Call { name, args } => {
                self.emit_call(name, args);
            }
            Expr::Comparison { .. } => self.emit_condition(expr),
            _ => {}
        }
    }
    
    fn emit_call(&mut self, name: &str, args: &[Expr]) {
        // Evaluar argumentos en registros (Windows x64)
        for (i, arg) in args.iter().enumerate().take(4) {
            self.emit_expression(arg);
            match i {
                0 => self.emit_bytes(&[0x48, 0x89, 0xC1]),  // mov rcx, rax
                1 => self.emit_bytes(&[0x48, 0x89, 0xC2]),  // mov rdx, rax
                2 => self.emit_bytes(&[0x49, 0x89, 0xC0]),  // mov r8, rax
                3 => self.emit_bytes(&[0x49, 0x89, 0xC1]),  // mov r9, rax
                _ => {}
            }
        }
        
        // Shadow space
        self.emit_bytes(&[0x48, 0x83, 0xEC, 0x20]);
        
        // call rel32 (placeholder)
        self.emit_bytes(&[0xE8]);
        let call_offset = self.code.len();
        self.function_calls.push((call_offset, name.to_string()));
        self.emit_i32(0);
        
        // Restaurar stack
        self.emit_bytes(&[0x48, 0x83, 0xC4, 0x20]);
    }
    
    // ========================================
    // Helpers
    // ========================================
    
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_codegen_v2_creation() {
        let cg = CodeGeneratorV2::new(Target::Windows);
        assert_eq!(cg.base_address, 0x400000);
    }
    
    #[test]
    fn test_codegen_v2_linux() {
        let cg = CodeGeneratorV2::new(Target::Linux);
        assert_eq!(cg.target, Target::Linux);
    }
}
