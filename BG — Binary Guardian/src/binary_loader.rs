// ============================================================
// BG — Binary Guardian: Binary Loader
// ============================================================
// Carga y analiza binarios PE, ELF y Raw.
// Extrae secciones, código, imports, exports, TLS, overlay.
//
// Usa goblin para parsing robusto de PE/ELF.
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use std::path::Path;
use std::fmt;
use super::arch_map::StructuralIntegrity;

// ============================================================
// Section Info
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionKind {
    Code,
    Data,
    ReadOnly,
    Unknown,
}

impl fmt::Display for SectionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SectionKind::Code => write!(f, "CODE"),
            SectionKind::Data => write!(f, "DATA"),
            SectionKind::ReadOnly => write!(f, "RODATA"),
            SectionKind::Unknown => write!(f, "???"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SectionInfo {
    pub name: String,
    pub offset: usize,
    pub size: usize,
    pub kind: SectionKind,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

impl SectionInfo {
    pub fn is_rwx(&self) -> bool { self.readable && self.writable && self.executable }
    pub fn is_data_executable(&self) -> bool { self.writable && self.executable && !self.name.starts_with(".text") }
}

// ============================================================
// Import/Export Entries
// ============================================================

#[derive(Debug, Clone)]
pub struct ImportEntry {
    pub library: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ExportEntry {
    pub name: String,
}

// ============================================================
// Binary Info
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryFormat { PE, ELF, Raw }

impl fmt::Display for BinaryFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryFormat::PE => write!(f, "PE (Windows)"),
            BinaryFormat::ELF => write!(f, "ELF (Linux)"),
            BinaryFormat::Raw => write!(f, "Raw Binary"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryInfo {
    pub format: BinaryFormat,
    pub entry_point: u64,
    pub sections: Vec<SectionInfo>,
    pub code_bytes: Vec<u8>,
    pub imports: Vec<ImportEntry>,
    pub exports: Vec<ExportEntry>,
    pub total_size: usize,
    pub header_size: usize,
    /// TLS callbacks detectados (PE only)
    pub tls_callbacks: Vec<u64>,
    /// Overlay: datos después de la última sección
    pub overlay_offset: Option<usize>,
    pub overlay_size: usize,
}

impl fmt::Display for BinaryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "═══════════════════════════════════════════════")?;
        writeln!(f, "  Binary Info")?;
        writeln!(f, "═══════════════════════════════════════════════")?;
        writeln!(f, "  Format:        {}", self.format)?;
        writeln!(f, "  Entry point:   0x{:X}", self.entry_point)?;
        writeln!(f, "  Total size:    {} bytes", self.total_size)?;
        writeln!(f, "  Header size:   {} bytes", self.header_size)?;
        writeln!(f, "  Sections:      {}", self.sections.len())?;
        writeln!(f, "  Imports:       {}", self.imports.len())?;
        writeln!(f, "  Exports:       {}", self.exports.len())?;
        if !self.tls_callbacks.is_empty() {
            writeln!(f, "  TLS Callbacks: {} ⚠", self.tls_callbacks.len())?;
        }
        if let Some(ov_off) = self.overlay_offset {
            writeln!(f, "  Overlay:       {} bytes at 0x{:X} ⚠", self.overlay_size, ov_off)?;
        }
        writeln!(f)?;
        for section in &self.sections {
            let perms = format!("{}{}{}",
                if section.readable { "R" } else { "-" },
                if section.writable { "W" } else { "-" },
                if section.executable { "X" } else { "-" });
            writeln!(f, "  [{:>8}] {:8} off=0x{:08X} size={:>8} {}",
                perms, section.kind, section.offset, section.size, section.name)?;
        }
        Ok(())
    }
}

// ============================================================
// Binary Loader
// ============================================================

pub struct BinaryLoader;

impl BinaryLoader {
    pub fn load_file(path: &Path) -> Result<BinaryInfo, String> {
        let data = std::fs::read(path).map_err(|e| format!("Cannot read '{}': {}", path.display(), e))?;
        Self::load_bytes(&data)
    }

    pub fn load_bytes(data: &[u8]) -> Result<BinaryInfo, String> {
        if data.len() < 4 {
            return Self::load_raw(data);
        }

        match goblin::Object::parse(data) {
            Ok(goblin::Object::PE(pe)) => Self::load_pe(&pe, data),
            Ok(goblin::Object::Elf(elf)) => Self::load_elf(&elf, data),
            _ => Self::load_raw(data),
        }
    }

    // ============================================================
    // PE Loading
    // ============================================================

    fn load_pe(pe: &goblin::pe::PE, data: &[u8]) -> Result<BinaryInfo, String> {
        let mut sections = Vec::new();
        let mut code_bytes = Vec::new();
        let entry_point = pe.entry as u64;
        let header_size = pe.header.optional_header
            .map(|oh| oh.windows_fields.size_of_headers as usize)
            .unwrap_or(0);

        let mut last_section_end: usize = 0;

        for section in &pe.sections {
            let name = String::from_utf8_lossy(
                &section.name[..section.name.iter().position(|&b| b == 0).unwrap_or(section.name.len())]
            ).to_string();
            let chars = section.characteristics;
            let size = section.size_of_raw_data as usize;
            let offset = section.pointer_to_raw_data as usize;
            let executable = chars & 0x20000000 != 0;  // IMAGE_SCN_MEM_EXECUTE
            let readable = chars & 0x40000000 != 0;     // IMAGE_SCN_MEM_READ
            let writable = chars & 0x80000000 != 0;     // IMAGE_SCN_MEM_WRITE
            let code_flag = chars & 0x00000020 != 0;    // IMAGE_SCN_CNT_CODE

            let kind = if code_flag || executable { SectionKind::Code }
            else if writable { SectionKind::Data }
            else if readable { SectionKind::ReadOnly }
            else { SectionKind::Unknown };

            if (code_flag || executable) && offset + size <= data.len() {
                code_bytes.extend_from_slice(&data[offset..offset + size]);
            }

            let section_end = offset + size;
            if section_end > last_section_end { last_section_end = section_end; }

            sections.push(SectionInfo { name, offset, size, kind, readable, writable, executable });
        }

        // Extract imports
        let mut imports = Vec::new();
        for import in &pe.imports {
            imports.push(ImportEntry {
                library: import.dll.to_string(),
                name: import.name.to_string(),
            });
        }

        // Extract exports
        let mut exports = Vec::new();
        for export in &pe.exports {
            if let Some(name) = export.name {
                exports.push(ExportEntry { name: name.to_string() });
            }
        }

        // TLS callbacks detection
        let mut tls_callbacks = Vec::new();
        if let Some(ref opt) = pe.header.optional_header {
            if let Some(ref dd_tls) = opt.data_directories.get_tls_table() {
                // TLS directory is present — binary has TLS callbacks
                let _ = dd_tls; // The existence of the TLS directory itself is the signal
                // goblin doesn't directly expose callbacks, but
                // having a TLS directory with non-zero size indicates callbacks
                if dd_tls.size > 0 {
                    tls_callbacks.push(0); // Marker for "TLS callbacks present"
                }
            }
        }

        // Overlay detection
        let overlay_offset = if last_section_end > 0 && last_section_end < data.len() {
            Some(last_section_end)
        } else {
            None
        };
        let overlay_size = overlay_offset.map(|o| data.len() - o).unwrap_or(0);

        if code_bytes.is_empty() && !data.is_empty() {
            code_bytes.extend_from_slice(data);
        }

        Ok(BinaryInfo {
            format: BinaryFormat::PE,
            entry_point,
            sections,
            code_bytes,
            imports,
            exports,
            total_size: data.len(),
            header_size,
            tls_callbacks,
            overlay_offset,
            overlay_size,
        })
    }

    // ============================================================
    // ELF Loading
    // ============================================================

    fn load_elf(elf: &goblin::elf::Elf, data: &[u8]) -> Result<BinaryInfo, String> {
        let mut sections = Vec::new();
        let mut code_bytes = Vec::new();
        let entry_point = elf.entry;
        let header_size = elf.header.e_ehsize as usize;

        let mut last_section_end: usize = 0;

        for sh in &elf.section_headers {
            let name = elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("").to_string();
            let offset = sh.sh_offset as usize;
            let size = sh.sh_size as usize;
            let executable = sh.sh_flags as u32 & 0x4 != 0;  // SHF_EXECINSTR
            let writable = sh.sh_flags as u32 & 0x1 != 0;    // SHF_WRITE
            let readable = true; // ELF sections are always readable if loaded

            let kind = if executable { SectionKind::Code }
            else if writable { SectionKind::Data }
            else { SectionKind::ReadOnly };

            if executable && sh.sh_type == goblin::elf::section_header::SHT_PROGBITS
                && offset + size <= data.len() {
                code_bytes.extend_from_slice(&data[offset..offset + size]);
            }

            let section_end = offset + size;
            if section_end > last_section_end { last_section_end = section_end; }

            sections.push(SectionInfo { name, offset, size, kind, readable, writable, executable });
        }

        // Extract imports from .dynsym
        let mut imports = Vec::new();
        for sym in &elf.dynsyms {
            if sym.is_import() {
                if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                    if !name.is_empty() {
                        imports.push(ImportEntry {
                            library: String::new(),
                            name: name.to_string(),
                        });
                    }
                }
            }
        }

        // Extract exports from .dynsym
        let mut exports = Vec::new();
        for sym in &elf.dynsyms {
            if !sym.is_import() && sym.st_bind() == goblin::elf::sym::STB_GLOBAL {
                if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                    if !name.is_empty() {
                        exports.push(ExportEntry { name: name.to_string() });
                    }
                }
            }
        }

        // Overlay detection
        let overlay_offset = if last_section_end > 0 && last_section_end < data.len() {
            Some(last_section_end)
        } else {
            None
        };
        let overlay_size = overlay_offset.map(|o| data.len() - o).unwrap_or(0);

        if code_bytes.is_empty() && !data.is_empty() {
            code_bytes.extend_from_slice(data);
        }

        Ok(BinaryInfo {
            format: BinaryFormat::ELF,
            entry_point,
            sections,
            code_bytes,
            imports,
            exports,
            total_size: data.len(),
            header_size,
            tls_callbacks: Vec::new(),
            overlay_offset,
            overlay_size,
        })
    }

    // ============================================================
    // Raw Binary
    // ============================================================

    fn load_raw(data: &[u8]) -> Result<BinaryInfo, String> {
        let sections = vec![SectionInfo {
            name: ".code".to_string(),
            offset: 0,
            size: data.len(),
            kind: SectionKind::Code,
            readable: true,
            writable: false,
            executable: true,
        }];
        Ok(BinaryInfo {
            format: BinaryFormat::Raw,
            entry_point: 0,
            sections,
            code_bytes: data.to_vec(),
            imports: Vec::new(),
            exports: Vec::new(),
            total_size: data.len(),
            header_size: 0,
            tls_callbacks: Vec::new(),
            overlay_offset: None,
            overlay_size: 0,
        })
    }

    // ============================================================
    // Structural Validation
    // ============================================================

    /// Validación estructural determinista del binario.
    /// No heurístico: cada check es una propiedad matemática.
    pub fn validate_structure(info: &BinaryInfo) -> StructuralIntegrity {
        let mut integrity = StructuralIntegrity::new();

        // Entry point validation
        if !info.sections.is_empty() && info.format != BinaryFormat::Raw {
            integrity.entry_point_checked = true;
            integrity.entry_point_valid = info.sections.iter().any(|s|
                s.executable && info.entry_point >= s.offset as u64
                    && info.entry_point < (s.offset + s.size) as u64
            );
            integrity.entry_at_section_start = info.sections.iter().any(|s|
                s.executable && info.entry_point == s.offset as u64
            );
        }

        // Code to data ratio
        let total_code: usize = info.sections.iter()
            .filter(|s| s.kind == SectionKind::Code)
            .map(|s| s.size).sum();
        let total_data: usize = info.sections.iter()
            .filter(|s| s.kind != SectionKind::Code)
            .map(|s| s.size).sum();
        integrity.code_to_data_ratio = if total_code + total_data > 0 {
            total_code as f64 / (total_code + total_data) as f64
        } else { 0.0 };

        // Section overlap detection
        for i in 0..info.sections.len() {
            for j in (i + 1)..info.sections.len() {
                let a = &info.sections[i];
                let b = &info.sections[j];
                if a.size > 0 && b.size > 0 {
                    let a_end = a.offset + a.size;
                    let b_end = b.offset + b.size;
                    if a.offset < b_end && b.offset < a_end {
                        integrity.overlapping_sections = true;
                    }
                }
            }
        }

        // Anomalous permissions (data+execute)
        integrity.anomalous_permissions = info.sections.iter()
            .filter(|s| s.is_data_executable())
            .count();

        // Header ratio
        integrity.header_ratio = if info.total_size > 0 {
            info.header_size as f64 / info.total_size as f64
        } else { 0.0 };

        // TLS callbacks
        integrity.has_tls_callbacks = !info.tls_callbacks.is_empty();
        integrity.tls_callback_count = info.tls_callbacks.len();

        // Overlay
        integrity.has_overlay = info.overlay_offset.is_some();
        integrity.overlay_size = info.overlay_size;

        integrity
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let info = BinaryInfo {
            format: BinaryFormat::Raw,
            entry_point: 0,
            sections: vec![SectionInfo {
                name: ".code".to_string(), offset: 0, size: 100,
                kind: SectionKind::Code, readable: true, writable: false, executable: true,
            }],
            code_bytes: vec![0x90; 100],
            imports: Vec::new(),
            exports: Vec::new(),
            total_size: 100,
            header_size: 0,
            tls_callbacks: Vec::new(),
            overlay_offset: None,
            overlay_size: 0,
        };
        let output = format!("{}", info);
        assert!(output.contains("Raw Binary"));
    }

    #[test]
    fn test_raw_binary() {
        let data = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
        let info = BinaryLoader::load_bytes(&data).unwrap();
        assert_eq!(info.format, BinaryFormat::Raw);
        assert_eq!(info.code_bytes, data);
        assert!(info.imports.is_empty());
        assert!(info.tls_callbacks.is_empty());
        assert!(info.overlay_offset.is_none());
    }

    #[test]
    fn test_validate_raw() {
        let data = vec![0x90; 64];
        let info = BinaryLoader::load_bytes(&data).unwrap();
        let integrity = BinaryLoader::validate_structure(&info);
        assert!(!integrity.entry_point_checked);
        assert!(!integrity.has_tls_callbacks);
        assert!(!integrity.has_overlay);
    }

    #[test]
    fn test_empty_binary() {
        let info = BinaryLoader::load_bytes(&[]).unwrap();
        assert_eq!(info.format, BinaryFormat::Raw);
        assert!(info.code_bytes.is_empty());
    }

    #[test]
    fn test_overlay_detection() {
        // Raw binary has no overlay (no sections with end < total)
        let data = vec![0x90; 128];
        let info = BinaryLoader::load_bytes(&data).unwrap();
        assert!(info.overlay_offset.is_none());
    }

    #[test]
    fn test_section_overlap_detection() {
        let info = BinaryInfo {
            format: BinaryFormat::PE,
            entry_point: 0,
            sections: vec![
                SectionInfo {
                    name: ".text".to_string(), offset: 0x200, size: 0x400,
                    kind: SectionKind::Code, readable: true, writable: false, executable: true,
                },
                SectionInfo {
                    name: ".data".to_string(), offset: 0x400, size: 0x300,
                    kind: SectionKind::Data, readable: true, writable: true, executable: false,
                },
            ],
            code_bytes: vec![0x90; 100],
            imports: Vec::new(),
            exports: Vec::new(),
            total_size: 0x700,
            header_size: 0x200,
            tls_callbacks: Vec::new(),
            overlay_offset: None,
            overlay_size: 0,
        };
        let integrity = BinaryLoader::validate_structure(&info);
        // .text: 0x200..0x600, .data: 0x400..0x700 → overlap
        assert!(integrity.overlapping_sections);
    }

    #[test]
    fn test_data_executable_detection() {
        let section = SectionInfo {
            name: ".rdata".to_string(), offset: 0, size: 100,
            kind: SectionKind::Data,
            readable: true, writable: true, executable: true,
        };
        assert!(section.is_data_executable());
        assert!(section.is_rwx());
    }
}
