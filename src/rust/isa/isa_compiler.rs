// ============================================================
// ADead-BIB ISA Compiler — AST → ADeadIR → Bytes
// ============================================================
// Reemplaza codegen_v2.rs. En vez de emitir bytes directamente,
// genera instrucciones ADeadOp tipadas que luego se codifican.
//
// Pipeline: AST → ADeadIR (Vec<ADeadOp>) → Encoder → bytes
//
// Sin ASM. Sin NASM. Sin LLVM. Solo ISA puro.
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com
// ============================================================

use crate::frontend::ast::*;
use super::{ADeadIR, ADeadOp, Reg, Operand, Condition, Label, CallTarget};
use super::encoder::Encoder;
use std::collections::HashMap;

/// Target de compilación
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Target {
    Windows,
    Linux,
    Raw,
}

/// Función compilada (metadatos)
#[derive(Clone, Debug)]
pub struct CompiledFunction {
    pub name: String,
    pub label: Label,
    pub params: Vec<String>,
}

/// ISA Compiler — Compilador que genera ADeadIR en vez de bytes directos.
pub struct IsaCompiler {
    ir: ADeadIR,

    // Strings
    strings: Vec<String>,
    string_offsets: HashMap<String, u64>,

    // Funciones
    functions: HashMap<String, CompiledFunction>,

    // Estado actual
    current_function: Option<String>,
    variables: HashMap<String, i32>,
    stack_offset: i32,

    // Configuración
    target: Target,
    base_address: u64,
    data_rva: u64,
}

impl IsaCompiler {
    pub fn new(target: Target) -> Self {
        let (base, data_rva) = match target {
            Target::Windows => (0x0000000140000000, 0x2078),
            Target::Linux => (0x400000, 0x1000),
            Target::Raw => (0x0, 0x1000),
        };

        Self {
            ir: ADeadIR::new(),
            strings: Vec::new(),
            string_offsets: HashMap::new(),
            functions: HashMap::new(),
            current_function: None,
            variables: HashMap::new(),
            stack_offset: 0,
            target,
            base_address: base,
            data_rva,
        }
    }

