// ============================================================
// BG — Binary Guardian: Binary Loader
// ============================================================
// Carga binarios PE/ELF/raw y extrae secciones de código
// para análisis por el Capability Mapper.
//
// Soporta:
//   - PE (Windows .exe/.dll)
//   - ELF (Linux executables/shared objects)
//   - Raw flat binary (boot sectors, firmware)
//
// Usa goblin para parsing cross-platform de formatos binarios.
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use std::path::Path;
use std::fmt;

/// Tipo de formato binario detectado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryFormat {
    PE,
    ELF,
    Raw,
}

impl fmt::Display for BinaryFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryFormat::PE => write!(f, "PE (Windows)"),
            BinaryFormat::ELF => write!(f, "ELF (Linux)"),
            BinaryFormat::Raw => write!(f, "Raw Binary"),
        }
    }
}

/// Tipo de sección.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionKind {
    Code,
    Data,
    ReadOnly,
    RWX,
    Unknown,
}

impl fmt::Display for SectionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SectionKind::Code => write!(f, "CODE"),
            SectionKind::Data => write!(f, "DATA"),
            SectionKind::ReadOnly => write!(f, "RODATA"),
            SectionKind::RWX => write!(f, "RWX"),
            SectionKind::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

/// Información de una sección del binario.
#[derive(Debug, Clone)]
pub struct SectionInfo {
    pub name: String,
    pub kind: SectionKind,
    pub offset: usize,
    pub size: usize,
    pub virtual_address: u64,
    pub executable: bool,
    pub writable: bool,
    pub readable: bool,
}

impl fmt::Display for SectionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let perms = format!("{}{}{}",
            if self.readable { "R" } else { "-" },
            if self.writable { "W" } else { "-" },
            if self.executable { "X" } else { "-" },
        );
        write!(f, "  {:16} {:>8} bytes  @0x{:08X}  [{}]  {}",
            self.name, self.size, self.virtual_address, perms, self.kind)
    }
}

/// Información extraída de un binario.
#[derive(Debug, Clone)]
pub struct BinaryInfo {
    pub format: BinaryFormat,
    pub path: String,
    pub total_size: usize,
    pub entry_point: u64,
    pub sections: Vec<SectionInfo>,
    /// Bytes de código extraídos (todas las secciones ejecutables concatenadas)
    pub code_bytes: Vec<u8>,
    /// Indica si se encontraron secciones RWX
    pub has_rwx: bool,
    pub rwx_count: usize,
}

impl fmt::Display for BinaryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Binary: {}", self.path)?;
        writeln!(f, "Format: {}", self.format)?;
        writeln!(f, "Size:   {} bytes", self.total_size)?;
        writeln!(f, "Entry:  0x{:016X}", self.entry_point)?;
        writeln!(f, "Sections ({}):", self.sections.len())?;
        for s in &self.sections {
            writeln!(f, "{}", s)?;
        }
        if self.has_rwx {
            writeln!(f, "⚠ WARNING: {} RWX section(s) detected!", self.rwx_count)?;
        }
        Ok(())
    }
}

/// Binary Loader — Carga y parsea binarios PE/ELF/Raw.
pub struct BinaryLoader;

impl BinaryLoader {
    /// Carga un binario desde un archivo y extrae su información.
    pub fn load_file(path: &Path) -> Result<BinaryInfo, String> {
        let data = std::fs::read(path)
            .map_err(|e| format!("Cannot read file '{}': {}", path.display(), e))?;

        let path_str = path.display().to_string();
        Self::load_bytes(&data, &path_str)
    }

    /// Carga un binario desde bytes en memoria.
    pub fn load_bytes(data: &[u8], name: &str) -> Result<BinaryInfo, String> {
        if data.len() < 4 {
            return Ok(Self::load_raw(data, name));
        }

        match goblin::Object::parse(data) {
            Ok(goblin::Object::PE(pe)) => Ok(Self::load_pe(&pe, data, name)),
            Ok(goblin::Object::Elf(elf)) => Ok(Self::load_elf(&elf, data, name)),
            _ => Ok(Self::load_raw(data, name)),
        }
    }

