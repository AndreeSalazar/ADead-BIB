// ============================================================
// ADead-BIB Format v1.0 — Binary Specification
// ============================================================
//
// File Layout:
//   [BibHeader]              — 64 bytes
//   [SectionHeader] × N      — 48 bytes each
//   [Section Data]            — variable
//
// Section Types:
//   Code     — raw machine code (x86-64, ARM64, etc.)
//   Data     — initialized read/write data
//   RoData   — read-only constants and strings
//   Bss      — uninitialized data (zero-filled)
//   Import   — serialized import table
//   Export   — serialized export table
//   Symbol   — symbol table for linking
//   Reloc    — relocation entries
//   Meta     — metadata (compiler version, flags)
//   Debug    — debug info (optional)
//
// Magic: 0x42494241 = "ABIB" (ADead Binary Intermediate Binary)
// ============================================================

use std::fmt;

// ============================================================
// Constants
// ============================================================

/// Magic number: "ABIB" in little-endian
pub const BIB_MAGIC: u32 = 0x4249_4241;

/// Current format version
pub const BIB_VERSION: u16 = 1;

/// Section header size in bytes
pub const SECTION_HEADER_SIZE: usize = 48;

/// Main header size in bytes
pub const BIB_HEADER_SIZE: usize = 64;

// ============================================================
// Architecture
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Arch {
    X86_64  = 0x0001,
    Arm64   = 0x0002,
    Riscv64 = 0x0003,
    Wasm32  = 0x0010,
    Unknown = 0xFFFF,
}

impl Arch {
    pub fn from_u16(v: u16) -> Self {
        match v {
            0x0001 => Arch::X86_64,
            0x0002 => Arch::Arm64,
            0x0003 => Arch::Riscv64,
            0x0010 => Arch::Wasm32,
            _      => Arch::Unknown,
        }
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arch::X86_64  => write!(f, "x86-64"),
            Arch::Arm64   => write!(f, "ARM64"),
            Arch::Riscv64 => write!(f, "RISC-V 64"),
            Arch::Wasm32  => write!(f, "WebAssembly 32"),
            Arch::Unknown => write!(f, "Unknown"),
        }
    }
}

// ============================================================
// BIB Header (64 bytes)
// ============================================================

#[derive(Debug, Clone)]
pub struct BibHeader {
    /// Magic number: BIB_MAGIC (0x42494241)
    pub magic: u32,
    /// Format version
    pub version: u16,
    /// Target architecture
    pub arch: Arch,
    /// Number of sections
    pub section_count: u32,
    /// Symbol index of the entry point function
    pub entry_point_symbol: u64,
    /// Flags (bitfield)
    pub flags: u32,
    /// Total file size in bytes
    pub file_size: u64,
    /// Offset to first section header
    pub section_table_offset: u64,
    /// Reserved for future use
    pub reserved: [u8; 16],
}

impl BibHeader {
    pub fn new(arch: Arch) -> Self {
        BibHeader {
            magic: BIB_MAGIC,
            version: BIB_VERSION,
            arch,
            section_count: 0,
            entry_point_symbol: 0,
            flags: 0,
            file_size: 0,
            section_table_offset: BIB_HEADER_SIZE as u64,
            reserved: [0; 16],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == BIB_MAGIC && self.version <= BIB_VERSION
    }

    /// Serialize to 64 bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; BIB_HEADER_SIZE];
        buf[0..4].copy_from_slice(&self.magic.to_le_bytes());
        buf[4..6].copy_from_slice(&self.version.to_le_bytes());
        buf[6..8].copy_from_slice(&(self.arch as u16).to_le_bytes());
        buf[8..12].copy_from_slice(&self.section_count.to_le_bytes());
        buf[12..20].copy_from_slice(&self.entry_point_symbol.to_le_bytes());
        buf[20..24].copy_from_slice(&self.flags.to_le_bytes());
        buf[24..32].copy_from_slice(&self.file_size.to_le_bytes());
        buf[32..40].copy_from_slice(&self.section_table_offset.to_le_bytes());
        buf[40..56].copy_from_slice(&self.reserved);
        // bytes 56..64 padding
        buf
    }

    /// Deserialize from bytes
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < BIB_HEADER_SIZE { return None; }
        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != BIB_MAGIC { return None; }

        Some(BibHeader {
            magic,
            version: u16::from_le_bytes([data[4], data[5]]),
            arch: Arch::from_u16(u16::from_le_bytes([data[6], data[7]])),
            section_count: u32::from_le_bytes([data[8], data[9], data[10], data[11]]),
            entry_point_symbol: u64::from_le_bytes(data[12..20].try_into().unwrap()),
            flags: u32::from_le_bytes([data[20], data[21], data[22], data[23]]),
            file_size: u64::from_le_bytes(data[24..32].try_into().unwrap()),
            section_table_offset: u64::from_le_bytes(data[32..40].try_into().unwrap()),
            reserved: data[40..56].try_into().unwrap(),
        })
    }
}

