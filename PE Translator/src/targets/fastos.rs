// ============================================================
// ADead FsOS Backend — FastOS Native Executable Generator
// ============================================================
// Generates executables in the FsOS format for FastOS.
//
// FsOS Format Layout:
//   [FsOS Header]    — 64 bytes (magic: "FsOS")
//   [Section Table]  — 32 bytes per section
//   [Code]           — .text
//   [Data]           — .data / .rodata
//
// This is the native format that FastOS kernel_main loads
// and executes directly, without PE/ELF overhead.
//
// Magic: 0x534F7346 = "FsOS" in little-endian
// ============================================================

use crate::bib::format::*;
use super::{BinaryBackend, BackendConfig, OutputFormat};

/// FsOS magic: "FsOS"
const FSOS_MAGIC: u32 = 0x534F_7346;
const FSOS_HEADER_SIZE: usize = 64;
const FSOS_SECTION_HEADER_SIZE: usize = 32;

/// FsOS section types
#[derive(Clone, Copy)]
#[repr(u16)]
enum FsosSectionType {
    Code   = 0x01,
    Data   = 0x02,
    RoData = 0x03,
    Bss    = 0x04,
}

pub struct FsOSBackend;

impl FsOSBackend {
    pub fn new() -> Self {
        FsOSBackend
    }
}

impl BinaryBackend for FsOSBackend {
    fn name(&self) -> &str {
        "FsOS (FastOS)"
    }

    fn supported_formats(&self) -> &[OutputFormat] {
        &[OutputFormat::FsOS]
    }

    fn validate(&self, module: &BibModule, config: &BackendConfig) -> Result<(), String> {
        if module.header.arch != Arch::X86_64 {
            return Err(format!("FsOS backend only supports x86-64, got {}", module.header.arch));
        }
        if config.format != OutputFormat::FsOS {
            return Err("FsOS backend only supports FsOS format".into());
        }
        if module.code().is_none() || module.code().unwrap().is_empty() {
            return Err("No code section in BIB module".into());
        }
        Ok(())
    }

    fn translate(&self, module: &BibModule, config: &BackendConfig) -> Result<Vec<u8>, String> {
        self.validate(module, config)?;

        let code = module.code().unwrap();
        let rodata = module.rodata().unwrap_or(&[]);
        let data_sec = module.data().unwrap_or(&[]);

        // Count sections
        let mut section_count: u16 = 1; // always have code
        if !rodata.is_empty() { section_count += 1; }
        if !data_sec.is_empty() { section_count += 1; }

        let section_table_size = section_count as usize * FSOS_SECTION_HEADER_SIZE;
        let data_start = FSOS_HEADER_SIZE + section_table_size;

        // Layout sections
        let mut sections: Vec<(FsosSectionType, &[u8], u32)> = Vec::new();
        let mut offset = data_start as u32;

        // .text
        sections.push((FsosSectionType::Code, code, offset));
        offset += code.len() as u32;
        offset = align_up_u32(offset, 16);

        // .rodata
        if !rodata.is_empty() {
            sections.push((FsosSectionType::RoData, rodata, offset));
            offset += rodata.len() as u32;
            offset = align_up_u32(offset, 16);
        }

        // .data
        if !data_sec.is_empty() {
            sections.push((FsosSectionType::Data, data_sec, offset));
            offset += data_sec.len() as u32;
            offset = align_up_u32(offset, 16);
        }

        let total_size = offset as usize;
        let mut image = vec![0u8; total_size];

        // --- FsOS Header (64 bytes) ---
        image[0..4].copy_from_slice(&FSOS_MAGIC.to_le_bytes());
        image[4..6].copy_from_slice(&1u16.to_le_bytes()); // version
        image[6..8].copy_from_slice(&section_count.to_le_bytes());
        // Entry point offset (relative to code section start)
        let entry_offset = if let Some(sym) = module.entry_symbol() {
            sym.offset as u32
        } else {
            0
        };
        image[8..12].copy_from_slice(&entry_offset.to_le_bytes());
        // Load address
        image[12..20].copy_from_slice(&config.image_base.to_le_bytes());
        // Total file size
        image[20..24].copy_from_slice(&(total_size as u32).to_le_bytes());
        // Code section file offset (for quick access)
        image[24..28].copy_from_slice(&(data_start as u32).to_le_bytes());
        // Code size
        image[28..32].copy_from_slice(&(code.len() as u32).to_le_bytes());
        // Arch (x86-64 = 1)
        image[32..34].copy_from_slice(&(module.header.arch as u16).to_le_bytes());
        // Reserved: bytes 34..64

        // --- Section Table ---
        for (i, (sec_type, sec_data, sec_offset)) in sections.iter().enumerate() {
            let base = FSOS_HEADER_SIZE + i * FSOS_SECTION_HEADER_SIZE;
            // Section type (2 bytes)
            image[base..base+2].copy_from_slice(&(*sec_type as u16).to_le_bytes());
            // Flags (2 bytes) — 0 for now
            // File offset (4 bytes)
            image[base+4..base+8].copy_from_slice(&sec_offset.to_le_bytes());
            // Size (4 bytes)
            image[base+8..base+12].copy_from_slice(&(sec_data.len() as u32).to_le_bytes());
            // Virtual address (4 bytes) — relative to image base
            let va = *sec_offset - data_start as u32;
            image[base+12..base+16].copy_from_slice(&va.to_le_bytes());
            // Alignment (4 bytes)
            image[base+16..base+20].copy_from_slice(&16u32.to_le_bytes());
            // Reserved: bytes 20..32
        }

        // --- Section Data ---
        for (_, sec_data, sec_offset) in &sections {
            let start = *sec_offset as usize;
            let end = start + sec_data.len();
            if end <= image.len() {
                image[start..end].copy_from_slice(sec_data);
            }
        }

        Ok(image)
    }
}

fn align_up_u32(value: u32, alignment: u32) -> u32 {
    if alignment == 0 { return value; }
    (value + alignment - 1) & !(alignment - 1)
}
