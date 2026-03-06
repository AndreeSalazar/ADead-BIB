// PE (Portable Executable) Generator
// Genera binarios Windows .exe funcionales con soporte para imports (printf)
// Versión validada según especificación PE

use std::fs::File;
use std::io::Write;

pub fn generate_pe(
    opcodes: &[u8],
    _data: &[u8],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    generate_pe_with_offsets(opcodes, _data, output_path, &[], &[])
}

/// FASM-inspired PE generator with precise offset tracking.
/// Uses exact IAT call and string address offsets from the encoder
/// instead of fragile byte-pattern scanning.
pub fn generate_pe_with_offsets(
    opcodes: &[u8],
    _data: &[u8],
    output_path: &str,
    iat_call_offsets: &[usize],
    string_imm64_offsets: &[usize],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(output_path)?;

    // Constants
    let file_align: usize = 0x200;
    let section_align: u32 = 0x1000;

    // Calculate code size aligned to file alignment
    let code_raw_size = ((opcodes.len() + file_align - 1) / file_align * file_align) as u32;
    // Virtual size is actual size
    let code_virtual_size = opcodes.len() as u32;

    // Calculate .text virtual pages (aligned to section alignment)
    let text_virtual_pages = (code_virtual_size + section_align - 1) / section_align;

    // .idata RVA: dynamically placed after .text
    let idata_rva: u32 = 0x1000 + text_virtual_pages * section_align;

    // Layout offsets within .idata (v1.5.0):
    // 0x00-0x27: IDT (Import Directory Table) - 1 entry (msvcrt) + null (2 * 20 bytes = 40 bytes)
    // 0x28-0x4F: ILT (Import Lookup Table) - 4 entries (printf, scanf, malloc, free) + null (5 * 8 = 40 bytes)
    // 0x50-0x77: IAT (Import Address Table) - 4 entries + null (5 * 8 = 40 bytes)
    // 0x78+: Strings area

    // Reserve enough space for IDT+ILT+IAT+strings
    let mut idata = vec![0u8; file_align]; // initial capacity

    // IDT[0] for msvcrt.dll
    idata[0..4].copy_from_slice(&(idata_rva + 0x28).to_le_bytes()); // OriginalFirstThunk (ILT)
    idata[12..16].copy_from_slice(&(idata_rva + 0x78).to_le_bytes()); // Name (DLL name)
    idata[16..20].copy_from_slice(&(idata_rva + 0x50).to_le_bytes()); // FirstThunk (IAT)

    // ILT entries (PE32+ uses 64-bit thunks) — 4 entries + null
    let printf_hint_rva: u32 = idata_rva + 0x84;
    let scanf_hint_rva: u32 = idata_rva + 0x8E;
    let malloc_hint_rva: u32 = idata_rva + 0x96;
    let free_hint_rva: u32 = idata_rva + 0xA0;

    // ILT[0] -> printf, ILT[1] -> scanf, ILT[2] -> malloc, ILT[3] -> free, ILT[4] = null
    idata[0x28..0x30].copy_from_slice(&(printf_hint_rva as u64).to_le_bytes());
    idata[0x30..0x38].copy_from_slice(&(scanf_hint_rva as u64).to_le_bytes());
    idata[0x38..0x40].copy_from_slice(&(malloc_hint_rva as u64).to_le_bytes());
    idata[0x40..0x48].copy_from_slice(&(free_hint_rva as u64).to_le_bytes());
    // ILT[4] = null (already zeros at 0x48)

    // IAT entries (same as ILT initially, loader overwrites) — at 0x50
    idata[0x50..0x58].copy_from_slice(&(printf_hint_rva as u64).to_le_bytes());
    idata[0x58..0x60].copy_from_slice(&(scanf_hint_rva as u64).to_le_bytes());
    idata[0x60..0x68].copy_from_slice(&(malloc_hint_rva as u64).to_le_bytes());
    idata[0x68..0x70].copy_from_slice(&(free_hint_rva as u64).to_le_bytes());
    // IAT[4] = null (already zeros at 0x70)

    // Strings
    let dll_name = b"msvcrt.dll\0";
    idata[0x78..0x78 + dll_name.len()].copy_from_slice(dll_name);

    // Hint/Name for printf @ 0x84
    idata[0x84] = 0;
    idata[0x85] = 0; // Hint = 0
    let printf_name = b"printf\0";
    idata[0x86..0x86 + printf_name.len()].copy_from_slice(printf_name);

    // Hint/Name for scanf @ 0x8E
    idata[0x8E] = 0;
    idata[0x8F] = 0; // Hint = 0
    let scanf_name = b"scanf\0";
    idata[0x90..0x90 + scanf_name.len()].copy_from_slice(scanf_name);

    // Hint/Name for malloc @ 0x96
    idata[0x96] = 0;
    idata[0x97] = 0; // Hint = 0
    let malloc_name = b"malloc\0";
    idata[0x98..0x98 + malloc_name.len()].copy_from_slice(malloc_name);

    // Hint/Name for free @ 0xA0
    idata[0xA0] = 0;
    idata[0xA1] = 0; // Hint = 0
    let free_name = b"free\0";
    idata[0xA2..0xA2 + free_name.len()].copy_from_slice(free_name);

    // Append program strings to .idata after 0xA8
    let program_strings_offset = 0xA8usize;
    if program_strings_offset + _data.len() > idata.len() {
        let needed = program_strings_offset + _data.len();
        let aligned = ((needed + file_align - 1) / file_align) * file_align;
        idata.resize(aligned, 0);
    }
    idata[program_strings_offset..program_strings_offset + _data.len()].copy_from_slice(_data);

    let idata_raw_size = idata.len() as u32;
    let idata_virtual_size = idata_raw_size;

    // Calculate SizeOfImage dynamically
    let idata_virtual_pages = (idata_virtual_size + section_align - 1) / section_align;
    let size_of_image = idata_rva + idata_virtual_pages * section_align;

    // FASM-inspired precise patching: use tracked offsets from encoder when available,
    // fall back to byte scanning only when no offsets are provided (backward compat).
    let mut patched_opcodes = opcodes.to_vec();
    let iat_delta = idata_rva as i32 - 0x2000i32;
    let image_base: u64 = 0x0000000140000000;
    let old_string_base = image_base + 0x20A8;
    let new_string_base = image_base + idata_rva as u64 + 0xA8;
    let string_delta = new_string_base as i64 - old_string_base as i64;

    if iat_delta != 0 || string_delta != 0 {
        if !iat_call_offsets.is_empty() || !string_imm64_offsets.is_empty() {
            // PRECISE MODE: use exact offsets from encoder (no byte scanning)
            for &offset in iat_call_offsets {
                if offset + 4 <= patched_opcodes.len() {
                    let old_val = i32::from_le_bytes([
                        patched_opcodes[offset],
                        patched_opcodes[offset + 1],
                        patched_opcodes[offset + 2],
                        patched_opcodes[offset + 3],
                    ]);
                    let new_val = old_val + iat_delta;
                    patched_opcodes[offset..offset + 4].copy_from_slice(&new_val.to_le_bytes());
                }
            }
            for &offset in string_imm64_offsets {
                if offset + 8 <= patched_opcodes.len() {
                    let imm64 = u64::from_le_bytes([
                        patched_opcodes[offset],
                        patched_opcodes[offset + 1],
                        patched_opcodes[offset + 2],
                        patched_opcodes[offset + 3],
                        patched_opcodes[offset + 4],
                        patched_opcodes[offset + 5],
                        patched_opcodes[offset + 6],
                        patched_opcodes[offset + 7],
                    ]);
                    if imm64 >= old_string_base && imm64 < old_string_base + 0x10000 {
                        let new_imm64 = (imm64 as i64 + string_delta) as u64;
                        patched_opcodes[offset..offset + 8]
                            .copy_from_slice(&new_imm64.to_le_bytes());
                    }
                }
            }
        } else {
            // LEGACY MODE: byte-pattern scanning (backward compat for callers without offsets)
            let mut i = 0;
            while i < patched_opcodes.len() {
                if i + 5 < patched_opcodes.len()
                    && patched_opcodes[i] == 0xFF
                    && patched_opcodes[i + 1] == 0x15
                {
                    let old_offset = i32::from_le_bytes([
                        patched_opcodes[i + 2],
                        patched_opcodes[i + 3],
                        patched_opcodes[i + 4],
                        patched_opcodes[i + 5],
                    ]);
                    let new_offset = old_offset + iat_delta;
                    patched_opcodes[i + 2..i + 6].copy_from_slice(&new_offset.to_le_bytes());
                    i += 6;
                    continue;
                }
                if i + 9 < patched_opcodes.len() && patched_opcodes[i] == 0x48 {
                    let opcode = patched_opcodes[i + 1];
                    if opcode >= 0xB8 && opcode <= 0xBF {
                        let imm64 = u64::from_le_bytes([
                            patched_opcodes[i + 2],
                            patched_opcodes[i + 3],
                            patched_opcodes[i + 4],
                            patched_opcodes[i + 5],
                            patched_opcodes[i + 6],
                            patched_opcodes[i + 7],
                            patched_opcodes[i + 8],
                            patched_opcodes[i + 9],
                        ]);
                        if imm64 >= old_string_base && imm64 < old_string_base + 0x10000 {
                            let new_imm64 = (imm64 as i64 + string_delta) as u64;
                            patched_opcodes[i + 2..i + 10]
                                .copy_from_slice(&new_imm64.to_le_bytes());
                        }
                        i += 10;
                        continue;
                    }
                }
                i += 1;
            }
        }
    }
    let opcodes = &patched_opcodes;

    // Headers
    // DOS Header (64 bytes)
    let mut dos = vec![0u8; 64];
    dos[0] = 0x4D; // 'M'
    dos[1] = 0x5A; // 'Z'
    dos[0x3C] = 0x40; // e_lfanew
    file.write_all(&dos)?;

    // PE Signature
    file.write_all(b"PE\0\0")?;

    // COFF Header (20 bytes)
    let mut coff = vec![0u8; 20];
    coff[0] = 0x64;
    coff[1] = 0x86; // x64
    coff[2] = 0x02;
    coff[3] = 0x00; // NumberOfSections: 2
    coff[16] = 0xF0;
    coff[17] = 0x00; // SizeOfOptionalHeader
    coff[18] = 0x22;
    coff[19] = 0x00; // Characteristics
    file.write_all(&coff)?;

    // Optional Header (240 bytes)
    let mut opt = vec![0u8; 240];
    opt[0] = 0x0B;
    opt[1] = 0x02; // Magic PE32+
    opt[2] = 14; // Linker version
    opt[4..8].copy_from_slice(&code_raw_size.to_le_bytes()); // SizeOfCode
    opt[8..12].copy_from_slice(&idata_raw_size.to_le_bytes()); // SizeOfInitializedData
    opt[16..20].copy_from_slice(&0x1000u32.to_le_bytes()); // AddressOfEntryPoint
    opt[20..24].copy_from_slice(&0x1000u32.to_le_bytes()); // BaseOfCode
    opt[24..32].copy_from_slice(&0x0000000140000000u64.to_le_bytes()); // ImageBase
    opt[32..36].copy_from_slice(&section_align.to_le_bytes()); // SectionAlignment
    opt[36..40].copy_from_slice(&(file_align as u32).to_le_bytes()); // FileAlignment
    opt[40] = 6; // MajorOSVersion
    opt[48] = 6; // MajorSubsystemVersion

    opt[56..60].copy_from_slice(&size_of_image.to_le_bytes()); // SizeOfImage
    opt[60..64].copy_from_slice(&0x400u32.to_le_bytes()); // SizeOfHeaders
    opt[68] = 0x03; // Subsystem CUI
                    // DllCharacteristics: NX_COMPAT (0x0100) — required by modern Windows
    opt[70..72].copy_from_slice(&0x0100u16.to_le_bytes());
    opt[72..80].copy_from_slice(&0x100000u64.to_le_bytes()); // StackReserve
    opt[80..88].copy_from_slice(&0x1000u64.to_le_bytes()); // StackCommit
    opt[88..96].copy_from_slice(&0x100000u64.to_le_bytes()); // HeapReserve
    opt[96..104].copy_from_slice(&0x1000u64.to_le_bytes()); // HeapCommit
    opt[108..112].copy_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes

    // Data Directory [1] Import Table
    opt[120..124].copy_from_slice(&idata_rva.to_le_bytes());
    opt[124..128].copy_from_slice(&40u32.to_le_bytes());

    // Data Directory [12] IAT (Import Address Table)
    opt[208..212].copy_from_slice(&(idata_rva + 0x50).to_le_bytes());
    opt[212..216].copy_from_slice(&40u32.to_le_bytes()); // 5 entries * 8 = 40

    file.write_all(&opt)?;

    // Section Headers
    // .text
    let mut sec_text = vec![0u8; 40];
    sec_text[0..5].copy_from_slice(b".text");
    sec_text[8..12].copy_from_slice(&code_virtual_size.to_le_bytes()); // VirtualSize
    sec_text[12..16].copy_from_slice(&0x1000u32.to_le_bytes()); // VirtualAddress
    sec_text[16..20].copy_from_slice(&code_raw_size.to_le_bytes()); // SizeOfRawData
    sec_text[20..24].copy_from_slice(&0x400u32.to_le_bytes()); // PointerToRawData
    sec_text[36..40].copy_from_slice(&0x60000020u32.to_le_bytes()); // Characteristics
    file.write_all(&sec_text)?;

    // .idata
    let mut sec_idata = vec![0u8; 40];
    sec_idata[0..6].copy_from_slice(b".idata");
    sec_idata[8..12].copy_from_slice(&idata_virtual_size.to_le_bytes()); // VirtualSize
    sec_idata[12..16].copy_from_slice(&idata_rva.to_le_bytes()); // VirtualAddress
    sec_idata[16..20].copy_from_slice(&idata_raw_size.to_le_bytes()); // SizeOfRawData
    let idata_ptr = 0x400 + code_raw_size;
    sec_idata[20..24].copy_from_slice(&idata_ptr.to_le_bytes()); // PointerToRawData
    sec_idata[36..40].copy_from_slice(&0xC0000040u32.to_le_bytes()); // Characteristics
    file.write_all(&sec_idata)?;

    // Padding to 0x400
    let headers_size = 64 + 4 + 20 + 240 + 40 + 40;
    let padding = 0x400 - headers_size;
    file.write_all(&vec![0u8; padding])?;

    // Write .text
    file.write_all(opcodes)?;
    let text_padding = code_raw_size - opcodes.len() as u32;
    file.write_all(&vec![0u8; text_padding as usize])?;

    // Write .idata
    file.write_all(&idata)?;

    Ok(())
}
