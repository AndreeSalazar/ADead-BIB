// ============================================================
// ADead-BIB Reader — Deserialize .bib file into BibModule
// ============================================================

use super::format::*;

/// Read a BibModule from raw bytes
pub fn deserialize(data: &[u8]) -> Result<BibModule, String> {
    // 1. Parse header
    let header = BibHeader::from_bytes(data)
        .ok_or_else(|| "Invalid BIB header or magic".to_string())?;

    if !header.is_valid() {
        return Err("BIB header validation failed".to_string());
    }

    let mut module = BibModule::new(header.arch);
    module.header = header.clone();

    // 2. Parse section headers
    let table_off = header.section_table_offset as usize;
    for i in 0..header.section_count as usize {
        let off = table_off + i * SECTION_HEADER_SIZE;
        if off + SECTION_HEADER_SIZE > data.len() {
            return Err(format!("Section header {} out of bounds", i));
        }
        let sec = SectionHeader::from_bytes(&data[off..off + SECTION_HEADER_SIZE])
            .ok_or_else(|| format!("Invalid section header {}", i))?;
        module.sections.push(sec);
    }

    // 3. Read section data
    for sec in &module.sections {
        let start = sec.offset as usize;
        let end = start + sec.size as usize;
        if end > data.len() {
            return Err(format!("Section '{}' data out of bounds", sec.name_str()));
        }
        module.section_data.push(data[start..end].to_vec());
    }

    // 4. Deserialize high-level tables from special sections
    for (i, sec) in module.sections.iter().enumerate() {
        match sec.section_type {
            SectionType::Import => {
                module.imports = deserialize_imports(&module.section_data[i])?;
            }
            SectionType::Symbol => {
                module.symbols = deserialize_symbols(&module.section_data[i])?;
            }
            SectionType::Reloc => {
                module.relocations = deserialize_relocations(&module.section_data[i])?;
            }
            SectionType::Meta => {
                module.metadata = deserialize_metadata(&module.section_data[i])?;
            }
            _ => {}
        }
    }

    Ok(module)
}

/// Read a BibModule from a file
pub fn read_file(path: &str) -> Result<BibModule, String> {
    let data = std::fs::read(path)
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;
    deserialize(&data)
}

// ============================================================
// Deserialization helpers
// ============================================================

fn read_u16(data: &[u8], off: &mut usize) -> Result<u16, String> {
    if *off + 2 > data.len() { return Err("Unexpected end of data (u16)".into()); }
    let v = u16::from_le_bytes([data[*off], data[*off + 1]]);
    *off += 2;
    Ok(v)
}

fn read_u32(data: &[u8], off: &mut usize) -> Result<u32, String> {
    if *off + 4 > data.len() { return Err("Unexpected end of data (u32)".into()); }
    let v = u32::from_le_bytes([data[*off], data[*off+1], data[*off+2], data[*off+3]]);
    *off += 4;
    Ok(v)
}

fn read_u64(data: &[u8], off: &mut usize) -> Result<u64, String> {
    if *off + 8 > data.len() { return Err("Unexpected end of data (u64)".into()); }
    let v = u64::from_le_bytes(data[*off..*off+8].try_into().unwrap());
    *off += 8;
    Ok(v)
}

fn read_i64(data: &[u8], off: &mut usize) -> Result<i64, String> {
    if *off + 8 > data.len() { return Err("Unexpected end of data (i64)".into()); }
    let v = i64::from_le_bytes(data[*off..*off+8].try_into().unwrap());
    *off += 8;
    Ok(v)
}

fn read_u8(data: &[u8], off: &mut usize) -> Result<u8, String> {
    if *off >= data.len() { return Err("Unexpected end of data (u8)".into()); }
    let v = data[*off];
    *off += 1;
    Ok(v)
}

fn read_string(data: &[u8], off: &mut usize) -> Result<String, String> {
    let len = read_u16(data, off)? as usize;
    if *off + len > data.len() { return Err("String data out of bounds".into()); }
    let s = std::str::from_utf8(&data[*off..*off + len])
        .map_err(|e| format!("Invalid UTF-8: {}", e))?
        .to_string();
    *off += len;
    Ok(s)
}

