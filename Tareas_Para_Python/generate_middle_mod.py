#!/usr/bin/env python3
"""Actualiza middle/mod.rs para incluir ast_to_ir"""

from pathlib import Path

MIDDLE_PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\compiler\middle")

MOD_RS = '''//! Middle tier: IR, Optimizer, UB Detector, AST→IR
pub mod ir;
pub mod optimizer;
pub mod ub_detector;
pub mod ast_to_ir;

pub use ir::*;
pub use optimizer::Optimizer;
pub use ub_detector::UbDetector;
pub use ast_to_ir::ast_to_ir;
'''

def main():
    print("🔧 Actualizando middle/mod.rs...")
    path = MIDDLE_PATH / "mod.rs"
    path.write_text(MOD_RS, encoding='utf-8')
    print(f"  ✅ mod.rs actualizado")

if __name__ == "__main__":
    main()
