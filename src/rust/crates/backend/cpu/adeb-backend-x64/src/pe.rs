// ============================================================
// PE Generator — Windows PE x86-64 Output
// ============================================================
//
// Extracted from lib.rs monolith. Single responsibility:
//   - Generate valid PE/COFF executables for Windows x64
//   - Patch IAT call offsets and string addresses
//   - Build .text + .idata sections
//
// ============================================================

use crate::iat_registry;

fn align_up_u32(v: u32, a: u32) -> u32 {
    if a == 0 {
        return v;
    }
    (v + (a - 1)) & !(a - 1)
}

fn align_up_usize(v: usize, a: usize) -> usize {
    if a == 0 {
        return v;
    }
    (v + (a - 1)) & !(a - 1)
}

fn push_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn push_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn push_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

/// The assumed idata_rva that the ISA compiler uses during code generation.
/// The PE builder patches code bytes when the actual idata_rva differs.
pub const ASSUMED_IDATA_RVA: u32 = 0x2000;

pub fn generate_pe_with_offsets(
    code: &[u8],
    data: &[u8],
    output_path: &str,
    iat_call_offsets: &[usize],
    string_imm64_offsets: &[usize],
) -> Result<(), Box<dyn std::error::Error>> {
    generate_pe_filtered(code, data, output_path, iat_call_offsets, string_imm64_offsets, &std::collections::HashSet::new())
}

