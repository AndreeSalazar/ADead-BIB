//! ADead-BIB Compiler Backend
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