    /// Compila un programa completo y retorna (code_bytes, data_bytes).
    pub fn compile(&mut self, program: &Program) -> (Vec<u8>, Vec<u8>) {
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

        // Fase 3: Si hay main y otras funciones, saltar a main
        let main_label = self.functions.get("main").map(|f| f.label);
        if has_main && program.functions.len() > 1 {
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
        if !program.statements.is_empty() {
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

        // Fase 8: Resolver llamadas a funciones por nombre
        let mut code = result.code;
        for (offset, name) in &result.unresolved_calls {
            if let Some(func) = self.functions.get(name) {
                // Necesitamos saber la posición real del label en el código
                // El encoder ya resolvió los labels internos, pero las llamadas
                // por nombre quedan pendientes. Re-encode para obtener posiciones.
                // Por ahora, las llamadas internas usan CallTarget::Relative(label)
                // y solo Name() se usa para funciones externas no resueltas.
                let _ = (offset, func);
            }
        }

        // Fase 9: Generar sección de datos
        let data = self.generate_data_section();

        (code, data)
    }

    // ========================================
    // Recolección de strings
    // ========================================

    fn collect_all_strings(&mut self, program: &Program) {
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

    fn collect_strings_from_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::Print(Expr::String(s)) | Stmt::Println(Expr::String(s)) => {
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

    fn generate_data_section(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for s in &self.strings {
            data.extend_from_slice(s.as_bytes());
            data.push(0);
        }
        data
    }

    fn get_string_address(&self, s: &str) -> u64 {
        if let Some(&offset) = self.string_offsets.get(s) {
            self.base_address + self.data_rva + offset
        } else {
            self.base_address + self.data_rva
        }
    }

    // ========================================
    // Compilación de funciones
    // ========================================

    fn compile_function(&mut self, func: &Function) {
        self.current_function = Some(func.name.clone());
        self.variables.clear();
        self.stack_offset = -8;

        let is_interrupt = func.attributes.is_interrupt;
        let is_exception = func.attributes.is_exception;
        let is_naked = func.attributes.is_naked;

        // Label de entrada
        if let Some(compiled) = self.functions.get(&func.name) {
            let label = compiled.label;
            self.ir.emit(ADeadOp::Label(label));
        }

        if is_interrupt || is_exception {
            // @interrupt / @exception: push all registers (auto-generated wrapper)
            self.emit_interrupt_prologue();
        } else if !is_naked {
            // Normal function prologue
            self.emit_prologue();

            // Registrar y guardar parámetros
            for (i, param) in func.params.iter().enumerate() {
                let param_offset = if i <= 3 {
                    let off = self.stack_offset;
                    self.stack_offset -= 8;
                    off
                } else {
                    16 + ((i - 4) as i32 * 8)
                };
                self.variables.insert(param.name.clone(), param_offset);

                // Guardar parámetros de registros en stack
                if i <= 3 {
                    let src_reg = match i {
                        0 => Reg::RCX,
                        1 => Reg::RDX,
                        2 => Reg::R8,
                        3 => Reg::R9,
                        _ => unreachable!(),
                    };
                    self.ir.emit(ADeadOp::Mov {
                        dst: Operand::Mem { base: Reg::RBP, disp: param_offset },
                        src: Operand::Reg(src_reg),
                    });
                }
            }
        }
        // @naked: no prologue at all

        // Body
        for stmt in &func.body {
            self.emit_statement(stmt);
        }

        if is_interrupt || is_exception {
            // @interrupt / @exception: pop all registers + iretq
            self.emit_interrupt_epilogue();
        } else if !is_naked {
            // Normal function epilogue
            self.emit_epilogue();
        }
        // @naked: no epilogue at all

        self.current_function = None;
    }

    fn compile_top_level(&mut self, stmts: &[Stmt]) {
        self.current_function = Some("__entry".to_string());
        self.variables.clear();
        self.stack_offset = -8;

        // For bare-metal (Target::Raw), emit instructions directly — no prologue/epilogue.
        // Boot sectors and flat binaries need raw machine code, not 64-bit function frames.
        let is_raw = self.target == Target::Raw;

        if !is_raw {
            self.emit_prologue();
        }

        for stmt in stmts {
            self.emit_statement(stmt);
        }

        if !is_raw {
            self.emit_epilogue();
        }
        self.current_function = None;
    }

    // ========================================
    // Prologue / Epilogue
    // ========================================

    fn emit_prologue(&mut self) {
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RBP) });
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::RBP),
            src: Operand::Reg(Reg::RSP),
        });
        // sub rsp, 128 (espacio fijo para locales — se puede optimizar después)
        self.ir.emit(ADeadOp::Sub {
            dst: Operand::Reg(Reg::RSP),
            src: Operand::Imm32(128),
        });
    }

    fn emit_epilogue(&mut self) {
        self.ir.emit(ADeadOp::Xor { dst: Reg::EAX, src: Reg::EAX });
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::RSP),
            src: Operand::Reg(Reg::RBP),
        });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RBP });
        self.ir.emit(ADeadOp::Ret);
    }

    // ========================================
    // Interrupt Prologue / Epilogue
    // ========================================

    fn emit_interrupt_prologue(&mut self) {
        // Push all general purpose registers (64-bit)
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RAX) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RBX) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RCX) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RDX) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RSI) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RDI) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RBP) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::R8) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::R9) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::R10) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::R11) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::R12) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::R13) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::R14) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::R15) });
    }

    fn emit_interrupt_epilogue(&mut self) {
        // Pop all general purpose registers (reverse order)
        self.ir.emit(ADeadOp::Pop { dst: Reg::R15 });
        self.ir.emit(ADeadOp::Pop { dst: Reg::R14 });
        self.ir.emit(ADeadOp::Pop { dst: Reg::R13 });
        self.ir.emit(ADeadOp::Pop { dst: Reg::R12 });
        self.ir.emit(ADeadOp::Pop { dst: Reg::R11 });
        self.ir.emit(ADeadOp::Pop { dst: Reg::R10 });
        self.ir.emit(ADeadOp::Pop { dst: Reg::R9 });
        self.ir.emit(ADeadOp::Pop { dst: Reg::R8 });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RBP });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RDI });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RSI });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RDX });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RCX });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RBX });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RAX });
        // IRETQ — return from interrupt
        self.ir.emit(ADeadOp::Iret);
    }

    // ========================================
    // Statements
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

            // ========== OS-LEVEL / MACHINE CODE (v3.1-OS) ==========
            Stmt::Cli => {
                self.ir.emit(ADeadOp::Cli);
            }
            Stmt::Sti => {
                self.ir.emit(ADeadOp::Sti);
            }
            Stmt::Hlt => {
                self.ir.emit(ADeadOp::Hlt);
            }
            Stmt::Iret => {
                self.ir.emit(ADeadOp::Iret);
            }
            Stmt::Cpuid => {
                self.ir.emit(ADeadOp::Cpuid);
            }
            Stmt::IntCall { vector } => {
                self.ir.emit(ADeadOp::Int { vector: *vector });
            }
            Stmt::RegAssign { reg_name, value } => {
                self.emit_reg_assign(reg_name, value);
            }
            Stmt::MemWrite { addr, value } => {
                self.emit_mem_write(addr, value);
            }
            Stmt::PortOut { port, value } => {
                self.emit_port_out(port, value);
            }
            Stmt::RawBlock { bytes } => {
                self.ir.emit(ADeadOp::RawBytes(bytes.clone()));
            }
            Stmt::OrgDirective { address } => {
                // Store origin for address calculations
                self.base_address = *address;
            }
            Stmt::AlignDirective { alignment } => {
                // Emit NOP padding to align to boundary
                let align = *alignment as usize;
                if align > 0 {
                    // We'll emit a placeholder; actual alignment resolved at link time
                    // For flat binary, pad with NOPs
                    self.ir.emit(ADeadOp::RawBytes(vec![0x90])); // marker NOP
                }
            }
            Stmt::FarJump { selector, offset } => {
                self.ir.emit(ADeadOp::FarJmp {
                    selector: *selector,
                    offset: *offset,
                });
            }

            // Pointer/memory statements (v3.2)
            Stmt::VarDecl { var_type: _, name, value } => {
                if let Some(val) = value {
                    self.emit_assign(name, val);
                } else {
                    // Declare with zero
                    let offset = self.stack_offset;
                    self.variables.insert(name.clone(), offset);
                    self.stack_offset -= 8;
                    self.ir.emit(ADeadOp::Xor { dst: Reg::RAX, src: Reg::RAX });
                    self.ir.emit(ADeadOp::Mov {
                        dst: Operand::Mem { base: Reg::RBP, disp: offset },
                        src: Operand::Reg(Reg::RAX),
                    });
                }
            }
            Stmt::CompoundAssign { name, op, value } => {
                self.emit_compound_assign(name, op, value);
            }
            Stmt::Increment { name, is_pre: _, is_increment } => {
                if let Some(&offset) = self.variables.get(name.as_str()) {
                    if *is_increment {
                        self.ir.emit(ADeadOp::Inc {
                            dst: Operand::Mem { base: Reg::RBP, disp: offset },
                        });
                    } else {
                        self.ir.emit(ADeadOp::Dec {
                            dst: Operand::Mem { base: Reg::RBP, disp: offset },
                        });
                    }
                }
            }
            Stmt::DoWhile { body, condition } => {
                let loop_start = self.ir.new_label();
                self.ir.emit(ADeadOp::Label(loop_start));
                for s in body {
                    self.emit_statement(s);
                }
                self.emit_condition(condition);
                self.ir.emit(ADeadOp::Test { left: Reg::RAX, right: Reg::RAX });
                self.ir.emit(ADeadOp::Jcc { cond: Condition::NotEqual, target: loop_start });
            }

            _ => {}
        }
    }

    // ========================================
    // OS-Level Helpers
    // ========================================

    fn string_to_reg(name: &str) -> Option<Reg> {
        match name {
            "rax" => Some(Reg::RAX), "rbx" => Some(Reg::RBX),
            "rcx" => Some(Reg::RCX), "rdx" => Some(Reg::RDX),
            "rsi" => Some(Reg::RSI), "rdi" => Some(Reg::RDI),
            "rbp" => Some(Reg::RBP), "rsp" => Some(Reg::RSP),
            "r8"  => Some(Reg::R8),  "r9"  => Some(Reg::R9),
            "r10" => Some(Reg::R10), "r11" => Some(Reg::R11),
            "r12" => Some(Reg::R12), "r13" => Some(Reg::R13),
            "r14" => Some(Reg::R14), "r15" => Some(Reg::R15),
            "eax" => Some(Reg::EAX), "ebx" => Some(Reg::EBX),
            "ecx" => Some(Reg::ECX), "edx" => Some(Reg::EDX),
            "esi" => Some(Reg::ESI), "edi" => Some(Reg::EDI),
            "esp" => Some(Reg::ESP), "ebp" => Some(Reg::EBP),
            "ax"  => Some(Reg::AX),  "bx"  => Some(Reg::BX),
            "cx"  => Some(Reg::CX),  "dx"  => Some(Reg::DX),
            "si"  => Some(Reg::SI),  "di"  => Some(Reg::DI),
            "sp"  => Some(Reg::SP),  "bp"  => Some(Reg::BP),
            "al"  => Some(Reg::AL),  "ah"  => Some(Reg::AH),
            "bl"  => Some(Reg::BL),  "bh"  => Some(Reg::BH),
            "cl"  => Some(Reg::CL),  "ch"  => Some(Reg::CH),
            "dl"  => Some(Reg::DL),  "dh"  => Some(Reg::DH),
            "cr0" => Some(Reg::CR0), "cr2" => Some(Reg::CR2),
            "cr3" => Some(Reg::CR3), "cr4" => Some(Reg::CR4),
            "cs"  => Some(Reg::CS),  "ds"  => Some(Reg::DS),
            "es"  => Some(Reg::ES),  "fs"  => Some(Reg::FS),
            "gs"  => Some(Reg::GS),  "ss"  => Some(Reg::SS),
            _ => None,
        }
    }

    fn emit_reg_assign(&mut self, reg_name: &str, value: &Expr) {
        self.emit_expression(value);
        if let Some(reg) = Self::string_to_reg(reg_name) {
            if reg.is_control() {
                // mov crN, rax
                let cr_num = match reg {
                    Reg::CR0 => 0, Reg::CR2 => 2, Reg::CR3 => 3, Reg::CR4 => 4,
                    _ => 0,
                };
                self.ir.emit(ADeadOp::MovToCr { cr: cr_num, src: Reg::RAX });
            } else if reg.is_segment() {
                // Segment register assignment via raw bytes
                // mov <seg>, ax requires specific encoding
                let seg_code: u8 = match reg {
                    Reg::DS => 0xD8, Reg::ES => 0xC0, Reg::SS => 0xD0,
                    Reg::FS => 0xE0, Reg::GS => 0xE8,
                    _ => 0xD8,
                };
                // 8E /r = mov Sreg, r/m16
                self.ir.emit(ADeadOp::RawBytes(vec![0x8E, seg_code]));
            } else {
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(reg),
                    src: Operand::Reg(Reg::RAX),
                });
            }
        }
    }

    fn emit_mem_write(&mut self, addr: &Expr, value: &Expr) {
        // Evaluate value → RAX, then addr → RBX, then mov [RBX], RAX
        self.emit_expression(value);
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RAX) });
        self.emit_expression(addr);
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::RBX),
            src: Operand::Reg(Reg::RAX),
        });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RAX });
        // mov [rbx], rax
        self.ir.emit(ADeadOp::RawBytes(vec![0x48, 0x89, 0x03]));
    }

    fn emit_port_out(&mut self, port: &Expr, value: &Expr) {
        // Evaluate value → AL, port → immediate or DX
        self.emit_expression(value);
        match port {
            Expr::Number(p) if *p >= 0 && *p <= 255 => {
                self.ir.emit(ADeadOp::OutByte {
                    port: Operand::Imm8(*p as i8),
                    src: Operand::Reg(Reg::AL),
                });
            }
            _ => {
                // Port in DX
                self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RAX) });
                self.emit_expression(port);
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RDX),
                    src: Operand::Reg(Reg::RAX),
                });
                self.ir.emit(ADeadOp::Pop { dst: Reg::RAX });
                self.ir.emit(ADeadOp::OutByte {
                    port: Operand::Reg(Reg::DX),
                    src: Operand::Reg(Reg::AL),
                });
            }
        }
    }

    fn emit_compound_assign(&mut self, name: &str, op: &CompoundOp, value: &Expr) {
        // Load current value
        if let Some(&offset) = self.variables.get(name) {
            self.emit_expression(value);
            self.ir.emit(ADeadOp::Mov {
                dst: Operand::Reg(Reg::RBX),
                src: Operand::Reg(Reg::RAX),
            });
            self.ir.emit(ADeadOp::Mov {
                dst: Operand::Reg(Reg::RAX),
                src: Operand::Mem { base: Reg::RBP, disp: offset },
            });
            match op {
                CompoundOp::AddAssign => self.ir.emit(ADeadOp::Add {
                    dst: Operand::Reg(Reg::RAX), src: Operand::Reg(Reg::RBX),
                }),
                CompoundOp::SubAssign => self.ir.emit(ADeadOp::Sub {
                    dst: Operand::Reg(Reg::RAX), src: Operand::Reg(Reg::RBX),
                }),
                CompoundOp::MulAssign => self.ir.emit(ADeadOp::Mul { dst: Reg::RAX, src: Reg::RBX }),
                CompoundOp::DivAssign => self.ir.emit(ADeadOp::Div { src: Reg::RBX }),
                CompoundOp::AndAssign => self.ir.emit(ADeadOp::And { dst: Reg::RAX, src: Reg::RBX }),
                CompoundOp::OrAssign  => self.ir.emit(ADeadOp::Or  { dst: Reg::RAX, src: Reg::RBX }),
                CompoundOp::XorAssign => self.ir.emit(ADeadOp::Xor { dst: Reg::RAX, src: Reg::RBX }),
                CompoundOp::ShlAssign => {
                    // Move shift amount to CL, then SHL
                    self.ir.emit(ADeadOp::Shl { dst: Reg::RAX, amount: 1 }); // simplified
                }
                CompoundOp::ShrAssign => {
                    self.ir.emit(ADeadOp::Shr { dst: Reg::RAX, amount: 1 });
                }
                CompoundOp::ModAssign => {
                    self.ir.emit(ADeadOp::Div { src: Reg::RBX });
                    self.ir.emit(ADeadOp::Mov {
                        dst: Operand::Reg(Reg::RAX),
                        src: Operand::Reg(Reg::RDX),
                    });
                }
            }
            self.ir.emit(ADeadOp::Mov {
                dst: Operand::Mem { base: Reg::RBP, disp: offset },
                src: Operand::Reg(Reg::RAX),
            });
        }
    }

    // ========================================
    // Print / Println
    // ========================================

    fn emit_print(&mut self, expr: &Expr) {
        if let Expr::String(s) = expr {
            let processed = s.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
            if !self.strings.contains(&processed) {
                self.strings.push(processed.clone());
            }
            let string_addr = self.get_string_address(&processed);

            match self.target {
                Target::Linux => {
                    // sys_write(1, buf, len)
                    self.ir.emit(ADeadOp::Mov {
                        dst: Operand::Reg(Reg::RAX),
                        src: Operand::Imm32(1),
                    });
                    self.ir.emit(ADeadOp::Mov {
                        dst: Operand::Reg(Reg::RDI),
                        src: Operand::Imm32(1),
                    });
                    self.ir.emit(ADeadOp::Mov {
                        dst: Operand::Reg(Reg::RSI),
                        src: Operand::Imm64(string_addr),
                    });
                    self.ir.emit(ADeadOp::Mov {
                        dst: Operand::Reg(Reg::RDX),
                        src: Operand::Imm32(processed.len() as i32),
                    });
                    self.ir.emit(ADeadOp::Syscall);
                }
                Target::Windows | Target::Raw => {
                    self.ir.emit(ADeadOp::Mov {
                        dst: Operand::Reg(Reg::RCX),
                        src: Operand::Imm64(string_addr),
                    });
                    self.emit_call_printf();
                }
            }
        } else {
            self.emit_expression(expr);

            let is_float = matches!(expr, Expr::Float(_));
            let is_integer = matches!(expr,
                Expr::Number(_) | Expr::Variable(_) | Expr::BinaryOp { .. } |
                Expr::Bool(_) | Expr::Call { .. } | Expr::IntCast(_) | Expr::Len(_)
            );

            match self.target {
                Target::Windows | Target::Raw => {
                    if is_float {
                        let fmt_addr = self.get_string_address("%.2f");
                        self.ir.emit(ADeadOp::Mov {
                            dst: Operand::Reg(Reg::RDX),
                            src: Operand::Reg(Reg::RAX),
                        });
                        self.ir.emit(ADeadOp::MovQ { dst: Reg::XMM1, src: Reg::RDX });
                        self.ir.emit(ADeadOp::Mov {
                            dst: Operand::Reg(Reg::RCX),
                            src: Operand::Imm64(fmt_addr),
                        });
                        self.emit_call_printf();
                    } else if is_integer {
                        let fmt_addr = self.get_string_address("%d");
                        self.ir.emit(ADeadOp::Mov {
                            dst: Operand::Reg(Reg::RDX),
                            src: Operand::Reg(Reg::RAX),
                        });
                        self.ir.emit(ADeadOp::Mov {
                            dst: Operand::Reg(Reg::RCX),
                            src: Operand::Imm64(fmt_addr),
                        });
                        self.emit_call_printf();
                    } else {
                        let fmt_addr = self.get_string_address("%s");
                        self.ir.emit(ADeadOp::Mov {
                            dst: Operand::Reg(Reg::RDX),
                            src: Operand::Reg(Reg::RAX),
                        });
                        self.ir.emit(ADeadOp::Mov {
                            dst: Operand::Reg(Reg::RCX),
                            src: Operand::Imm64(fmt_addr),
                        });
                        self.emit_call_printf();
                    }
                }
                Target::Linux => {}
            }
        }
    }

    fn emit_println(&mut self, expr: &Expr) {
        self.emit_print(expr);
        // Print newline
        let newline = "\n".to_string();
        if !self.strings.contains(&newline) {
            self.strings.push(newline);
        }
        let nl_addr = self.get_string_address("\n");
        match self.target {
            Target::Windows | Target::Raw => {
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RCX),
                    src: Operand::Imm64(nl_addr),
                });
                self.emit_call_printf();
            }
            Target::Linux => {}
        }
    }

    fn emit_print_num(&mut self, expr: &Expr) {
        self.emit_expression(expr);
        let fmt_addr = self.get_string_address("%d");
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::RDX),
            src: Operand::Reg(Reg::RAX),
        });
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::RCX),
            src: Operand::Imm64(fmt_addr),
        });
        self.emit_call_printf();
    }

    fn emit_call_printf(&mut self) {
        // Shadow space (Windows x64 ABI)
        self.ir.emit(ADeadOp::Sub {
            dst: Operand::Reg(Reg::RSP),
            src: Operand::Imm8(32),
        });
        // call [rip+offset] — IAT printf at RVA 0x2040
        // El encoder calcula el offset RIP-relative automáticamente
        self.ir.emit(ADeadOp::CallIAT { iat_rva: 0x2040 });
        // Restaurar stack
        self.ir.emit(ADeadOp::Add {
            dst: Operand::Reg(Reg::RSP),
            src: Operand::Imm8(32),
        });
    }

    // ========================================
    // Assign
    // ========================================

    fn emit_assign(&mut self, name: &str, value: &Expr) {
        // Optimización: x = x + 1 → inc, x = x - 1 → dec
        if let Some(&offset) = self.variables.get(name) {
            if let Expr::BinaryOp { op, left, right } = value {
                if let Expr::Variable(var_name) = left.as_ref() {
                    if var_name == name {
                        if let Expr::Number(n) = right.as_ref() {
                            if *n == 1 {
                                match op {
                                    BinOp::Add => {
                                        self.ir.emit(ADeadOp::Inc {
                                            dst: Operand::Mem { base: Reg::RBP, disp: offset },
                                        });
                                        return;
                                    }
                                    BinOp::Sub => {
                                        self.ir.emit(ADeadOp::Dec {
                                            dst: Operand::Mem { base: Reg::RBP, disp: offset },
                                        });
                                        return;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        self.emit_expression(value);

        let offset = if let Some(&off) = self.variables.get(name) {
            off
        } else {
            let off = self.stack_offset;
            self.variables.insert(name.to_string(), off);
            self.stack_offset -= 8;
            off
        };

        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Mem { base: Reg::RBP, disp: offset },
            src: Operand::Reg(Reg::RAX),
        });
    }

    // ========================================
    // Control Flow
    // ========================================

    fn emit_if(&mut self, condition: &Expr, then_body: &[Stmt], else_body: Option<&[Stmt]>) {
        self.emit_condition(condition);
        self.ir.emit(ADeadOp::Test { left: Reg::RAX, right: Reg::RAX });

        let else_label = self.ir.new_label();
        self.ir.emit(ADeadOp::Jcc { cond: Condition::Equal, target: else_label });

        for stmt in then_body {
            self.emit_statement(stmt);
        }

        if let Some(else_stmts) = else_body {
            let end_label = self.ir.new_label();
            self.ir.emit(ADeadOp::Jmp { target: end_label });
            self.ir.emit(ADeadOp::Label(else_label));
            for stmt in else_stmts {
                self.emit_statement(stmt);
            }
            self.ir.emit(ADeadOp::Label(end_label));
        } else {
            self.ir.emit(ADeadOp::Label(else_label));
        }
    }

    fn emit_while(&mut self, condition: &Expr, body: &[Stmt]) {
        let loop_start = self.ir.new_label();
        let loop_end = self.ir.new_label();

        self.ir.emit(ADeadOp::Label(loop_start));
        self.emit_condition(condition);
        self.ir.emit(ADeadOp::Test { left: Reg::RAX, right: Reg::RAX });
        self.ir.emit(ADeadOp::Jcc { cond: Condition::Equal, target: loop_end });

        for stmt in body {
            self.emit_statement(stmt);
        }

        self.ir.emit(ADeadOp::Jmp { target: loop_start });
        self.ir.emit(ADeadOp::Label(loop_end));
    }

    fn emit_for(&mut self, var: &str, start: &Expr, end: &Expr, body: &[Stmt]) {
        // Evaluar start → RCX, end → R8
        self.emit_expression(start);
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::RCX),
            src: Operand::Reg(Reg::RAX),
        });
        self.emit_expression(end);
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::R8),
            src: Operand::Reg(Reg::RAX),
        });

        let var_offset = self.stack_offset;
        self.variables.insert(var.to_string(), var_offset);
        self.stack_offset -= 8;

        let loop_start = self.ir.new_label();
        let loop_end = self.ir.new_label();

        self.ir.emit(ADeadOp::Label(loop_start));
        self.ir.emit(ADeadOp::Cmp {
            left: Operand::Reg(Reg::RCX),
            right: Operand::Reg(Reg::R8),
        });
        self.ir.emit(ADeadOp::Jcc { cond: Condition::GreaterEq, target: loop_end });

        // Guardar RCX en variable
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Mem { base: Reg::RBP, disp: var_offset },
            src: Operand::Reg(Reg::RCX),
        });

        // Preservar RCX y R8
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RCX) });
        self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::R8) });

        for stmt in body {
            self.emit_statement(stmt);
        }

        self.ir.emit(ADeadOp::Pop { dst: Reg::R8 });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RCX });
        self.ir.emit(ADeadOp::Inc { dst: Operand::Reg(Reg::RCX) });
        self.ir.emit(ADeadOp::Jmp { target: loop_start });
        self.ir.emit(ADeadOp::Label(loop_end));
    }

    fn emit_return(&mut self, expr: Option<&Expr>) {
        if let Some(e) = expr {
            self.emit_expression(e);
        } else {
            self.ir.emit(ADeadOp::Xor { dst: Reg::EAX, src: Reg::EAX });
        }
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::RSP),
            src: Operand::Reg(Reg::RBP),
        });
        self.ir.emit(ADeadOp::Pop { dst: Reg::RBP });
        self.ir.emit(ADeadOp::Ret);
    }

    // ========================================
    // Conditions
    // ========================================

    fn emit_condition(&mut self, expr: &Expr) {
        match expr {
            Expr::Comparison { op, left, right } => {
                self.emit_expression(left);
                self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RAX) });
                self.emit_expression(right);
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RBX),
                    src: Operand::Reg(Reg::RAX),
                });
                self.ir.emit(ADeadOp::Pop { dst: Reg::RAX });
                self.ir.emit(ADeadOp::Cmp {
                    left: Operand::Reg(Reg::RAX),
                    right: Operand::Reg(Reg::RBX),
                });

                let cond = match op {
                    CmpOp::Eq => Condition::Equal,
                    CmpOp::Ne => Condition::NotEqual,
                    CmpOp::Lt => Condition::Less,
                    CmpOp::Le => Condition::LessEq,
                    CmpOp::Gt => Condition::Greater,
                    CmpOp::Ge => Condition::GreaterEq,
                };
                self.ir.emit(ADeadOp::SetCC { cond, dst: Reg::AL });
                self.ir.emit(ADeadOp::MovZx { dst: Reg::RAX, src: Reg::AL });
            }
            Expr::Bool(b) => {
                let val = if *b { 1 } else { 0 };
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::EAX),
                    src: Operand::Imm32(val),
                });
            }
            _ => self.emit_expression(expr),
        }
    }

    // ========================================
    // Expressions → RAX
    // ========================================

    fn emit_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RAX),
                    src: Operand::Imm64(*n as u64),
                });
            }
            Expr::Float(f) => {
                let bits = f.to_bits();
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RAX),
                    src: Operand::Imm64(bits),
                });
            }
            Expr::Bool(b) => {
                let val = if *b { 1u64 } else { 0u64 };
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RAX),
                    src: Operand::Imm64(val),
                });
            }
            Expr::Variable(name) => {
                if let Some(&offset) = self.variables.get(name) {
                    self.ir.emit(ADeadOp::Mov {
                        dst: Operand::Reg(Reg::RAX),
                        src: Operand::Mem { base: Reg::RBP, disp: offset },
                    });
                } else {
                    self.ir.emit(ADeadOp::Xor { dst: Reg::EAX, src: Reg::EAX });
                }
            }
            Expr::BinaryOp { op, left, right } => {
                self.emit_expression(left);
                self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RAX) });
                self.emit_expression(right);
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RBX),
                    src: Operand::Reg(Reg::RAX),
                });
                self.ir.emit(ADeadOp::Pop { dst: Reg::RAX });

                match op {
                    BinOp::Add => self.ir.emit(ADeadOp::Add {
                        dst: Operand::Reg(Reg::RAX),
                        src: Operand::Reg(Reg::RBX),
                    }),
                    BinOp::Sub => self.ir.emit(ADeadOp::Sub {
                        dst: Operand::Reg(Reg::RAX),
                        src: Operand::Reg(Reg::RBX),
                    }),
                    BinOp::Mul => self.ir.emit(ADeadOp::Mul { dst: Reg::RAX, src: Reg::RBX }),
                    BinOp::Div => self.ir.emit(ADeadOp::Div { src: Reg::RBX }),
                    BinOp::Mod => {
                        self.ir.emit(ADeadOp::Div { src: Reg::RBX });
                        // Remainder is in RDX after idiv
                        self.ir.emit(ADeadOp::Mov {
                            dst: Operand::Reg(Reg::RAX),
                            src: Operand::Reg(Reg::RDX),
                        });
                    }
                    BinOp::And => self.ir.emit(ADeadOp::And { dst: Reg::RAX, src: Reg::RBX }),
                    BinOp::Or => self.ir.emit(ADeadOp::Or { dst: Reg::RAX, src: Reg::RBX }),
                }
            }
            Expr::UnaryOp { op, expr: inner } => {
                self.emit_expression(inner);
                match op {
                    UnaryOp::Neg => self.ir.emit(ADeadOp::Neg { dst: Reg::RAX }),
                    UnaryOp::Not => self.ir.emit(ADeadOp::Not { dst: Reg::RAX }),
                }
            }
            Expr::Call { name, args } => {
                self.emit_call(name, args);
            }
            Expr::Comparison { .. } => self.emit_condition(expr),
            Expr::Input => {
                self.emit_input();
            }
            Expr::IntCast(inner) => {
                self.emit_expression(inner);
            }
            Expr::FloatCast(inner) => {
                self.emit_expression(inner);
                self.ir.emit(ADeadOp::CvtSi2Sd { dst: Reg::XMM0, src: Reg::RAX });
                self.ir.emit(ADeadOp::MovQ { dst: Reg::RAX, src: Reg::XMM0 });
            }
            Expr::BoolCast(inner) => {
                self.emit_expression(inner);
                self.ir.emit(ADeadOp::Test { left: Reg::RAX, right: Reg::RAX });
                self.ir.emit(ADeadOp::SetCC { cond: Condition::NotEqual, dst: Reg::AL });
                self.ir.emit(ADeadOp::MovZx { dst: Reg::RAX, src: Reg::AL });
            }
            // OS-Level expressions
            Expr::RegRead { reg_name } => {
                if let Some(reg) = Self::string_to_reg(reg_name) {
                    if reg.is_control() {
                        let cr_num = match reg {
                            Reg::CR0 => 0, Reg::CR2 => 2, Reg::CR3 => 3, Reg::CR4 => 4,
                            _ => 0,
                        };
                        self.ir.emit(ADeadOp::MovFromCr { cr: cr_num, dst: Reg::RAX });
                    } else {
                        self.ir.emit(ADeadOp::Mov {
                            dst: Operand::Reg(Reg::RAX),
                            src: Operand::Reg(reg),
                        });
                    }
                }
            }
            Expr::MemRead { addr } => {
                self.emit_expression(addr);
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RBX),
                    src: Operand::Reg(Reg::RAX),
                });
                // mov rax, [rbx]
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RAX),
                    src: Operand::Mem { base: Reg::RBX, disp: 0 },
                });
            }
            Expr::PortIn { port } => {
                match port.as_ref() {
                    Expr::Number(p) if *p >= 0 && *p <= 255 => {
                        self.ir.emit(ADeadOp::InByte {
                            port: Operand::Imm8(*p as i8),
                        });
                        // Result in AL, zero-extend to RAX
                        self.ir.emit(ADeadOp::MovZx { dst: Reg::RAX, src: Reg::AL });
                    }
                    _ => {
                        self.emit_expression(port);
                        self.ir.emit(ADeadOp::Mov {
                            dst: Operand::Reg(Reg::RDX),
                            src: Operand::Reg(Reg::RAX),
                        });
                        self.ir.emit(ADeadOp::InByte {
                            port: Operand::Reg(Reg::DX),
                        });
                        self.ir.emit(ADeadOp::MovZx { dst: Reg::RAX, src: Reg::AL });
                    }
                }
            }
            Expr::CpuidExpr => {
                self.ir.emit(ADeadOp::Cpuid);
                // EAX already has result
            }
            // Bitwise operations
            Expr::BitwiseOp { op, left, right } => {
                self.emit_expression(left);
                self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RAX) });
                self.emit_expression(right);
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RBX),
                    src: Operand::Reg(Reg::RAX),
                });
                self.ir.emit(ADeadOp::Pop { dst: Reg::RAX });
                match op {
                    BitwiseOp::And => self.ir.emit(ADeadOp::And { dst: Reg::RAX, src: Reg::RBX }),
                    BitwiseOp::Or  => self.ir.emit(ADeadOp::Or  { dst: Reg::RAX, src: Reg::RBX }),
                    BitwiseOp::Xor => self.ir.emit(ADeadOp::Xor { dst: Reg::RAX, src: Reg::RBX }),
                    BitwiseOp::LeftShift  => self.ir.emit(ADeadOp::Shl { dst: Reg::RAX, amount: 1 }),
                    BitwiseOp::RightShift => self.ir.emit(ADeadOp::Shr { dst: Reg::RAX, amount: 1 }),
                }
            }
            Expr::BitwiseNot(inner) => {
                self.emit_expression(inner);
                // Bitwise NOT: not rax (F7 /2)
                self.ir.emit(ADeadOp::RawBytes(vec![0x48, 0xF7, 0xD0]));
            }
            Expr::PreIncrement(inner) | Expr::PostIncrement(inner) => {
                self.emit_expression(inner);
                self.ir.emit(ADeadOp::Inc { dst: Operand::Reg(Reg::RAX) });
            }
            Expr::PreDecrement(inner) | Expr::PostDecrement(inner) => {
                self.emit_expression(inner);
                self.ir.emit(ADeadOp::Dec { dst: Operand::Reg(Reg::RAX) });
            }
            Expr::Nullptr | Expr::Null => {
                self.ir.emit(ADeadOp::Xor { dst: Reg::RAX, src: Reg::RAX });
            }
            Expr::String(s) => {
                let processed = s.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
                if !self.strings.contains(&processed) {
                    self.strings.push(processed.clone());
                }
                let addr = self.get_string_address(&processed);
                self.ir.emit(ADeadOp::Mov {
                    dst: Operand::Reg(Reg::RAX),
                    src: Operand::Imm64(addr),
                });
            }
            _ => {
                self.ir.emit(ADeadOp::Xor { dst: Reg::RAX, src: Reg::RAX });
            }
        }
    }

    fn emit_call(&mut self, name: &str, args: &[Expr]) {
        for (i, arg) in args.iter().enumerate().take(4) {
            self.emit_expression(arg);
            let dst = match i {
                0 => Reg::RCX,
                1 => Reg::RDX,
                2 => Reg::R8,
                3 => Reg::R9,
                _ => unreachable!(),
            };
            self.ir.emit(ADeadOp::Mov {
                dst: Operand::Reg(dst),
                src: Operand::Reg(Reg::RAX),
            });
        }

        // Shadow space
        self.ir.emit(ADeadOp::Sub {
            dst: Operand::Reg(Reg::RSP),
            src: Operand::Imm8(32),
        });

        // Call usando label de la función
        if let Some(func) = self.functions.get(name) {
            let label = func.label;
            self.ir.emit(ADeadOp::Call { target: CallTarget::Relative(label) });
        } else {
            self.ir.emit(ADeadOp::Call { target: CallTarget::Name(name.to_string()) });
        }

        // Restaurar stack
        self.ir.emit(ADeadOp::Add {
            dst: Operand::Reg(Reg::RSP),
            src: Operand::Imm8(32),
        });
    }

    fn emit_input(&mut self) {
        let input_offset = self.stack_offset;
        self.stack_offset -= 8;

        self.ir.emit(ADeadOp::Xor { dst: Reg::RAX, src: Reg::RAX });
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Mem { base: Reg::RBP, disp: input_offset },
            src: Operand::Reg(Reg::RAX),
        });

        let fmt_addr = self.get_string_address("%d");
        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::RCX),
            src: Operand::Imm64(fmt_addr),
        });
        self.ir.emit(ADeadOp::Lea {
            dst: Reg::RDX,
            src: Operand::Mem { base: Reg::RBP, disp: input_offset },
        });

        // call scanf via IAT (scanf @ 0x2048)
        self.ir.emit(ADeadOp::Sub {
            dst: Operand::Reg(Reg::RSP),
            src: Operand::Imm8(32),
        });
        self.ir.emit(ADeadOp::CallIAT { iat_rva: 0x2048 });
        self.ir.emit(ADeadOp::Add {
            dst: Operand::Reg(Reg::RSP),
            src: Operand::Imm8(32),
        });

        self.ir.emit(ADeadOp::Mov {
            dst: Operand::Reg(Reg::RAX),
            src: Operand::Mem { base: Reg::RBP, disp: input_offset },
        });
    }

    /// Retorna referencia a la IR generada (para debug/inspección).
    pub fn ir(&self) -> &ADeadIR {
        &self.ir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::parser::Parser;

    #[test]
    fn test_hello_world_compiles() {
        let source = r#"
fn main() {
    println("Hello, ADead-BIB!")
}
"#;
        let program = Parser::parse_program(source).unwrap();
        let mut compiler = IsaCompiler::new(Target::Windows);
        let (code, data) = compiler.compile(&program);
        assert!(!code.is_empty(), "Code should not be empty");
        assert!(!data.is_empty(), "Data should contain strings");
    }

    #[test]
    fn test_variables_compile() {
        let source = r#"
fn main() {
    let x = 42
    let y = 10
    let result = x + y
    println(result)
}
"#;
        let program = Parser::parse_program(source).unwrap();
        let mut compiler = IsaCompiler::new(Target::Windows);
        let (code, _data) = compiler.compile(&program);
        assert!(!code.is_empty());
    }

    #[test]
    fn test_ir_is_readable() {
        let source = r#"
fn main() {
    println("Hello!")
}
"#;
        let program = Parser::parse_program(source).unwrap();
        let mut compiler = IsaCompiler::new(Target::Windows);
        let _ = compiler.compile(&program);

        // Verify we can print every instruction
        for op in compiler.ir().ops() {
            let s = format!("{}", op);
            assert!(!s.is_empty());
        }
    }
}
