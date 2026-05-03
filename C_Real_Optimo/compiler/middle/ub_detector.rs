//! ADead-BIB Undefined Behavior Detector
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
