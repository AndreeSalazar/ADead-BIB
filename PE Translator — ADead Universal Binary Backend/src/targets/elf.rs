// ============================================================
// ADead ELF Backend — Linux ELF64 Generator (Stub)
// ============================================================
// Future: Translates ADead-BIB modules into ELF64 executables.
//
// Layout:
//   ELF Header (64 bytes)
//   Program Headers
//   Section Headers
//   .text, .rodata, .data, .bss, .dynamic, .symtab, .strtab
//
// Target: Linux x86-64, System V ABI
// ============================================================

use crate::bib::format::*;
use super::{BinaryBackend, BackendConfig, OutputFormat};

// ELF64 constants
const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1; // Little-endian
const EV_CURRENT: u8 = 1;
const ELFOSABI_NONE: u8 = 0;
const ET_EXEC: u16 = 2;
const EM_X86_64: u16 = 62;

const ELF_HEADER_SIZE: usize = 64;
const PHDR_SIZE: usize = 56;

// Program header types
const PT_LOAD: u32 = 1;

// Flags
const PF_X: u32 = 1; // Execute
const PF_W: u32 = 2; // Write
const PF_R: u32 = 4; // Read

pub struct ElfBackend;

impl ElfBackend {
    pub fn new() -> Self {
        ElfBackend
    }
}

impl BinaryBackend for ElfBackend {
    fn name(&self) -> &str {
        "ELF (Linux)"
    }

    fn supported_formats(&self) -> &[OutputFormat] {
        &[OutputFormat::ElfExe, OutputFormat::ElfSo]
    }

    fn validate(&self, module: &BibModule, config: &BackendConfig) -> Result<(), String> {
        if module.header.arch != Arch::X86_64 {
            return Err(format!("ELF backend only supports x86-64, got {}", module.header.arch));
        }
        if !matches!(config.format, OutputFormat::ElfExe | OutputFormat::ElfSo) {
            return Err("ELF backend only supports ElfExe and ElfSo formats".into());
        }
        if module.code().is_none() || module.code().unwrap().is_empty() {
            return Err("No code section in BIB module".into());
        }
        Ok(())
    }

    fn translate(&self, module: &BibModule, config: &BackendConfig) -> Result<Vec<u8>, String> {
        self.validate(module, config)?;

        let code = module.code().unwrap();
        let image_base = config.image_base;

        // Simple static ELF: header + 1 PHDR (LOAD) + code
        let phdr_offset = ELF_HEADER_SIZE;
        let code_offset = align_up(ELF_HEADER_SIZE + PHDR_SIZE, 0x1000);
        let code_vaddr = image_base + code_offset as u64;
        let total_size = code_offset + code.len();

        let mut image = vec![0u8; align_up(total_size, 16)];

        // --- ELF Header ---
        image[0..4].copy_from_slice(&ELF_MAGIC);
        image[4] = ELFCLASS64;
        image[5] = ELFDATA2LSB;
        image[6] = EV_CURRENT;
        image[7] = ELFOSABI_NONE;
        // e_type
        image[16..18].copy_from_slice(&ET_EXEC.to_le_bytes());
        // e_machine
        image[18..20].copy_from_slice(&EM_X86_64.to_le_bytes());
        // e_version
        image[20..24].copy_from_slice(&1u32.to_le_bytes());
        // e_entry
        let entry_vaddr = if let Some(sym) = module.entry_symbol() {
            code_vaddr + sym.offset
        } else {
            code_vaddr
        };
        image[24..32].copy_from_slice(&entry_vaddr.to_le_bytes());
        // e_phoff
        image[32..40].copy_from_slice(&(phdr_offset as u64).to_le_bytes());
        // e_shoff = 0 (no section headers for minimal ELF)
        // e_flags = 0
        // e_ehsize
        image[52..54].copy_from_slice(&(ELF_HEADER_SIZE as u16).to_le_bytes());
        // e_phentsize
        image[54..56].copy_from_slice(&(PHDR_SIZE as u16).to_le_bytes());
        // e_phnum
        image[56..58].copy_from_slice(&1u16.to_le_bytes());
        // e_shentsize, e_shnum, e_shstrndx = 0

        // --- Program Header (LOAD segment: code) ---
        let ph = phdr_offset;
        image[ph..ph+4].copy_from_slice(&PT_LOAD.to_le_bytes()); // p_type
        image[ph+4..ph+8].copy_from_slice(&(PF_R | PF_X).to_le_bytes()); // p_flags
        image[ph+8..ph+16].copy_from_slice(&(code_offset as u64).to_le_bytes()); // p_offset
        image[ph+16..ph+24].copy_from_slice(&code_vaddr.to_le_bytes()); // p_vaddr
        image[ph+24..ph+32].copy_from_slice(&code_vaddr.to_le_bytes()); // p_paddr
        image[ph+32..ph+40].copy_from_slice(&(code.len() as u64).to_le_bytes()); // p_filesz
        image[ph+40..ph+48].copy_from_slice(&(code.len() as u64).to_le_bytes()); // p_memsz
        image[ph+48..ph+56].copy_from_slice(&0x1000u64.to_le_bytes()); // p_align

        // --- Code ---
        image[code_offset..code_offset + code.len()].copy_from_slice(code);

        Ok(image)
    }
}

fn align_up(value: usize, alignment: usize) -> usize {
    if alignment == 0 { return value; }
    (value + alignment - 1) & !(alignment - 1)
}
