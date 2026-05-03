//! Code Generator - Stub
#![allow(dead_code)]

use crate::middle::IR;

#[derive(Debug, Default)]
pub struct Codegen {
    pub code: Vec<u8>,
}

impl Codegen {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }
    
    pub fn generate(&mut self, _ir: &IR) -> Vec<u8> {
        self.code.clone()
    }
}
