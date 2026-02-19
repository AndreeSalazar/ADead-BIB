// ============================================================
// ADead-BIB Writer — Serialize BibModule to .bib file
// ============================================================

use std::io::Write;
use super::format::*;

/// Serialize a BibModule to bytes
pub fn serialize(module: &BibModule) -> Vec<u8> {
    let mut out = Vec::new();

    // 1. Compute section data offsets
    let section_table_start = BIB_HEADER_SIZE;
    let section_table_size = module.sections.len() * SECTION_HEADER_SIZE;
    let data_start = section_table_start + section_table_size;

    // Build section headers with correct offsets
    let mut headers: Vec<SectionHeader> = Vec::new();
    let mut current_offset = data_start as u64;

    for (i, sec) in module.sections.iter().enumerate() {
        let mut h = sec.clone();
        h.offset = current_offset;
        h.size = module.section_data[i].len() as u64;
        // Align next section to 16 bytes
        current_offset += h.size;
        current_offset = (current_offset + 15) & !15;
        headers.push(h);
    }

    // Serialize import table as a section
    let import_data = serialize_imports(&module.imports);
    if !import_data.is_empty() {
        let mut h = SectionHeader::new(".import", SectionType::Import);
        h.offset = current_offset;
        h.size = import_data.len() as u64;
        h.flags = section_flags::READABLE;
        current_offset += h.size;
        current_offset = (current_offset + 15) & !15;
        headers.push(h);
    }

    // Serialize symbol table as a section
    let symbol_data = serialize_symbols(&module.symbols);
    if !symbol_data.is_empty() {
        let mut h = SectionHeader::new(".symtab", SectionType::Symbol);
        h.offset = current_offset;
        h.size = symbol_data.len() as u64;
        h.flags = section_flags::READABLE;
        current_offset += h.size;
        current_offset = (current_offset + 15) & !15;
        headers.push(h);
    }

    // Serialize relocation table as a section
    let reloc_data = serialize_relocations(&module.relocations);
    if !reloc_data.is_empty() {
        let mut h = SectionHeader::new(".reloc", SectionType::Reloc);
        h.offset = current_offset;
        h.size = reloc_data.len() as u64;
        h.flags = section_flags::READABLE;
        current_offset += h.size;
        current_offset = (current_offset + 15) & !15;
        headers.push(h);
    }

    // Serialize metadata as a section
    let meta_data = serialize_metadata(&module.metadata);
    if !meta_data.is_empty() {
        let mut h = SectionHeader::new(".meta", SectionType::Meta);
        h.offset = current_offset;
        h.size = meta_data.len() as u64;
        h.flags = section_flags::READABLE;
        current_offset += h.size;
        current_offset = (current_offset + 15) & !15;
        headers.push(h);
    }

    let total_size = current_offset;

    // 2. Write header
    let mut header = module.header.clone();
    header.section_count = headers.len() as u32;
    header.file_size = total_size;
    header.section_table_offset = section_table_start as u64;
    out.extend_from_slice(&header.to_bytes());

    // 3. Write section headers
    for h in &headers {
        out.extend_from_slice(&h.to_bytes());
    }

    // 4. Write section data (with alignment padding)
    for (i, sec_data) in module.section_data.iter().enumerate() {
        // Pad to section offset
        while out.len() < headers[i].offset as usize {
            out.push(0);
        }
        out.extend_from_slice(sec_data);
    }

    // 5. Write extra sections (import, symbol, reloc, meta)
    let extra_sections_start = module.sections.len();
    let extra_data: Vec<&Vec<u8>> = [&import_data, &symbol_data, &reloc_data, &meta_data]
        .iter()
        .filter(|d| !d.is_empty())
        .copied()
        .collect();

    for (i, data) in extra_data.iter().enumerate() {
        let idx = extra_sections_start + i;
        if idx < headers.len() {
            while out.len() < headers[idx].offset as usize {
                out.push(0);
            }
            out.extend_from_slice(data);
        }
    }

    // Pad to total size
    while out.len() < total_size as usize {
        out.push(0);
    }

    out
}

