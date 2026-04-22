// ============================================================
// Bridge Integration Test
// ============================================================
// Tests that adeb-bridge can parse COFF .obj files produced
// by ASM-BIB and merge them with ADead-BIB compiled code.
// ============================================================

use adeb_bridge::coff_reader::CoffObject;
use adeb_bridge::linker::BridgeLinker;

/// Build a minimal valid COFF .obj in memory (AMD64, 1 section .text with a RET)
fn build_minimal_coff_obj() -> Vec<u8> {
    let mut buf = Vec::new();

    // === COFF Header (20 bytes) ===
    let machine: u16 = 0x8664;          // AMD64
    let num_sections: u16 = 1;
    let timestamp: u32 = 0;
    let symbol_table_offset: u32 = 20 + 40 + 1; // header + section_header + code_byte
    let num_symbols: u32 = 1;
    let optional_header_size: u16 = 0;
    let characteristics: u16 = 0;

    buf.extend_from_slice(&machine.to_le_bytes());
    buf.extend_from_slice(&num_sections.to_le_bytes());
    buf.extend_from_slice(&timestamp.to_le_bytes());
    buf.extend_from_slice(&symbol_table_offset.to_le_bytes());
    buf.extend_from_slice(&num_symbols.to_le_bytes());
    buf.extend_from_slice(&optional_header_size.to_le_bytes());
    buf.extend_from_slice(&characteristics.to_le_bytes());

    // === Section Header: .text (40 bytes) ===
    let mut name = [0u8; 8];
    name[..5].copy_from_slice(b".text");
    buf.extend_from_slice(&name);
    buf.extend_from_slice(&0u32.to_le_bytes());    // VirtualSize
    buf.extend_from_slice(&0u32.to_le_bytes());    // VirtualAddress
    buf.extend_from_slice(&1u32.to_le_bytes());    // SizeOfRawData (1 byte: RET)
    buf.extend_from_slice(&60u32.to_le_bytes());   // PointerToRawData (20 + 40 = 60)
    buf.extend_from_slice(&0u32.to_le_bytes());    // PointerToRelocations
    buf.extend_from_slice(&0u32.to_le_bytes());    // PointerToLinenumbers
    buf.extend_from_slice(&0u16.to_le_bytes());    // NumberOfRelocations
    buf.extend_from_slice(&0u16.to_le_bytes());    // NumberOfLinenumbers
    buf.extend_from_slice(&0x60000020u32.to_le_bytes()); // Characteristics (CODE|EXEC|READ)

    // === Raw data: .text section (1 byte: RET = 0xC3) ===
    buf.push(0xC3);

    // === Symbol Table (1 entry, 18 bytes) ===
    // Symbol name: "asm_ret" — inline (8 bytes, null padded)
    let mut sym_name = [0u8; 8];
    sym_name[..7].copy_from_slice(b"asm_ret");
    buf.extend_from_slice(&sym_name);
    buf.extend_from_slice(&0u32.to_le_bytes());    // Value (offset 0 in .text)
    buf.extend_from_slice(&1i16.to_le_bytes());    // SectionNumber (1 = .text)
    buf.extend_from_slice(&0x20u16.to_le_bytes()); // Type (DTYPE_FUNCTION)
    buf.push(2);                                    // StorageClass (IMAGE_SYM_CLASS_EXTERNAL)
    buf.push(0);                                    // NumberOfAuxSymbols

    // === String Table (4 bytes minimum: size field only) ===
    buf.extend_from_slice(&4u32.to_le_bytes());

    buf
}

#[test]
fn test_parse_minimal_coff() {
    let data = build_minimal_coff_obj();
    let obj = CoffObject::parse(&data).expect("Failed to parse minimal COFF");

    assert_eq!(obj.machine, 0x8664);
    assert_eq!(obj.sections.len(), 1);
    assert_eq!(obj.sections[0].name, ".text");
    assert_eq!(obj.sections[0].data, vec![0xC3]); // RET

    let exported = obj.exported_symbols();
    assert_eq!(exported.len(), 1);
    assert_eq!(exported[0].name, "asm_ret");
    assert_eq!(exported[0].value, 0);
    assert!(exported[0].is_function);
}

#[test]
fn test_merge_simple() {
    let data = build_minimal_coff_obj();

    let mut linker = BridgeLinker::new();
    linker.add_obj_bytes(&data).expect("Failed to add obj");

    // Simulate C code: SUB RSP,28h; XOR ECX,ECX; CALL printf; ADD RSP,28h; RET
    let c_code: Vec<u8> = vec![
        0x48, 0x83, 0xEC, 0x28,  // sub rsp, 0x28
        0x33, 0xC9,              // xor ecx, ecx
        0xFF, 0x15, 0x00, 0x00, 0x00, 0x00, // call [rip+0] (placeholder)
        0x48, 0x83, 0xC4, 0x28,  // add rsp, 0x28
        0xC3,                    // ret
    ];
    let c_data: Vec<u8> = b"Hello\0".to_vec();

    let result = linker.merge_simple(&c_code, &c_data, &[8], &[]);

    // C code should be at start
    assert_eq!(&result.code[..c_code.len()], &c_code[..]);
    // ASM code (0xC3) should be appended after C code
    assert_eq!(result.code[c_code.len()], 0xC3);
    assert_eq!(result.asm_code_offset, c_code.len() as u32);

    // Symbol map should contain asm_ret at the right offset
    let asm_ret_offset = result.symbol_map.get("asm_ret").expect("asm_ret not in symbol map");
    assert_eq!(*asm_ret_offset, c_code.len() as u32);
}

#[test]
fn test_summary() {
    let data = build_minimal_coff_obj();
    let obj = CoffObject::parse(&data).unwrap();
    let summary = obj.summary();
    assert!(summary.contains("1 sections"));
    assert!(summary.contains("asm_ret"));
    assert!(summary.contains(".text: 1 bytes"));
}

#[test]
fn test_reject_invalid() {
    let result = CoffObject::parse(&[0; 10]);
    assert!(result.is_err());

    let mut bad = vec![0u8; 20];
    bad[0] = 0x4C; bad[1] = 0x01; // i386 instead of AMD64
    let result = CoffObject::parse(&bad);
    assert!(result.is_err());
}
