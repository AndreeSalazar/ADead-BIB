#!/usr/bin/env python3
"""Genera backend/codegen.rs - IR → x86-64 codegen"""
 
from pathlib import Path
 
BACKEND_PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\compiler\backend")
 
CODEGEN_RS = '''//! IR → x86-64 Code Generator
//! Windows x64 ABI: RCX, RDX, R8, R9, shadow space 32 bytes
//! Generado automáticamente
#![allow(dead_code)]
 
use crate::middle::ir::*;
use super::encoder::{X86Encoder, Reg64};
use std::collections::HashMap;
 
// ============== CODEGEN ==============
 
pub struct Codegen {
    pub encoder: X86Encoder,
    pub data_section: Vec<u8>,
    pub strings: Vec<(String, u32)>,  // (string, offset in data)
    reg_alloc: HashMap<u32, Reg64>,   // IR reg -> x86 reg
    stack_offset: i32,
    label_offsets: HashMap<BlockId, usize>,
    pending_jumps: Vec<(usize, BlockId)>,  // (patch offset, target block)
}
 
// Windows x64 ABI registers
const ARG_REGS: [Reg64; 4] = [Reg64::RCX, Reg64::RDX, Reg64::R8, Reg64::R9];
const CALLER_SAVED: [Reg64; 7] = [Reg64::RAX, Reg64::RCX, Reg64::RDX, Reg64::R8, Reg64::R9, Reg64::R10, Reg64::R11];
const CALLEE_SAVED: [Reg64; 5] = [Reg64::RBX, Reg64::RSI, Reg64::RDI, Reg64::R12, Reg64::R13];
const SHADOW_SPACE: i32 = 32;
 
impl Codegen {
    pub fn new() -> Self {
        Self {
            encoder: X86Encoder::new(),
            data_section: Vec::new(),
            strings: Vec::new(),
            reg_alloc: HashMap::new(),
            stack_offset: 0,
            label_offsets: HashMap::new(),
            pending_jumps: Vec::new(),
        }
    }
 
    pub fn generate(&mut self, module: &IrModule) -> Vec<u8> {
        for func in &module.functions {
            self.generate_function(func);
        }
        self.patch_jumps();
        self.encoder.code.clone()
    }
 
    fn generate_function(&mut self, func: &IrFunction) {
        self.reg_alloc.clear();
        self.label_offsets.clear();
        self.pending_jumps.clear();
 
        // Prologue
        self.emit_prologue(func);
 
        // Copy params from ABI regs to stack
        for (i, (name, _ty)) in func.params.iter().enumerate() {
            if i < ARG_REGS.len() {
                let offset = -((i as i32 + 1) * 8) - SHADOW_SPACE;
                self.encoder.mov_mr_disp(Reg64::RBP, offset, ARG_REGS[i]);
            }
        }
 
        // Generate blocks
        for block in &func.blocks {
            self.label_offsets.insert(block.id, self.encoder.len());
            for instr in &block.instrs {
                self.generate_instr(instr);
            }
        }
    }
 
    fn emit_prologue(&mut self, func: &IrFunction) {
        // push rbp
        self.encoder.push_r(Reg64::RBP);
        // mov rbp, rsp
        self.encoder.mov_rr(Reg64::RBP, Reg64::RSP);
 
        // Calculate stack space needed
        let locals = func.next_reg as i32 * 8;
        let stack_size = align16(locals + SHADOW_SPACE + 8);
        self.stack_offset = -SHADOW_SPACE - 8;
 
        if stack_size > 0 {
            // sub rsp, stack_size
            self.encoder.sub_ri(Reg64::RSP, stack_size);
        }
    }
 
    fn emit_epilogue(&mut self) {
        // mov rsp, rbp
        self.encoder.mov_rr(Reg64::RSP, Reg64::RBP);
        // pop rbp
        self.encoder.pop_r(Reg64::RBP);
        // ret
        self.encoder.ret();
    }
 
    fn generate_instr(&mut self, instr: &IrInstr) {
        match instr {
            IrInstr::Add { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.add_rr(Reg64::RAX, Reg64::RCX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Sub { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.sub_rr(Reg64::RAX, Reg64::RCX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Mul { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.imul_rr(Reg64::RAX, Reg64::RCX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Div { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.cqo();  // sign extend RAX to RDX:RAX
                self.encoder.idiv_r(Reg64::RCX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Mod { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.cqo();
                self.encoder.idiv_r(Reg64::RCX);
                self.encoder.mov_rr(r_dst, Reg64::RDX);  // remainder in RDX
            }
            IrInstr::Neg { dst, src } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, src);
                self.encoder.neg_r(Reg64::RAX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::And { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.and_rr(Reg64::RAX, Reg64::RCX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Or { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.or_rr(Reg64::RAX, Reg64::RCX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Xor { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.xor_rr(Reg64::RAX, Reg64::RCX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Shl { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.shl_rcl(Reg64::RAX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Shr { dst, lhs, rhs } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                // Use SAR for signed shift
                self.encoder.emit(0x48); self.encoder.emit(0xD3); self.encoder.emit(0xF8);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Eq { dst, lhs, rhs } => {
                self.emit_compare(*dst, lhs, rhs, 0x04); // SETE
            }
            IrInstr::Ne { dst, lhs, rhs } => {
                self.emit_compare(*dst, lhs, rhs, 0x05); // SETNE
            }
            IrInstr::Lt { dst, lhs, rhs } => {
                self.emit_compare(*dst, lhs, rhs, 0x0C); // SETL
            }
            IrInstr::Le { dst, lhs, rhs } => {
                self.emit_compare(*dst, lhs, rhs, 0x0E); // SETLE
            }
            IrInstr::Gt { dst, lhs, rhs } => {
                self.emit_compare(*dst, lhs, rhs, 0x0F); // SETG
            }
            IrInstr::Ge { dst, lhs, rhs } => {
                self.emit_compare(*dst, lhs, rhs, 0x0D); // SETGE
            }
            IrInstr::Alloca { dst, ty, count: _ } => {
                let r_dst = self.alloc_reg(*dst);
                self.stack_offset -= ty.size() as i32;
                self.stack_offset = align8(self.stack_offset);
                // lea dst, [rbp + offset]
                self.encoder.lea(r_dst, Reg64::RBP, self.stack_offset);
            }
            IrInstr::Load { dst, ptr } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, ptr);
                self.encoder.mov_rm(r_dst, Reg64::RAX);
            }
            IrInstr::Store { ptr, val } => {
                self.load_value(Reg64::RAX, ptr);
                self.load_value(Reg64::RCX, val);
                self.encoder.mov_mr(Reg64::RAX, Reg64::RCX);
            }
            IrInstr::Jmp { target } => {
                let patch_offset = self.encoder.len() + 1;
                self.encoder.jmp_rel32(0); // placeholder
                self.pending_jumps.push((patch_offset, *target));
            }
            IrInstr::JmpIf { cond, then_bb, else_bb } => {
                self.load_value(Reg64::RAX, cond);
                self.encoder.test_rr(Reg64::RAX, Reg64::RAX);
 
                let patch_then = self.encoder.len() + 2;
                self.encoder.jne_rel32(0); // if true, jump to then
                self.pending_jumps.push((patch_then, *then_bb));
 
                let patch_else = self.encoder.len() + 1;
                self.encoder.jmp_rel32(0); // else, jump to else
                self.pending_jumps.push((patch_else, *else_bb));
            }
            IrInstr::Ret { val } => {
                if let Some(v) = val {
                    self.load_value(Reg64::RAX, v);
                }
                self.emit_epilogue();
            }
            IrInstr::Call { dst, func, args } => {
                // Push args in reverse, first 4 go to regs
                for (i, arg) in args.iter().enumerate().take(4) {
                    self.load_value(ARG_REGS[i], arg);
                }
                // CALL rel32 (placeholder, needs relocation)
                self.encoder.call_rel32(0);
 
                if let Some(d) = dst {
                    let r_dst = self.alloc_reg(*d);
                    self.encoder.mov_rr(r_dst, Reg64::RAX);
                }
            }
            IrInstr::Copy { dst, src } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(r_dst, src);
            }
            IrInstr::Cast { dst, src, to_ty: _ } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(r_dst, src);
            }
            IrInstr::Not { dst, src } => {
                let r_dst = self.alloc_reg(*dst);
                self.load_value(Reg64::RAX, src);
                self.encoder.not_r(Reg64::RAX);
                self.encoder.mov_rr(r_dst, Reg64::RAX);
            }
            IrInstr::Phi { dst, incoming: _ } => {
                // PHI nodes should be lowered before codegen
                let _ = self.alloc_reg(*dst);
            }
            IrInstr::Nop => {
                self.encoder.nop();
            }
        }
    }
 
    fn emit_compare(&mut self, dst: IrReg, lhs: &IrValue, rhs: &IrValue, cc: u8) {
        let r_dst = self.alloc_reg(dst);
        self.load_value(Reg64::RAX, lhs);
        self.load_value(Reg64::RCX, rhs);
        self.encoder.cmp_rr(Reg64::RAX, Reg64::RCX);
        // XOR to clear, then SETcc
        self.encoder.xor_rr(r_dst, r_dst);
        self.encoder.setcc(cc, r_dst);
    }
 
    fn load_value(&mut self, dst: Reg64, val: &IrValue) {
        match val {
            IrValue::Const(c) => {
                let imm = match c {
                    IrConst::I8(v) => *v as i64 as u64,
                    IrConst::I16(v) => *v as i64 as u64,
                    IrConst::I32(v) => *v as i64 as u64,
                    IrConst::I64(v) => *v as u64,
                    IrConst::F32(v) => (*v as f64).to_bits(),
                    IrConst::F64(v) => v.to_bits(),
                    IrConst::Null => 0,
                };
                self.encoder.mov_ri(dst, imm);
            }
            IrValue::Reg(r) => {
                if let Some(&src) = self.reg_alloc.get(&r.id) {
                    if src != dst {
                        self.encoder.mov_rr(dst, src);
                    }
                } else {
                    // Load from stack
                    let offset = self.reg_stack_offset(r.id);
                    self.encoder.mov_rm_disp(dst, Reg64::RBP, offset);
                }
            }
            IrValue::Global(_name) => {
                // Will need relocation
                self.encoder.mov_ri(dst, 0);
            }
            IrValue::Param(idx) => {
                let offset = -((*idx as i32 + 1) * 8) - SHADOW_SPACE;
                self.encoder.mov_rm_disp(dst, Reg64::RBP, offset);
            }
        }
    }
 
    fn alloc_reg(&mut self, ir_reg: IrReg) -> Reg64 {
        // Simple: spill to stack, use RAX/RCX for temps
        // For real compiler: proper register allocation
        if let Some(&r) = self.reg_alloc.get(&ir_reg.id) {
            return r;
        }
 
        // Spill to stack
        self.stack_offset -= 8;
        let offset = self.stack_offset;
 
        // Store to stack and remember offset
        self.reg_alloc.insert(ir_reg.id, Reg64::R10); // temp marker
        Reg64::R10
    }
 
    fn reg_stack_offset(&self, id: u32) -> i32 {
        -SHADOW_SPACE - 8 - (id as i32 * 8)
    }
 
    fn patch_jumps(&mut self) {
        for (patch_offset, target_block) in &self.pending_jumps {
            if let Some(&target_offset) = self.label_offsets.get(target_block) {
                let rel = (target_offset as i32) - (*patch_offset as i32) - 4;
                let bytes = rel.to_le_bytes();
                self.encoder.code[*patch_offset] = bytes[0];
                self.encoder.code[*patch_offset + 1] = bytes[1];
                self.encoder.code[*patch_offset + 2] = bytes[2];
                self.encoder.code[*patch_offset + 3] = bytes[3];
            }
        }
    }
 
    pub fn add_string(&mut self, s: &str) -> u32 {
        let offset = self.data_section.len() as u32;
        self.data_section.extend_from_slice(s.as_bytes());
        self.data_section.push(0); // null terminator
        self.strings.push((s.to_string(), offset));
        offset
    }
}
 
impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}
 
fn align16(n: i32) -> i32 { (n + 15) & !15 }
fn align8(n: i32) -> i32 { (n + 7) & !7 }
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_codegen_basic() {
        let mut builder = IrBuilder::new("test");
        builder.begin_function("main", vec![], IrType::I32);
        builder.ret(Some(IrValue::Const(IrConst::I32(42))));
        builder.end_function();
 
        let module = builder.finish();
        let mut codegen = Codegen::new();
        let code = codegen.generate(&module);
 
        assert!(!code.is_empty());
    }
}
'''

def main():
    print("🔧 Generando backend/codegen.rs...")
    path = BACKEND_PATH / "codegen.rs"
    path.write_text(CODEGEN_RS, encoding='utf-8')
    print(f"  ✅ codegen.rs → {CODEGEN_RS.count(chr(10))} líneas")
 
if __name__ == "__main__":
    main()
 