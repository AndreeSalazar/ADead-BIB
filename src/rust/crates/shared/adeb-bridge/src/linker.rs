// ============================================================
// Bridge Linker — Merges ASM-BIB .obj into ADead-BIB PE output
// ============================================================
// Takes COFF objects from ASM-BIB and merges their .text/.data
// sections with ADead-BIB's compiled C/C++ code, resolving
// cross-references between ASM functions and C symbols.
// ============================================================

use crate::coff_reader::{CoffObject, CoffRelocation, IMAGE_REL_AMD64_REL32, IMAGE_REL_AMD64_ADDR64, IMAGE_REL_AMD64_ADDR32NB};
use std::collections::HashMap;

// ── Merged Output ───────────────────────────────────────────

/// Result of merging ASM-BIB .obj with ADead-BIB compiled code
#[derive(Debug, Clone)]
pub struct MergedBinary {
    /// Combined .text (C code first, then ASM code appended)
    pub code: Vec<u8>,
    /// Combined .data (C data first, then ASM data appended)
    pub data: Vec<u8>,
    /// Map: symbol_name → offset in merged code section
    pub symbol_map: HashMap<String, u32>,
    /// IAT call offsets that need patching (from C code)
    pub iat_call_offsets: Vec<usize>,
    /// String imm64 offsets that need patching (from C code)
    pub string_imm64_offsets: Vec<usize>,
    /// Offset where ASM code starts in the merged .text
    pub asm_code_offset: u32,
    /// Offset where ASM data starts in the merged .data
    pub asm_data_offset: u32,
}

// ── Bridge Linker ───────────────────────────────────────────

/// Links ASM-BIB COFF objects with ADead-BIB compiled output
pub struct BridgeLinker {
    obj_files: Vec<CoffObject>,
}

impl BridgeLinker {
    pub fn new() -> Self {
        Self {
            obj_files: Vec::new(),
        }
    }

    /// Load a COFF .obj file from disk
    pub fn add_obj_file(&mut self, path: &str) -> Result<(), String> {
        let obj = CoffObject::from_file(path)?;
        println!("  [bridge] Loaded: {}", path);
        println!("  [bridge] {}", obj.summary());
        self.obj_files.push(obj);
        Ok(())
    }

    /// Load a COFF .obj from raw bytes
    pub fn add_obj_bytes(&mut self, data: &[u8]) -> Result<(), String> {
        let obj = CoffObject::parse(data)?;
        self.obj_files.push(obj);
        Ok(())
    }

    /// Get all exported ASM symbols across all loaded .obj files
    pub fn asm_exports(&self) -> HashMap<String, (usize, u32)> {
        let mut map = HashMap::new();
        for (obj_idx, obj) in self.obj_files.iter().enumerate() {
            for sym in obj.exported_symbols() {
                map.insert(sym.name.clone(), (obj_idx, sym.value));
            }
        }
        map
    }