fn deserialize_imports(data: &[u8]) -> Result<Vec<ImportModule>, String> {
    if data.is_empty() { return Ok(Vec::new()); }
    let mut off = 0;
    let count = read_u32(data, &mut off)? as usize;
    let mut modules = Vec::with_capacity(count);

    for _ in 0..count {
        let name_len = read_u16(data, &mut off)? as usize;
        if off + name_len > data.len() { return Err("Import module name out of bounds".into()); }
        let name = std::str::from_utf8(&data[off..off + name_len])
            .map_err(|e| format!("Invalid module name: {}", e))?
            .to_string();
        off += name_len;

        let sym_count = read_u32(data, &mut off)? as usize;
        let mut symbols = Vec::with_capacity(sym_count);
        for _ in 0..sym_count {
            let hint = read_u16(data, &mut off)?;
            let sym_name = read_string(data, &mut off)?;
            symbols.push(ImportEntry {
                module: name.clone(),
                symbol: sym_name,
                hint,
            });
        }
        modules.push(ImportModule { name, symbols });
    }
    Ok(modules)
}

fn deserialize_symbols(data: &[u8]) -> Result<Vec<Symbol>, String> {
    if data.is_empty() { return Ok(Vec::new()); }
    let mut off = 0;
    let count = read_u32(data, &mut off)? as usize;
    let mut symbols = Vec::with_capacity(count);

    for _ in 0..count {
        let sym_type_raw = read_u8(data, &mut off)?;
        let bind_raw = read_u8(data, &mut off)?;
        let section_index = read_u16(data, &mut off)?;
        let offset = read_u64(data, &mut off)?;
        let size = read_u64(data, &mut off)?;
        let name = read_string(data, &mut off)?;

        let sym_type = match sym_type_raw {
            0x01 => SymbolType::Function,
            0x02 => SymbolType::Data,
            0x03 => SymbolType::Section,
            0x04 => SymbolType::External,
            _ => return Err(format!("Unknown symbol type: 0x{:02X}", sym_type_raw)),
        };
        let bind = match bind_raw {
            0x00 => SymbolBind::Local,
            0x01 => SymbolBind::Global,
            0x02 => SymbolBind::Weak,
            _ => return Err(format!("Unknown symbol bind: 0x{:02X}", bind_raw)),
        };

        symbols.push(Symbol {
            name,
            sym_type,
            bind,
            section_index,
            offset,
            size,
        });
    }
    Ok(symbols)
}

fn deserialize_relocations(data: &[u8]) -> Result<Vec<Relocation>, String> {
    if data.is_empty() { return Ok(Vec::new()); }
    let mut off = 0;
    let count = read_u32(data, &mut off)? as usize;
    let mut relocs = Vec::with_capacity(count);

    for _ in 0..count {
        let offset = read_u64(data, &mut off)?;
        let reloc_type_raw = read_u8(data, &mut off)?;
        let symbol_index = read_u32(data, &mut off)?;
        let addend = read_i64(data, &mut off)?;

        let reloc_type = match reloc_type_raw {
            0x01 => RelocType::Abs64,
            0x02 => RelocType::Rel32,
            0x03 => RelocType::Plt32,
            0x04 => RelocType::Abs32,
            0x05 => RelocType::HighAdj,
            _ => return Err(format!("Unknown reloc type: 0x{:02X}", reloc_type_raw)),
        };

        relocs.push(Relocation {
            offset,
            reloc_type,
            symbol_index,
            addend,
        });
    }
    Ok(relocs)
}

fn deserialize_metadata(data: &[u8]) -> Result<Vec<(String, String)>, String> {
    if data.is_empty() { return Ok(Vec::new()); }
    let mut off = 0;
    let count = read_u32(data, &mut off)? as usize;
    let mut meta = Vec::with_capacity(count);

    for _ in 0..count {
        let key = read_string(data, &mut off)?;
        let val = read_string(data, &mut off)?;
        meta.push((key, val));
    }
    Ok(meta)
}