pub fn generate_pe_filtered(
    code: &[u8],
    data: &[u8],
    output_path: &str,
    iat_call_offsets: &[usize],
    string_imm64_offsets: &[usize],
    used_iat_slots: &std::collections::HashSet<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_alignment: u32 = 0x200;
    let section_alignment: u32 = 0x1000;

    let image_base: u64 = 0x0000000140000000;
    let text_rva: u32 = 0x1000;

    // Dynamic idata_rva: place .idata after .text's virtual pages
    let text_virtual_pages = align_up_u32(code.len() as u32, section_alignment);
    let idata_rva: u32 = text_rva + text_virtual_pages.max(section_alignment);

    let idata_result = iat_registry::build_idata_filtered(idata_rva, used_iat_slots);
    let mut idata = idata_result.bytes;
    if idata.len() != idata_result.program_strings_offset as usize {
        idata.resize(idata_result.program_strings_offset as usize, 0);
    }
    idata.extend_from_slice(data);

    // Patch code bytes when idata_rva differs from the assumed value
    let mut code = code.to_vec();
    let rva_delta = idata_rva as i64 - ASSUMED_IDATA_RVA as i64;
    if rva_delta != 0 {
        // Patch IAT call offsets: each is a RIP-relative disp32 (FF 15 [disp32])
        // The disp32 encodes (iat_rva - current_rip), so we adjust by the delta
        for &off in iat_call_offsets {
            if off + 4 <= code.len() {
                let old_disp = i32::from_le_bytes([
                    code[off], code[off + 1], code[off + 2], code[off + 3],
                ]);
                let new_disp = old_disp + rva_delta as i32;
                code[off..off + 4].copy_from_slice(&new_disp.to_le_bytes());
            }
        }

        // Patch string imm64 offsets: each is an absolute address (imagebase + idata_rva + string_offset)
        // Shift by the RVA delta
        for &off in string_imm64_offsets {
            if off + 8 <= code.len() {
                let old_addr = u64::from_le_bytes([
                    code[off], code[off + 1], code[off + 2], code[off + 3],
                    code[off + 4], code[off + 5], code[off + 6], code[off + 7],
                ]);
                let new_addr = (old_addr as i64 + rva_delta) as u64;
                code[off..off + 8].copy_from_slice(&new_addr.to_le_bytes());
            }
        }
    }

    let text_raw_size = align_up_u32(code.len() as u32, file_alignment);
    let idata_raw_size = align_up_u32(idata.len() as u32, file_alignment);

    let headers_size = 0x200u32;
    let text_raw_ptr = headers_size;
    let idata_raw_ptr = text_raw_ptr + text_raw_size;

    let text_virtual_size = code.len() as u32;
    let idata_virtual_size = idata.len() as u32;

    let size_of_image = align_up_u32(idata_rva + idata_virtual_size, section_alignment);
    let size_of_headers = headers_size;

    let e_lfanew: u32 = 0x80;
    let mut dos = vec![0u8; e_lfanew as usize];
    dos[0..2].copy_from_slice(b"MZ");
    dos[0x3C..0x40].copy_from_slice(&e_lfanew.to_le_bytes());

    let number_of_sections: u16 = 2;
    let size_of_optional_header: u16 = 0xF0;
    let characteristics: u16 = 0x0022;

    let mut headers = Vec::new();
    headers.extend_from_slice(&dos);
    headers.extend_from_slice(b"PE\0\0");

    push_u16(&mut headers, 0x8664);
    push_u16(&mut headers, number_of_sections);
    push_u32(&mut headers, 0);
    push_u32(&mut headers, 0);
    push_u32(&mut headers, 0);
    push_u16(&mut headers, size_of_optional_header);
    push_u16(&mut headers, characteristics);

    let mut opt = Vec::new();
    push_u16(&mut opt, 0x20B);
    opt.push(0);
    opt.push(0);
    push_u32(&mut opt, text_raw_size);
    push_u32(&mut opt, idata_raw_size);
    push_u32(&mut opt, 0);
    push_u32(&mut opt, text_rva);
    push_u32(&mut opt, text_rva);
    push_u64(&mut opt, image_base);
    push_u32(&mut opt, section_alignment);
    push_u32(&mut opt, file_alignment);
    push_u16(&mut opt, 6);
    push_u16(&mut opt, 0);
    push_u16(&mut opt, 0);
    push_u16(&mut opt, 0);
    push_u16(&mut opt, 6);
    push_u16(&mut opt, 0);
    push_u32(&mut opt, 0);
    push_u32(&mut opt, size_of_image);
    push_u32(&mut opt, size_of_headers);
    push_u32(&mut opt, 0);
    push_u16(&mut opt, 3);
    push_u16(&mut opt, 0x8100);
    push_u64(&mut opt, 0x100000);
    push_u64(&mut opt, 0x1000);
    push_u64(&mut opt, 0x100000);
    push_u64(&mut opt, 0x1000);
    push_u32(&mut opt, 0);
    push_u32(&mut opt, 16);

    for dir_index in 0..16 {
        if dir_index == 1 {
            push_u32(&mut opt, idata_result.import_dir_rva);
            push_u32(&mut opt, idata_result.import_dir_size);
        } else if dir_index == 12 {
            push_u32(&mut opt, idata_result.iat_rva);
            push_u32(&mut opt, idata_result.iat_size);
        } else {
            push_u32(&mut opt, 0);
            push_u32(&mut opt, 0);
        }
    }

    if opt.len() != size_of_optional_header as usize {
        return Err(format!(
            "Optional header size mismatch: got {}, expected {}",
            opt.len(),
            size_of_optional_header
        )
        .into());
    }
    headers.extend_from_slice(&opt);

    let mut sh = Vec::new();

    let mut name = [0u8; 8];
    name[..5].copy_from_slice(b".text");
    sh.extend_from_slice(&name);
    push_u32(&mut sh, text_virtual_size);
    push_u32(&mut sh, text_rva);
    push_u32(&mut sh, text_raw_size);
    push_u32(&mut sh, text_raw_ptr);
    push_u32(&mut sh, 0);
    push_u32(&mut sh, 0);
    push_u16(&mut sh, 0);
    push_u16(&mut sh, 0);
    push_u32(&mut sh, 0x60000020);

    let mut name2 = [0u8; 8];
    name2[..6].copy_from_slice(b".idata");
    sh.extend_from_slice(&name2);
    push_u32(&mut sh, idata_virtual_size);
    push_u32(&mut sh, idata_rva);
    push_u32(&mut sh, idata_raw_size);
    push_u32(&mut sh, idata_raw_ptr);
    push_u32(&mut sh, 0);
    push_u32(&mut sh, 0);
    push_u16(&mut sh, 0);
    push_u16(&mut sh, 0);
    push_u32(&mut sh, 0xC0000040);

    headers.extend_from_slice(&sh);

    if headers.len() > headers_size as usize {
        return Err("PE headers exceed 0x200".into());
    }
    headers.resize(headers_size as usize, 0);

    let mut out = Vec::new();
    out.extend_from_slice(&headers);

    let mut text_raw = code;
    text_raw.resize(text_raw_size as usize, 0x90);
    out.resize(text_raw_ptr as usize, 0);
    out.extend_from_slice(&text_raw);

    let mut idata_raw = idata;
    idata_raw.resize(idata_raw_size as usize, 0);
    out.resize(idata_raw_ptr as usize, 0);
    out.extend_from_slice(&idata_raw);

    let final_len = align_up_usize(out.len(), file_alignment as usize);
    out.resize(final_len, 0);

    std::fs::write(output_path, &out)?;
    Ok(())
}
