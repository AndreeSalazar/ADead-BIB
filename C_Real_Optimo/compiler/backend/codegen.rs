//! IR → x86-64 Code Generator
//! Windows x64 ABI: RCX, RDX, R8, R9, shadow space 32 bytes
//! Generado automáticamente
#![allow(dead_code)]
 
use crate::middle::ir::*;
use super::encoder::{X86Encoder, Reg64};
use std::collections::{HashMap, HashSet};
 
// ============== CODEGEN ==============
 
pub struct Codegen {
    pub encoder: X86Encoder,
    pub data_section: Vec<u8>,
    pub strings: Vec<(String, u32)>,  // (string, offset in data)
    var_offsets: HashMap<u32, i32>, // IR reg id -> direct stack offset from RBP
    next_offset: i32,                // next available offset
    label_offsets: HashMap<BlockId, usize>,
    pending_jumps: Vec<(usize, BlockId)>,  // (patch offset, target block)
    pub func_offsets: HashMap<String, usize>, // function name → code offset
    pub pending_calls: Vec<(usize, String)>,  // (patch offset, func name)
    pub external_calls: Vec<(usize, String)>, // (patch offset, external func)
    /// B-06: nombre de global → byte offset dentro de `data_section`
    pub global_offsets: HashMap<String, u32>,
    /// B-01: ids de IrReg que provienen de Alloca (su "valor" es su dirección).
    /// Resto de regs son valores computados por ops.
    alloca_regs: HashSet<u32>,
    /// B-01: tamaño en bytes del slot reservado para cada alloca (≥ 8).
    /// Permite reservar 20+ bytes para `int arr[5]`, etc.
    alloca_sizes: HashMap<u32, i32>,
}

// Windows x64 ABI registers
const ARG_REGS: [Reg64; 4] = [Reg64::RCX, Reg64::RDX, Reg64::R8, Reg64::R9];
const CALLER_SAVED: [Reg64; 7] = [Reg64::RAX, Reg64::RCX, Reg64::RDX, Reg64::R8, Reg64::R9, Reg64::R10, Reg64::R11];
const CALLEE_SAVED: [Reg64; 5] = [Reg64::RBX, Reg64::RSI, Reg64::RDI, Reg64::R12, Reg64::R13];
const SHADOW_SPACE: i32 = 32;

// B-06: PE absolute addressing
// PE32+ ImageBase = 0x140000000, .text @ RVA 0x1000, .data @ RVA 0x2000
// (válido mientras text section ocupe < 0x1000 bytes; nuestros .exe son ~1-2 KB).
const IMAGE_BASE: u64 = 0x140000000;
const DATA_RVA:   u64 = 0x2000;
 
impl Codegen {
    pub fn new() -> Self {
        Self {
            encoder: X86Encoder::new(),
            data_section: Vec::new(),
            strings: Vec::new(),
            var_offsets: HashMap::new(),
            next_offset: -8,  // First var at [RBP-8]
            label_offsets: HashMap::new(),
            pending_jumps: Vec::new(),
            func_offsets: HashMap::new(),
            pending_calls: Vec::new(),
            external_calls: Vec::new(),
            global_offsets: HashMap::new(),
            alloca_regs: HashSet::new(),
            alloca_sizes: HashMap::new(),
        }
    }
 
    pub fn generate(&mut self, module: &IrModule) -> Vec<u8> {
        // B-06 [pass 1]: layout de globals en data_section.
        // Cada global ocupa max(ty.size(), 8) bytes (alineamiento natural).
        for (name, ty, init) in &module.globals {
            let off = self.data_section.len() as u32;
            let size = ty.size().max(1);
            // valor inicial: low bytes en little-endian, padding a 8 con ceros
            let imm: u64 = init.map(|v| v as u64).unwrap_or(0);
            let bytes = imm.to_le_bytes();
            let n = size.min(8);
            self.data_section.extend_from_slice(&bytes[..n]);
            // alinear a 8
            while self.data_section.len() % 8 != 0 {
                self.data_section.push(0);
            }
            self.global_offsets.insert(name.clone(), off);
        }

        // [pass 2]: generar código de funciones
        for func in &module.functions {
            self.generate_function(func);
        }
        self.patch_calls();
        self.encoder.code.clone()
    }
    
