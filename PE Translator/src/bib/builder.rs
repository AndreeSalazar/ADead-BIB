// ============================================================
// ADead-BIB Builder — High-level API to construct BIB modules
// ============================================================
// Usage:
//   let module = BibBuilder::new(Arch::X86_64)
//       .add_code(&machine_code)
//       .add_rodata(&string_constants)
//       .add_import("kernel32.dll", "ExitProcess", 0)
//       .add_import("d3d12.dll", "D3D12CreateDevice", 0)
//       .set_entry("main")
//       .build();
// ============================================================

use super::format::*;

pub struct BibBuilder {
    arch: Arch,
    code: Vec<u8>,
    data: Vec<u8>,
    rodata: Vec<u8>,
    bss_size: u64,
    imports: Vec<ImportModule>,
    exports: Vec<ExportEntry>,
    symbols: Vec<Symbol>,
    relocations: Vec<Relocation>,
    metadata: Vec<(String, String)>,
    entry_name: Option<String>,
}

impl BibBuilder {
    pub fn new(arch: Arch) -> Self {
        BibBuilder {
            arch,
            code: Vec::new(),
            data: Vec::new(),
            rodata: Vec::new(),
            bss_size: 0,
            imports: Vec::new(),
            exports: Vec::new(),
            symbols: Vec::new(),
            relocations: Vec::new(),
            metadata: Vec::new(),
            entry_name: None,
        }
    }

    /// Set raw machine code
    pub fn code(mut self, code: &[u8]) -> Self {
        self.code = code.to_vec();
        self
    }

    /// Set initialized data
    pub fn data(mut self, data: &[u8]) -> Self {
        self.data = data.to_vec();
        self
    }

    /// Set read-only data (constants, strings)
    pub fn rodata(mut self, rodata: &[u8]) -> Self {
        self.rodata = rodata.to_vec();
        self
    }

    /// Set BSS (uninitialized data) size
    pub fn bss(mut self, size: u64) -> Self {
        self.bss_size = size;
        self
    }

    /// Add an import from a module (DLL/SO)
    pub fn import(mut self, module: &str, symbol: &str, hint: u16) -> Self {
        // Find or create module
        let existing = self.imports.iter_mut().find(|m| m.name == module);
        if let Some(m) = existing {
            m.symbols.push(ImportEntry {
                module: module.to_string(),
                symbol: symbol.to_string(),
                hint,
            });
        } else {
            self.imports.push(ImportModule {
                name: module.to_string(),
                symbols: vec![ImportEntry {
                    module: module.to_string(),
                    symbol: symbol.to_string(),
                    hint,
                }],
            });
        }

        // Also add as external symbol
        self.symbols.push(Symbol {
            name: symbol.to_string(),
            sym_type: SymbolType::External,
            bind: SymbolBind::Global,
            section_index: 0xFFFF,
            offset: 0,
            size: 0,
        });

        self
    }

    /// Add an export
    pub fn export(mut self, name: &str, offset: u64, ordinal: u16) -> Self {
        self.exports.push(ExportEntry {
            name: name.to_string(),
            offset,
            ordinal,
        });
        self
    }

    /// Add a function symbol in the code section
    pub fn function(mut self, name: &str, offset: u64, size: u64) -> Self {
        self.symbols.push(Symbol {
            name: name.to_string(),
            sym_type: SymbolType::Function,
            bind: SymbolBind::Global,
            section_index: 0, // code section
            offset,
            size,
        });
        self
    }

    /// Add a data symbol
    pub fn data_symbol(mut self, name: &str, section_index: u16, offset: u64, size: u64) -> Self {
        self.symbols.push(Symbol {
            name: name.to_string(),
            sym_type: SymbolType::Data,
            bind: SymbolBind::Global,
            section_index,
            offset,
            size,
        });
        self
    }

    /// Add a relocation
    pub fn relocation(mut self, offset: u64, reloc_type: RelocType, symbol_index: u32, addend: i64) -> Self {
        self.relocations.push(Relocation {
            offset,
            reloc_type,
            symbol_index,
            addend,
        });
        self
    }

    /// Add metadata key-value pair
    pub fn meta(mut self, key: &str, value: &str) -> Self {
        self.metadata.push((key.to_string(), value.to_string()));
        self
    }

    /// Set entry point function name
    pub fn entry(mut self, name: &str) -> Self {
        self.entry_name = Some(name.to_string());
        self
    }

    /// Build the BibModule
    pub fn build(self) -> BibModule {
        let mut module = BibModule::new(self.arch);

        // Resolve entry point symbol index
        if let Some(ref entry_name) = self.entry_name {
            for (i, sym) in self.symbols.iter().enumerate() {
                if sym.name == *entry_name {
                    module.header.entry_point_symbol = i as u64;
                    break;
                }
            }
        }

        // Build sections
        let mut section_idx = 0u32;

        // Code section
        if !self.code.is_empty() {
            let mut sec = SectionHeader::new(".code", SectionType::Code);
            sec.flags = section_flags::READABLE | section_flags::EXECUTABLE;
            sec.size = self.code.len() as u64;
            sec.alignment = 16;
            module.sections.push(sec);
            module.section_data.push(self.code);
            section_idx += 1;
        }

        // Data section
        if !self.data.is_empty() {
            let mut sec = SectionHeader::new(".data", SectionType::Data);
            sec.flags = section_flags::READABLE | section_flags::WRITABLE;
            sec.size = self.data.len() as u64;
            sec.alignment = 16;
            module.sections.push(sec);
            module.section_data.push(self.data);
            section_idx += 1;
        }

        // RoData section
        if !self.rodata.is_empty() {
            let mut sec = SectionHeader::new(".rodata", SectionType::RoData);
            sec.flags = section_flags::READABLE;
            sec.size = self.rodata.len() as u64;
            sec.alignment = 16;
            module.sections.push(sec);
            module.section_data.push(self.rodata);
            section_idx += 1;
        }

        // BSS section
        if self.bss_size > 0 {
            let mut sec = SectionHeader::new(".bss", SectionType::Bss);
            sec.flags = section_flags::READABLE | section_flags::WRITABLE;
            sec.size = self.bss_size;
            sec.alignment = 16;
            module.sections.push(sec);
            module.section_data.push(Vec::new()); // no file data
            section_idx += 1;
        }

        let _ = section_idx;

        // Store high-level tables
        module.imports = self.imports;
        module.exports = self.exports;
        module.symbols = self.symbols;
        module.relocations = self.relocations;
        module.metadata = self.metadata;

        // Update header
        module.header.section_count = module.sections.len() as u32;

        module
    }
}
