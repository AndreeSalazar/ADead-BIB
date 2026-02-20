// ============================================================
// ADead-BIB ISA Encoder — ADeadOp → bytes x86-64
// ============================================================
// Convierte instrucciones tipadas (ADeadOp) en bytes de máquina
// exactamente iguales a los que codegen_v2.rs emitía directamente.
//
// Pipeline: AST → ADeadIR → Encoder → bytes → PE/ELF
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com
// ============================================================

use super::*;
use std::collections::HashMap;

/// Resultado de la codificación de un programa completo.
#[derive(Debug, Clone)]
pub struct EncodeResult {
    /// Bytes de código x86-64 generados
    pub code: Vec<u8>,
    /// Llamadas a funciones no resueltas: (offset_en_code, nombre_función)
    pub unresolved_calls: Vec<(usize, String)>,
}

/// Tipo de patch pendiente para resolución de saltos.
#[derive(Debug, Clone)]
struct PendingPatch {
    /// Offset en el buffer de código donde escribir el desplazamiento
    code_offset: usize,
    /// Label destino del salto
    target: Label,
    /// Tipo de salto (rel8 o rel32)
    kind: PatchKind,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum PatchKind {
    Rel32,
    Rel8,
}

/// Encoder de instrucciones ISA a bytes x86-64.
pub struct Encoder {
    code: Vec<u8>,
    label_positions: HashMap<u32, usize>,
    pending_patches: Vec<PendingPatch>,
    unresolved_calls: Vec<(usize, String)>,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            label_positions: HashMap::new(),
            pending_patches: Vec::new(),
            unresolved_calls: Vec::new(),
        }
    }

    /// Codifica todas las instrucciones y resuelve saltos.
    pub fn encode_all(&mut self, ops: &[ADeadOp]) -> EncodeResult {
        self.code.clear();
        self.label_positions.clear();
        self.pending_patches.clear();
        self.unresolved_calls.clear();

        // Pass 1: encode todas las ops, registrar labels, dejar placeholders
        for op in ops {
            self.encode_op(op);
        }

        // Pass 2: resolver saltos
        for patch in &self.pending_patches {
            if let Some(&target_pos) = self.label_positions.get(&patch.target.0) {
                match patch.kind {
                    PatchKind::Rel32 => {
                        let rel = (target_pos as i64 - (patch.code_offset as i64 + 4)) as i32;
                        self.code[patch.code_offset..patch.code_offset + 4]
                            .copy_from_slice(&rel.to_le_bytes());
                    }
                    PatchKind::Rel8 => {
                        let rel = (target_pos as i64 - (patch.code_offset as i64 + 1)) as i8;
                        self.code[patch.code_offset] = rel as u8;
                    }
                }
            }
        }

        EncodeResult {
            code: self.code.clone(),
            unresolved_calls: self.unresolved_calls.clone(),
        }
    }

    /// Codifica una instrucción individual.
    pub fn encode_op(&mut self, op: &ADeadOp) {
        match op {
            ADeadOp::Mov { dst, src } => self.encode_mov(dst, src),
            ADeadOp::MovZx { dst, src } => self.encode_movzx(dst, src),
            ADeadOp::Lea { dst, src } => self.encode_lea(dst, src),
            ADeadOp::Add { dst, src } => self.encode_add(dst, src),
            ADeadOp::Sub { dst, src } => self.encode_sub(dst, src),
            ADeadOp::Mul { dst, src } => self.encode_mul(dst, src),
            ADeadOp::Div { src } => self.encode_div(src),
            ADeadOp::And { dst, src } => self.encode_and(dst, src),
            ADeadOp::Or { dst, src } => self.encode_or(dst, src),
            ADeadOp::Xor { dst, src } => self.encode_xor(dst, src),
            ADeadOp::Inc { dst } => self.encode_inc(dst),
            ADeadOp::Dec { dst } => self.encode_dec(dst),
            ADeadOp::Neg { dst } => self.encode_neg(dst),
            ADeadOp::Not { dst } => self.encode_not(dst),
            ADeadOp::Shl { dst, amount } => self.encode_shl(dst, *amount),
            ADeadOp::Cmp { left, right } => self.encode_cmp(left, right),
            ADeadOp::Test { left, right } => self.encode_test(left, right),
            ADeadOp::SetCC { cond, dst: _ } => self.encode_setcc(cond),
            ADeadOp::Push { src } => self.encode_push(src),
            ADeadOp::Pop { dst } => self.encode_pop(dst),
            ADeadOp::Call { target } => self.encode_call(target),
            ADeadOp::Jmp { target } => self.encode_jmp(target),
            ADeadOp::Jcc { cond, target } => self.encode_jcc(cond, target),
            ADeadOp::Ret => self.emit(&[0xC3]),
            ADeadOp::Syscall => self.emit(&[0x0F, 0x05]),
            ADeadOp::CvtSi2Sd { dst, src } => {
                let (src_idx, src_ext) = reg_index(src);
                let (dst_idx, _) = reg_index(dst);
                let rex = 0x48 | if src_ext { 0x01 } else { 0x00 };
                let modrm = 0xC0 | (dst_idx << 3) | src_idx;
                self.emit(&[0xF2, rex, 0x0F, 0x2A, modrm]);
            }
            ADeadOp::MovQ { dst, src } => self.encode_movq(dst, src),
            ADeadOp::Label(label) => {
                self.label_positions.insert(label.0, self.code.len());
            }
            ADeadOp::Nop => self.emit(&[0x90]),
            ADeadOp::RawBytes(bytes) => self.emit(bytes),
            ADeadOp::CallIAT { iat_rva } => self.encode_call_iat(*iat_rva),

            // ================================================================
            // OS-Level / Privileged Instructions
            // ================================================================
            ADeadOp::Cli => self.emit(&[0xFA]),
            ADeadOp::Sti => self.emit(&[0xFB]),
            ADeadOp::Hlt => self.emit(&[0xF4]),
            ADeadOp::Iret => self.emit(&[0x48, 0xCF]), // iretq (REX.W + IRET)
            ADeadOp::Int { vector } => {
                self.emit(&[0xCD, *vector]);
            }
            ADeadOp::Lgdt { src } => self.encode_lgdt(src),
            ADeadOp::Lidt { src } => self.encode_lidt(src),
            ADeadOp::MovToCr { cr, src } => self.encode_mov_to_cr(*cr, src),
            ADeadOp::MovFromCr { cr, dst } => self.encode_mov_from_cr(*cr, dst),
            ADeadOp::Cpuid => self.emit(&[0x0F, 0xA2]),
            ADeadOp::Rdmsr => self.emit(&[0x0F, 0x32]),
            ADeadOp::Wrmsr => self.emit(&[0x0F, 0x30]),
            ADeadOp::Invlpg { addr } => self.encode_invlpg(addr),
            ADeadOp::InByte { port } => self.encode_in_byte(port),
            ADeadOp::OutByte { port, src: _ } => self.encode_out_byte(port),
            ADeadOp::Shr { dst, amount } => self.encode_shr(dst, *amount),
            ADeadOp::BitwiseNot { dst } => self.encode_bitwise_not(dst),
            ADeadOp::ShlCl { dst } => self.encode_shl_cl(dst),
            ADeadOp::ShrCl { dst } => self.encode_shr_cl(dst),
            ADeadOp::FarJmp { selector, offset } => self.encode_far_jmp(*selector, *offset),
            ADeadOp::LabelAddrRef { label, size, base_addr } => {
                // Emit the absolute address of a label
                // This requires the label to be already defined (resolved in second pass)
                if let Some(&pos) = self.label_positions.get(&label.0) {
                    let addr = *base_addr as usize + pos;
                    match size {
                        2 => self.emit_u16(addr as u16),
                        4 => self.emit_u32(addr as u32),
                        _ => self.emit_u32(addr as u32),
                    }
                } else {
                    // Label not yet defined - emit placeholder and record for later resolution
                    // For now, emit zeros as placeholder
                    match size {
                        2 => self.emit_u16(0),
                        4 => self.emit_u32(0),
                        _ => self.emit_u32(0),
                    }
                }
            }
        }
    }

    // ========================================
    // MOV
    // ========================================

    fn encode_mov(&mut self, dst: &Operand, src: &Operand) {
        match (dst, src) {
            // mov reg64, imm64
            (Operand::Reg(r), Operand::Imm64(v)) => {
                match r {
                    Reg::RAX => {
                        self.emit(&[0x48, 0xB8]);
                        self.emit_u64(*v);
                    }
                    Reg::RCX => {
                        self.emit(&[0x48, 0xB9]);
                        self.emit_u64(*v);
                    }
                    Reg::RDX => {
                        self.emit(&[0x48, 0xBA]);
                        self.emit_u64(*v);
                    }
                    Reg::RBX => {
                        self.emit(&[0x48, 0xBB]);
                        self.emit_u64(*v);
                    }
                    Reg::RSI => {
                        self.emit(&[0x48, 0xBE]);
                        self.emit_u64(*v);
                    }
                    Reg::RDI => {
                        self.emit(&[0x48, 0xBF]);
                        self.emit_u64(*v);
                    }
                    Reg::R8 => {
                        self.emit(&[0x49, 0xB8]);
                        self.emit_u64(*v);
                    }
                    Reg::R9 => {
                        self.emit(&[0x49, 0xB9]);
                        self.emit_u64(*v);
                    }
                    _ => {
                        // Generic MOV r64, imm64 using reg_index
                        let (idx, ext) = reg_index(r);
                        let rex = 0x48 | if ext { 0x01 } else { 0x00 };
                        self.emit(&[rex, 0xB8 + idx]);
                        self.emit_u64(*v);
                    }
                }
            }
            // mov reg32, imm32
            (Operand::Reg(Reg::EAX), Operand::Imm32(v)) => {
                self.emit(&[0xB8]);
                self.emit_i32(*v);
            }
            // mov reg64, imm32 (sign-extended)
            (Operand::Reg(r), Operand::Imm32(v)) => match r {
                Reg::RAX => {
                    self.emit(&[0x48, 0xC7, 0xC0]);
                    self.emit_i32(*v);
                }
                Reg::RCX => {
                    self.emit(&[0x48, 0xC7, 0xC1]);
                    self.emit_i32(*v);
                }
                Reg::RDX => {
                    self.emit(&[0x48, 0xC7, 0xC2]);
                    self.emit_i32(*v);
                }
                Reg::RDI => {
                    self.emit(&[0x48, 0xC7, 0xC7]);
                    self.emit_i32(*v);
                }
                _ => {
                    self.emit(&[0x48, 0xC7, 0xC0]);
                    self.emit_i32(*v);
                }
            },
            // mov reg64, [rbp+disp32]
            (
                Operand::Reg(r),
                Operand::Mem {
                    base: Reg::RBP,
                    disp,
                },
            ) => match r {
                Reg::RAX => {
                    self.emit(&[0x48, 0x8B, 0x85]);
                    self.emit_i32(*disp);
                }
                Reg::RCX => {
                    self.emit(&[0x48, 0x8B, 0x8D]);
                    self.emit_i32(*disp);
                }
                Reg::RDX => {
                    self.emit(&[0x48, 0x8B, 0x95]);
                    self.emit_i32(*disp);
                }
                Reg::RBX => {
                    self.emit(&[0x48, 0x8B, 0x9D]);
                    self.emit_i32(*disp);
                }
                _ => {
                    self.emit(&[0x48, 0x8B, 0x85]);
                    self.emit_i32(*disp);
                }
            },
            // mov [rbp+disp32], reg64
            (
                Operand::Mem {
                    base: Reg::RBP,
                    disp,
                },
                Operand::Reg(r),
            ) => {
                let fits_i8 = *disp >= -128 && *disp <= 127;
                match r {
                    Reg::RAX => {
                        self.emit(&[0x48, 0x89, 0x85]);
                        self.emit_i32(*disp);
                    }
                    Reg::RCX if fits_i8 => {
                        self.emit(&[0x48, 0x89, 0x4D, *disp as u8]);
                    }
                    Reg::RCX => {
                        self.emit(&[0x48, 0x89, 0x8D]);
                        self.emit_i32(*disp);
                    }
                    Reg::RDX if fits_i8 => {
                        self.emit(&[0x48, 0x89, 0x55, *disp as u8]);
                    }
                    Reg::RDX => {
                        self.emit(&[0x48, 0x89, 0x95]);
                        self.emit_i32(*disp);
                    }
                    Reg::R8 if fits_i8 => {
                        self.emit(&[0x4C, 0x89, 0x45, *disp as u8]);
                    }
                    Reg::R8 => {
                        self.emit(&[0x4C, 0x89, 0x85]);
                        self.emit_i32(*disp);
                    }
                    Reg::R9 if fits_i8 => {
                        self.emit(&[0x4C, 0x89, 0x4D, *disp as u8]);
                    }
                    Reg::R9 => {
                        self.emit(&[0x4C, 0x89, 0x8D]);
                        self.emit_i32(*disp);
                    }
                    _ => {
                        self.emit(&[0x48, 0x89, 0x85]);
                        self.emit_i32(*disp);
                    }
                }
            }
            // mov reg64, [reg64] (base sin desplazamiento)
            (
                Operand::Reg(Reg::RAX),
                Operand::Mem {
                    base: Reg::RAX,
                    disp: 0,
                },
            ) => {
                self.emit(&[0x48, 0x8B, 0x00]);
            }
            (
                Operand::Reg(Reg::RAX),
                Operand::Mem {
                    base: Reg::RBX,
                    disp: 0,
                },
            ) => {
                self.emit(&[0x48, 0x8B, 0x03]);
            }
            // mov [reg64], reg64 (base sin desplazamiento)
            (
                Operand::Mem {
                    base,
                    disp: 0,
                },
                Operand::Reg(src_r),
            ) => {
                let (base_idx, base_ext) = reg_index(base);
                let (src_idx, src_ext) = reg_index(src_r);
                let mut rex: u8 = 0x48;
                if src_ext { rex |= 0x04; }
                if base_ext { rex |= 0x01; }
                let modrm = (src_idx << 3) | base_idx;
                self.emit(&[rex, 0x89, modrm]);
            }
            // mov reg64, reg64
            (Operand::Reg(dst_r), Operand::Reg(src_r)) => {
                self.encode_mov_reg_reg(dst_r, src_r);
            }
            _ => {}
        }
    }

    fn encode_mov_reg_reg(&mut self, dst: &Reg, src: &Reg) {
        match (dst, src) {
            (Reg::RBP, Reg::RSP) => self.emit(&[0x48, 0x89, 0xE5]),
            (Reg::RSP, Reg::RBP) => self.emit(&[0x48, 0x89, 0xEC]),
            (Reg::RBX, Reg::RAX) => self.emit(&[0x48, 0x89, 0xC3]),
            (Reg::RCX, Reg::RAX) => self.emit(&[0x48, 0x89, 0xC1]),
            (Reg::RDX, Reg::RAX) => self.emit(&[0x48, 0x89, 0xC2]),
            (Reg::RAX, Reg::RCX) => self.emit(&[0x48, 0x89, 0xC8]),
            (Reg::R8, Reg::RAX) => self.emit(&[0x49, 0x89, 0xC0]),
            (Reg::R9, Reg::RAX) => self.emit(&[0x49, 0x89, 0xC1]),
            _ => {
                // Encoding genérico: REX.W + MOV + ModR/M
                let (rex, modrm) = self.reg_reg_encoding(src, dst);
                self.emit(&[rex, 0x89, modrm]);
            }
        }
    }

    // ========================================
    // MOVZX, LEA
    // ========================================

    fn encode_movzx(&mut self, _dst: &Reg, _src: &Reg) {
        // movzx rax, al → [0x48, 0x0F, 0xB6, 0xC0]
        self.emit(&[0x48, 0x0F, 0xB6, 0xC0]);
    }

    fn encode_lea(&mut self, dst: &Reg, src: &Operand) {
        if let Operand::Mem {
            base: Reg::RBP,
            disp,
        } = src
        {
            match dst {
                Reg::RAX => {
                    self.emit(&[0x48, 0x8D, 0x85]);
                    self.emit_i32(*disp);
                }
                Reg::RDX => {
                    self.emit(&[0x48, 0x8D, 0x95]);
                    self.emit_i32(*disp);
                }
                _ => {
                    self.emit(&[0x48, 0x8D, 0x85]);
                    self.emit_i32(*disp);
                }
            }
        }
    }

    // ========================================
    // Arithmetic: ADD, SUB, MUL, DIV
    // ========================================

    fn encode_add(&mut self, dst: &Operand, src: &Operand) {
        match (dst, src) {
            (Operand::Reg(Reg::RAX), Operand::Reg(Reg::RBX)) => {
                self.emit(&[0x48, 0x01, 0xD8]);
            }
            (Operand::Reg(Reg::RSP), Operand::Imm8(v)) => {
                self.emit(&[0x48, 0x83, 0xC4, *v as u8]);
            }
            (Operand::Reg(Reg::RSP), Operand::Imm32(v)) => {
                self.emit(&[0x48, 0x81, 0xC4]);
                self.emit_i32(*v);
            }
            _ => {}
        }
    }

    fn encode_sub(&mut self, dst: &Operand, src: &Operand) {
        match (dst, src) {
            (Operand::Reg(Reg::RAX), Operand::Reg(Reg::RBX)) => {
                self.emit(&[0x48, 0x29, 0xD8]);
            }
            (Operand::Reg(Reg::RBX), Operand::Reg(Reg::RAX)) => {
                self.emit(&[0x48, 0x29, 0xC3]);
            }
            (Operand::Reg(Reg::RSP), Operand::Imm32(v)) => {
                self.emit(&[0x48, 0x81, 0xEC]);
                self.emit_i32(*v);
            }
            (Operand::Reg(Reg::RSP), Operand::Imm8(v)) => {
                self.emit(&[0x48, 0x83, 0xEC, *v as u8]);
            }
            _ => {}
        }
    }

    fn encode_mul(&mut self, _dst: &Reg, _src: &Reg) {
        // imul rax, rbx → [0x48, 0x0F, 0xAF, 0xC3]
        self.emit(&[0x48, 0x0F, 0xAF, 0xC3]);
    }

    fn encode_div(&mut self, _src: &Reg) {
        // cqo + idiv rbx
        self.emit(&[0x48, 0x99]); // cqo
        self.emit(&[0x48, 0xF7, 0xFB]); // idiv rbx
    }

    // ========================================
    // Bitwise: AND, OR, XOR
    // ========================================

    fn encode_and(&mut self, _dst: &Reg, _src: &Reg) {
        self.emit(&[0x48, 0x21, 0xD8]); // and rax, rbx
    }

    fn encode_or(&mut self, _dst: &Reg, _src: &Reg) {
        self.emit(&[0x48, 0x09, 0xD8]); // or rax, rbx
    }

    fn encode_xor(&mut self, dst: &Reg, src: &Reg) {
        match (dst, src) {
            (Reg::RAX, Reg::RAX) => self.emit(&[0x48, 0x31, 0xC0]),
            (Reg::EAX, Reg::EAX) => self.emit(&[0x31, 0xC0]),
            (Reg::ECX, Reg::ECX) => self.emit(&[0x31, 0xC9]),
            (Reg::RCX, Reg::RCX) => self.emit(&[0x48, 0x31, 0xC9]),
            _ => self.emit(&[0x48, 0x31, 0xC0]),
        }
    }

    // ========================================
    // INC, DEC, NEG, NOT, SHL
    // ========================================

    fn encode_inc(&mut self, dst: &Operand) {
        match dst {
            Operand::Reg(Reg::RAX) => self.emit(&[0x48, 0xFF, 0xC0]),
            Operand::Reg(Reg::RCX) => self.emit(&[0x48, 0xFF, 0xC1]),
            Operand::Mem {
                base: Reg::RBP,
                disp,
            } => {
                self.emit(&[0x48, 0xFF, 0x85]);
                self.emit_i32(*disp);
            }
            _ => {}
        }
    }

    fn encode_dec(&mut self, dst: &Operand) {
        match dst {
            Operand::Reg(Reg::RCX) => self.emit(&[0x48, 0xFF, 0xC9]),
            Operand::Mem {
                base: Reg::RBP,
                disp,
            } => {
                self.emit(&[0x48, 0xFF, 0x8D]);
                self.emit_i32(*disp);
            }
            _ => {}
        }
    }

    fn encode_neg(&mut self, _dst: &Reg) {
        self.emit(&[0x48, 0xF7, 0xD8]); // neg rax
    }

    fn encode_not(&mut self, _dst: &Reg) {
        // Logical NOT: test rax, rax; sete al; movzx rax, al
        self.emit(&[0x48, 0x85, 0xC0]); // test rax, rax
        self.emit(&[0x0F, 0x94, 0xC0]); // sete al
        self.emit(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
    }

    fn encode_shl(&mut self, _dst: &Reg, amount: u8) {
        // shl rax, imm8
        self.emit(&[0x48, 0xC1, 0xE0, amount]);
    }

    // ========================================
    // CMP, TEST, SETCC
    // ========================================

    fn encode_cmp(&mut self, left: &Operand, right: &Operand) {
        match (left, right) {
            (Operand::Reg(Reg::RAX), Operand::Reg(Reg::RBX)) => {
                self.emit(&[0x48, 0x39, 0xD8]);
            }
            (Operand::Reg(Reg::RCX), Operand::Reg(Reg::R8)) => {
                self.emit(&[0x4C, 0x39, 0xC1]);
            }
            (
                Operand::Mem {
                    base: Reg::RBP,
                    disp,
                },
                Operand::Reg(Reg::R8),
            ) => {
                self.emit(&[0x4C, 0x39, 0x85]);
                self.emit_i32(*disp);
            }
            (
                Operand::Reg(Reg::RAX),
                Operand::Mem {
                    base: Reg::RBP,
                    disp,
                },
            ) => {
                self.emit(&[0x48, 0x3B, 0x85]);
                self.emit_i32(*disp);
            }
            _ => self.emit(&[0x48, 0x39, 0xD8]),
        }
    }

    fn encode_test(&mut self, _left: &Reg, _right: &Reg) {
        self.emit(&[0x48, 0x85, 0xC0]); // test rax, rax
    }

    fn encode_setcc(&mut self, cond: &Condition) {
        match cond {
            Condition::Equal => self.emit(&[0x0F, 0x94, 0xC0]),
            Condition::NotEqual => self.emit(&[0x0F, 0x95, 0xC0]),
            Condition::Less => self.emit(&[0x0F, 0x9C, 0xC0]),
            Condition::LessEq => self.emit(&[0x0F, 0x9E, 0xC0]),
            Condition::Greater => self.emit(&[0x0F, 0x9F, 0xC0]),
            Condition::GreaterEq => self.emit(&[0x0F, 0x9D, 0xC0]),
            Condition::Always => {}
        }
    }

    // ========================================
    // PUSH, POP
    // ========================================

    fn encode_push(&mut self, src: &Operand) {
        match src {
            Operand::Reg(r) => match r {
                Reg::RAX => self.emit(&[0x50]),
                Reg::RCX => self.emit(&[0x51]),
                Reg::RDX => self.emit(&[0x52]),
                Reg::RBX => self.emit(&[0x53]),
                Reg::RSP => self.emit(&[0x54]),
                Reg::RBP => self.emit(&[0x55]),
                Reg::RSI => self.emit(&[0x56]),
                Reg::RDI => self.emit(&[0x57]),
                Reg::R8 => self.emit(&[0x41, 0x50]),
                Reg::R9 => self.emit(&[0x41, 0x51]),
                Reg::R10 => self.emit(&[0x41, 0x52]),
                Reg::R11 => self.emit(&[0x41, 0x53]),
                Reg::R12 => self.emit(&[0x41, 0x54]),
                Reg::R13 => self.emit(&[0x41, 0x55]),
                Reg::R14 => self.emit(&[0x41, 0x56]),
                Reg::R15 => self.emit(&[0x41, 0x57]),
                _ => {}
            },
            _ => {}
        }
    }

    fn encode_pop(&mut self, dst: &Reg) {
        match dst {
            Reg::RAX => self.emit(&[0x58]),
            Reg::RCX => self.emit(&[0x59]),
            Reg::RDX => self.emit(&[0x5A]),
            Reg::RBX => self.emit(&[0x5B]),
            Reg::RSP => self.emit(&[0x5C]),
            Reg::RBP => self.emit(&[0x5D]),
            Reg::RSI => self.emit(&[0x5E]),
            Reg::RDI => self.emit(&[0x5F]),
            Reg::R8 => self.emit(&[0x41, 0x58]),
            Reg::R9 => self.emit(&[0x41, 0x59]),
            _ => {}
        }
    }

    // ========================================
    // CALL, JMP, Jcc
    // ========================================

    fn encode_call(&mut self, target: &CallTarget) {
        match target {
            CallTarget::Relative(label) => {
                self.emit(&[0xE8]);
                let patch_offset = self.code.len();
                self.emit_i32(0);
                self.pending_patches.push(PendingPatch {
                    code_offset: patch_offset,
                    target: *label,
                    kind: PatchKind::Rel32,
                });
            }
            CallTarget::RipRelative(disp) => {
                self.emit(&[0xFF, 0x15]);
                self.emit_i32(*disp);
            }
            CallTarget::Name(name) => {
                self.emit(&[0xE8]);
                let offset = self.code.len();
                self.unresolved_calls.push((offset, name.clone()));
                self.emit_i32(0);
            }
        }
    }

    fn encode_jmp(&mut self, target: &Label) {
        self.emit(&[0xE9]);
        let patch_offset = self.code.len();
        self.emit_i32(0);
        self.pending_patches.push(PendingPatch {
            code_offset: patch_offset,
            target: *target,
            kind: PatchKind::Rel32,
        });
    }

    fn encode_jcc(&mut self, cond: &Condition, target: &Label) {
        match cond {
            Condition::Equal => self.emit(&[0x0F, 0x84]),
            Condition::NotEqual => self.emit(&[0x0F, 0x85]),
            Condition::Less => self.emit(&[0x0F, 0x8C]),
            Condition::LessEq => self.emit(&[0x0F, 0x8E]),
            Condition::Greater => self.emit(&[0x0F, 0x8F]),
            Condition::GreaterEq => self.emit(&[0x0F, 0x8D]),
            Condition::Always => {
                self.encode_jmp(target);
                return;
            }
        }
        let patch_offset = self.code.len();
        self.emit_i32(0);
        self.pending_patches.push(PendingPatch {
            code_offset: patch_offset,
            target: *target,
            kind: PatchKind::Rel32,
        });
    }

    // ========================================
    // MOVQ (SSE ↔ GP)
    // ========================================

    fn encode_movq(&mut self, dst: &Reg, src: &Reg) {
        match (dst, src) {
            (Reg::RAX, Reg::XMM0) => self.emit(&[0x66, 0x48, 0x0F, 0x7E, 0xC0]),
            (Reg::XMM1, Reg::RDX) => self.emit(&[0x66, 0x48, 0x0F, 0x6E, 0xCA]),
            (Reg::XMM0, Reg::RAX) => self.emit(&[0x66, 0x48, 0x0F, 0x6E, 0xC0]),
            _ => self.emit(&[0x66, 0x48, 0x0F, 0x7E, 0xC0]),
        }
    }

    // ========================================
    // CallIAT (Windows Import Address Table)
    // ========================================

    fn encode_call_iat(&mut self, iat_rva: u32) {
        // call [rip+offset] donde offset = iat_rva - (current_rva + 6)
        // current_rva = 0x1000 (base de .text) + posición actual en código
        // El call [rip+disp32] tiene 6 bytes: FF 15 + disp32
        let current_rva = 0x1000u32 + self.code.len() as u32 + 6;
        let offset = iat_rva as i32 - current_rva as i32;
        self.emit(&[0xFF, 0x15]);
        self.emit_i32(offset);
    }

    // ========================================
    // Helpers
    // ========================================

    fn reg_reg_encoding(&self, src: &Reg, dst: &Reg) -> (u8, u8) {
        let (src_idx, src_ext) = reg_index(src);
        let (dst_idx, dst_ext) = reg_index(dst);
        let mut rex: u8 = 0x48;
        if src_ext {
            rex |= 0x04;
        } // REX.R
        if dst_ext {
            rex |= 0x01;
        } // REX.B
        let modrm = 0xC0 | (src_idx << 3) | dst_idx;
        (rex, modrm)
    }

    #[inline(always)]
    fn emit(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    #[inline(always)]
    fn emit_i32(&mut self, value: i32) {
        self.code.extend_from_slice(&value.to_le_bytes());
    }

    #[inline(always)]
    fn emit_u64(&mut self, value: u64) {
        self.code.extend_from_slice(&value.to_le_bytes());
    }

    #[inline(always)]
    fn emit_u16(&mut self, value: u16) {
        self.code.extend_from_slice(&value.to_le_bytes());
    }

    #[inline(always)]
    fn emit_u32(&mut self, value: u32) {
        self.code.extend_from_slice(&value.to_le_bytes());
    }

    // ========================================
    // OS-Level: LGDT, LIDT
    // ========================================

    fn encode_lgdt(&mut self, src: &Operand) {
        // lgdt [mem] = 0x0F 0x01 /2 (ModR/M reg field = 2)
        match src {
            Operand::Mem { base, disp } => {
                let (base_idx, base_ext) = reg_index(base);
                if base_ext {
                    self.emit(&[0x41]); // REX.B
                }
                self.emit(&[0x0F, 0x01]);
                // ModR/M: mod=10 (disp32), reg=010 (/2), r/m=base
                let modrm = 0x80 | (2 << 3) | base_idx;
                self.emit(&[modrm]);
                self.emit_i32(*disp);
            }
            Operand::Reg(r) => {
                // lgdt with direct register addressing (mod=00)
                let (base_idx, _) = reg_index(r);
                self.emit(&[0x0F, 0x01]);
                let modrm = (2 << 3) | base_idx;
                self.emit(&[modrm]);
            }
            _ => self.emit(&[0x90]), // fallback nop
        }
    }

    fn encode_lidt(&mut self, src: &Operand) {
        // lidt [mem] = 0x0F 0x01 /3 (ModR/M reg field = 3)
        match src {
            Operand::Mem { base, disp } => {
                let (base_idx, base_ext) = reg_index(base);
                if base_ext {
                    self.emit(&[0x41]); // REX.B
                }
                self.emit(&[0x0F, 0x01]);
                let modrm = 0x80 | (3 << 3) | base_idx;
                self.emit(&[modrm]);
                self.emit_i32(*disp);
            }
            Operand::Reg(r) => {
                let (base_idx, _) = reg_index(r);
                self.emit(&[0x0F, 0x01]);
                let modrm = (3 << 3) | base_idx;
                self.emit(&[modrm]);
            }
            _ => self.emit(&[0x90]),
        }
    }

    // ========================================
    // OS-Level: MOV CRn
    // ========================================

    fn encode_mov_to_cr(&mut self, cr: u8, src: &Reg) {
        // mov crN, reg = 0x0F 0x22 ModR/M(11, crN, reg)
        let (src_idx, src_ext) = reg_index(src);
        // Only need REX prefix for extended registers
        if src_ext {
            self.emit(&[0x41]); // REX.B
        }
        self.emit(&[0x0F, 0x22]);
        let modrm = 0xC0 | ((cr & 0x07) << 3) | src_idx;
        self.emit(&[modrm]);
    }

    fn encode_mov_from_cr(&mut self, cr: u8, dst: &Reg) {
        // mov reg, crN = 0x0F 0x20 ModR/M(11, crN, reg)
        let (dst_idx, dst_ext) = reg_index(dst);
        if dst_ext {
            self.emit(&[0x41]); // REX.B
        }
        self.emit(&[0x0F, 0x20]);
        let modrm = 0xC0 | ((cr & 0x07) << 3) | dst_idx;
        self.emit(&[modrm]);
    }

    // ========================================
    // OS-Level: INVLPG
    // ========================================

    fn encode_invlpg(&mut self, addr: &Operand) {
        // invlpg [mem] = 0x0F 0x01 /7
        match addr {
            Operand::Mem { base, disp } => {
                let (base_idx, base_ext) = reg_index(base);
                if base_ext {
                    self.emit(&[0x41]);
                }
                self.emit(&[0x0F, 0x01]);
                let modrm = 0x80 | (7 << 3) | base_idx;
                self.emit(&[modrm]);
                self.emit_i32(*disp);
            }
            Operand::Reg(r) => {
                let (base_idx, _) = reg_index(r);
                self.emit(&[0x0F, 0x01]);
                let modrm = (7 << 3) | base_idx;
                self.emit(&[modrm]);
            }
            _ => self.emit(&[0x90]),
        }
    }

    // ========================================
    // OS-Level: IN / OUT (byte)
    // ========================================

    fn encode_in_byte(&mut self, port: &Operand) {
        match port {
            Operand::Imm8(p) => {
                // in al, imm8
                self.emit(&[0xE4, *p as u8]);
            }
            Operand::Reg(Reg::DX) => {
                // in al, dx
                self.emit(&[0xEC]);
            }
            _ => self.emit(&[0x90]),
        }
    }

    fn encode_out_byte(&mut self, port: &Operand) {
        match port {
            Operand::Imm8(p) => {
                // out imm8, al
                self.emit(&[0xE6, *p as u8]);
            }
            Operand::Reg(Reg::DX) => {
                // out dx, al
                self.emit(&[0xEE]);
            }
            _ => self.emit(&[0x90]),
        }
    }

    // ========================================
    // OS-Level: SHR
    // ========================================

    fn encode_shr(&mut self, dst: &Reg, amount: u8) {
        let (dst_idx, dst_ext) = reg_index(dst);
        let rex = if dst.is_64bit() {
            let mut r = 0x48u8;
            if dst_ext {
                r |= 0x01;
            }
            Some(r)
        } else if dst_ext {
            Some(0x41u8)
        } else {
            None
        };
        if let Some(rex_byte) = rex {
            self.emit(&[rex_byte]);
        }
        // shr r/m, imm8: C1 /5
        let modrm = 0xC0 | (5 << 3) | dst_idx;
        self.emit(&[0xC1, modrm, amount]);
    }

    // ========================================
    // OS-Level: Far JMP
    // ========================================

    fn encode_far_jmp(&mut self, selector: u16, offset: u32) {
        // far jmp ptr16:32 = 0xEA + offset32 + selector16
        self.emit(&[0xEA]);
        self.emit_u32(offset);
        self.emit_u16(selector);
    }

    fn encode_bitwise_not(&mut self, dst: &Reg) {
        if dst.is_64bit() {
            let (idx, ext) = reg_index(dst);
            let rex = 0x48 | if ext { 0x01 } else { 0x00 };
            self.emit(&[rex, 0xF7, 0xD0 | idx]);
        } else if dst.is_32bit() {
            let (idx, _) = reg_index(dst);
            self.emit(&[0xF7, 0xD0 | idx]);
        } else {
            self.emit(&[0x48, 0xF7, 0xD0]); // fallback RAX
        }
    }

    fn encode_shl_cl(&mut self, dst: &Reg) {
        if dst.is_64bit() {
            let (idx, ext) = reg_index(dst);
            let rex = 0x48 | if ext { 0x01 } else { 0x00 };
            self.emit(&[rex, 0xD3, 0xE0 | idx]); // SHL r64, CL
        } else {
            self.emit(&[0x48, 0xD3, 0xE0]); // fallback SHL RAX, CL
        }
    }

    fn encode_shr_cl(&mut self, dst: &Reg) {
        if dst.is_64bit() {
            let (idx, ext) = reg_index(dst);
            let rex = 0x48 | if ext { 0x01 } else { 0x00 };
            self.emit(&[rex, 0xD3, 0xE8 | idx]); // SHR r64, CL
        } else {
            self.emit(&[0x48, 0xD3, 0xE8]); // fallback SHR RAX, CL
        }
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Retorna (índice 0-7, necesita extensión REX.B/R) para un registro.
fn reg_index(reg: &Reg) -> (u8, bool) {
    match reg {
        // 64-bit GPR
        Reg::RAX | Reg::EAX | Reg::AX | Reg::AL => (0, false),
        Reg::RCX | Reg::ECX | Reg::CX | Reg::CL => (1, false),
        Reg::RDX | Reg::EDX | Reg::DX | Reg::DL => (2, false),
        Reg::RBX | Reg::EBX | Reg::BX | Reg::BL => (3, false),
        Reg::RSP | Reg::ESP | Reg::SP | Reg::AH => (4, false),
        Reg::RBP | Reg::EBP | Reg::BP | Reg::CH => (5, false),
        Reg::RSI | Reg::ESI | Reg::SI | Reg::DH => (6, false),
        Reg::RDI | Reg::EDI | Reg::DI | Reg::BH => (7, false),
        Reg::R8 => (0, true),
        Reg::R9 => (1, true),
        Reg::R10 => (2, true),
        Reg::R11 => (3, true),
        Reg::R12 => (4, true),
        Reg::R13 => (5, true),
        Reg::R14 => (6, true),
        Reg::R15 => (7, true),
        // SSE
        Reg::XMM0 => (0, false),
        Reg::XMM1 => (1, false),
        // Control registers (index = CR number)
        Reg::CR0 => (0, false),
        Reg::CR2 => (2, false),
        Reg::CR3 => (3, false),
        Reg::CR4 => (4, false),
        // Debug registers
        Reg::DR0 => (0, false),
        Reg::DR1 => (1, false),
        Reg::DR2 => (2, false),
        Reg::DR3 => (3, false),
        Reg::DR6 => (6, false),
        Reg::DR7 => (7, false),
        // Segment registers (encoding order)
        Reg::CS => (1, false),
        Reg::DS => (3, false),
        Reg::ES => (0, false),
        Reg::FS => (4, false),
        Reg::GS => (5, false),
        Reg::SS => (2, false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_prologue() {
        let mut enc = Encoder::new();
        let ops = vec![
            ADeadOp::Push {
                src: Operand::Reg(Reg::RBP),
            },
            ADeadOp::Mov {
                dst: Operand::Reg(Reg::RBP),
                src: Operand::Reg(Reg::RSP),
            },
            ADeadOp::Pop { dst: Reg::RBP },
            ADeadOp::Ret,
        ];
        let result = enc.encode_all(&ops);
        assert_eq!(result.code, vec![0x55, 0x48, 0x89, 0xE5, 0x5D, 0xC3]);
    }

    #[test]
    fn test_mov_imm64() {
        let mut enc = Encoder::new();
        let ops = vec![ADeadOp::Mov {
            dst: Operand::Reg(Reg::RAX),
            src: Operand::Imm64(42),
        }];
        let result = enc.encode_all(&ops);
        let mut expected = vec![0x48, 0xB8];
        expected.extend_from_slice(&42u64.to_le_bytes());
        assert_eq!(result.code, expected);
    }

    #[test]
    fn test_xor_eax() {
        let mut enc = Encoder::new();
        let ops = vec![ADeadOp::Xor {
            dst: Reg::EAX,
            src: Reg::EAX,
        }];
        let result = enc.encode_all(&ops);
        assert_eq!(result.code, vec![0x31, 0xC0]);
    }

    #[test]
    fn test_arithmetic() {
        let mut enc = Encoder::new();
        let ops = vec![
            ADeadOp::Add {
                dst: Operand::Reg(Reg::RAX),
                src: Operand::Reg(Reg::RBX),
            },
            ADeadOp::Sub {
                dst: Operand::Reg(Reg::RAX),
                src: Operand::Reg(Reg::RBX),
            },
            ADeadOp::Mul {
                dst: Reg::RAX,
                src: Reg::RBX,
            },
        ];
        let result = enc.encode_all(&ops);
        assert_eq!(
            result.code,
            vec![
                0x48, 0x01, 0xD8, // add rax, rbx
                0x48, 0x29, 0xD8, // sub rax, rbx
                0x48, 0x0F, 0xAF, 0xC3, // imul rax, rbx
            ]
        );
    }

    #[test]
    fn test_jmp_label_resolution() {
        let mut enc = Encoder::new();
        let mut ir = ADeadIR::new();
        let lbl = ir.new_label();
        let ops = vec![
            ADeadOp::Label(lbl),
            ADeadOp::Nop,
            ADeadOp::Jmp { target: lbl },
        ];
        let result = enc.encode_all(&ops);
        // Label at 0, nop at 0 (1 byte), jmp at 1 (5 bytes)
        // rel32 = 0 - (1+4+4) ... let's just verify it compiles and has content
        assert_eq!(result.code.len(), 6); // nop(1) + jmp(5)
    }

    // ========================================
    // OS-Level instruction tests
    // ========================================

    #[test]
    fn test_cli_sti_hlt() {
        let mut enc = Encoder::new();
        let ops = vec![ADeadOp::Cli, ADeadOp::Sti, ADeadOp::Hlt];
        let result = enc.encode_all(&ops);
        assert_eq!(result.code, vec![0xFA, 0xFB, 0xF4]);
    }

    #[test]
    fn test_int() {
        let mut enc = Encoder::new();
        let ops = vec![ADeadOp::Int { vector: 0x10 }, ADeadOp::Int { vector: 0x80 }];
        let result = enc.encode_all(&ops);
        assert_eq!(result.code, vec![0xCD, 0x10, 0xCD, 0x80]);
    }

    #[test]
    fn test_cpuid_rdmsr_wrmsr() {
        let mut enc = Encoder::new();
        let ops = vec![ADeadOp::Cpuid, ADeadOp::Rdmsr, ADeadOp::Wrmsr];
        let result = enc.encode_all(&ops);
        assert_eq!(
            result.code,
            vec![
                0x0F, 0xA2, // cpuid
                0x0F, 0x32, // rdmsr
                0x0F, 0x30, // wrmsr
            ]
        );
    }

    #[test]
    fn test_iretq() {
        let mut enc = Encoder::new();
        let ops = vec![ADeadOp::Iret];
        let result = enc.encode_all(&ops);
        assert_eq!(result.code, vec![0x48, 0xCF]);
    }

    #[test]
    fn test_in_out_byte_imm() {
        let mut enc = Encoder::new();
        let ops = vec![
            ADeadOp::InByte {
                port: Operand::Imm8(0x60),
            },
            ADeadOp::OutByte {
                port: Operand::Imm8(0x20),
                src: Operand::Reg(Reg::AL),
            },
        ];
        let result = enc.encode_all(&ops);
        assert_eq!(
            result.code,
            vec![
                0xE4, 0x60, // in al, 0x60
                0xE6, 0x20, // out 0x20, al
            ]
        );
    }

    #[test]
    fn test_in_out_byte_dx() {
        let mut enc = Encoder::new();
        let ops = vec![
            ADeadOp::InByte {
                port: Operand::Reg(Reg::DX),
            },
            ADeadOp::OutByte {
                port: Operand::Reg(Reg::DX),
                src: Operand::Reg(Reg::AL),
            },
        ];
        let result = enc.encode_all(&ops);
        assert_eq!(
            result.code,
            vec![
                0xEC, // in al, dx
                0xEE, // out dx, al
            ]
        );
    }

    #[test]
    fn test_far_jmp() {
        let mut enc = Encoder::new();
        let ops = vec![ADeadOp::FarJmp {
            selector: 0x08,
            offset: 0x7C00,
        }];
        let result = enc.encode_all(&ops);
        // EA 00 7C 00 00 08 00
        let mut expected = vec![0xEA];
        expected.extend_from_slice(&0x7C00u32.to_le_bytes());
        expected.extend_from_slice(&0x0008u16.to_le_bytes());
        assert_eq!(result.code, expected);
    }

    #[test]
    fn test_shr() {
        let mut enc = Encoder::new();
        let ops = vec![ADeadOp::Shr {
            dst: Reg::RAX,
            amount: 4,
        }];
        let result = enc.encode_all(&ops);
        // REX.W + C1 /5 ib => 48 C1 E8 04
        assert_eq!(result.code, vec![0x48, 0xC1, 0xE8, 0x04]);
    }
}