// ============================================================
// Section Types
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SectionType {
    Code    = 0x0001,
    Data    = 0x0002,
    RoData  = 0x0003,
    Bss     = 0x0004,
    Import  = 0x0010,
    Export  = 0x0011,
    Symbol  = 0x0020,
    Reloc   = 0x0030,
    Meta    = 0x0040,
    Debug   = 0x0050,
}

impl SectionType {
    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            0x0001 => Some(SectionType::Code),
            0x0002 => Some(SectionType::Data),
            0x0003 => Some(SectionType::RoData),
            0x0004 => Some(SectionType::Bss),
            0x0010 => Some(SectionType::Import),
            0x0011 => Some(SectionType::Export),
            0x0020 => Some(SectionType::Symbol),
            0x0030 => Some(SectionType::Reloc),
            0x0040 => Some(SectionType::Meta),
            0x0050 => Some(SectionType::Debug),
            _      => None,
        }
    }
}

// ============================================================
// Section Header (48 bytes)
// ============================================================

#[derive(Debug, Clone)]
pub struct SectionHeader {
    /// Section name (up to 16 bytes, null-padded)
    pub name: [u8; 16],
    /// Section type
    pub section_type: SectionType,
    /// Flags (readable, writable, executable)
    pub flags: u32,
    /// Offset of section data in file
    pub offset: u64,
    /// Size of section data in file
    pub size: u64,
    /// Alignment requirement (power of 2)
    pub alignment: u32,
    /// Reserved
    pub reserved: u32,
}

impl SectionHeader {
    pub fn new(name: &str, section_type: SectionType) -> Self {
        let mut name_buf = [0u8; 16];
        let bytes = name.as_bytes();
        let len = bytes.len().min(16);
        name_buf[..len].copy_from_slice(&bytes[..len]);

        SectionHeader {
            name: name_buf,
            section_type,
            flags: 0,
            offset: 0,
            size: 0,
            alignment: 16,
            reserved: 0,
        }
    }

    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(16);
        std::str::from_utf8(&self.name[..end]).unwrap_or("<invalid>")
    }

    /// Serialize to 48 bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; SECTION_HEADER_SIZE];
        buf[0..16].copy_from_slice(&self.name);
        buf[16..20].copy_from_slice(&(self.section_type as u32).to_le_bytes());
        buf[20..24].copy_from_slice(&self.flags.to_le_bytes());
        buf[24..32].copy_from_slice(&self.offset.to_le_bytes());
        buf[32..40].copy_from_slice(&self.size.to_le_bytes());
        buf[40..44].copy_from_slice(&self.alignment.to_le_bytes());
        buf[44..48].copy_from_slice(&self.reserved.to_le_bytes());
        buf
    }

    /// Deserialize from bytes
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < SECTION_HEADER_SIZE { return None; }
        let section_type_raw = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let section_type = SectionType::from_u32(section_type_raw)?;

        let mut name = [0u8; 16];
        name.copy_from_slice(&data[0..16]);

        Some(SectionHeader {
            name,
            section_type,
            flags: u32::from_le_bytes([data[20], data[21], data[22], data[23]]),
            offset: u64::from_le_bytes(data[24..32].try_into().unwrap()),
            size: u64::from_le_bytes(data[32..40].try_into().unwrap()),
            alignment: u32::from_le_bytes([data[40], data[41], data[42], data[43]]),
            reserved: u32::from_le_bytes([data[44], data[45], data[46], data[47]]),
        })
    }
}

/// Section flags
pub mod section_flags {
    pub const READABLE:   u32 = 1 << 0;
    pub const WRITABLE:   u32 = 1 << 1;
    pub const EXECUTABLE: u32 = 1 << 2;
}

// ============================================================
// Import Entry
// ============================================================

/// A single import: a function from an external module (DLL/SO)
#[derive(Debug, Clone)]
pub struct ImportEntry {
    /// Module name (e.g. "kernel32.dll", "libc.so.6")
    pub module: String,
    /// Symbol name (e.g. "ExitProcess", "printf")
    pub symbol: String,
    /// Hint/ordinal (optional, 0 if unknown)
    pub hint: u16,
}

/// A group of imports from one module
#[derive(Debug, Clone)]
pub struct ImportModule {
    pub name: String,
    pub symbols: Vec<ImportEntry>,
}

// ============================================================
// Export Entry
// ============================================================

#[derive(Debug, Clone)]
pub struct ExportEntry {
    /// Exported symbol name
    pub name: String,
    /// Offset within code section
    pub offset: u64,
    /// Ordinal (optional)
    pub ordinal: u16,
}

