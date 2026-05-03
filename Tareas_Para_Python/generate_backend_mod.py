#!/usr/bin/env python3
"""Genera backend/mod.rs - actualiza módulo backend"""

from pathlib import Path

BACKEND_PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\compiler\backend")

MOD_RS = '''//! ADead-BIB Compiler Backend
//! - Encoder: x86-64 instruction encoding (REX, ModRM, SIB, VEX)
//! - Codegen: IR → x86-64 machine code
//! - PE: Windows executable format
//! - ELF: Linux executable format
//! Generado automáticamente

pub mod encoder;
pub mod codegen;
pub mod pe;
pub mod elf;

pub use encoder::{X86Encoder, Reg64, Reg32, XmmReg};
pub use codegen::Codegen;
pub use pe::PeBuilder;
pub use elf::ElfBuilder;
'''

def main():
    print("🔧 Generando backend/mod.rs...")
    path = BACKEND_PATH / "mod.rs"
    path.write_text(MOD_RS, encoding='utf-8')
    print(f"  ✅ mod.rs → {MOD_RS.count(chr(10))} líneas")

if __name__ == "__main__":
    main()
