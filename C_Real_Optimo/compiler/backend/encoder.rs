//! x86-64 Encoder - FASM style
//! REX prefix, ModRM, SIB, VEX for AVX2
//! Generado automáticamente
#![allow(dead_code)]

// ============== REGISTERS ==============

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg64 {
    RAX = 0, RCX = 1, RDX = 2, RBX = 3,
    RSP = 4, RBP = 5, RSI = 6, RDI = 7,
    R8 = 8, R9 = 9, R10 = 10, R11 = 11,
    R12 = 12, R13 = 13, R14 = 14, R15 = 15,
}

impl Reg64 {
    pub fn code(self) -> u8 { self as u8 & 0x7 }
    pub fn needs_rex(self) -> bool { self as u8 >= 8 }
    pub fn rex_b(self) -> u8 { if self.needs_rex() { 0x01 } else { 0 } }
    pub fn rex_r(self) -> u8 { if self.needs_rex() { 0x04 } else { 0 } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg32 {
    EAX = 0, ECX = 1, EDX = 2, EBX = 3,
    ESP = 4, EBP = 5, ESI = 6, EDI = 7,
    R8D = 8, R9D = 9, R10D = 10, R11D = 11,
    R12D = 12, R13D = 13, R14D = 14, R15D = 15,
}

impl Reg32 {
    pub fn code(self) -> u8 { self as u8 & 0x7 }
    pub fn needs_rex(self) -> bool { self as u8 >= 8 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum XmmReg {
    XMM0 = 0, XMM1 = 1, XMM2 = 2, XMM3 = 3,
    XMM4 = 4, XMM5 = 5, XMM6 = 6, XMM7 = 7,
    XMM8 = 8, XMM9 = 9, XMM10 = 10, XMM11 = 11,
    XMM12 = 12, XMM13 = 13, XMM14 = 14, XMM15 = 15,
}

impl XmmReg {
    pub fn code(self) -> u8 { self as u8 & 0x7 }
    pub fn needs_rex(self) -> bool { self as u8 >= 8 }
}

// ============== ENCODER ==============

pub struct X86Encoder {
    pub code: Vec<u8>,
}

impl X86Encoder {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }
    
    pub fn len(&self) -> usize { self.code.len() }
    pub fn is_empty(&self) -> bool { self.code.is_empty() }
    
    pub fn emit(&mut self, byte: u8) { self.code.push(byte); }
    pub fn emit_bytes(&mut self, bytes: &[u8]) { self.code.extend_from_slice(bytes); }
    pub fn emit_u32(&mut self, val: u32) { self.emit_bytes(&val.to_le_bytes()); }
    pub fn emit_u64(&mut self, val: u64) { self.emit_bytes(&val.to_le_bytes()); }
    pub fn emit_i32(&mut self, val: i32) { self.emit_bytes(&val.to_le_bytes()); }
    
    // REX prefix: 0100WRXB
    fn rex(&mut self, w: bool, r: Reg64, x: u8, b: Reg64) {
        let byte = 0x40 
            | if w { 0x08 } else { 0 }
            | r.rex_r()
            | x
            | b.rex_b();
        if byte != 0x40 || w {
            self.emit(byte);
        }
    }
    
    fn rex_w(&mut self, r: Reg64, b: Reg64) {
        self.rex(true, r, 0, b);
    }
    
    // ModRM: mod(2) reg(3) rm(3)
    fn modrm(&mut self, mod_: u8, reg: u8, rm: u8) {
        self.emit((mod_ << 6) | ((reg & 7) << 3) | (rm & 7));
    }
    
    fn modrm_rr(&mut self, dst: Reg64, src: Reg64) {
        self.modrm(0b11, dst.code(), src.code());
    }
    
    // SIB: scale(2) index(3) base(3)
    fn sib(&mut self, scale: u8, index: u8, base: u8) {
        self.emit((scale << 6) | ((index & 7) << 3) | (base & 7));
    }
    
    // ============== ARITHMETIC ==============
    
    /// ADD r64, r64
    pub fn add_rr(&mut self, dst: Reg64, src: Reg64) {
        self.rex_w(src, dst);
        self.emit(0x01);
        self.modrm_rr(src, dst);
    }
    
    /// ADD r64, imm32
    pub fn add_ri(&mut self, dst: Reg64, imm: i32) {
        self.rex_w(Reg64::RAX, dst);
        if dst == Reg64::RAX {
            self.emit(0x05);
        } else {
            self.emit(0x81);
            self.modrm(0b11, 0, dst.code());
        }
        self.emit_i32(imm);
    }
    
    /// SUB r64, r64
    pub fn sub_rr(&mut self, dst: Reg64, src: Reg64) {
        self.rex_w(src, dst);
        self.emit(0x29);
        self.modrm_rr(src, dst);
    }
    
    /// SUB r64, imm32
    pub fn sub_ri(&mut self, dst: Reg64, imm: i32) {
        self.rex_w(Reg64::RAX, dst);
        if dst == Reg64::RAX {
            self.emit(0x2D);
        } else {
            self.emit(0x81);
            self.modrm(0b11, 5, dst.code());
        }
        self.emit_i32(imm);
    }
    
    /// IMUL r64, r64
    pub fn imul_rr(&mut self, dst: Reg64, src: Reg64) {
        self.rex_w(dst, src);
        self.emit(0x0F);
        self.emit(0xAF);
        self.modrm_rr(dst, src);
    }
    
    /// IDIV r64 (RDX:RAX / r64 -> RAX, RDX)
    pub fn idiv_r(&mut self, src: Reg64) {
        self.rex_w(Reg64::RAX, src);
        self.emit(0xF7);
        self.modrm(0b11, 7, src.code());
    }
    
    /// CQO (sign extend RAX to RDX:RAX)
    pub fn cqo(&mut self) {
        self.emit(0x48);
        self.emit(0x99);
    }
    
    /// CDQ (sign extend EAX to EDX:EAX for 32-bit IDIV)
    pub fn cdq(&mut self) {
        self.emit(0x99);
    }
    
    /// MOVZX r64, r8 (zero-extend 8-bit to 64-bit)
    pub fn movzx_rr(&mut self, dst: Reg64, src: Reg64) {
        self.rex_w(dst, src);
        self.emit(0x0F);
        self.emit(0xB6);
        self.modrm(0b11, dst.code(), src.code());
    }
    
    /// MOVSX r64, r8 (sign-extend 8-bit to 64-bit)
    pub fn movsx_rr(&mut self, dst: Reg64, src: Reg64) {
        self.rex_w(dst, src);
        self.emit(0x0F);
        self.emit(0xBE);
        self.modrm(0b11, dst.code(), src.code());
    }
    
    /// NEG r64
    pub fn neg_r(&mut self, dst: Reg64) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xF7);
        self.modrm(0b11, 3, dst.code());
    }
    
    /// INC r64
    pub fn inc_r(&mut self, dst: Reg64) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xFF);
        self.modrm(0b11, 0, dst.code());
    }
    
    /// DEC r64
    pub fn dec_r(&mut self, dst: Reg64) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xFF);
        self.modrm(0b11, 1, dst.code());
    }
    
    // ============== BITWISE ==============
    
    /// AND r64, r64
    pub fn and_rr(&mut self, dst: Reg64, src: Reg64) {
        self.rex_w(src, dst);
        self.emit(0x21);
        self.modrm_rr(src, dst);
    }
    
    /// OR r64, r64
    pub fn or_rr(&mut self, dst: Reg64, src: Reg64) {
        self.rex_w(src, dst);
        self.emit(0x09);
        self.modrm_rr(src, dst);
    }
    
    /// XOR r64, r64
    pub fn xor_rr(&mut self, dst: Reg64, src: Reg64) {
        self.rex_w(src, dst);
        self.emit(0x31);
        self.modrm_rr(src, dst);
    }
    
    /// NOT r64
    pub fn not_r(&mut self, dst: Reg64) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xF7);
        self.modrm(0b11, 2, dst.code());
    }
    
    /// SHL r64, CL
    pub fn shl_rcl(&mut self, dst: Reg64) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xD3);
        self.modrm(0b11, 4, dst.code());
    }
    
    /// SHL r64, imm8
    pub fn shl_ri(&mut self, dst: Reg64, imm: u8) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xC1);
        self.modrm(0b11, 4, dst.code());
        self.emit(imm);
    }
    
    /// SHR r64, imm8
    pub fn shr_ri(&mut self, dst: Reg64, imm: u8) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xC1);
        self.modrm(0b11, 5, dst.code());
        self.emit(imm);
    }
    
    /// SAR r64, imm8
    pub fn sar_ri(&mut self, dst: Reg64, imm: u8) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xC1);
        self.modrm(0b11, 7, dst.code());
        self.emit(imm);
    }
    
    // ============== COMPARE ==============
    
    /// CMP r64, r64
    pub fn cmp_rr(&mut self, a: Reg64, b: Reg64) {
        self.rex_w(b, a);
        self.emit(0x39);
        self.modrm_rr(b, a);
    }
    
    /// CMP r64, imm32
    pub fn cmp_ri(&mut self, dst: Reg64, imm: i32) {
        self.rex_w(Reg64::RAX, dst);
        if dst == Reg64::RAX {
            self.emit(0x3D);
        } else {
            self.emit(0x81);
            self.modrm(0b11, 7, dst.code());
        }
        self.emit_i32(imm);
    }
    
    /// TEST r64, r64
    pub fn test_rr(&mut self, a: Reg64, b: Reg64) {
        self.rex_w(b, a);
        self.emit(0x85);
        self.modrm_rr(b, a);
    }
    
    /// SETE r8
    pub fn sete(&mut self, dst: Reg64) {
        if dst.needs_rex() {
            self.emit(0x40 | dst.rex_b());
        }
        self.emit(0x0F);
        self.emit(0x94);
        self.modrm(0b11, 0, dst.code());
    }
    
    /// SETNE, SETL, SETLE, SETG, SETGE
    pub fn setcc(&mut self, cc: u8, dst: Reg64) {
        if dst.needs_rex() {
            self.emit(0x40 | dst.rex_b());
        }
        self.emit(0x0F);
        self.emit(0x90 + cc);
        self.modrm(0b11, 0, dst.code());
    }
    
    // ============== MEMORY ==============
    
    /// MOV r64, r64
    pub fn mov_rr(&mut self, dst: Reg64, src: Reg64) {
        self.rex_w(src, dst);
        self.emit(0x89);
        self.modrm_rr(src, dst);
    }
    
    /// MOV r64, imm64
    pub fn mov_ri(&mut self, dst: Reg64, imm: u64) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xB8 + dst.code());
        self.emit_u64(imm);
    }
    
    /// MOV r64, imm32 (sign extended)
    pub fn mov_ri32(&mut self, dst: Reg64, imm: i32) {
        self.rex_w(Reg64::RAX, dst);
        self.emit(0xC7);
        self.modrm(0b11, 0, dst.code());
        self.emit_i32(imm);
    }
    
    /// MOV r64, [r64]
    pub fn mov_rm(&mut self, dst: Reg64, base: Reg64) {
        self.rex_w(dst, base);
        self.emit(0x8B);
        if base == Reg64::RSP || base == Reg64::R12 {
            self.modrm(0b00, dst.code(), 0b100);
            self.sib(0, 0b100, base.code());
        } else if base == Reg64::RBP || base == Reg64::R13 {
            self.modrm(0b01, dst.code(), base.code());
            self.emit(0); // disp8 = 0
        } else {
            self.modrm(0b00, dst.code(), base.code());
        }
    }
    
    /// MOV r64, [r64 + disp32]
    pub fn mov_rm_disp(&mut self, dst: Reg64, base: Reg64, disp: i32) {
        self.rex_w(dst, base);
        self.emit(0x8B);
        if base == Reg64::RSP || base == Reg64::R12 {
            self.modrm(0b10, dst.code(), 0b100);
            self.sib(0, 0b100, base.code());
        } else {
            self.modrm(0b10, dst.code(), base.code());
        }
        self.emit_i32(disp);
    }
    
    /// MOV [r64], r64
    pub fn mov_mr(&mut self, base: Reg64, src: Reg64) {
        self.rex_w(src, base);
        self.emit(0x89);
        if base == Reg64::RSP || base == Reg64::R12 {
            self.modrm(0b00, src.code(), 0b100);
            self.sib(0, 0b100, base.code());
        } else if base == Reg64::RBP || base == Reg64::R13 {
            self.modrm(0b01, src.code(), base.code());
            self.emit(0);
        } else {
            self.modrm(0b00, src.code(), base.code());
        }
    }
    
    /// MOV [r64 + disp32], r64
    pub fn mov_mr_disp(&mut self, base: Reg64, disp: i32, src: Reg64) {
        self.rex_w(src, base);
        self.emit(0x89);
        if base == Reg64::RSP || base == Reg64::R12 {
            self.modrm(0b10, src.code(), 0b100);
            self.sib(0, 0b100, base.code());
        } else {
            self.modrm(0b10, src.code(), base.code());
        }
        self.emit_i32(disp);
    }
    
    /// LEA r64, [r64 + disp32]
    pub fn lea(&mut self, dst: Reg64, base: Reg64, disp: i32) {
        self.rex_w(dst, base);
        self.emit(0x8D);
        if base == Reg64::RSP || base == Reg64::R12 {
            self.modrm(0b10, dst.code(), 0b100);
            self.sib(0, 0b100, base.code());
        } else {
            self.modrm(0b10, dst.code(), base.code());
        }
        self.emit_i32(disp);
    }
    
    /// PUSH r64
    pub fn push_r(&mut self, src: Reg64) {
        if src.needs_rex() {
            self.emit(0x40 | src.rex_b());
        }
        self.emit(0x50 + src.code());
    }
    
    /// POP r64
    pub fn pop_r(&mut self, dst: Reg64) {
        if dst.needs_rex() {
            self.emit(0x40 | dst.rex_b());
        }
        self.emit(0x58 + dst.code());
    }
    
    // ============== CONTROL FLOW ==============
    
    /// CALL rel32
    pub fn call_rel32(&mut self, offset: i32) {
        self.emit(0xE8);
        self.emit_i32(offset);
    }
    
    /// CALL r64
    pub fn call_r(&mut self, target: Reg64) {
        if target.needs_rex() {
            self.emit(0x40 | target.rex_b());
        }
        self.emit(0xFF);
        self.modrm(0b11, 2, target.code());
    }
    
    /// JMP rel32
    pub fn jmp_rel32(&mut self, offset: i32) {
        self.emit(0xE9);
        self.emit_i32(offset);
    }
    
    /// JMP rel8
    pub fn jmp_rel8(&mut self, offset: i8) {
        self.emit(0xEB);
        self.emit(offset as u8);
    }
    
    /// Jcc rel32 (JE, JNE, JL, JLE, JG, JGE, etc.)
    pub fn jcc_rel32(&mut self, cc: u8, offset: i32) {
        self.emit(0x0F);
        self.emit(0x80 + cc);
        self.emit_i32(offset);
    }
    
    /// JE rel32
    pub fn je_rel32(&mut self, offset: i32) { self.jcc_rel32(0x04, offset); }
    /// JNE rel32
    pub fn jne_rel32(&mut self, offset: i32) { self.jcc_rel32(0x05, offset); }
    /// JL rel32
    pub fn jl_rel32(&mut self, offset: i32) { self.jcc_rel32(0x0C, offset); }
    /// JLE rel32
    pub fn jle_rel32(&mut self, offset: i32) { self.jcc_rel32(0x0E, offset); }
    /// JG rel32
    pub fn jg_rel32(&mut self, offset: i32) { self.jcc_rel32(0x0F, offset); }
    /// JGE rel32
    pub fn jge_rel32(&mut self, offset: i32) { self.jcc_rel32(0x0D, offset); }
    
    /// RET
    pub fn ret(&mut self) {
        self.emit(0xC3);
    }
    
    /// NOP
    pub fn nop(&mut self) {
        self.emit(0x90);
    }
    
    /// INT3 (breakpoint)
    pub fn int3(&mut self) {
        self.emit(0xCC);
    }
    
    // ============== SSE/AVX ==============
    
    /// MOVSD xmm, xmm
    pub fn movsd_rr(&mut self, dst: XmmReg, src: XmmReg) {
        self.emit(0xF2);
        if dst.needs_rex() || src.needs_rex() {
            self.emit(0x40 | if dst.needs_rex() { 0x04 } else { 0 } | if src.needs_rex() { 0x01 } else { 0 });
        }
        self.emit(0x0F);
        self.emit(0x10);
        self.modrm(0b11, dst.code(), src.code());
    }
    
    /// ADDSD xmm, xmm
    pub fn addsd_rr(&mut self, dst: XmmReg, src: XmmReg) {
        self.emit(0xF2);
        if dst.needs_rex() || src.needs_rex() {
            self.emit(0x40 | if dst.needs_rex() { 0x04 } else { 0 } | if src.needs_rex() { 0x01 } else { 0 });
        }
        self.emit(0x0F);
        self.emit(0x58);
        self.modrm(0b11, dst.code(), src.code());
    }
    
    /// SUBSD xmm, xmm
    pub fn subsd_rr(&mut self, dst: XmmReg, src: XmmReg) {
        self.emit(0xF2);
        self.emit(0x0F);
        self.emit(0x5C);
        self.modrm(0b11, dst.code(), src.code());
    }
    
    /// MULSD xmm, xmm
    pub fn mulsd_rr(&mut self, dst: XmmReg, src: XmmReg) {
        self.emit(0xF2);
        self.emit(0x0F);
        self.emit(0x59);
        self.modrm(0b11, dst.code(), src.code());
    }
    
    /// DIVSD xmm, xmm
    pub fn divsd_rr(&mut self, dst: XmmReg, src: XmmReg) {
        self.emit(0xF2);
        self.emit(0x0F);
        self.emit(0x5E);
        self.modrm(0b11, dst.code(), src.code());
    }
    
    // ============== VEX PREFIX (AVX) ==============
    
    /// VEX 2-byte prefix
    pub fn vex2(&mut self, r: bool, vvvv: u8, l: bool, pp: u8) {
        self.emit(0xC5);
        self.emit(
            (if r { 0 } else { 0x80 }) |
            ((!vvvv & 0xF) << 3) |
            (if l { 0x04 } else { 0 }) |
            (pp & 0x03)
        );
    }
    
    /// VEX 3-byte prefix
    pub fn vex3(&mut self, r: bool, x: bool, b: bool, mmmmm: u8, w: bool, vvvv: u8, l: bool, pp: u8) {
        self.emit(0xC4);
        self.emit(
            (if r { 0 } else { 0x80 }) |
            (if x { 0 } else { 0x40 }) |
            (if b { 0 } else { 0x20 }) |
            (mmmmm & 0x1F)
        );
        self.emit(
            (if w { 0x80 } else { 0 }) |
            ((!vvvv & 0xF) << 3) |
            (if l { 0x04 } else { 0 }) |
            (pp & 0x03)
        );
    }
    
    /// VADDPD ymm, ymm, ymm (AVX)
    pub fn vaddpd_rrr(&mut self, dst: XmmReg, src1: XmmReg, src2: XmmReg) {
        self.vex2(!dst.needs_rex(), src1.code(), true, 0x01);
        self.emit(0x58);
        self.modrm(0b11, dst.code(), src2.code());
    }
}

