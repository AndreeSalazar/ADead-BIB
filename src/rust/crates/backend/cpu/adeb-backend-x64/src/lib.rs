pub mod isa;

pub mod frontend {
    pub mod ast {
        pub use adeb_core::ast::*;
        pub use adeb_core::types::{RegSize, Type};
    }
}

pub mod backend {
    pub mod cpu {
        pub mod iat_registry {
            use std::collections::HashMap;

            pub const IAT_DLL: &str = "msvcrt.dll";

            pub const IAT_ENTRIES: [&str; 4] = ["printf", "scanf", "malloc", "free"];

            #[derive(Debug, Clone)]
            pub struct IdataBuildResult {
                pub bytes: Vec<u8>,
                pub slot_to_iat_rva: Vec<u32>,
                pub import_dir_rva: u32,
                pub import_dir_size: u32,
                pub iat_rva: u32,
                pub iat_size: u32,
                pub program_strings_offset: u32,
            }

            pub fn slot_for_function(name: &str) -> Option<usize> {
                IAT_ENTRIES.iter().position(|s| *s == name)
            }

            fn align_up(value: usize, align: usize) -> usize {
                if align == 0 {
                    return value;
                }
                (value + (align - 1)) & !(align - 1)
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

            pub fn build_idata(idata_rva: u32, _extra_imports: &[&str]) -> IdataBuildResult {
                let n = IAT_ENTRIES.len();

                let import_desc_offset = 0usize;
                let import_desc_size = 20 * 2;

                let mut cursor = import_desc_offset + import_desc_size;
                cursor = align_up(cursor, 8);

                let oft_offset = cursor;
                let oft_size = (n + 1) * 8;
                cursor += oft_size;
                cursor = align_up(cursor, 8);

                let iat_offset = cursor;
                let iat_size = (n + 1) * 8;
                cursor += iat_size;
                cursor = align_up(cursor, 2);

                let mut hint_name_offsets: Vec<u32> = Vec::with_capacity(n);
                for name in IAT_ENTRIES {
                    hint_name_offsets.push(cursor as u32);
                    cursor += 2;
                    cursor += name.as_bytes().len() + 1;
                    cursor = align_up(cursor, 2);
                }

                let dll_name_offset = cursor;
                cursor += IAT_DLL.as_bytes().len() + 1;
                cursor = align_up(cursor, 8);

                let program_strings_offset = cursor as u32;

                let import_dir_rva = idata_rva + import_desc_offset as u32;
                let import_dir_size = import_desc_size as u32;
                let oft_rva = idata_rva + oft_offset as u32;
                let iat_rva = idata_rva + iat_offset as u32;

                let mut slot_to_iat_rva = Vec::with_capacity(n);
                for i in 0..n {
                    slot_to_iat_rva.push(iat_rva + (i as u32 * 8));
                }

                let mut bytes = vec![0u8; program_strings_offset as usize];

                let dll_name_rva = idata_rva + dll_name_offset as u32;

                let desc0_off = import_desc_offset;
                {
                    let original_first_thunk = oft_rva;
                    let time_date_stamp = 0u32;
                    let forwarder_chain = 0u32;
                    let name_rva = dll_name_rva;
                    let first_thunk = iat_rva;

                    let mut tmp = Vec::with_capacity(20);
                    push_u32(&mut tmp, original_first_thunk);
                    push_u32(&mut tmp, time_date_stamp);
                    push_u32(&mut tmp, forwarder_chain);
                    push_u32(&mut tmp, name_rva);
                    push_u32(&mut tmp, first_thunk);

                    bytes[desc0_off..desc0_off + 20].copy_from_slice(&tmp);
                }

                for i in 0..n {
                    let hn_rva = idata_rva + hint_name_offsets[i];
                    let entry = hn_rva as u64;
                    let oft_entry_off = oft_offset + i * 8;
                    let iat_entry_off = iat_offset + i * 8;
                    bytes[oft_entry_off..oft_entry_off + 8].copy_from_slice(&entry.to_le_bytes());
                    bytes[iat_entry_off..iat_entry_off + 8].copy_from_slice(&entry.to_le_bytes());
                }

                {
                    let oft_null_off = oft_offset + n * 8;
                    let iat_null_off = iat_offset + n * 8;
                    bytes[oft_null_off..oft_null_off + 8].copy_from_slice(&0u64.to_le_bytes());
                    bytes[iat_null_off..iat_null_off + 8].copy_from_slice(&0u64.to_le_bytes());
                }

                for (i, name) in IAT_ENTRIES.iter().enumerate() {
                    let off = hint_name_offsets[i] as usize;
                    bytes[off..off + 2].copy_from_slice(&0u16.to_le_bytes());
                    let name_bytes = name.as_bytes();
                    bytes[off + 2..off + 2 + name_bytes.len()].copy_from_slice(name_bytes);
                    bytes[off + 2 + name_bytes.len()] = 0;
                }

                {
                    let off = dll_name_offset;
                    let dll_bytes = IAT_DLL.as_bytes();
                    bytes[off..off + dll_bytes.len()].copy_from_slice(dll_bytes);
                    bytes[off + dll_bytes.len()] = 0;
                }

                let iat_size_bytes = ((n + 1) * 8) as u32;

                IdataBuildResult {
                    bytes,
                    slot_to_iat_rva,
                    import_dir_rva,
                    import_dir_size,
                    iat_rva,
                    iat_size: iat_size_bytes,
                    program_strings_offset,
                }
            }

            pub fn build_iat_name_to_rva_map(idata_rva: u32) -> HashMap<String, u32> {
                let result = build_idata(idata_rva, &[]);
                let mut map = HashMap::new();
                for (i, name) in IAT_ENTRIES.iter().enumerate() {
                    map.insert((*name).to_string(), result.slot_to_iat_rva[i]);
                }
                map
            }
        }
    }
}

pub mod flat_binary {
    pub struct FlatBinaryGenerator {
        org: u64,
        fixed_size: Option<usize>,
    }

