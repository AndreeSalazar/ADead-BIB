// ============================================================
// ADead PE Backend — Windows PE/PE32+ Generator
// ============================================================
// Translates ADead-BIB modules into valid Windows PE executables.
//
// Generates:
//   DOS Header + Stub
//   PE Signature
//   COFF Header
//   Optional Header (PE32+)
//   Section Table (.text, .rdata, .data, .idata, .reloc)
//   Section Data
//
// Supports:
//   - Multiple DLL imports (kernel32, user32, d3d12, msvcrt, etc.)
//   - Relocations (.reloc section)
//   - Console (CUI) and GUI subsystems
//   - ASLR, DEP/NX, Large Address Aware
//   - Custom image base, alignment, stack/heap sizes
//
// Compatible with: Windows 7+ x64 loader (ntdll.dll)
// ============================================================

use crate::bib::format::*;
use super::{BinaryBackend, BackendConfig, OutputFormat, Subsystem};

// ============================================================
// PE Constants
// ============================================================

const DOS_HEADER_SIZE: usize = 64;
const PE_SIGNATURE: &[u8; 4] = b"PE\0\0";
const COFF_HEADER_SIZE: usize = 20;
const OPTIONAL_HEADER_SIZE: usize = 240; // PE32+ (64-bit)
const SECTION_HEADER_SIZE: usize = 40;
const HEADERS_ALIGNED: usize = 0x400; // SizeOfHeaders aligned to FileAlignment

// Machine types
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;

// PE32+ magic
const PE32_PLUS_MAGIC: u16 = 0x020B;

// Section characteristics
const IMAGE_SCN_CNT_CODE: u32              = 0x00000020;
const IMAGE_SCN_CNT_INITIALIZED_DATA: u32  = 0x00000040;
const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x00000080;
const IMAGE_SCN_MEM_EXECUTE: u32           = 0x20000000;
const IMAGE_SCN_MEM_READ: u32              = 0x40000000;
const IMAGE_SCN_MEM_WRITE: u32             = 0x80000000;
const IMAGE_SCN_MEM_DISCARDABLE: u32       = 0x02000000;

// DLL Characteristics
const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 0x0040;
const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16    = 0x0100;
const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u16 = 0x8000;

// Data directory indices
const IMAGE_DIRECTORY_ENTRY_IMPORT: usize  = 1;
const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5;
const IMAGE_DIRECTORY_ENTRY_IAT: usize     = 12;

// ============================================================
// PE Section descriptor (internal)
// ============================================================

struct PeSection {
    name: [u8; 8],
    virtual_size: u32,
    virtual_address: u32,
    raw_size: u32,
    raw_offset: u32,
    characteristics: u32,
    data: Vec<u8>,
}

impl PeSection {
    fn new(name: &str, chars: u32) -> Self {
        let mut n = [0u8; 8];
        let bytes = name.as_bytes();
        let len = bytes.len().min(8);
        n[..len].copy_from_slice(&bytes[..len]);
        PeSection {
            name: n,
            virtual_size: 0,
            virtual_address: 0,
            raw_size: 0,
            raw_offset: 0,
            characteristics: chars,
            data: Vec::new(),
        }
    }
}

// ============================================================
// Import table builder
// ============================================================

struct ImportTableBuilder {
    modules: Vec<ImportModuleLayout>,
}

struct ImportModuleLayout {
    name: String,
    symbols: Vec<String>,
    hints: Vec<u16>,
}

struct ImportTableResult {
    /// Complete .idata section bytes
    data: Vec<u8>,
    /// RVA of the Import Directory Table (start of .idata)
    idt_rva: u32,
    /// Size of the Import Directory Table
    idt_size: u32,
    /// RVA of the IAT
    iat_rva: u32,
    /// Size of the IAT
    iat_size: u32,
}

impl ImportTableBuilder {
    fn new() -> Self {
        ImportTableBuilder { modules: Vec::new() }
    }

    fn add_module(&mut self, name: &str, symbols: &[(&str, u16)]) {
        self.modules.push(ImportModuleLayout {
            name: name.to_string(),
            symbols: symbols.iter().map(|(s, _)| s.to_string()).collect(),
            hints: symbols.iter().map(|(_, h)| *h).collect(),
        });
    }

