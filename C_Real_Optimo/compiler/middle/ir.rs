//! Intermediate Representation - Stub
#![allow(dead_code)]

#[derive(Debug, Default)]
pub struct IR {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    // Stub for now
    Nop,
}

impl IR {
    pub fn new() -> Self {
        Self { instructions: Vec::new() }
    }
}