// ============================================================
// Symbol Entry
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SymbolType {
    Function = 0x01,
    Data     = 0x02,
    Section  = 0x03,
    External = 0x04,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SymbolBind {
    Local  = 0x00,
    Global = 0x01,
    Weak   = 0x02,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol name
    pub name: String,
    /// Symbol type
    pub sym_type: SymbolType,
    /// Binding
    pub bind: SymbolBind,
    /// Section index this symbol belongs to (0xFFFF = external)
    pub section_index: u16,
    /// Offset within section
    pub offset: u64,
    /// Size (0 if unknown)
    pub size: u64,
}

// ============================================================
// Relocation Entry
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RelocType {
    /// Absolute 64-bit address
    Abs64    = 0x01,
    /// PC-relative 32-bit offset
    Rel32    = 0x02,
    /// PC-relative 32-bit to PLT/IAT
    Plt32    = 0x03,
    /// 32-bit absolute (for data references)
    Abs32    = 0x04,
    /// High-adjust pair (PE specific)
    HighAdj  = 0x05,
}

#[derive(Debug, Clone)]
pub struct Relocation {
    /// Offset within the section where fixup is needed
    pub offset: u64,
    /// Relocation type
    pub reloc_type: RelocType,
    /// Symbol index this relocation refers to
    pub symbol_index: u32,
    /// Addend (signed)
    pub addend: i64,
}

// ============================================================
// Complete BIB Module (in-memory representation)
// ============================================================

#[derive(Debug, Clone)]
pub struct BibModule {
    pub header: BibHeader,
    pub sections: Vec<SectionHeader>,
    pub section_data: Vec<Vec<u8>>,

    // High-level tables (deserialized from sections)
    pub imports: Vec<ImportModule>,
    pub exports: Vec<ExportEntry>,
    pub symbols: Vec<Symbol>,
    pub relocations: Vec<Relocation>,
    pub metadata: Vec<(String, String)>,
}

impl BibModule {
    pub fn new(arch: Arch) -> Self {
        BibModule {
            header: BibHeader::new(arch),
            sections: Vec::new(),
            section_data: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            symbols: Vec::new(),
            relocations: Vec::new(),
            metadata: Vec::new(),
        }
    }

    /// Get code section data (first Code section)
    pub fn code(&self) -> Option<&[u8]> {
        for (i, sec) in self.sections.iter().enumerate() {
            if sec.section_type == SectionType::Code {
                return Some(&self.section_data[i]);
            }
        }
        None
    }

    /// Get data section data (first Data section)
    pub fn data(&self) -> Option<&[u8]> {
        for (i, sec) in self.sections.iter().enumerate() {
            if sec.section_type == SectionType::Data {
                return Some(&self.section_data[i]);
            }
        }
        None
    }

    /// Get read-only data section
    pub fn rodata(&self) -> Option<&[u8]> {
        for (i, sec) in self.sections.iter().enumerate() {
            if sec.section_type == SectionType::RoData {
                return Some(&self.section_data[i]);
            }
        }
        None
    }

    /// Find entry point symbol
    pub fn entry_symbol(&self) -> Option<&Symbol> {
        let idx = self.header.entry_point_symbol as usize;
        self.symbols.get(idx)
    }

    /// Find a symbol by name
    pub fn find_symbol(&self, name: &str) -> Option<(usize, &Symbol)> {
        self.symbols.iter().enumerate().find(|(_, s)| s.name == name)
    }

    /// Get all import DLL names
    pub fn import_dlls(&self) -> Vec<&str> {
        self.imports.iter().map(|m| m.name.as_str()).collect()
    }

    /// Total number of imported symbols
    pub fn import_count(&self) -> usize {
        self.imports.iter().map(|m| m.symbols.len()).sum()
    }
}

impl fmt::Display for BibModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== ADead-BIB Module ===")?;
        writeln!(f, "  Arch:       {}", self.header.arch)?;
        writeln!(f, "  Version:    {}", self.header.version)?;
        writeln!(f, "  Sections:   {}", self.sections.len())?;
        writeln!(f, "  Symbols:    {}", self.symbols.len())?;
        writeln!(f, "  Imports:    {} modules ({} symbols)",
            self.imports.len(), self.import_count())?;
        writeln!(f, "  Exports:    {}", self.exports.len())?;
        writeln!(f, "  Relocs:     {}", self.relocations.len())?;
        for (i, sec) in self.sections.iter().enumerate() {
            writeln!(f, "  Section[{}]: {} ({:?}, {} bytes)",
                i, sec.name_str(), sec.section_type, sec.size)?;
        }
        Ok(())
    }
}