    /// Build the .idata section at the given RVA base
    fn build(&self, idata_rva: u32) -> ImportTableResult {
        if self.modules.is_empty() {
            return ImportTableResult {
                data: Vec::new(),
                idt_rva: 0,
                idt_size: 0,
                iat_rva: 0,
                iat_size: 0,
            };
        }

        let module_count = self.modules.len();
        let _total_symbols: usize = self.modules.iter().map(|m| m.symbols.len()).sum();

        // Layout calculation:
        // IDT: (module_count + 1) * 20 bytes (null terminator entry)
        let idt_size = (module_count + 1) * 20;

        // ILT: one array per module, each has (sym_count + 1) * 8 bytes
        let mut ilt_offsets = Vec::new();
        let mut ilt_total = 0usize;
        for m in &self.modules {
            ilt_offsets.push(idt_size + ilt_total);
            ilt_total += (m.symbols.len() + 1) * 8;
        }

        // IAT: same layout as ILT (initially identical, loader overwrites)
        let iat_start = idt_size + ilt_total;
        let mut iat_offsets = Vec::new();
        let mut iat_total = 0usize;
        for m in &self.modules {
            iat_offsets.push(iat_start + iat_total);
            iat_total += (m.symbols.len() + 1) * 8;
        }

        // Strings area: DLL names + Hint/Name entries
        let strings_start = iat_start + iat_total;
        let mut strings = Vec::new();
        let mut dll_name_offsets = Vec::new();
        let mut hint_name_offsets: Vec<Vec<usize>> = Vec::new();

        for m in &self.modules {
            // DLL name
            dll_name_offsets.push(strings_start + strings.len());
            strings.extend_from_slice(m.name.as_bytes());
            strings.push(0); // null terminator
            // Align to 2
            if strings.len() % 2 != 0 { strings.push(0); }

            // Hint/Name entries for each symbol
            let mut sym_offsets = Vec::new();
            for (i, sym) in m.symbols.iter().enumerate() {
                sym_offsets.push(strings_start + strings.len());
                // Hint (2 bytes)
                let hint = m.hints[i];
                strings.extend_from_slice(&hint.to_le_bytes());
                // Name + null
                strings.extend_from_slice(sym.as_bytes());
                strings.push(0);
                // Align to 2
                if strings.len() % 2 != 0 { strings.push(0); }
            }
            hint_name_offsets.push(sym_offsets);
        }

        let total_size = strings_start + strings.len();
        let mut data = vec![0u8; total_size];

        // Write IDT entries
        for (mi, m) in self.modules.iter().enumerate() {
            let base = mi * 20;
            // OriginalFirstThunk (ILT RVA)
            let ilt_rva = idata_rva + ilt_offsets[mi] as u32;
            data[base..base+4].copy_from_slice(&ilt_rva.to_le_bytes());
            // TimeDateStamp = 0
            // ForwarderChain = 0
            // Name RVA
            let name_rva = idata_rva + dll_name_offsets[mi] as u32;
            data[base+12..base+16].copy_from_slice(&name_rva.to_le_bytes());
            // FirstThunk (IAT RVA)
            let iat_rva = idata_rva + iat_offsets[mi] as u32;
            data[base+16..base+20].copy_from_slice(&iat_rva.to_le_bytes());
        }
        // Null terminator IDT entry (already zeroed)

        // Write ILT entries
        for (mi, _m) in self.modules.iter().enumerate() {
            for si in 0..self.modules[mi].symbols.len() {
                let off = ilt_offsets[mi] + si * 8;
                let hint_rva = idata_rva + hint_name_offsets[mi][si] as u32;
                data[off..off+8].copy_from_slice(&(hint_rva as u64).to_le_bytes());
            }
            // Null terminator (already zeroed)
        }

        // Write IAT entries (identical to ILT initially)
        for (mi, m) in self.modules.iter().enumerate() {
            for (si, _) in m.symbols.iter().enumerate() {
                let off = iat_offsets[mi] + si * 8;
                let hint_rva = idata_rva + hint_name_offsets[mi][si] as u32;
                data[off..off+8].copy_from_slice(&(hint_rva as u64).to_le_bytes());
            }
        }

        // Write strings area
        data[strings_start..strings_start + strings.len()].copy_from_slice(&strings);

        ImportTableResult {
            data,
            idt_rva: idata_rva,
            idt_size: idt_size as u32,
            iat_rva: idata_rva + iat_start as u32,
            iat_size: iat_total as u32,
        }
    }
}

// ============================================================
// PE Backend
// ============================================================

pub struct PeBackend;

impl PeBackend {
    pub fn new() -> Self {
        PeBackend
    }
}

impl BinaryBackend for PeBackend {
    fn name(&self) -> &str {
        "PE (Windows)"
    }

    fn supported_formats(&self) -> &[OutputFormat] {
        &[OutputFormat::PeExe, OutputFormat::PeDll]
    }