    /// B-06: dirección absoluta de un global (PE absolute addressing).
    fn global_addr(&self, name: &str) -> Option<u64> {
        self.global_offsets.get(name).map(|&off| IMAGE_BASE + DATA_RVA + off as u64)
    }
 
    fn generate_function(&mut self, func: &IrFunction) {
        self.var_offsets.clear();
        self.label_offsets.clear();
        self.pending_jumps.clear();
        self.alloca_regs.clear();
        self.alloca_sizes.clear();
        self.next_offset = -8;  // Reset for each function
        
        // B-01: pre-pass para descubrir tamaño extra de allocas (arrays/structs).
        // Cada Alloca con `count = Some(N)` reserva N * sizeof(elem) bytes.
        for block in &func.blocks {
            for instr in &block.instrs {
                if let IrInstr::Alloca { dst, ty, count } = instr {
                    let elem = ty.size().max(1) as i32;
                    let n = match count {
                        Some(IrValue::Const(c)) => c.as_i64().unwrap_or(1) as i32,
                        _ => 1,
                    };
                    let bytes = ((elem * n + 7) & !7).max(8); // alineado a 8, mínimo 8
                    self.alloca_sizes.insert(dst.id, bytes);
                }
            }
        }

        // Record function entry offset for intra-module calls
        self.func_offsets.insert(func.name.clone(), self.encoder.len());

        // Pre-calculate stack space needed: 8 bytes por reg "normal"
        // + bytes extra reservados por allocas grandes.
        let num_vars = func.next_reg as i32;
        let extra: i32 = self.alloca_sizes.values().map(|b| (b - 8).max(0)).sum();
        let stack_size = align16((num_vars * 8) + extra + SHADOW_SPACE + 8);
        
        // Prologue: push rbp; mov rbp, rsp; sub rsp, stack_size
        self.encoder.push_r(Reg64::RBP);
        self.encoder.mov_rr(Reg64::RBP, Reg64::RSP);
        if stack_size > 0 {
            self.encoder.sub_ri(Reg64::RSP, stack_size);
        }

        // Copy params from ABI regs to stack slots
        for (i, (_name, _ty)) in func.params.iter().enumerate() {
            if i < ARG_REGS.len() {
                let offset = -((i as i32 + 1) * 8);
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

        // Patch jumps immediately while label_offsets are still valid
        self.patch_jumps();
    }
 
    fn emit_epilogue(&mut self) {
        // mov rsp, rbp
        self.encoder.mov_rr(Reg64::RSP, Reg64::RBP);
        // pop rbp
        self.encoder.pop_r(Reg64::RBP);
        // ret
        self.encoder.ret();
    }

    // Get or allocate stack slot for IR reg (direct variable storage)
    fn get_var_offset(&mut self, id: u32) -> i32 {
        if let Some(&offset) = self.var_offsets.get(&id) {
            return offset;
        }
        // Allocate new slot
        let offset = self.next_offset;
        self.next_offset -= 8;
        self.var_offsets.insert(id, offset);
        offset
    }

    /// B-01: como `get_var_offset` pero reserva el tamaño exacto requerido por
    /// `alloca_sizes[id]` (≥ 8). Para arrays/structs grandes esto evita que dos
    /// allocas contiguos pisen sus slots.
    fn get_alloca_offset(&mut self, id: u32) -> i32 {
        if let Some(&offset) = self.var_offsets.get(&id) {
            return offset;
        }
        let bytes = *self.alloca_sizes.get(&id).unwrap_or(&8);
        // El "offset" es la dirección del primer byte del slot. Como crecemos hacia
        // abajo, restamos `bytes` para reservar la región completa.
        self.next_offset -= bytes - 8; // ya teníamos 8 reservados implícitamente
        let offset = self.next_offset;
        self.next_offset -= 8;
        self.var_offsets.insert(id, offset);
        offset
    }

    // Store register value directly to stack slot
    fn store_var(&mut self, ir_reg: IrReg, src: Reg64) {
        let offset = self.get_var_offset(ir_reg.id);
        self.encoder.mov_mr_disp(Reg64::RBP, offset, src);
    }

    // Load value directly from stack slot
    fn load_var(&mut self, dst: Reg64, ir_reg: IrReg) {
        let offset = self.get_var_offset(ir_reg.id);
        self.encoder.mov_rm_disp(dst, Reg64::RBP, offset);
    }
 
    fn generate_instr(&mut self, instr: &IrInstr) {
        match instr {
            IrInstr::Add { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.add_rr(Reg64::RAX, Reg64::RCX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Sub { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.sub_rr(Reg64::RAX, Reg64::RCX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Mul { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.imul_rr(Reg64::RAX, Reg64::RCX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Div { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.cqo();  // sign extend RAX to RDX:RAX
                self.encoder.idiv_r(Reg64::RCX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Mod { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.cqo();
                self.encoder.idiv_r(Reg64::RCX);
                self.store_var(*dst, Reg64::RDX);  // remainder in RDX
            }
            IrInstr::Neg { dst, src } => {
                self.load_value(Reg64::RAX, src);
                self.encoder.neg_r(Reg64::RAX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::And { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.and_rr(Reg64::RAX, Reg64::RCX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Or { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.or_rr(Reg64::RAX, Reg64::RCX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Xor { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.xor_rr(Reg64::RAX, Reg64::RCX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Shl { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                self.encoder.shl_rcl(Reg64::RAX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Shr { dst, lhs, rhs } => {
                self.load_value(Reg64::RAX, lhs);
                self.load_value(Reg64::RCX, rhs);
                // Use SAR for signed shift
                self.encoder.emit(0x48); self.encoder.emit(0xD3); self.encoder.emit(0xF8);
                self.store_var(*dst, Reg64::RAX);
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
            IrInstr::Alloca { dst, ty: _, count: _ } => {
                // B-01: registrar este reg como "address-of stack-slot".
                // El offset se reserva al primer uso vía get_alloca_offset, garantizando
                // suficiente espacio para arrays / structs según `alloca_sizes`.
                self.alloca_regs.insert(dst.id);
                let _ = self.get_alloca_offset(dst.id);
            }
            IrInstr::Load { dst, ptr } => {
                match ptr {
                    IrValue::Reg(r) => {
                        let offset = self.get_var_offset(r.id);
                        if self.alloca_regs.contains(&r.id) {
                            // r es la dirección del slot → load directo desde slot.
                            self.encoder.mov_rm_disp(Reg64::RAX, Reg64::RBP, offset);
                            self.store_var(*dst, Reg64::RAX);
                        } else {
                            // B-01: r contiene un puntero (valor) → 1) cargar el ptr,
                            // 2) deref → mov rax, [rcx]. Necesario para `*p`, `arr[i]`.
                            self.encoder.mov_rm_disp(Reg64::RCX, Reg64::RBP, offset);
                            self.encoder.mov_rm(Reg64::RAX, Reg64::RCX);
                            self.store_var(*dst, Reg64::RAX);
                        }
                    }
                    IrValue::Global(name) => {
                        // B-06: load desde dirección absoluta del global
                        if let Some(addr) = self.global_addr(name) {
                            self.encoder.mov_ri(Reg64::RAX, addr);   // RAX = &global
                            self.encoder.mov_rm(Reg64::RAX, Reg64::RAX); // RAX = *RAX
                            self.store_var(*dst, Reg64::RAX);
                        } else {
                            self.encoder.mov_ri(Reg64::RAX, 0);
                            self.store_var(*dst, Reg64::RAX);
                        }
                    }
                    _ => {
                        self.load_value(Reg64::RAX, ptr);
                        self.encoder.mov_rm(Reg64::RAX, Reg64::RAX);
                        self.store_var(*dst, Reg64::RAX);
                    }
                }
            }
            IrInstr::Store { ptr, val } => {
                match ptr {
                    IrValue::Reg(r) => {
                        let offset = self.get_var_offset(r.id);
                        if self.alloca_regs.contains(&r.id) {
                            // Store directo al stack-slot (alloca)
                            self.load_value(Reg64::RCX, val);
                            self.encoder.mov_mr_disp(Reg64::RBP, offset, Reg64::RCX);
                        } else {
                            // B-01: r contiene un puntero (valor) → store con deref.
                            // 1) cargar el ptr, 2) cargar val, 3) mov [rax], rcx.
                            self.encoder.mov_rm_disp(Reg64::RAX, Reg64::RBP, offset);
                            self.load_value(Reg64::RCX, val);
                            self.encoder.mov_mr(Reg64::RAX, Reg64::RCX);
                        }
                    }
                    IrValue::Global(name) => {
                        // B-06: store a dirección absoluta del global
                        if let Some(addr) = self.global_addr(name) {
                            self.load_value(Reg64::RCX, val);        // RCX = val
                            self.encoder.mov_ri(Reg64::RAX, addr);   // RAX = &global
                            self.encoder.mov_mr(Reg64::RAX, Reg64::RCX); // *RAX = RCX
                        }
                    }
                    _ => {
                        self.load_value(Reg64::RAX, ptr);
                        self.load_value(Reg64::RCX, val);
                        self.encoder.mov_mr(Reg64::RAX, Reg64::RCX);
                    }
                }
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
                // Load first 4 args to ABI regs (Win64 fastcall)
                for (i, arg) in args.iter().enumerate().take(4) {
                    self.load_value(ARG_REGS[i], arg);
                }
                // Reserve shadow space (Win64 ABI requires 32 bytes)
                self.encoder.sub_ri(Reg64::RSP, SHADOW_SPACE);
                
                // Emit CALL rel32 with placeholder, record patch site
                let patch_offset = self.encoder.len() + 1; // +1 to skip 0xE8 opcode
                self.encoder.call_rel32(0);
                self.pending_calls.push((patch_offset, func.clone()));
                
                // Restore shadow space
                self.encoder.add_ri(Reg64::RSP, SHADOW_SPACE);

                if let Some(d) = dst {
                    self.store_var(*d, Reg64::RAX);
                }
            }
            IrInstr::Copy { dst, src } => {
                self.load_value(Reg64::RAX, src);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Cast { dst, src, to_ty: _ } => {
                self.load_value(Reg64::RAX, src);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Not { dst, src } => {
                self.load_value(Reg64::RAX, src);
                self.encoder.not_r(Reg64::RAX);
                self.store_var(*dst, Reg64::RAX);
            }
            IrInstr::Phi { dst, incoming: _ } => {
                // PHI nodes should be lowered before codegen
                let _ = self.get_var_offset(dst.id);
            }
            IrInstr::Nop => {
                self.encoder.nop();
            }
        }
    }
 
    fn emit_compare(&mut self, dst: IrReg, lhs: &IrValue, rhs: &IrValue, cc: u8) {
        self.load_value(Reg64::RAX, lhs);
        self.load_value(Reg64::RCX, rhs);
        self.encoder.cmp_rr(Reg64::RAX, Reg64::RCX);
        // SETcc AL based on flags, then MOVZX to zero-extend to full RAX
        // NOTE: Do NOT xor rax before setcc — it would clobber the flags from cmp!
        self.encoder.setcc(cc, Reg64::RAX);
        self.encoder.movzx_rr(Reg64::RAX, Reg64::RAX);
        self.store_var(dst, Reg64::RAX);
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
                let offset = self.get_var_offset(r.id);
                if self.alloca_regs.contains(&r.id) {
                    // B-01: el "valor" de un alloca ES su dirección.
                    // Emitimos `lea dst, [rbp+offset]`.
                    self.encoder.lea(dst, Reg64::RBP, offset);
                } else {
                    // Reg "normal": load del valor desde stack-slot.
                    self.encoder.mov_rm_disp(dst, Reg64::RBP, offset);
                }
            }
            IrValue::Global(name) => {
                // B-06: dirección absoluta del global (no su valor)
                let addr = self.global_addr(name).unwrap_or(0);
                self.encoder.mov_ri(dst, addr);
            }
            IrValue::Param(idx) => {
                let offset = -((*idx as i32 + 1) * 8);
                self.encoder.mov_rm_disp(dst, Reg64::RBP, offset);
            }
        }
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
    
    fn patch_calls(&mut self) {
        // Resolve intra-module function calls; unknown ones become external symbols
        let mut still_pending = Vec::new();
        for (patch_offset, fname) in self.pending_calls.drain(..).collect::<Vec<_>>() {
            if let Some(&target) = self.func_offsets.get(&fname) {
                let rel = (target as i32) - (patch_offset as i32) - 4;
                let bytes = rel.to_le_bytes();
                self.encoder.code[patch_offset] = bytes[0];
                self.encoder.code[patch_offset + 1] = bytes[1];
                self.encoder.code[patch_offset + 2] = bytes[2];
                self.encoder.code[patch_offset + 3] = bytes[3];
            } else {
                // External symbol: PE/ELF builder will resolve via IAT/PLT
                self.external_calls.push((patch_offset, fname.clone()));
                still_pending.push((patch_offset, fname));
            }
        }
        self.pending_calls = still_pending;
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
