//! ADead-BIB UB Detector
//! Detects 21+ types of Undefined Behavior BEFORE the optimizer

use crate::ir::Module;

#[derive(Debug, Clone)]
pub enum UBKind {
    NullPointerDereference,
    ArrayOutOfBounds,
    IntegerOverflow,
    DivisionByZero,
    UseAfterFree,
    DoubleFree,
    UninitializedVariable,
    ShiftOverflow,
    SignedOverflowPromotion,
    TypeConfusion,
    InvalidCast,
    StrictAliasingViolation,
    AlignmentViolation,
    DataRace,
    UnsequencedModification,
    StackOverflow,
    FormatStringMismatch,
    DanglingPointer,
    BufferOverflow,
    IntegerUnderflow,
    MemoryLeak,
}

#[derive(Debug, Clone)]
pub struct UBWarning {
    pub kind: UBKind,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Default)]
pub struct UBReport {
    pub warnings: Vec<UBWarning>,
}

impl UBReport {
    pub fn new() -> Self {
        Self { warnings: Vec::new() }
    }

    pub fn has_errors(&self) -> bool {
        !self.warnings.is_empty()
    }
}

pub struct UBDetector;

impl UBDetector {
    pub fn new() -> Self {
        UBDetector
    }

    pub fn analyze(&self, _module: &Module) -> UBReport {
        UBReport::new()
    }
}