impl Default for X86Encoder {
    fn default() -> Self {
        Self::new()
    }
}

// ============== CONDITION CODES ==============

pub mod cc {
    pub const O: u8 = 0x00;   // Overflow
    pub const NO: u8 = 0x01;  // Not overflow
    pub const B: u8 = 0x02;   // Below (unsigned <)
    pub const AE: u8 = 0x03;  // Above or equal (unsigned >=)
    pub const E: u8 = 0x04;   // Equal
    pub const NE: u8 = 0x05;  // Not equal
    pub const BE: u8 = 0x06;  // Below or equal (unsigned <=)
    pub const A: u8 = 0x07;   // Above (unsigned >)
    pub const S: u8 = 0x08;   // Sign (negative)
    pub const NS: u8 = 0x09;  // Not sign
    pub const P: u8 = 0x0A;   // Parity even
    pub const NP: u8 = 0x0B;  // Parity odd
    pub const L: u8 = 0x0C;   // Less (signed <)
    pub const GE: u8 = 0x0D;  // Greater or equal (signed >=)
    pub const LE: u8 = 0x0E;  // Less or equal (signed <=)
    pub const G: u8 = 0x0F;   // Greater (signed >)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_rr() {
        let mut enc = X86Encoder::new();
        enc.add_rr(Reg64::RAX, Reg64::RBX);
        assert_eq!(enc.code, vec![0x48, 0x01, 0xD8]);
    }
    
    #[test]
    fn test_mov_ri() {
        let mut enc = X86Encoder::new();
        enc.mov_ri(Reg64::RAX, 0x12345678);
        assert_eq!(enc.code.len(), 10);
    }
    
    #[test]
    fn test_ret() {
        let mut enc = X86Encoder::new();
        enc.ret();
        assert_eq!(enc.code, vec![0xC3]);
    }
}