/// Write BibModule to a file
pub fn write_file(module: &BibModule, path: &str) -> std::io::Result<()> {
    let data = serialize(module);
    let mut file = std::fs::File::create(path)?;
    file.write_all(&data)?;
    Ok(())
}

// ============================================================
// Serialization helpers
// ============================================================

/// Serialize imports: [u32 module_count] [modules...]
/// Module: [u16 name_len] [name bytes] [u32 sym_count] [symbols...]
/// Symbol: [u16 hint] [u16 name_len] [name bytes]
fn serialize_imports(imports: &[ImportModule]) -> Vec<u8> {
    if imports.is_empty() { return Vec::new(); }
    let mut buf = Vec::new();
    buf.extend_from_slice(&(imports.len() as u32).to_le_bytes());
    for m in imports {
        let name = m.name.as_bytes();
        buf.extend_from_slice(&(name.len() as u16).to_le_bytes());
        buf.extend_from_slice(name);
        buf.extend_from_slice(&(m.symbols.len() as u32).to_le_bytes());
        for sym in &m.symbols {
            buf.extend_from_slice(&sym.hint.to_le_bytes());
            let sname = sym.symbol.as_bytes();
            buf.extend_from_slice(&(sname.len() as u16).to_le_bytes());
            buf.extend_from_slice(sname);
        }
    }
    buf
}

/// Serialize symbols: [u32 count] [symbols...]
/// Symbol: [u8 type] [u8 bind] [u16 section] [u64 offset] [u64 size] [u16 name_len] [name]
fn serialize_symbols(symbols: &[Symbol]) -> Vec<u8> {
    if symbols.is_empty() { return Vec::new(); }
    let mut buf = Vec::new();
    buf.extend_from_slice(&(symbols.len() as u32).to_le_bytes());
    for sym in symbols {
        buf.push(sym.sym_type as u8);
        buf.push(sym.bind as u8);
        buf.extend_from_slice(&sym.section_index.to_le_bytes());
        buf.extend_from_slice(&sym.offset.to_le_bytes());
        buf.extend_from_slice(&sym.size.to_le_bytes());
        let name = sym.name.as_bytes();
        buf.extend_from_slice(&(name.len() as u16).to_le_bytes());
        buf.extend_from_slice(name);
    }
    buf
}

/// Serialize relocations: [u32 count] [relocs...]
/// Reloc: [u64 offset] [u8 type] [u32 symbol_index] [i64 addend]
fn serialize_relocations(relocs: &[Relocation]) -> Vec<u8> {
    if relocs.is_empty() { return Vec::new(); }
    let mut buf = Vec::new();
    buf.extend_from_slice(&(relocs.len() as u32).to_le_bytes());
    for r in relocs {
        buf.extend_from_slice(&r.offset.to_le_bytes());
        buf.push(r.reloc_type as u8);
        buf.extend_from_slice(&r.symbol_index.to_le_bytes());
        buf.extend_from_slice(&r.addend.to_le_bytes());
    }
    buf
}

/// Serialize metadata: [u32 count] [entries...]
/// Entry: [u16 key_len] [key] [u16 val_len] [val]
fn serialize_metadata(meta: &[(String, String)]) -> Vec<u8> {
    if meta.is_empty() { return Vec::new(); }
    let mut buf = Vec::new();
    buf.extend_from_slice(&(meta.len() as u32).to_le_bytes());
    for (k, v) in meta {
        let kb = k.as_bytes();
        let vb = v.as_bytes();
        buf.extend_from_slice(&(kb.len() as u16).to_le_bytes());
        buf.extend_from_slice(kb);
        buf.extend_from_slice(&(vb.len() as u16).to_le_bytes());
        buf.extend_from_slice(vb);
    }
    buf
}
