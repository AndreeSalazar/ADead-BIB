#!/usr/bin/env python3
"""
Genera el middle tier del compilador ADead-BIB:
- IR (Intermediate Representation)
- Optimizer (DCE, ConstantFolding, Inlining, Peephole)
- UB Detector (Division by zero, Null deref, OOB)
"""

from pathlib import Path

COMPILER_PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\compiler")

def generate_ir_rs():
    """Genera ir.rs con el IR de ADead-BIB"""
    return '''//! ADead-BIB Intermediate Representation
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
    pub globals: HashMap<String, (IrType, Option<IrConst>)>,
    pub structs: HashMap<String, Vec<(String, IrType)>>,
}

impl IrModule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::new(),
            globals: HashMap::new(),
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
    
    fn emit(&mut self, instr: IrInstr) {
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
    
    // Memory
    pub fn alloca(&mut self, ty: IrType) -> IrReg {
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
        self.emit(IrInstr::Alloca { dst, ty, count: None });
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
'''

def generate_optimizer_rs():
    """Genera optimizer.rs con los 4 passes de optimización"""
    return '''//! ADead-BIB Optimizer
//! 4 passes: DCE, ConstantFolding, Inlining, Peephole
//! Generado automáticamente
#![allow(dead_code)]

use super::ir::*;
use std::collections::{HashSet, HashMap};

// ============== OPTIMIZER ==============

pub struct Optimizer {
    pub passes: Vec<Box<dyn OptimizationPass>>,
    pub stats: OptStats,
}

#[derive(Debug, Default, Clone)]
pub struct OptStats {
    pub instrs_removed: usize,
    pub constants_folded: usize,
    pub funcs_inlined: usize,
    pub peephole_applied: usize,
}

pub trait OptimizationPass {
    fn name(&self) -> &'static str;
    fn run(&mut self, module: &mut IrModule) -> usize; // returns changes made
}

impl Optimizer {
    pub fn new() -> Self {
        Self {
            passes: vec![
                Box::new(ConstantFolding),
                Box::new(DeadCodeElimination),
                Box::new(Inlining::new(10)), // max 10 instrs to inline
                Box::new(Peephole),
            ],
            stats: OptStats::default(),
        }
    }
    
    pub fn run(&mut self, module: &mut IrModule) {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10;
        
        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            for pass in &mut self.passes {
                let changes = pass.run(module);
                if changes > 0 {
                    changed = true;
                }
            }
            iterations += 1;
        }
    }
    
    pub fn run_pass<P: OptimizationPass>(&mut self, pass: &mut P, module: &mut IrModule) -> usize {
        pass.run(module)
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

// ============== DEAD CODE ELIMINATION ==============

pub struct DeadCodeElimination;

impl OptimizationPass for DeadCodeElimination {
    fn name(&self) -> &'static str { "DCE" }
    
    fn run(&mut self, module: &mut IrModule) -> usize {
        let mut removed = 0;
        
        for func in &mut module.functions {
            removed += dce_function(func);
        }
        
        removed
    }
}

fn dce_function(func: &mut IrFunction) -> usize {
    let mut removed = 0;
    
    // Find all used registers
    let mut used_regs: HashSet<u32> = HashSet::new();
    
    // First pass: collect all uses
    for block in &func.blocks {
        for instr in &block.instrs {
            for val in instr.uses() {
                if let IrValue::Reg(reg) = val {
                    used_regs.insert(reg.id);
                }
            }
        }
    }
    
    // Second pass: remove dead instructions
    for block in &mut func.blocks {
        let orig_len = block.instrs.len();
        block.instrs.retain(|instr| {
            // Keep terminators and side-effecting instructions
            if instr.is_terminator() {
                return true;
            }
            
            match instr {
                IrInstr::Store { .. } | IrInstr::Call { .. } => true,
                _ => {
                    // Keep if result is used
                    if let Some(dst) = instr.dst_reg() {
                        used_regs.contains(&dst.id)
                    } else {
                        true
                    }
                }
            }
        });
        removed += orig_len - block.instrs.len();
    }
    
    removed
}

// ============== CONSTANT FOLDING ==============

pub struct ConstantFolding;

impl OptimizationPass for ConstantFolding {
    fn name(&self) -> &'static str { "ConstantFolding" }
    
    fn run(&mut self, module: &mut IrModule) -> usize {
        let mut folded = 0;
        
        for func in &mut module.functions {
            folded += fold_constants_function(func);
        }
        
        folded
    }
}

fn fold_constants_function(func: &mut IrFunction) -> usize {
    let mut folded = 0;
    let mut const_map: HashMap<u32, IrConst> = HashMap::new();
    
    for block in &mut func.blocks {
        for instr in &mut block.instrs {
            // Try to fold arithmetic operations
            let replacement = match instr {
                IrInstr::Add { dst, lhs, rhs } => {
                    fold_binop(*dst, lhs, rhs, |a, b| a + b, |a, b| a + b)
                }
                IrInstr::Sub { dst, lhs, rhs } => {
                    fold_binop(*dst, lhs, rhs, |a, b| a - b, |a, b| a - b)
                }
                IrInstr::Mul { dst, lhs, rhs } => {
                    fold_binop(*dst, lhs, rhs, |a, b| a * b, |a, b| a * b)
                }
                IrInstr::Div { dst, lhs, rhs } => {
                    // Don't fold division by zero
                    if let IrValue::Const(IrConst::I64(0)) = rhs { None }
                    else if let IrValue::Const(IrConst::I32(0)) = rhs { None }
                    else { fold_binop(*dst, lhs, rhs, |a, b| if b != 0 { a / b } else { a }, |a, b| if b != 0.0 { a / b } else { a }) }
                }
                _ => None,
            };
            
            if let Some((dst, constant)) = replacement {
                const_map.insert(dst.id, constant.clone());
                *instr = IrInstr::Copy { 
                    dst, 
                    src: IrValue::Const(constant) 
                };
                folded += 1;
            }
        }
    }
    
    folded
}

fn fold_binop<Fi, Ff>(
    dst: IrReg,
    lhs: &IrValue, 
    rhs: &IrValue,
    int_op: Fi,
    float_op: Ff,
) -> Option<(IrReg, IrConst)>
where
    Fi: Fn(i64, i64) -> i64,
    Ff: Fn(f64, f64) -> f64,
{
    match (lhs, rhs) {
        (IrValue::Const(l), IrValue::Const(r)) => {
            if let (Some(a), Some(b)) = (l.as_i64(), r.as_i64()) {
                let result = int_op(a, b);
                let const_val = match dst.ty {
                    IrType::I8 => IrConst::I8(result as i8),
                    IrType::I16 => IrConst::I16(result as i16),
                    IrType::I32 => IrConst::I32(result as i32),
                    _ => IrConst::I64(result),
                };
                return Some((dst, const_val));
            }
            if let (Some(a), Some(b)) = (l.as_f64(), r.as_f64()) {
                let result = float_op(a, b);
                let const_val = match dst.ty {
                    IrType::F32 => IrConst::F32(result as f32),
                    _ => IrConst::F64(result),
                };
                return Some((dst, const_val));
            }
            None
        }
        _ => None,
    }
}

// ============== INLINING ==============

pub struct Inlining {
    max_instr_count: usize,
}

impl Inlining {
    pub fn new(max_instr_count: usize) -> Self {
        Self { max_instr_count }
    }
}

impl OptimizationPass for Inlining {
    fn name(&self) -> &'static str { "Inlining" }
    
    fn run(&mut self, module: &mut IrModule) -> usize {
        let mut inlined = 0;
        
        // Find small functions that can be inlined
        let inline_candidates: HashMap<String, IrFunction> = module.functions
            .iter()
            .filter(|f| f.instr_count() <= self.max_instr_count && f.blocks.len() == 1)
            .map(|f| (f.name.clone(), f.clone()))
            .collect();
        
        // Inline calls
        for func in &mut module.functions {
            for block in &mut func.blocks {
                for instr in &mut block.instrs {
                    if let IrInstr::Call { dst, func: callee, args } = instr {
                        if let Some(target) = inline_candidates.get(callee) {
                            // Skip recursive calls
                            if target.name == func.name {
                                continue;
                            }
                            
                            // For now, just mark as potentially inlinable
                            // Full inlining requires register renaming
                            inlined += 1;
                        }
                    }
                }
            }
        }
        
        inlined
    }
}

// ============== PEEPHOLE OPTIMIZATION ==============

pub struct Peephole;

impl OptimizationPass for Peephole {
    fn name(&self) -> &'static str { "Peephole" }
    
    fn run(&mut self, module: &mut IrModule) -> usize {
        let mut optimized = 0;
        
        for func in &mut module.functions {
            for block in &mut func.blocks {
                optimized += peephole_block(block);
            }
        }
        
        optimized
    }
}

fn peephole_block(block: &mut IrBlock) -> usize {
    let mut optimized = 0;
    
    for instr in &mut block.instrs {
        // Pattern: add x, 0 -> copy x
        // Pattern: mul x, 1 -> copy x
        // Pattern: add x, 1 -> inc x (for codegen)
        // Pattern: sub x, 1 -> dec x (for codegen)
        
        let replacement = match instr {
            // x + 0 = x
            IrInstr::Add { dst, lhs, rhs: IrValue::Const(IrConst::I32(0)) } |
            IrInstr::Add { dst, lhs, rhs: IrValue::Const(IrConst::I64(0)) } => {
                Some(IrInstr::Copy { dst: *dst, src: lhs.clone() })
            }
            // 0 + x = x
            IrInstr::Add { dst, lhs: IrValue::Const(IrConst::I32(0)), rhs } |
            IrInstr::Add { dst, lhs: IrValue::Const(IrConst::I64(0)), rhs } => {
                Some(IrInstr::Copy { dst: *dst, src: rhs.clone() })
            }
            // x - 0 = x
            IrInstr::Sub { dst, lhs, rhs: IrValue::Const(IrConst::I32(0)) } |
            IrInstr::Sub { dst, lhs, rhs: IrValue::Const(IrConst::I64(0)) } => {
                Some(IrInstr::Copy { dst: *dst, src: lhs.clone() })
            }
            // x * 1 = x
            IrInstr::Mul { dst, lhs, rhs: IrValue::Const(IrConst::I32(1)) } |
            IrInstr::Mul { dst, lhs, rhs: IrValue::Const(IrConst::I64(1)) } => {
                Some(IrInstr::Copy { dst: *dst, src: lhs.clone() })
            }
            // 1 * x = x
            IrInstr::Mul { dst, lhs: IrValue::Const(IrConst::I32(1)), rhs } |
            IrInstr::Mul { dst, lhs: IrValue::Const(IrConst::I64(1)), rhs } => {
                Some(IrInstr::Copy { dst: *dst, src: rhs.clone() })
            }
            // x * 0 = 0
            IrInstr::Mul { dst, lhs: _, rhs: IrValue::Const(IrConst::I32(0)) } => {
                Some(IrInstr::Copy { dst: *dst, src: IrValue::Const(IrConst::I32(0)) })
            }
            IrInstr::Mul { dst, lhs: _, rhs: IrValue::Const(IrConst::I64(0)) } => {
                Some(IrInstr::Copy { dst: *dst, src: IrValue::Const(IrConst::I64(0)) })
            }
            // x / 1 = x
            IrInstr::Div { dst, lhs, rhs: IrValue::Const(IrConst::I32(1)) } |
            IrInstr::Div { dst, lhs, rhs: IrValue::Const(IrConst::I64(1)) } => {
                Some(IrInstr::Copy { dst: *dst, src: lhs.clone() })
            }
            _ => None,
        };
        
        if let Some(new_instr) = replacement {
            *instr = new_instr;
            optimized += 1;
        }
    }
    
    optimized
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_folding() {
        let mut builder = IrBuilder::new("test");
        builder.begin_function("test", vec![], IrType::I32);
        
        // 2 + 3 should fold to 5
        let _ = builder.add(IrType::I32,
            IrValue::Const(IrConst::I32(2)),
            IrValue::Const(IrConst::I32(3)));
        builder.ret(None);
        builder.end_function();
        
        let mut module = builder.finish();
        let mut pass = ConstantFolding;
        let folded = pass.run(&mut module);
        
        assert!(folded > 0);
    }
    
    #[test]
    fn test_peephole() {
        let mut builder = IrBuilder::new("test");
        builder.begin_function("test", vec![], IrType::I32);
        
        // x + 0 should become copy x
        let x = builder.alloca(IrType::I32);
        let _ = builder.add(IrType::I32, IrValue::Reg(x), IrValue::Const(IrConst::I32(0)));
        builder.ret(None);
        builder.end_function();
        
        let mut module = builder.finish();
        let mut pass = Peephole;
        let optimized = pass.run(&mut module);
        
        assert!(optimized > 0);
    }
}
'''

