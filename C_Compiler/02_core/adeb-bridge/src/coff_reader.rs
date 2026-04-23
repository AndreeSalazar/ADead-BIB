// ============================================================
// COFF Object File Reader for ASM-BIB .obj imports
// ============================================================
// Parses x86-64 COFF .obj files (IMAGE_FILE_MACHINE_AMD64)
// and extracts all sections, symbols, and relocations needed
// for linking ASM functions into ADead-BIB PE output.
// ============================================================

use std::collections::HashMap;

// ── COFF Constants ──────────────────────────────────────────

const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;

// Symbol storage classes
const IMAGE_SYM_CLASS_EXTERNAL: u8 = 2;
const IMAGE_SYM_CLASS_STATIC: u8 = 3;
const IMAGE_SYM_CLASS_LABEL: u8 = 6;

// Section number special values
const IMAGE_SYM_UNDEFINED: i16 = 0;

// Relocation types (AMD64)
pub const IMAGE_REL_AMD64_ADDR64: u16 = 1;
pub const IMAGE_REL_AMD64_ADDR32: u16 = 2;
pub const IMAGE_REL_AMD64_ADDR32NB: u16 = 3;
pub const IMAGE_REL_AMD64_REL32: u16 = 4;
pub const IMAGE_REL_AMD64_REL32_1: u16 = 5;
pub const IMAGE_REL_AMD64_REL32_2: u16 = 6;
pub const IMAGE_REL_AMD64_REL32_3: u16 = 7;
pub const IMAGE_REL_AMD64_REL32_4: u16 = 8;
pub const IMAGE_REL_AMD64_REL32_5: u16 = 9;

// Section flags
pub const IMAGE_SCN_CNT_CODE: u32 = 0x00000020;
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 0x00000040;
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x00000080;
pub const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000;
pub const IMAGE_SCN_MEM_READ: u32 = 0x40000000;
pub const IMAGE_SCN_MEM_WRITE: u32 = 0x80000000;

// ── Data Structures ─────────────────────────────────────────

/// A parsed COFF object file from ASM-BIB
#[derive(Debug, Clone)]
pub struct CoffObject {
    pub machine: u16,
    pub sections: Vec<CoffSection>,
    pub symbols: Vec<CoffSymbol>,
    pub string_table: Vec<u8>,
}

/// A section within the COFF object
#[derive(Debug, Clone)]
pub struct CoffSection {
    pub name: String,
    pub virtual_size: u32,
    pub characteristics: u32,
    pub data: Vec<u8>,
    pub relocations: Vec<CoffRelocation>,
}

/// A relocation entry
#[derive(Debug, Clone)]
pub struct CoffRelocation {
    pub virtual_address: u32,
    pub symbol_index: u32,
    pub rel_type: u16,
}

/// A symbol table entry
#[derive(Debug, Clone)]
pub struct CoffSymbol {
    pub name: String,
    pub value: u32,
    pub section_number: i16,
    pub storage_class: u8,
    pub num_aux: u8,
    pub is_function: bool,
}

// ── Parsing Helpers ─────────────────────────────────────────

fn read_u16(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

fn read_u32(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        data[offset], data[offset + 1],
        data[offset + 2], data[offset + 3],
    ])
}

fn read_i16(data: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([data[offset], data[offset + 1]])
}

fn read_name_from_string_table(string_table: &[u8], offset: u32) -> String {
    let start = offset as usize;
    if start >= string_table.len() {
        return String::from("<invalid>");
    }
    let end = string_table[start..]
        .iter()
        .position(|&b| b == 0)
        .map(|p| start + p)
        .unwrap_or(string_table.len());
    String::from_utf8_lossy(&string_table[start..end]).to_string()
}