    /// Parsea un PE (Windows executable).
    fn load_pe(pe: &goblin::pe::PE, data: &[u8], name: &str) -> BinaryInfo {
        let mut sections = Vec::new();
        let mut code_bytes = Vec::new();
        let mut has_rwx = false;
        let mut rwx_count = 0;

        for section in &pe.sections {
            let sec_name = String::from_utf8_lossy(
                &section.name[..section.name.iter().position(|&b| b == 0).unwrap_or(section.name.len())]
            ).to_string();

            let characteristics = section.characteristics;
            let executable = (characteristics & 0x20000000) != 0; // IMAGE_SCN_MEM_EXECUTE
            let readable = (characteristics & 0x40000000) != 0;   // IMAGE_SCN_MEM_READ
            let writable = (characteristics & 0x80000000) != 0;   // IMAGE_SCN_MEM_WRITE

            let kind = if executable && writable {
                has_rwx = true;
                rwx_count += 1;
                SectionKind::RWX
            } else if executable {
                SectionKind::Code
            } else if writable {
                SectionKind::Data
            } else if readable {
                SectionKind::ReadOnly
            } else {
                SectionKind::Unknown
            };

            let offset = section.pointer_to_raw_data as usize;
            let size = section.size_of_raw_data as usize;

            if executable && offset + size <= data.len() {
                code_bytes.extend_from_slice(&data[offset..offset + size]);
            }

            sections.push(SectionInfo {
                name: sec_name,
                kind,
                offset,
                size,
                virtual_address: section.virtual_address as u64,
                executable,
                writable,
                readable,
            });
        }

        let entry_point = pe.entry as u64;

        BinaryInfo {
            format: BinaryFormat::PE,
            path: name.to_string(),
            total_size: data.len(),
            entry_point,
            sections,
            code_bytes,
            has_rwx,
            rwx_count,
        }
    }

    /// Parsea un ELF (Linux executable).
    fn load_elf(elf: &goblin::elf::Elf, data: &[u8], name: &str) -> BinaryInfo {
        let mut sections = Vec::new();
        let mut code_bytes = Vec::new();
        let mut has_rwx = false;
        let mut rwx_count = 0;

        for sh in &elf.section_headers {
            let sec_name = elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("").to_string();

            let executable = (sh.sh_flags & 0x4) != 0;  // SHF_EXECINSTR
            let writable = (sh.sh_flags & 0x1) != 0;    // SHF_WRITE
            let readable = (sh.sh_flags & 0x2) != 0;    // SHF_ALLOC

            let kind = if executable && writable {
                has_rwx = true;
                rwx_count += 1;
                SectionKind::RWX
            } else if executable {
                SectionKind::Code
            } else if writable {
                SectionKind::Data
            } else if readable {
                SectionKind::ReadOnly
            } else {
                SectionKind::Unknown
            };

            let offset = sh.sh_offset as usize;
            let size = sh.sh_size as usize;

            if executable && sh.sh_type == goblin::elf::section_header::SHT_PROGBITS
                && offset + size <= data.len()
            {
                code_bytes.extend_from_slice(&data[offset..offset + size]);
            }

            sections.push(SectionInfo {
                name: sec_name,
                kind,
                offset,
                size,
                virtual_address: sh.sh_addr,
                executable,
                writable,
                readable,
            });
        }

        BinaryInfo {
            format: BinaryFormat::ELF,
            path: name.to_string(),
            total_size: data.len(),
            entry_point: elf.entry,
            sections,
            code_bytes,
            has_rwx,
            rwx_count,
        }
    }

    /// Trata el binario como flat raw binary (boot sector, firmware).
    fn load_raw(data: &[u8], name: &str) -> BinaryInfo {
        BinaryInfo {
            format: BinaryFormat::Raw,
            path: name.to_string(),
            total_size: data.len(),
            entry_point: 0,
            sections: vec![SectionInfo {
                name: ".code".into(),
                kind: SectionKind::Code,
                offset: 0,
                size: data.len(),
                virtual_address: 0,
                executable: true,
                writable: false,
                readable: true,
            }],
            code_bytes: data.to_vec(),
            has_rwx: false,
            rwx_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_binary() {
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
        let info = BinaryLoader::load_bytes(&code, "test.bin").unwrap();
        assert_eq!(info.format, BinaryFormat::Raw);
        assert_eq!(info.code_bytes.len(), 5);
        assert_eq!(info.sections.len(), 1);
        assert!(!info.has_rwx);
    }

    #[test]
    fn test_empty_binary() {
        let code = vec![0x90];
        let info = BinaryLoader::load_bytes(&code, "empty.bin").unwrap();
        assert_eq!(info.format, BinaryFormat::Raw);
    }

    #[test]
    fn test_display() {
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
        let info = BinaryLoader::load_bytes(&code, "test.bin").unwrap();
        let display = format!("{}", info);
        assert!(display.contains("test.bin"));
        assert!(display.contains("Raw Binary"));
    }
}