    fn validate(&self, module: &BibModule, config: &BackendConfig) -> Result<(), String> {
        if module.header.arch != Arch::X86_64 {
            return Err(format!("PE backend only supports x86-64, got {}", module.header.arch));
        }
        if !matches!(config.format, OutputFormat::PeExe | OutputFormat::PeDll) {
            return Err("PE backend only supports PeExe and PeDll formats".into());
        }
        if module.code().is_none() || module.code().unwrap().is_empty() {
            return Err("No code section in BIB module".into());
        }
        Ok(())
    }

    fn translate(&self, module: &BibModule, config: &BackendConfig) -> Result<Vec<u8>, String> {
        self.validate(module, config)?;

        let file_align = config.file_alignment as usize;
        let section_align = config.section_alignment as usize;

        // ============================================================
        // 1. Prepare sections
        // ============================================================

        let code = module.code().unwrap_or(&[]);
        let rodata = module.rodata().unwrap_or(&[]);
        let data_sec = module.data().unwrap_or(&[]);

        let mut sections: Vec<PeSection> = Vec::new();

        // .text — code
        let mut text = PeSection::new(".text",
            IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ);
        text.data = code.to_vec();
        sections.push(text);

        // .rdata — read-only data + import strings
        let has_rodata = !rodata.is_empty();
        if has_rodata {
            let mut rdata = PeSection::new(".rdata",
                IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ);
            rdata.data = rodata.to_vec();
            sections.push(rdata);
        }

        // .data — read/write data
        if !data_sec.is_empty() {
            let mut dsec = PeSection::new(".data",
                IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE);
            dsec.data = data_sec.to_vec();
            sections.push(dsec);
        }

        // .idata — imports (placeholder, will be filled)
        let idata_index = sections.len();
        let idata = PeSection::new(".idata",
            IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE);
        sections.push(idata);

        // .reloc — base relocations (placeholder)
        let reloc_index = sections.len();
        let reloc_sec = PeSection::new(".reloc",
            IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_DISCARDABLE);
        sections.push(reloc_sec);

        // ============================================================
        // 2. Compute layout (VAs and file offsets)
        // ============================================================

        let num_sections = sections.len();
        let headers_size = DOS_HEADER_SIZE + 4 + COFF_HEADER_SIZE
            + OPTIONAL_HEADER_SIZE + num_sections * SECTION_HEADER_SIZE;
        let headers_aligned = align_up(headers_size, file_align);

        let mut current_va = align_up(headers_aligned, section_align) as u32;
        let mut current_file = headers_aligned;

        for sec in &mut sections {
            sec.virtual_address = current_va;
            sec.virtual_size = if sec.data.is_empty() { 0 } else { sec.data.len() as u32 };
            sec.raw_offset = current_file as u32;
            sec.raw_size = align_up(sec.data.len(), file_align) as u32;
            current_va += align_up(sec.data.len().max(1), section_align) as u32;
            current_file += sec.raw_size as usize;
        }

        // ============================================================
        // 3. Build import table
        // ============================================================

        let idata_va = sections[idata_index].virtual_address;
        let mut import_builder = ImportTableBuilder::new();

        for imp_mod in &module.imports {
            let syms: Vec<(&str, u16)> = imp_mod.symbols.iter()
                .map(|s| (s.symbol.as_str(), s.hint))
                .collect();
            import_builder.add_module(&imp_mod.name, &syms);
        }

        let import_result = import_builder.build(idata_va);
        sections[idata_index].data = import_result.data;
        sections[idata_index].virtual_size = sections[idata_index].data.len() as u32;
        sections[idata_index].raw_size = align_up(sections[idata_index].data.len(), file_align) as u32;

        // ============================================================
        // 4. Build base relocation table (minimal — empty if no relocs)
        // ============================================================

        // For now, generate an empty .reloc (8-byte block header with 0 entries)
        // This satisfies the PE loader requirement for ASLR
        if config.dynamic_base {
            let text_va = sections[0].virtual_address;
            let mut reloc_data = Vec::new();
            // Block header: PageRVA + BlockSize (minimum 8 bytes)
            reloc_data.extend_from_slice(&text_va.to_le_bytes());
            reloc_data.extend_from_slice(&8u32.to_le_bytes()); // empty block
            sections[reloc_index].data = reloc_data;
        }
        sections[reloc_index].virtual_size = sections[reloc_index].data.len() as u32;
        sections[reloc_index].raw_size = align_up(sections[reloc_index].data.len(), file_align) as u32;

        // ============================================================
        // 5. Recalculate layout after filling import/reloc
        // ============================================================

        current_va = align_up(headers_aligned, section_align) as u32;
        current_file = headers_aligned;

        for sec in &mut sections {
            sec.virtual_address = current_va;
            sec.raw_offset = current_file as u32;
            sec.raw_size = align_up(sec.data.len().max(1), file_align) as u32;
            sec.virtual_size = sec.data.len() as u32;
            current_va += align_up(sec.data.len().max(1), section_align) as u32;
            current_file += sec.raw_size as usize;
        }

        let size_of_image = current_va;
        let total_file_size = current_file;

        // Recalculate import RVAs after relayout
        let idata_va_final = sections[idata_index].virtual_address;
        if !module.imports.is_empty() {
            let import_result = import_builder.build(idata_va_final);
            sections[idata_index].data = import_result.data;
        }

        // ============================================================
        // 6. Resolve entry point
        // ============================================================

        let entry_rva = if let Some(sym) = module.entry_symbol() {
            sections[0].virtual_address + sym.offset as u32
        } else {
            sections[0].virtual_address // default: start of .text
        };

        // ============================================================
        // 7. Build PE image
        // ============================================================

        let mut image = vec![0u8; total_file_size];

        // --- DOS Header (64 bytes) ---
        image[0] = 0x4D; // 'M'
        image[1] = 0x5A; // 'Z'
        image[0x3C..0x40].copy_from_slice(&(DOS_HEADER_SIZE as u32).to_le_bytes()); // e_lfanew

        // --- PE Signature ---
        let pe_off = DOS_HEADER_SIZE;
        image[pe_off..pe_off+4].copy_from_slice(PE_SIGNATURE);

        // --- COFF Header (20 bytes) ---
        let coff_off = pe_off + 4;
        image[coff_off..coff_off+2].copy_from_slice(&IMAGE_FILE_MACHINE_AMD64.to_le_bytes());
        image[coff_off+2..coff_off+4].copy_from_slice(&(num_sections as u16).to_le_bytes());
        // TimeDateStamp, PointerToSymbolTable, NumberOfSymbols = 0
        image[coff_off+16..coff_off+18].copy_from_slice(&(OPTIONAL_HEADER_SIZE as u16).to_le_bytes());
        // Characteristics: EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE
        let mut file_chars: u16 = 0x0002 | 0x0020; // EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE
        if matches!(config.format, OutputFormat::PeDll) {
            file_chars |= 0x2000; // DLL
        }
        image[coff_off+18..coff_off+20].copy_from_slice(&file_chars.to_le_bytes());

        // --- Optional Header PE32+ (240 bytes) ---
        let opt_off = coff_off + COFF_HEADER_SIZE;
        image[opt_off..opt_off+2].copy_from_slice(&PE32_PLUS_MAGIC.to_le_bytes());
        image[opt_off+2] = 14; // MajorLinkerVersion
        image[opt_off+3] = 0;  // MinorLinkerVersion

        // SizeOfCode
        let code_size = sections[0].raw_size;
        image[opt_off+4..opt_off+8].copy_from_slice(&code_size.to_le_bytes());

        // SizeOfInitializedData
        let init_data_size: u32 = sections.iter()
            .filter(|s| s.characteristics & IMAGE_SCN_CNT_INITIALIZED_DATA != 0)
            .map(|s| s.raw_size)
            .sum();
        image[opt_off+8..opt_off+12].copy_from_slice(&init_data_size.to_le_bytes());

        // AddressOfEntryPoint
        image[opt_off+16..opt_off+20].copy_from_slice(&entry_rva.to_le_bytes());

        // BaseOfCode
        image[opt_off+20..opt_off+24].copy_from_slice(&sections[0].virtual_address.to_le_bytes());

        // ImageBase (64-bit)
        image[opt_off+24..opt_off+32].copy_from_slice(&config.image_base.to_le_bytes());

        // SectionAlignment
        image[opt_off+32..opt_off+36].copy_from_slice(&config.section_alignment.to_le_bytes());

        // FileAlignment
        image[opt_off+36..opt_off+40].copy_from_slice(&config.file_alignment.to_le_bytes());

        // MajorOperatingSystemVersion: 6
        image[opt_off+40..opt_off+42].copy_from_slice(&6u16.to_le_bytes());
        // MajorSubsystemVersion: 6
        image[opt_off+48..opt_off+50].copy_from_slice(&6u16.to_le_bytes());

        // SizeOfImage
        image[opt_off+56..opt_off+60].copy_from_slice(&size_of_image.to_le_bytes());

        // SizeOfHeaders
        image[opt_off+60..opt_off+64].copy_from_slice(&(headers_aligned as u32).to_le_bytes());

        // Subsystem
        let subsystem: u16 = match config.subsystem {
            Subsystem::Console => 3,
            Subsystem::Gui => 2,
            Subsystem::Native => 1,
        };
        image[opt_off+68..opt_off+70].copy_from_slice(&subsystem.to_le_bytes());

        // DllCharacteristics
        let mut dll_chars: u16 = IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE;
        if config.dynamic_base { dll_chars |= IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE; }
        if config.nx_compat { dll_chars |= IMAGE_DLLCHARACTERISTICS_NX_COMPAT; }
        image[opt_off+70..opt_off+72].copy_from_slice(&dll_chars.to_le_bytes());

        // Stack/Heap sizes
        image[opt_off+72..opt_off+80].copy_from_slice(&config.stack_reserve.to_le_bytes());
        image[opt_off+80..opt_off+88].copy_from_slice(&config.stack_commit.to_le_bytes());
        image[opt_off+88..opt_off+96].copy_from_slice(&config.heap_reserve.to_le_bytes());
        image[opt_off+96..opt_off+104].copy_from_slice(&config.heap_commit.to_le_bytes());

        // NumberOfRvaAndSizes: 16
        image[opt_off+108..opt_off+112].copy_from_slice(&16u32.to_le_bytes());

        // --- Data Directories (16 entries × 8 bytes = 128 bytes) ---
        let dd_off = opt_off + 112;

        // [1] Import Table
        if !module.imports.is_empty() {
            let import_result_final = import_builder.build(sections[idata_index].virtual_address);
            image[dd_off + IMAGE_DIRECTORY_ENTRY_IMPORT * 8..dd_off + IMAGE_DIRECTORY_ENTRY_IMPORT * 8 + 4]
                .copy_from_slice(&import_result_final.idt_rva.to_le_bytes());
            image[dd_off + IMAGE_DIRECTORY_ENTRY_IMPORT * 8 + 4..dd_off + IMAGE_DIRECTORY_ENTRY_IMPORT * 8 + 8]
                .copy_from_slice(&import_result_final.idt_size.to_le_bytes());

            // [12] IAT
            image[dd_off + IMAGE_DIRECTORY_ENTRY_IAT * 8..dd_off + IMAGE_DIRECTORY_ENTRY_IAT * 8 + 4]
                .copy_from_slice(&import_result_final.iat_rva.to_le_bytes());
            image[dd_off + IMAGE_DIRECTORY_ENTRY_IAT * 8 + 4..dd_off + IMAGE_DIRECTORY_ENTRY_IAT * 8 + 8]
                .copy_from_slice(&import_result_final.iat_size.to_le_bytes());
        }

        // [5] Base Relocation
        if config.dynamic_base && !sections[reloc_index].data.is_empty() {
            image[dd_off + IMAGE_DIRECTORY_ENTRY_BASERELOC * 8..dd_off + IMAGE_DIRECTORY_ENTRY_BASERELOC * 8 + 4]
                .copy_from_slice(&sections[reloc_index].virtual_address.to_le_bytes());
            image[dd_off + IMAGE_DIRECTORY_ENTRY_BASERELOC * 8 + 4..dd_off + IMAGE_DIRECTORY_ENTRY_BASERELOC * 8 + 8]
                .copy_from_slice(&(sections[reloc_index].data.len() as u32).to_le_bytes());
        }

        // --- Section Headers ---
        let sec_hdr_off = opt_off + OPTIONAL_HEADER_SIZE;
        for (i, sec) in sections.iter().enumerate() {
            let off = sec_hdr_off + i * SECTION_HEADER_SIZE;
            image[off..off+8].copy_from_slice(&sec.name);
            image[off+8..off+12].copy_from_slice(&sec.virtual_size.to_le_bytes());
            image[off+12..off+16].copy_from_slice(&sec.virtual_address.to_le_bytes());
            image[off+16..off+20].copy_from_slice(&sec.raw_size.to_le_bytes());
            image[off+20..off+24].copy_from_slice(&sec.raw_offset.to_le_bytes());
            // PointerToRelocations, PointerToLinenumbers, NumberOfRelocations, NumberOfLinenumbers = 0
            image[off+36..off+40].copy_from_slice(&sec.characteristics.to_le_bytes());
        }

        // --- Section Data ---
        for sec in &sections {
            let start = sec.raw_offset as usize;
            let end = start + sec.data.len();
            if end <= image.len() {
                image[start..end].copy_from_slice(&sec.data);
            }
        }

        Ok(image)
    }
}

// ============================================================
// Utility
// ============================================================

fn align_up(value: usize, alignment: usize) -> usize {
    if alignment == 0 { return value; }
    (value + alignment - 1) & !(alignment - 1)
}