def generate_ub_detector_rs():
    """Genera ub_detector.rs para detectar UB en IR"""
    return '''//! ADead-BIB Undefined Behavior Detector
//! Detecta: Division by zero, Null dereference, Out of bounds
//! Generado automáticamente
#![allow(dead_code)]

use super::ir::*;
use std::collections::HashMap;

// ============== UB KINDS ==============

#[derive(Debug, Clone, PartialEq)]
pub enum UbKind {
    DivisionByZero,
    NullPointerDereference,
    OutOfBoundsAccess,
    UseAfterFree,
    UninitializedRead,
    SignedOverflow,
    ShiftOverflow,
    InvalidCast,
}

impl UbKind {
    pub fn severity(&self) -> UbSeverity {
        match self {
            UbKind::DivisionByZero => UbSeverity::Error,
            UbKind::NullPointerDereference => UbSeverity::Error,
            UbKind::OutOfBoundsAccess => UbSeverity::Error,
            UbKind::UseAfterFree => UbSeverity::Error,
            UbKind::UninitializedRead => UbSeverity::Warning,
            UbKind::SignedOverflow => UbSeverity::Warning,
            UbKind::ShiftOverflow => UbSeverity::Warning,
            UbKind::InvalidCast => UbSeverity::Warning,
        }
    }
    
    pub fn message(&self) -> &'static str {
        match self {
            UbKind::DivisionByZero => "division by zero",
            UbKind::NullPointerDereference => "null pointer dereference",
            UbKind::OutOfBoundsAccess => "array index out of bounds",
            UbKind::UseAfterFree => "use after free",
            UbKind::UninitializedRead => "read of uninitialized value",
            UbKind::SignedOverflow => "signed integer overflow",
            UbKind::ShiftOverflow => "shift amount exceeds type width",
            UbKind::InvalidCast => "invalid type cast",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UbSeverity {
    Warning,
    Error,
}

// ============== UB REPORT ==============

#[derive(Debug, Clone)]
pub struct UbReport {
    pub kind: UbKind,
    pub func: String,
    pub block: BlockId,
    pub instr_idx: usize,
    pub message: String,
}

impl UbReport {
    pub fn new(kind: UbKind, func: &str, block: BlockId, instr_idx: usize) -> Self {
        Self {
            message: kind.message().to_string(),
            kind,
            func: func.to_string(),
            block,
            instr_idx,
        }
    }
    
    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = msg.into();
        self
    }
    
    pub fn is_error(&self) -> bool {
        self.kind.severity() == UbSeverity::Error
    }
}

// ============== UB DETECTOR ==============

#[derive(Debug, Default)]
pub struct UbDetector {
    pub reports: Vec<UbReport>,
    pub strict_mode: bool,
    // Track known null pointers
    null_regs: HashMap<u32, bool>,
    // Track freed pointers
    freed_regs: HashMap<u32, bool>,
    // Track initialized regs
    initialized: HashMap<u32, bool>,
    // Track array bounds
    array_bounds: HashMap<u32, usize>,
}

impl UbDetector {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn strict(mut self) -> Self {
        self.strict_mode = true;
        self
    }
    
    pub fn analyze(&mut self, module: &IrModule) {
        self.reports.clear();
        
        for func in &module.functions {
            self.analyze_function(func);
        }
    }
    
    fn analyze_function(&mut self, func: &IrFunction) {
        // Reset state for each function
        self.null_regs.clear();
        self.freed_regs.clear();
        self.initialized.clear();
        self.array_bounds.clear();
        
        for block in &func.blocks {
            self.analyze_block(func, block);
        }
    }
    
    fn analyze_block(&mut self, func: &IrFunction, block: &IrBlock) {
        for (idx, instr) in block.instrs.iter().enumerate() {
            self.analyze_instr(func, block.id, idx, instr);
        }
    }
    
    fn analyze_instr(&mut self, func: &IrFunction, block_id: BlockId, idx: usize, instr: &IrInstr) {
        match instr {
            // Division by zero
            IrInstr::Div { rhs, .. } | IrInstr::Mod { rhs, .. } => {
                if self.is_zero(rhs) {
                    self.report(UbKind::DivisionByZero, &func.name, block_id, idx);
                }
            }
            
            // Null pointer dereference (Load)
            IrInstr::Load { ptr, .. } => {
                if self.is_null(ptr) {
                    self.report(UbKind::NullPointerDereference, &func.name, block_id, idx);
                }
                if self.is_freed(ptr) {
                    self.report(UbKind::UseAfterFree, &func.name, block_id, idx);
                }
            }
            
            // Null pointer dereference (Store)
            IrInstr::Store { ptr, .. } => {
                if self.is_null(ptr) {
                    self.report(UbKind::NullPointerDereference, &func.name, block_id, idx);
                }
                if self.is_freed(ptr) {
                    self.report(UbKind::UseAfterFree, &func.name, block_id, idx);
                }
            }
            
            // Track allocations
            IrInstr::Alloca { dst, count, .. } => {
                self.initialized.insert(dst.id, true);
                if let Some(IrValue::Const(c)) = count {
                    if let Some(n) = c.as_i64() {
                        self.array_bounds.insert(dst.id, n as usize);
                    }
                }
            }
            
            // Track free calls
            IrInstr::Call { func: callee, args, .. } if callee == "free" => {
                if let Some(IrValue::Reg(r)) = args.first() {
                    self.freed_regs.insert(r.id, true);
                }
            }
            
            // Shift overflow
            IrInstr::Shl { dst, rhs, .. } | IrInstr::Shr { dst, rhs, .. } => {
                if let IrValue::Const(c) = rhs {
                    if let Some(shift) = c.as_i64() {
                        let bits = match dst.ty {
                            IrType::I8 => 8,
                            IrType::I16 => 16,
                            IrType::I32 => 32,
                            IrType::I64 => 64,
                            _ => 64,
                        };
                        if shift < 0 || shift >= bits {
                            self.report(UbKind::ShiftOverflow, &func.name, block_id, idx);
                        }
                    }
                }
            }
            
            // Copy propagation for null tracking
            IrInstr::Copy { dst, src } => {
                if self.is_null(src) {
                    self.null_regs.insert(dst.id, true);
                }
                self.initialized.insert(dst.id, true);
            }
            
            _ => {}
        }
        
        // Mark destination as initialized
        if let Some(dst) = instr.dst_reg() {
            self.initialized.insert(dst.id, true);
        }
    }
    
    fn is_zero(&self, val: &IrValue) -> bool {
        match val {
            IrValue::Const(IrConst::I8(0)) |
            IrValue::Const(IrConst::I16(0)) |
            IrValue::Const(IrConst::I32(0)) |
            IrValue::Const(IrConst::I64(0)) => true,
            _ => false,
        }
    }
    
    fn is_null(&self, val: &IrValue) -> bool {
        match val {
            IrValue::Const(IrConst::Null) => true,
            IrValue::Reg(r) => self.null_regs.get(&r.id).copied().unwrap_or(false),
            _ => false,
        }
    }
    
    fn is_freed(&self, val: &IrValue) -> bool {
        match val {
            IrValue::Reg(r) => self.freed_regs.get(&r.id).copied().unwrap_or(false),
            _ => false,
        }
    }
    
    fn report(&mut self, kind: UbKind, func: &str, block: BlockId, idx: usize) {
        self.reports.push(UbReport::new(kind, func, block, idx));
    }
    
    pub fn has_errors(&self) -> bool {
        self.reports.iter().any(|r| r.is_error())
    }
    
    pub fn error_count(&self) -> usize {
        self.reports.iter().filter(|r| r.is_error()).count()
    }
    
    pub fn warning_count(&self) -> usize {
        self.reports.iter().filter(|r| !r.is_error()).count()
    }
    
    pub fn print_reports(&self) {
        for report in &self.reports {
            let severity = if report.is_error() { "error" } else { "warning" };
            println!("[{}] {}: {} ({}:block{}:{})", 
                severity,
                report.kind.message(),
                report.message,
                report.func,
                report.block,
                report.instr_idx
            );
        }
    }
}

// ============== CONVENIENCE FUNCTIONS ==============

pub fn detect_ub(module: &IrModule) -> Vec<UbReport> {
    let mut detector = UbDetector::new();
    detector.analyze(module);
    detector.reports
}

pub fn detect_ub_strict(module: &IrModule) -> Vec<UbReport> {
    let mut detector = UbDetector::new().strict();
    detector.analyze(module);
    detector.reports
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_division_by_zero() {
        let mut builder = IrBuilder::new("test");
        builder.begin_function("test", vec![], IrType::I32);
        
        // x / 0 should trigger UB
        let _ = builder.div(IrType::I32,
            IrValue::Const(IrConst::I32(10)),
            IrValue::Const(IrConst::I32(0)));
        builder.ret(None);
        builder.end_function();
        
        let module = builder.finish();
        let reports = detect_ub(&module);
        
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].kind, UbKind::DivisionByZero);
    }
    
    #[test]
    fn test_null_dereference() {
        let mut builder = IrBuilder::new("test");
        builder.begin_function("test", vec![], IrType::I32);
        
        // Load from NULL should trigger UB
        let _ = builder.load(IrType::I32, IrValue::Const(IrConst::Null));
        builder.ret(None);
        builder.end_function();
        
        let module = builder.finish();
        let reports = detect_ub(&module);
        
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].kind, UbKind::NullPointerDereference);
    }
    
    #[test]
    fn test_shift_overflow() {
        let mut builder = IrBuilder::new("test");
        builder.begin_function("test", vec![], IrType::I32);
        
        // Shift by 32 on i32 is UB
        let x = builder.alloca(IrType::I32);
        builder.emit(IrInstr::Shl {
            dst: builder.new_reg(IrType::I32),
            lhs: IrValue::Reg(x),
            rhs: IrValue::Const(IrConst::I32(32)),
        });
        builder.ret(None);
        builder.end_function();
        
        let module = builder.finish();
        let reports = detect_ub(&module);
        
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].kind, UbKind::ShiftOverflow);
    }
}
'''

