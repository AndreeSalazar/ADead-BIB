// ============================================================
// Real .obj Integration Test
// ============================================================
// Tests that adeb-bridge can read a REAL COFF .obj file
// produced by ASM-BIB's native encoder from stdlib_ring3.pasm.
//
// This test reads the .obj from the ASM-BIB examples/bridge/
// directory and verifies all exported symbols are found.
// ============================================================

use adeb_bridge::coff_reader::CoffObject;
use adeb_bridge::linker::BridgeLinker;

/// Path to the real ASM-BIB generated .obj file
/// Adjust this path if your workspace layout differs.
const OBJ_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../../../ASM-BIB/examples/bridge/stdlib_ring3_native.obj"
);

#[test]
fn test_read_real_asm_bib_obj() {
    // Try to load the file — skip test if not present
    let data = match std::fs::read(OBJ_PATH) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("SKIP: Cannot read {}: {} — run ASM-BIB --native --obj first", OBJ_PATH, e);
            return;
        }
    };

    let obj = CoffObject::parse(&data).expect("Failed to parse ASM-BIB .obj");

    // Verify it's AMD64
    assert_eq!(obj.machine, 0x8664, "Must be AMD64 COFF");

    // Print summary for diagnostics
    println!("{}", obj.summary());

    // Verify .text section exists and has code
    let text = obj.text_section().expect(".text section must exist");
    assert!(!text.data.is_empty(), ".text must contain code bytes");
    println!(".text size: {} bytes", text.data.len());

    // Verify exported symbols — all 20 functions from stdlib_ring3.pasm
    let exported = obj.exported_symbols();
    let names: Vec<&str> = exported.iter().map(|s| s.name.as_str()).collect();
    println!("Exported symbols ({}):", names.len());
    for name in &names {
        println!("  - {}", name);
    }

    // Check all expected functions are present
    let expected = [
        "asm_strlen", "asm_strcpy", "asm_strcmp", "asm_strcat", "asm_strchr",
        "asm_memcpy", "asm_memset", "asm_memcmp",
        "asm_abs", "asm_min", "asm_max", "asm_clamp", "asm_swap",
        "asm_popcount", "asm_bsr64", "asm_bsf64", "asm_bswap32", "asm_bswap64",
        "asm_is_aligned", "asm_align_up", "asm_noop",
    ];

    for func in &expected {
        assert!(
            names.contains(func),
            "Missing exported symbol: '{}'. Found: {:?}",
            func, names
        );
    }

    assert!(
        exported.len() >= expected.len(),
        "Expected at least {} exports, got {}",
        expected.len(), exported.len()
    );
}

#[test]
fn test_merge_real_obj_with_c_code() {
    let data = match std::fs::read(OBJ_PATH) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("SKIP: .obj not found");
            return;
        }
    };

    let mut linker = BridgeLinker::new();
    linker.add_obj_bytes(&data).expect("Failed to add obj");

    // Simulate ADead-BIB C code that calls asm_strlen
    // sub rsp, 28h; lea rcx, [rip+data]; call asm_strlen; add rsp, 28h; ret
    let c_code: Vec<u8> = vec![
        0x48, 0x83, 0xEC, 0x28,             // sub rsp, 0x28
        0x48, 0x8D, 0x0D, 0x00, 0x00, 0x00, 0x00, // lea rcx, [rip+0] (placeholder)
        0xE8, 0x00, 0x00, 0x00, 0x00,       // call asm_strlen (placeholder, needs patching)
        0x48, 0x83, 0xC4, 0x28,             // add rsp, 0x28
        0xC3,                                // ret
    ];
    let c_data = b"Hello from ADead-BIB!\0".to_vec();

    let result = linker.merge_simple(&c_code, &c_data, &[], &[]);

    // Verify merged output
    assert!(result.code.len() > c_code.len(), "ASM code must be appended");
    assert_eq!(result.asm_code_offset, c_code.len() as u32);

    // Verify all ASM symbols are in the symbol map
    let sym_map = &result.symbol_map;
    assert!(sym_map.contains_key("asm_strlen"), "asm_strlen must be in symbol map");
    assert!(sym_map.contains_key("asm_memcpy"), "asm_memcpy must be in symbol map");
    assert!(sym_map.contains_key("asm_abs"), "asm_abs must be in symbol map");
    assert!(sym_map.contains_key("asm_noop"), "asm_noop must be in symbol map");

    // Verify symbol offsets are >= asm_code_offset (they're in the ASM region)
    for (name, offset) in sym_map {
        println!("  {} @ offset 0x{:04X}", name, offset);
        assert!(*offset >= result.asm_code_offset,
            "Symbol '{}' at 0x{:X} should be >= ASM start 0x{:X}",
            name, offset, result.asm_code_offset);
    }

    println!("\nMerge successful:");
    println!("  C code:   {} bytes", c_code.len());
    println!("  ASM code: {} bytes (starts at 0x{:X})", 
        result.code.len() - c_code.len(), result.asm_code_offset);
    println!("  Total:    {} bytes code, {} bytes data",
        result.code.len(), result.data.len());
    println!("  Symbols:  {}", sym_map.len());
}
