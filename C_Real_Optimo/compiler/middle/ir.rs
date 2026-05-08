//! ADead-BIB Intermediate Representation
//! Generado automáticamente
#![allow(dead_code)]

use std::collections::HashMap;

// ============== IR TYPES ==============

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrType {
    Void,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Ptr(IrPtrType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrPtrType {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Void,
    Struct(u32), // struct ID
}

impl IrType {
    pub fn size(&self) -> usize {
        match self {
            IrType::Void => 0,
            IrType::I8 => 1,
            IrType::I16 => 2,
            IrType::I32 | IrType::F32 => 4,
            IrType::I64 | IrType::F64 | IrType::Ptr(_) => 8,
        }
    }
    
    pub fn is_integer(&self) -> bool {
        matches!(self, IrType::I8 | IrType::I16 | IrType::I32 | IrType::I64)
    }
    
    pub fn is_float(&self) -> bool {
        matches!(self, IrType::F32 | IrType::F64)
    }
    
    pub fn is_pointer(&self) -> bool {
        matches!(self, IrType::Ptr(_))
    }
}

// ============== IR VALUES ==============

#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    Const(IrConst),
    Reg(IrReg),
    Global(String),
    Param(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrConst {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Null,
}

impl IrConst {
    pub fn ty(&self) -> IrType {
        match self {
            IrConst::I8(_) => IrType::I8,
            IrConst::I16(_) => IrType::I16,
            IrConst::I32(_) => IrType::I32,
            IrConst::I64(_) => IrType::I64,
            IrConst::F32(_) => IrType::F32,
            IrConst::F64(_) => IrType::F64,
            IrConst::Null => IrType::Ptr(IrPtrType::Void),
        }
    }
    
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            IrConst::I8(v) => Some(*v as i64),
            IrConst::I16(v) => Some(*v as i64),
            IrConst::I32(v) => Some(*v as i64),
            IrConst::I64(v) => Some(*v),
            _ => None,
        }
    }
    
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            IrConst::F32(v) => Some(*v as f64),
            IrConst::F64(v) => Some(*v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrReg {
    pub id: u32,
    pub ty: IrType,
}

impl IrReg {
    pub fn new(id: u32, ty: IrType) -> Self {
        Self { id, ty }
    }
}

// ============== IR INSTRUCTIONS ==============

#[derive(Debug, Clone, PartialEq)]
pub enum IrInstr {
    // Arithmetic
    Add { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Sub { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Mul { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Div { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Mod { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Neg { dst: IrReg, src: IrValue },
    
    // Bitwise
    And { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Or  { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Xor { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Not { dst: IrReg, src: IrValue },
    Shl { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Shr { dst: IrReg, lhs: IrValue, rhs: IrValue },
    
    // Comparison
    Eq  { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Ne  { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Lt  { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Le  { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Gt  { dst: IrReg, lhs: IrValue, rhs: IrValue },
    Ge  { dst: IrReg, lhs: IrValue, rhs: IrValue },
    
    // Memory
    Alloca { dst: IrReg, ty: IrType, count: Option<IrValue> },
    Load   { dst: IrReg, ptr: IrValue },
    Store  { ptr: IrValue, val: IrValue },
    
    // Control flow
    Jmp   { target: BlockId },
    JmpIf { cond: IrValue, then_bb: BlockId, else_bb: BlockId },
    Ret   { val: Option<IrValue> },
    
    // Calls
    Call { dst: Option<IrReg>, func: String, args: Vec<IrValue> },
    
    // Conversions
    Cast { dst: IrReg, src: IrValue, to_ty: IrType },
    
    // Misc
    Phi { dst: IrReg, incoming: Vec<(BlockId, IrValue)> },
    Copy { dst: IrReg, src: IrValue },
    Nop,
}

impl IrInstr {
    pub fn dst_reg(&self) -> Option<IrReg> {
        match self {
            IrInstr::Add { dst, .. } |
            IrInstr::Sub { dst, .. } |
            IrInstr::Mul { dst, .. } |
            IrInstr::Div { dst, .. } |
            IrInstr::Mod { dst, .. } |
            IrInstr::Neg { dst, .. } |
            IrInstr::And { dst, .. } |
            IrInstr::Or  { dst, .. } |
            IrInstr::Xor { dst, .. } |
            IrInstr::Not { dst, .. } |
            IrInstr::Shl { dst, .. } |
            IrInstr::Shr { dst, .. } |
            IrInstr::Eq  { dst, .. } |
            IrInstr::Ne  { dst, .. } |
            IrInstr::Lt  { dst, .. } |
            IrInstr::Le  { dst, .. } |
            IrInstr::Gt  { dst, .. } |
            IrInstr::Ge  { dst, .. } |
            IrInstr::Alloca { dst, .. } |
            IrInstr::Load { dst, .. } |
            IrInstr::Cast { dst, .. } |
            IrInstr::Phi { dst, .. } |
            IrInstr::Copy { dst, .. } => Some(*dst),
            IrInstr::Call { dst, .. } => *dst,
            _ => None,
        }
    }
    
    pub fn uses(&self) -> Vec<&IrValue> {
        match self {
            IrInstr::Add { lhs, rhs, .. } |
            IrInstr::Sub { lhs, rhs, .. } |
            IrInstr::Mul { lhs, rhs, .. } |
            IrInstr::Div { lhs, rhs, .. } |
            IrInstr::Mod { lhs, rhs, .. } |
            IrInstr::And { lhs, rhs, .. } |
            IrInstr::Or  { lhs, rhs, .. } |
            IrInstr::Xor { lhs, rhs, .. } |
            IrInstr::Shl { lhs, rhs, .. } |
            IrInstr::Shr { lhs, rhs, .. } |
            IrInstr::Eq  { lhs, rhs, .. } |
            IrInstr::Ne  { lhs, rhs, .. } |
            IrInstr::Lt  { lhs, rhs, .. } |
            IrInstr::Le  { lhs, rhs, .. } |
            IrInstr::Gt  { lhs, rhs, .. } |
            IrInstr::Ge  { lhs, rhs, .. } => vec![lhs, rhs],
            IrInstr::Neg { src, .. } |
            IrInstr::Not { src, .. } |
            IrInstr::Cast { src, .. } |
            IrInstr::Copy { src, .. } => vec![src],
            IrInstr::Load { ptr, .. } => vec![ptr],
            IrInstr::Store { ptr, val } => vec![ptr, val],
            IrInstr::Alloca { count: Some(c), .. } => vec![c],
            IrInstr::JmpIf { cond, .. } => vec![cond],
            IrInstr::Ret { val: Some(v) } => vec![v],
            IrInstr::Call { args, .. } => args.iter().collect(),
            IrInstr::Phi { incoming, .. } => incoming.iter().map(|(_, v)| v).collect(),
            _ => vec![],
        }
    }
    
    pub fn is_terminator(&self) -> bool {
        matches!(self, IrInstr::Jmp { .. } | IrInstr::JmpIf { .. } | IrInstr::Ret { .. })
    }
}

// ============== IR BLOCKS ==============

pub type BlockId = u32;

#[derive(Debug, Clone)]
pub struct IrBlock {
    pub id: BlockId,
    pub name: String,
    pub instrs: Vec<IrInstr>,
    pub preds: Vec<BlockId>,
    pub succs: Vec<BlockId>,
}

impl IrBlock {
    pub fn new(id: BlockId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            instrs: Vec::new(),
            preds: Vec::new(),
            succs: Vec::new(),
        }
    }
    
    pub fn push(&mut self, instr: IrInstr) {
        self.instrs.push(instr);
    }
    
    pub fn terminator(&self) -> Option<&IrInstr> {
        self.instrs.last().filter(|i| i.is_terminator())
    }
}

// ============== IR FUNCTIONS ==============

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub ret_ty: IrType,
    pub blocks: Vec<IrBlock>,
    pub locals: HashMap<String, IrReg>,
    pub next_reg: u32,
    pub next_block: BlockId,
}

impl IrFunction {
    pub fn new(name: impl Into<String>, params: Vec<(String, IrType)>, ret_ty: IrType) -> Self {
        let mut func = Self {
            name: name.into(),
            params,
            ret_ty,
            blocks: Vec::new(),
            locals: HashMap::new(),
            next_reg: 0,
            next_block: 0,
        };
        func.new_block("entry");
        func
    }
    
    pub fn new_reg(&mut self, ty: IrType) -> IrReg {
        let reg = IrReg::new(self.next_reg, ty);
        self.next_reg += 1;
        reg
    }
    
    pub fn new_block(&mut self, name: impl Into<String>) -> BlockId {
        let id = self.next_block;
        self.next_block += 1;
        self.blocks.push(IrBlock::new(id, name));
        id
    }
    
    pub fn block(&self, id: BlockId) -> Option<&IrBlock> {
        self.blocks.iter().find(|b| b.id == id)
    }
    
    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut IrBlock> {
        self.blocks.iter_mut().find(|b| b.id == id)
    }
    
    pub fn entry_block(&self) -> Option<&IrBlock> {
        self.blocks.first()
    }
    
    pub fn push_instr(&mut self, block_id: BlockId, instr: IrInstr) {
        if let Some(block) = self.block_mut(block_id) {
            block.push(instr);
        }
    }
    
    pub fn instr_count(&self) -> usize {
        self.blocks.iter().map(|b| b.instrs.len()).sum()
    }
}

// ============== IR MODULE ==============

#[derive(Debug, Clone, Default)]
pub struct IrModule {
    pub name: String,
    pub functions: Vec<IrFunction>,
    /// (name, type, optional initial value for `.data` section)
    pub globals: Vec<(String, IrType, Option<i64>)>,
    pub strings: Vec<String>,
    pub structs: HashMap<String, Vec<(String, IrType)>>,
}

impl IrModule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::new(),
            globals: Vec::new(),
            strings: Vec::new(),
            structs: HashMap::new(),
        }
    }
    
    pub fn add_function(&mut self, func: IrFunction) {
        self.functions.push(func);
    }
    
    pub fn find_function(&self, name: &str) -> Option<&IrFunction> {
        self.functions.iter().find(|f| f.name == name)
    }
    
    pub fn find_function_mut(&mut self, name: &str) -> Option<&mut IrFunction> {
        self.functions.iter_mut().find(|f| f.name == name)
    }
}

// ============== IR BUILDER ==============

pub struct IrBuilder {
    pub module: IrModule,
    current_func: Option<usize>,
    current_block: BlockId,
}

impl IrBuilder {
    pub fn new(module_name: impl Into<String>) -> Self {
        Self {
            module: IrModule::new(module_name),
            current_func: None,
            current_block: 0,
        }
    }
    
    pub fn begin_function(&mut self, name: impl Into<String>, params: Vec<(String, IrType)>, ret_ty: IrType) {
        let func = IrFunction::new(name, params, ret_ty);
        self.module.functions.push(func);
        self.current_func = Some(self.module.functions.len() - 1);
        self.current_block = 0;
    }
    
    pub fn end_function(&mut self) {
        self.current_func = None;
    }
    
    fn current_function(&mut self) -> Option<&mut IrFunction> {
        self.current_func.and_then(|i| self.module.functions.get_mut(i))
    }
    
    pub fn set_block(&mut self, block_id: BlockId) {
        self.current_block = block_id;
    }
    
    pub fn new_block(&mut self, name: impl Into<String>) -> BlockId {
        if let Some(func) = self.current_function() {
            func.new_block(name)
        } else {
            0
        }
    }
    
    pub fn new_reg(&mut self, ty: IrType) -> IrReg {
        if let Some(func) = self.current_function() {
            func.new_reg(ty)
        } else {
            IrReg::new(0, ty)
        }
    }
    
    pub fn emit(&mut self, instr: IrInstr) {
        let block_id = self.current_block;
        if let Some(func) = self.current_function() {
            func.push_instr(block_id, instr);
        }
    }
    
    // Arithmetic
    pub fn add(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Add { dst, lhs, rhs });
        dst
    }
    
    pub fn sub(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Sub { dst, lhs, rhs });
        dst
    }
    
    pub fn mul(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Mul { dst, lhs, rhs });
        dst
    }
    
    pub fn div(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Div { dst, lhs, rhs });
        dst
    }
    
    pub fn rem(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Mod { dst, lhs, rhs });
        dst
    }
    
    // Bitwise
    pub fn bit_and(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::And { dst, lhs, rhs });
        dst
    }
    
    pub fn bit_or(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Or { dst, lhs, rhs });
        dst
    }
    
    pub fn bit_xor(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Xor { dst, lhs, rhs });
        dst
    }
    
    pub fn shl(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Shl { dst, lhs, rhs });
        dst
    }
    
    pub fn shr(&mut self, ty: IrType, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Shr { dst, lhs, rhs });
        dst
    }
    
    // Comparisons
    pub fn cmp_eq(&mut self, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(IrType::I32);
        self.emit(IrInstr::Eq { dst, lhs, rhs });
        dst
    }
    pub fn cmp_ne(&mut self, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(IrType::I32);
        self.emit(IrInstr::Ne { dst, lhs, rhs });
        dst
    }
    pub fn cmp_lt(&mut self, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(IrType::I32);
        self.emit(IrInstr::Lt { dst, lhs, rhs });
        dst
    }
    pub fn cmp_le(&mut self, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(IrType::I32);
        self.emit(IrInstr::Le { dst, lhs, rhs });
        dst
    }
    pub fn cmp_gt(&mut self, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(IrType::I32);
        self.emit(IrInstr::Gt { dst, lhs, rhs });
        dst
    }
    pub fn cmp_ge(&mut self, lhs: IrValue, rhs: IrValue) -> IrReg {
        let dst = self.new_reg(IrType::I32);
        self.emit(IrInstr::Ge { dst, lhs, rhs });
        dst
    }
    
    // Unary
    pub fn neg(&mut self, ty: IrType, src: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Neg { dst, src });
        dst
    }
    
    pub fn bit_not(&mut self, ty: IrType, src: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Not { dst, src });
        dst
    }
    
    // Memory
    pub fn alloca(&mut self, ty: IrType) -> IrReg {
        self.alloca_n(ty, None)
    }

    /// B-01: alloca con count opcional para arrays / structs.
    /// `count = Some(N)` reserva N * sizeof(ty) bytes contiguos.
    pub fn alloca_n(&mut self, ty: IrType, count: Option<IrValue>) -> IrReg {
        let ptr_ty = match ty {
            IrType::I8 => IrPtrType::I8,
            IrType::I16 => IrPtrType::I16,
            IrType::I32 => IrPtrType::I32,
            IrType::I64 => IrPtrType::I64,
            IrType::F32 => IrPtrType::F32,
            IrType::F64 => IrPtrType::F64,
            _ => IrPtrType::Void,
        };
        let dst = self.new_reg(IrType::Ptr(ptr_ty));
        self.emit(IrInstr::Alloca { dst, ty, count });
        dst
    }
    
    pub fn load(&mut self, ty: IrType, ptr: IrValue) -> IrReg {
        let dst = self.new_reg(ty);
        self.emit(IrInstr::Load { dst, ptr });
        dst
    }
    
    pub fn store(&mut self, ptr: IrValue, val: IrValue) {
        self.emit(IrInstr::Store { ptr, val });
    }
    
    // Control flow
    pub fn jmp(&mut self, target: BlockId) {
        self.emit(IrInstr::Jmp { target });
    }
    
    pub fn jmp_if(&mut self, cond: IrValue, then_bb: BlockId, else_bb: BlockId) {
        self.emit(IrInstr::JmpIf { cond, then_bb, else_bb });
    }
    
    pub fn ret(&mut self, val: Option<IrValue>) {
        self.emit(IrInstr::Ret { val });
    }
    
    // Calls
    pub fn call(&mut self, ret_ty: IrType, func: impl Into<String>, args: Vec<IrValue>) -> Option<IrReg> {
        let dst = if ret_ty != IrType::Void {
            Some(self.new_reg(ret_ty))
        } else {
            None
        };
        self.emit(IrInstr::Call { dst, func: func.into(), args });
        dst
    }
    

    // Globals
    pub fn add_global(&mut self, name: &str, ty: IrType, init: Option<i64>) {
        self.module.globals.push((name.to_string(), ty, init));
    }
    
    // Strings
    pub fn add_string(&mut self, s: &str) -> usize {
        let idx = self.module.strings.len();
        self.module.strings.push(s.to_string());
        idx
    }

    pub fn finish(self) -> IrModule {
        self.module
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ir_builder() {
        let mut builder = IrBuilder::new("test");
        builder.begin_function("main", vec![], IrType::I32);
        
        let a = builder.add(IrType::I32, 
            IrValue::Const(IrConst::I32(2)), 
            IrValue::Const(IrConst::I32(3)));
        builder.ret(Some(IrValue::Reg(a)));
        
        builder.end_function();
        let module = builder.finish();
        
        assert_eq!(module.functions.len(), 1);
        assert_eq!(module.functions[0].name, "main");
    }
    
    #[test]
    fn test_ir_types() {
        assert_eq!(IrType::I32.size(), 4);
        assert_eq!(IrType::I64.size(), 8);
        assert!(IrType::I32.is_integer());
        assert!(IrType::F64.is_float());
    }
}