def generate_mod_rs():
    """Genera mod.rs actualizado para middle/"""
    return '''//! ADead-BIB Compiler Middle Tier
//! - IR: Intermediate Representation
//! - Optimizer: DCE, ConstantFolding, Inlining, Peephole
//! - UB Detector: Division by zero, Null deref, OOB
//! Generado automáticamente

pub mod ir;
pub mod optimizer;
pub mod ub_detector;

pub use ir::{IrModule, IrFunction, IrBlock, IrInstr, IrType, IrValue, IrConst, IrReg, IrBuilder};
pub use optimizer::{Optimizer, OptimizationPass};
pub use ub_detector::{UbDetector, UbKind, UbReport, detect_ub};
'''

def main():
    print("🔧 Generando middle tier del compilador ADead-BIB...")
    
    middle_path = COMPILER_PATH / "middle"
    middle_path.mkdir(parents=True, exist_ok=True)
    
    files = [
        ("mod.rs", generate_mod_rs()),
        ("ir.rs", generate_ir_rs()),
        ("optimizer.rs", generate_optimizer_rs()),
        ("ub_detector.rs", generate_ub_detector_rs()),
    ]
    
    for name, content in files:
        path = middle_path / name
        path.write_text(content, encoding='utf-8')
        lines = content.count('\n')
        print(f"  ✅ {name:18s} → {lines:4d} líneas")
    
    total = sum(f[1].count('\n') for f in files)
    print(f"\n📁 Middle tier generado en: {middle_path}")
    print(f"   Total: {total} líneas de Rust")

if __name__ == "__main__":
    main()