fn read_symbol_name(entry: &[u8], string_table: &[u8]) -> String {
    // First 4 bytes: if zero, next 4 bytes are offset into string table
    let first4 = read_u32(entry, 0);
    if first4 == 0 {
        let offset = read_u32(entry, 4);
        read_name_from_string_table(string_table, offset)
    } else {
        // Inline name (up to 8 bytes, null-padded)
        let end = entry[..8].iter().position(|&b| b == 0).unwrap_or(8);
        String::from_utf8_lossy(&entry[..end]).to_string()
    }
}

fn read_section_name(entry: &[u8], string_table: &[u8]) -> String {
    // Section names starting with '/' are offsets into string table
    if entry[0] == b'/' {
        let s = String::from_utf8_lossy(&entry[1..8]);
        let s = s.trim_end_matches('\0');
        if let Ok(offset) = s.parse::<u32>() {
            return read_name_from_string_table(string_table, offset);
        }
    }
    let end = entry[..8].iter().position(|&b| b == 0).unwrap_or(8);
    String::from_utf8_lossy(&entry[..end]).to_string()
}

// ── Main Parser ─────────────────────────────────────────────

impl CoffObject {
    /// Parse a COFF .obj file from raw bytes
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 20 {
            return Err("COFF file too small for header".into());
        }

        let machine = read_u16(data, 0);
        if machine != IMAGE_FILE_MACHINE_AMD64 {
            return Err(format!(
                "Unsupported COFF machine type: 0x{:04X} (expected AMD64 0x{:04X})",
                machine, IMAGE_FILE_MACHINE_AMD64
            ));
        }

        let num_sections = read_u16(data, 2) as usize;
        let symbol_table_offset = read_u32(data, 8) as usize;
        let num_symbols = read_u32(data, 12) as usize;
        let optional_header_size = read_u16(data, 16) as usize;

        // Read string table (immediately after symbol table)
        let string_table_offset = symbol_table_offset + num_symbols * 18;
        let string_table = if string_table_offset < data.len() {
            let st_size = if string_table_offset + 4 <= data.len() {
                read_u32(data, string_table_offset) as usize
            } else {
                0
            };
            if st_size > 4 && string_table_offset + st_size <= data.len() {
                data[string_table_offset..string_table_offset + st_size].to_vec()
            } else {
                vec![0; 4]
            }
        } else {
            vec![0; 4]
        };

        // Read symbols
        let mut symbols = Vec::with_capacity(num_symbols);
        let mut sym_offset = symbol_table_offset;
        let mut sym_idx = 0;
        while sym_idx < num_symbols {
            if sym_offset + 18 > data.len() {
                break;
            }
            let entry = &data[sym_offset..sym_offset + 18];
            let name = read_symbol_name(entry, &string_table);
            let value = read_u32(entry, 8);
            let section_number = read_i16(entry, 12);
            let _type_field = read_u16(entry, 14);
            let storage_class = entry[16];
            let num_aux = entry[17];

            let is_function = (_type_field >> 8) == 0x20 // DTYPE_FUNCTION
                || (storage_class == IMAGE_SYM_CLASS_EXTERNAL && section_number > 0);

            symbols.push(CoffSymbol {
                name,
                value,
                section_number,
                storage_class,
                num_aux,
                is_function,
            });

            // Skip auxiliary symbol records
            sym_offset += 18 * (1 + num_aux as usize);
            sym_idx += 1 + num_aux as usize;
        }

        // Read sections
        let section_header_offset = 20 + optional_header_size;
        let mut sections = Vec::with_capacity(num_sections);