    impl FlatBinaryGenerator {
        pub fn new(org: u64) -> Self {
            Self {
                org,
                fixed_size: None,
            }
        }

        pub fn set_fixed_size(&mut self, size: usize) {
            self.fixed_size = Some(size);
        }

        pub fn generate(&self, code: &[u8], data: &[u8]) -> Vec<u8> {
            let _ = self.org;
            let mut out = Vec::with_capacity(code.len() + data.len());
            out.extend_from_slice(code);
            out.extend_from_slice(data);
            if let Some(sz) = self.fixed_size {
                if out.len() < sz {
                    out.resize(sz, 0);
                } else if out.len() > sz {
                    out.truncate(sz);
                }
            }
            out
        }
    }
}

pub mod elf {
    pub fn generate_elf(
        _code: &[u8],
        _data: &[u8],
        _output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err("ELF output not implemented in this workspace snapshot".into())
    }
}

pub mod po {
    pub struct PoOutput;

    impl PoOutput {
        pub fn new() -> Self {
            Self
        }

        pub fn generate(
            &self,
            code: &[u8],
            data: &[u8],
            output_path: &str,
        ) -> Result<usize, Box<dyn std::error::Error>> {
            let header_size = 32u32;
            let code_offset = header_size;
            let code_size = code.len() as u32;
            let data_offset = code_offset + code_size;
            let data_size = data.len() as u32;

            let mut header = [0u8; 32];
            header[0..4].copy_from_slice(&0x506F4F53u32.to_le_bytes());
            header[4] = 0x80;
            header[5] = 64u8;
            header[6..8].copy_from_slice(&0u16.to_le_bytes());
            header[8..12].copy_from_slice(&code_offset.to_le_bytes());
            header[12..16].copy_from_slice(&code_size.to_le_bytes());
            header[16..20].copy_from_slice(&data_offset.to_le_bytes());
            header[20..24].copy_from_slice(&data_size.to_le_bytes());
            header[24..28].copy_from_slice(&0u32.to_le_bytes());
            header[28..32].copy_from_slice(&0u32.to_le_bytes());

            let mut bin = Vec::new();
            bin.extend_from_slice(&header);
            bin.extend_from_slice(code);
            bin.extend_from_slice(data);
            std::fs::write(output_path, &bin)?;
            Ok(bin.len())
        }
    }

    impl Default for PoOutput {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod pe {
    use crate::backend::cpu::iat_registry;

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

    pub fn generate_pe_with_offsets(
        code: &[u8],
        data: &[u8],
        output_path: &str,
        _iat_call_offsets: &[usize],
        _string_imm64_offsets: &[usize],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_alignment: u32 = 0x200;
        let section_alignment: u32 = 0x1000;

        let image_base: u64 = 0x0000000140000000;
        let text_rva: u32 = 0x1000;
        let idata_rva: u32 = 0x2000;

        let idata_result = iat_registry::build_idata(idata_rva, &[]);
        let mut idata = idata_result.bytes;
        if idata.len() != idata_result.program_strings_offset as usize {
            idata.resize(idata_result.program_strings_offset as usize, 0);
        }
        idata.extend_from_slice(data);

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

        let mut text_raw = Vec::from(code);
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
}