    /// Merge ADead-BIB compiled code+data with all loaded ASM-BIB .obj files
    ///
    /// # Arguments
    /// * `c_code` — Machine code from ADead-BIB's ISA compiler
    /// * `c_data` — Data section from ADead-BIB's ISA compiler
    /// * `c_iat_offsets` — IAT call offsets in C code
    /// * `c_string_offsets` — String imm64 offsets in C code
    /// * `c_symbol_map` — Symbols defined in C code (name → offset)
    pub fn merge(
        &self,
        c_code: &[u8],
        c_data: &[u8],
        c_iat_offsets: &[usize],
        c_string_offsets: &[usize],
        c_symbol_map: &HashMap<String, u32>,
    ) -> Result<MergedBinary, String> {
        let mut merged_code = c_code.to_vec();
        let mut merged_data = c_data.to_vec();
        let mut symbol_map: HashMap<String, u32> = c_symbol_map.clone();
        let mut iat_call_offsets = c_iat_offsets.to_vec();
        let mut string_imm64_offsets = c_string_offsets.to_vec();

        let asm_code_offset = merged_code.len() as u32;
        let asm_data_offset = merged_data.len() as u32;

        // Phase 1: Append all ASM .text and .data sections
        let mut obj_code_offsets: Vec<u32> = Vec::new();
        let mut obj_data_offsets: Vec<u32> = Vec::new();

        for obj in &self.obj_files {
            let code_start = merged_code.len() as u32;
            obj_code_offsets.push(code_start);

            if let Some(text) = obj.text_section() {
                merged_code.extend_from_slice(&text.data);
            }

            let data_start = merged_data.len() as u32;
            obj_data_offsets.push(data_start);

            if let Some(data) = obj.data_section() {
                merged_data.extend_from_slice(&data.data);
            }

            // Also append .rdata if present
            if let Some(rdata) = obj.rdata_section() {
                merged_data.extend_from_slice(&rdata.data);
            }
        }

        // Phase 2: Register all ASM exported symbols with their merged offsets
        for (obj_idx, obj) in self.obj_files.iter().enumerate() {
            for sym in obj.exported_symbols() {
                if sym.section_number > 0 {
                    let section_idx = (sym.section_number - 1) as usize;
                    if let Some(section) = obj.sections.get(section_idx) {
                        let base_offset = if section.name == ".text" {
                            obj_code_offsets[obj_idx]
                        } else {
                            obj_data_offsets[obj_idx]
                        };
                        let merged_offset = base_offset + sym.value;
                        symbol_map.insert(sym.name.clone(), merged_offset);
                    }
                }
            }
        }

        // Phase 3: Apply relocations from ASM .obj files
        for (obj_idx, obj) in self.obj_files.iter().enumerate() {
            if let Some(text) = obj.text_section() {
                let code_base = obj_code_offsets[obj_idx] as usize;

                for reloc in &text.relocations {
                    let patch_offset = code_base + reloc.virtual_address as usize;

                    // Resolve target symbol
                    let target_sym = obj.symbols.get(reloc.symbol_index as usize)
                        .ok_or_else(|| format!("Invalid symbol index {} in relocation", reloc.symbol_index))?;

                    let target_offset = symbol_map.get(&target_sym.name)
                        .copied()
                        .ok_or_else(|| format!("Unresolved symbol '{}' in ASM .obj", target_sym.name))?;

                    match reloc.rel_type {
                        IMAGE_REL_AMD64_REL32 => {
                            // RIP-relative: target - (patch_addr + 4)
                            if patch_offset + 4 <= merged_code.len() {
                                let rip = (patch_offset + 4) as i64;
                                let delta = target_offset as i64 - rip;
                                let bytes = (delta as i32).to_le_bytes();
                                merged_code[patch_offset..patch_offset + 4].copy_from_slice(&bytes);
                            }
                        }
                        IMAGE_REL_AMD64_ADDR64 => {
                            // Absolute 64-bit address (needs imagebase fixup later)
                            if patch_offset + 8 <= merged_code.len() {
                                let bytes = (target_offset as u64).to_le_bytes();
                                merged_code[patch_offset..patch_offset + 8].copy_from_slice(&bytes);
                            }
                        }
                        IMAGE_REL_AMD64_ADDR32NB => {
                            // 32-bit RVA (no base)
                            if patch_offset + 4 <= merged_code.len() {
                                let bytes = target_offset.to_le_bytes();
                                merged_code[patch_offset..patch_offset + 4].copy_from_slice(&bytes);
                            }
                        }
                        _ => {
                            // Unknown relocation type — skip with warning
                            eprintln!("  [bridge] WARNING: unsupported relocation type 0x{:04X} at offset 0x{:X}",
                                reloc.rel_type, patch_offset);
                        }
                    }
                }
            }
        }

        Ok(MergedBinary {
            code: merged_code,
            data: merged_data,
            symbol_map,
            iat_call_offsets,
            string_imm64_offsets,
            asm_code_offset,
            asm_data_offset,
        })
    }

    /// Quick merge: just append ASM code/data without relocation processing
    /// Useful for simple cases where ASM functions are self-contained
    pub fn merge_simple(
        &self,
        c_code: &[u8],
        c_data: &[u8],
        c_iat_offsets: &[usize],
        c_string_offsets: &[usize],
    ) -> MergedBinary {
        let mut merged_code = c_code.to_vec();
        let mut merged_data = c_data.to_vec();
        let asm_code_offset = merged_code.len() as u32;
        let asm_data_offset = merged_data.len() as u32;
        let mut symbol_map = HashMap::new();

        for obj in &self.obj_files {
            let code_start = merged_code.len() as u32;

            if let Some(text) = obj.text_section() {
                merged_code.extend_from_slice(&text.data);
            }

            if let Some(data) = obj.data_section() {
                merged_data.extend_from_slice(&data.data);
            }

            // Register exported symbols
            for sym in obj.exported_symbols() {
                symbol_map.insert(sym.name.clone(), code_start + sym.value);
            }
        }

        MergedBinary {
            code: merged_code,
            data: merged_data,
            symbol_map,
            iat_call_offsets: c_iat_offsets.to_vec(),
            string_imm64_offsets: c_string_offsets.to_vec(),
            asm_code_offset,
            asm_data_offset,
        }
    }
}

impl Default for BridgeLinker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_simple_empty() {
        let linker = BridgeLinker::new();
        let result = linker.merge_simple(
            &[0x90, 0x90], // NOP NOP
            &[0x48, 0x65], // "He"
            &[],
            &[],
        );
        assert_eq!(result.code, vec![0x90, 0x90]);
        assert_eq!(result.data, vec![0x48, 0x65]);
        assert_eq!(result.asm_code_offset, 2);
        assert_eq!(result.asm_data_offset, 2);
    }
}
