//! ADead-BIB Optimizer
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