        for i in 0..num_sections {
            let sh_off = section_header_offset + i * 40;
            if sh_off + 40 > data.len() {
                break;
            }

            let name = read_section_name(&data[sh_off..sh_off + 8], &string_table);
            let virtual_size = read_u32(data, sh_off + 8);
            let raw_data_size = read_u32(data, sh_off + 16) as usize;
            let raw_data_ptr = read_u32(data, sh_off + 20) as usize;
            let reloc_ptr = read_u32(data, sh_off + 24) as usize;
            let num_relocs = read_u16(data, sh_off + 32) as usize;
            let characteristics = read_u32(data, sh_off + 36);

            // Read section data
            let section_data = if raw_data_ptr > 0 && raw_data_ptr + raw_data_size <= data.len() {
                data[raw_data_ptr..raw_data_ptr + raw_data_size].to_vec()
            } else {
                Vec::new()
            };

            // Read relocations
            let mut relocations = Vec::with_capacity(num_relocs);
            for r in 0..num_relocs {
                let r_off = reloc_ptr + r * 10;
                if r_off + 10 > data.len() {
                    break;
                }
                relocations.push(CoffRelocation {
                    virtual_address: read_u32(data, r_off),
                    symbol_index: read_u32(data, r_off + 4),
                    rel_type: read_u16(data, r_off + 8),
                });
            }

            sections.push(CoffSection {
                name,
                virtual_size,
                characteristics,
                data: section_data,
                relocations,
            });
        }

        Ok(CoffObject {
            machine,
            sections,
            symbols,
            string_table,
        })
    }

    /// Load and parse a COFF .obj from a file path
    pub fn from_file(path: &str) -> Result<Self, String> {
        let data = std::fs::read(path)
            .map_err(|e| format!("Cannot read COFF file '{}': {}", path, e))?;
        Self::parse(&data)
    }

    /// Get the .text section (code)
    pub fn text_section(&self) -> Option<&CoffSection> {
        self.sections.iter().find(|s| s.name == ".text")
    }

    /// Get the .data section (initialized data)
    pub fn data_section(&self) -> Option<&CoffSection> {
        self.sections.iter().find(|s| s.name == ".data")
    }

    /// Get the .rdata section (read-only data)
    pub fn rdata_section(&self) -> Option<&CoffSection> {
        self.sections.iter().find(|s| s.name == ".rdata")
    }

    /// Get all exported (EXTERNAL) symbols with a section
    pub fn exported_symbols(&self) -> Vec<&CoffSymbol> {
        self.symbols
            .iter()
            .filter(|s| {
                s.storage_class == IMAGE_SYM_CLASS_EXTERNAL
                    && s.section_number > 0
            })
            .collect()
    }

    /// Get all external (undefined) symbols that need resolving
    pub fn undefined_symbols(&self) -> Vec<&CoffSymbol> {
        self.symbols
            .iter()
            .filter(|s| {
                s.storage_class == IMAGE_SYM_CLASS_EXTERNAL
                    && s.section_number == IMAGE_SYM_UNDEFINED
                    && !s.name.is_empty()
            })
            .collect()
    }

    /// Build a map: symbol_name → offset_in_text_section
    pub fn symbol_offsets(&self) -> HashMap<String, u32> {
        let mut map = HashMap::new();
        for sym in &self.symbols {
            if sym.section_number > 0 && sym.storage_class == IMAGE_SYM_CLASS_EXTERNAL {
                map.insert(sym.name.clone(), sym.value);
            }
        }
        map
    }

    /// Summary string for diagnostics
    pub fn summary(&self) -> String {
        let exported = self.exported_symbols();
        let undefined = self.undefined_symbols();
        let text_size = self.text_section().map(|s| s.data.len()).unwrap_or(0);
        let data_size = self.data_section().map(|s| s.data.len()).unwrap_or(0);

        format!(
            "COFF Object: {} sections, {} symbols\n\
             .text: {} bytes, .data: {} bytes\n\
             Exported: {}\n\
             Undefined (needs linking): {}",
            self.sections.len(),
            self.symbols.len(),
            text_size,
            data_size,
            exported.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join(", "),
            undefined.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join(", "),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_invalid_machine() {
        // Minimal header with wrong machine type
        let mut data = vec![0u8; 20];
        data[0] = 0x4C; data[1] = 0x01; // i386
        let result = CoffObject::parse(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported COFF machine type"));
    }

    #[test]
    fn test_reject_too_small() {
        let data = vec![0u8; 10];
        let result = CoffObject::parse(&data);
        assert!(result.is_err());
    }
}
