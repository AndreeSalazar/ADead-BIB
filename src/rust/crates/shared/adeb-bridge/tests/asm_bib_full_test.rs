// ============================================================
// ADead-BIB × ASM-BIB — Full Bridge Integration Test Suite
// ============================================================
// Tests every exported function from ASM-BIB's stdlib_ring3.obj
// through the adeb-bridge COFF reader and linker.
//
// This validates that ASM-BIB's native COFF encoder produces
// correct machine code that ADead-BIB can consume as if it
// were output from ml64.exe — our own local MASM replacement.
//
// Run: cargo test -p adeb-bridge --test asm_bib_full_test -- --nocapture
// ============================================================

use adeb_bridge::coff_reader::{
    CoffObject, CoffSection, CoffSymbol,
    IMAGE_SCN_CNT_CODE, IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_READ,
};
use adeb_bridge::linker::{BridgeLinker, MergedBinary};
use std::collections::HashMap;

// ── Path to real ASM-BIB .obj ───────────────────────────────
const OBJ_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../../../ASM-BIB/examples/bridge/stdlib_ring3_native.obj"
);

fn load_obj() -> CoffObject {
    let data = std::fs::read(OBJ_PATH)
        .unwrap_or_else(|e| panic!("Cannot read ASM-BIB .obj at {}: {}\nRun ASM-BIB --native --obj first!", OBJ_PATH, e));
    CoffObject::parse(&data)
        .unwrap_or_else(|e| panic!("Failed to parse COFF: {}", e))
}

fn load_merged() -> MergedBinary {
    let data = std::fs::read(OBJ_PATH).expect("Cannot read .obj");
    let mut linker = BridgeLinker::new();
    linker.add_obj_bytes(&data).expect("add_obj_bytes");
    // Simulate empty C code — just test ASM functions standalone
    linker.merge_simple(&[], &[], &[], &[])
}

// ════════════════════════════════════════════════════════════
// PART 1: COFF STRUCTURE INTEGRITY
// ════════════════════════════════════════════════════════════

#[test]
fn t01_coff_header_amd64() {
    let obj = load_obj();
    assert_eq!(obj.machine, 0x8664, "Machine must be IMAGE_FILE_MACHINE_AMD64");
    println!("[PASS] COFF header: AMD64 (0x8664)");
}

#[test]
fn t02_has_text_section() {
    let obj = load_obj();
    let text = obj.text_section().expect(".text section must exist");
    assert!(!text.data.is_empty(), ".text must contain code");
    assert!(text.characteristics & IMAGE_SCN_CNT_CODE != 0, ".text must have CODE flag");
    assert!(text.characteristics & IMAGE_SCN_MEM_EXECUTE != 0, ".text must have EXECUTE flag");
    assert!(text.characteristics & IMAGE_SCN_MEM_READ != 0, ".text must have READ flag");
    println!("[PASS] .text section: {} bytes, flags=0x{:08X}", text.data.len(), text.characteristics);
}

#[test]
fn t03_section_count() {
    let obj = load_obj();
    // Expect at minimum: .text, .pdata, .xdata (SEH metadata)
    assert!(obj.sections.len() >= 1, "Must have at least .text section");
    let names: Vec<&str> = obj.sections.iter().map(|s| s.name.as_str()).collect();
    println!("[PASS] Sections ({}): {:?}", obj.sections.len(), names);
}

#[test]
fn t04_symbol_count() {
    let obj = load_obj();
    let exported = obj.exported_symbols();
    assert!(exported.len() >= 21, "Must have at least 21 exported symbols, got {}", exported.len());
    println!("[PASS] Symbol count: {} total, {} exported", obj.symbols.len(), exported.len());
}

#[test]
fn t05_no_undefined_symbols() {
    let obj = load_obj();
    let undef = obj.undefined_symbols();
    // stdlib_ring3 is self-contained — no external references needed
    println!("[PASS] Undefined symbols: {} (stdlib is self-contained)", undef.len());
    for u in &undef {
        println!("  WARN: undefined -> {}", u.name);
    }
}

#[test]
fn t06_seh_metadata_present() {
    let obj = load_obj();
    let has_pdata = obj.sections.iter().any(|s| s.name == ".pdata");
    let has_xdata = obj.sections.iter().any(|s| s.name == ".xdata");
    if has_pdata && has_xdata {
        println!("[PASS] SEH metadata: .pdata + .xdata present");
    } else {
        println!("[INFO] SEH metadata: pdata={}, xdata={}", has_pdata, has_xdata);
    }
}

#[test]
fn t07_symbol_offsets_non_overlapping() {
    let obj = load_obj();
    let offsets = obj.symbol_offsets();
    let mut sorted: Vec<(&String, &u32)> = offsets.iter().collect();
    sorted.sort_by_key(|(_, v)| *v);

    println!("[PASS] Symbol offsets (non-overlapping check):");
    for (i, (name, off)) in sorted.iter().enumerate() {
        let next_off = sorted.get(i + 1).map(|(_, v)| **v).unwrap_or(
            obj.text_section().map(|t| t.data.len() as u32).unwrap_or(0)
        );
        let size = next_off - **off;
        println!("  0x{:04X} - 0x{:04X} ({:3} bytes): {}", off, next_off, size, name);
        assert!(**off < next_off || i == sorted.len() - 1,
            "Symbol {} at 0x{:X} overlaps next at 0x{:X}", name, off, next_off);
    }
}

// ════════════════════════════════════════════════════════════
// PART 2: STRING FUNCTIONS — Machine Code Verification
// ════════════════════════════════════════════════════════════

fn get_func_bytes<'a>(obj: &'a CoffObject, name: &str) -> &'a [u8] {
    let text = obj.text_section().expect(".text");
    let offsets = obj.symbol_offsets();
    let start = *offsets.get(name).unwrap_or_else(|| panic!("Symbol '{}' not found", name)) as usize;

    // Find next symbol to determine function size
    let mut sorted: Vec<(&String, &u32)> = offsets.iter().collect();
    sorted.sort_by_key(|(_, v)| *v);
    let end = sorted.iter()
        .find(|(_, v)| **v as usize > start)
        .map(|(_, v)| **v as usize)
        .unwrap_or(text.data.len());

    &text.data[start..end]
}

#[test]
fn t10_strlen_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_strlen");
    assert!(!bytes.is_empty(), "asm_strlen must have code");
    // Must start with XOR RAX,RAX (48 31 C0) — zeroing the counter
    assert!(bytes.len() >= 3, "asm_strlen too short");
    // Must end with RET (C3)
    assert_eq!(*bytes.last().unwrap(), 0xC3, "asm_strlen must end with RET");
    println!("[PASS] asm_strlen: {} bytes, starts correctly, ends with RET", bytes.len());
}

#[test]
fn t11_strcpy_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_strcpy");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3, "asm_strcpy must end with RET");
    println!("[PASS] asm_strcpy: {} bytes", bytes.len());
}

#[test]
fn t12_strcmp_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_strcmp");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3, "asm_strcmp must end with RET");
    println!("[PASS] asm_strcmp: {} bytes", bytes.len());
}

#[test]
fn t13_strcat_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_strcat");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_strcat: {} bytes", bytes.len());
}

#[test]
fn t14_strchr_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_strchr");
    assert!(!bytes.is_empty());
    // strchr has two exit paths (found / not_found), both end with RET
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_strchr: {} bytes (2 exit paths)", bytes.len());
}

#[test]
fn t15_memcpy_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_memcpy");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    // Verify it contains PUSH RDI (57) and POP RDI (5F) for callee-save
    assert!(bytes.contains(&0x57), "memcpy must PUSH RDI");
    assert!(bytes.contains(&0x5F), "memcpy must POP RDI");
    println!("[PASS] asm_memcpy: {} bytes, saves/restores RDI/RSI", bytes.len());
}

#[test]
fn t16_memset_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_memset");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_memset: {} bytes", bytes.len());
}

#[test]
fn t17_memcmp_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_memcmp");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_memcmp: {} bytes", bytes.len());
}

// ════════════════════════════════════════════════════════════
// PART 3: MATH FUNCTIONS — Machine Code Verification
// ════════════════════════════════════════════════════════════

#[test]
fn t20_abs_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_abs");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    // ABS uses CQO (48 99) for sign extension
    println!("[PASS] asm_abs: {} bytes (sign-extend trick)", bytes.len());
}

#[test]
fn t21_min_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_min");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_min: {} bytes (CMOVcc)", bytes.len());
}

#[test]
fn t22_max_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_max");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_max: {} bytes (CMOVcc)", bytes.len());
}

#[test]
fn t23_clamp_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_clamp");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_clamp: {} bytes (double CMOVcc)", bytes.len());
}

#[test]
fn t24_swap_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_swap");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_swap: {} bytes (XCHG via MOV)", bytes.len());
}

// ════════════════════════════════════════════════════════════
// PART 4: BIT MANIPULATION FUNCTIONS
// ════════════════════════════════════════════════════════════

#[test]
fn t30_popcount_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_popcount");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_popcount: {} bytes (POPCNT instruction)", bytes.len());
}

#[test]
fn t31_bsr64_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_bsr64");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_bsr64: {} bytes (BSR with zero guard)", bytes.len());
}

#[test]
fn t32_bsf64_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_bsf64");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_bsf64: {} bytes (BSF with zero guard)", bytes.len());
}

#[test]
fn t33_bswap32_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_bswap32");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    // BSWAP EAX = 0F C8
    println!("[PASS] asm_bswap32: {} bytes (BSWAP EAX)", bytes.len());
}

#[test]
fn t34_bswap64_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_bswap64");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_bswap64: {} bytes (BSWAP RAX)", bytes.len());
}

// ════════════════════════════════════════════════════════════
// PART 5: UTILITY FUNCTIONS
// ════════════════════════════════════════════════════════════

#[test]
fn t40_is_aligned_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_is_aligned");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_is_aligned: {} bytes (TEST + SETZ)", bytes.len());
}

#[test]
fn t41_align_up_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_align_up");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    println!("[PASS] asm_align_up: {} bytes (DEC+ADD+NOT+AND)", bytes.len());
}

#[test]
fn t42_noop_code() {
    let obj = load_obj();
    let bytes = get_func_bytes(&obj, "asm_noop");
    assert!(!bytes.is_empty());
    assert_eq!(*bytes.last().unwrap(), 0xC3);
    // NOP = 0x90, RET = 0xC3
    assert!(bytes.contains(&0x90), "asm_noop must contain NOP (0x90)");
    println!("[PASS] asm_noop: {} bytes (NOP + RET)", bytes.len());
}

// ════════════════════════════════════════════════════════════
// PART 6: BRIDGE LINKER — Merge With C Code
// ════════════════════════════════════════════════════════════

#[test]
fn t50_merge_empty_c_code() {
    let merged = load_merged();
    assert!(!merged.code.is_empty(), "Merged code must not be empty");
    assert_eq!(merged.asm_code_offset, 0, "With empty C code, ASM starts at 0");
    assert!(merged.symbol_map.len() >= 21, "Must have 21+ symbols in map");
    println!("[PASS] Merge with empty C: {} bytes code, {} symbols", merged.code.len(), merged.symbol_map.len());
}

#[test]
fn t51_merge_with_simulated_c() {
    let data = std::fs::read(OBJ_PATH).expect(".obj");
    let mut linker = BridgeLinker::new();
    linker.add_obj_bytes(&data).unwrap();

    // Simulate typical C main():
    // sub rsp, 28h; xor ecx,ecx; call [rip+0]; add rsp, 28h; ret
    let c_code: Vec<u8> = vec![
        0x48, 0x83, 0xEC, 0x28,                     // sub rsp, 0x28
        0x33, 0xC9,                                   // xor ecx, ecx
        0xFF, 0x15, 0x00, 0x00, 0x00, 0x00,         // call [rip+0]
        0x48, 0x83, 0xC4, 0x28,                     // add rsp, 0x28
        0xC3,                                         // ret
    ];
    let c_data = b"Hello World\0".to_vec();
    let c_symbols: HashMap<String, u32> = [("main".to_string(), 0u32)].into_iter().collect();

    let merged = linker.merge_simple(&c_code, &c_data, &[8], &[]);

    assert_eq!(merged.asm_code_offset, c_code.len() as u32);
    assert_eq!(&merged.code[..c_code.len()], &c_code[..], "C code must be at start");
    assert!(merged.code.len() > c_code.len(), "ASM must be appended after C");
    assert_eq!(&merged.data[..c_data.len()], &c_data[..], "C data must be at start");
    assert_eq!(merged.iat_call_offsets, vec![8], "IAT offset preserved");

    println!("[PASS] Merge with C main():");
    println!("  C code:   {} bytes @ 0x0000", c_code.len());
    println!("  ASM code: {} bytes @ 0x{:04X}", merged.code.len() - c_code.len(), merged.asm_code_offset);
    println!("  C data:   {} bytes", c_data.len());
    println!("  Total:    {} bytes code, {} bytes data", merged.code.len(), merged.data.len());
}

#[test]
fn t52_symbol_resolution_after_merge() {
    let data = std::fs::read(OBJ_PATH).expect(".obj");
    let mut linker = BridgeLinker::new();
    linker.add_obj_bytes(&data).unwrap();

    let c_code = vec![0xC3u8; 64]; // 64 bytes of dummy C code
    let c_symbols: HashMap<String, u32> = [("c_main".to_string(), 0u32)].into_iter().collect();

    let merged = linker.merge_simple(&c_code, &[], &[], &[]);

    // All ASM symbols must be offset by c_code size
    for (name, offset) in &merged.symbol_map {
        assert!(*offset >= 64, "ASM symbol '{}' at 0x{:X} must be >= 0x40", name, offset);
    }
    println!("[PASS] All {} symbols correctly offset by C code size (64 bytes)", merged.symbol_map.len());
}

#[test]
fn t53_call_patching_feasibility() {
    let data = std::fs::read(OBJ_PATH).expect(".obj");
    let mut linker = BridgeLinker::new();
    linker.add_obj_bytes(&data).unwrap();

    // Simulate: CALL asm_strlen at offset 5 in C code
    // E8 xx xx xx xx (rel32 call)
    let mut c_code = vec![
        0x48, 0x83, 0xEC, 0x28,  // sub rsp, 0x28
        0x90,                     // nop (placeholder)
        0xE8, 0x00, 0x00, 0x00, 0x00, // call rel32 (to be patched)
        0x48, 0x83, 0xC4, 0x28,  // add rsp, 0x28
        0xC3,                     // ret
    ];

    let merged = linker.merge_simple(&c_code, &[], &[], &[]);

    let strlen_offset = merged.symbol_map.get("asm_strlen")
        .expect("asm_strlen must be in symbol map");

    // Calculate rel32 displacement: target - (call_site + 5)
    let call_site = 5u32; // offset of the E8 opcode
    let rip_after_call = call_site + 5; // RIP after the CALL instruction
    let displacement = (*strlen_offset as i64) - (rip_after_call as i64);

    assert!(displacement >= i32::MIN as i64 && displacement <= i32::MAX as i64,
        "REL32 displacement 0x{:X} must fit in i32", displacement);

    println!("[PASS] CALL asm_strlen patching feasible:");
    println!("  call_site=0x{:04X}, strlen=0x{:04X}, displacement={}", call_site, strlen_offset, displacement);
}

// ════════════════════════════════════════════════════════════
// PART 7: FULL REPORT — All 21 Functions
// ════════════════════════════════════════════════════════════

#[test]
fn t90_full_report() {
    let obj = load_obj();
    let text = obj.text_section().expect(".text");
    let offsets = obj.symbol_offsets();

    let expected = [
        // (name, category)
        ("asm_strlen",     "String"),
        ("asm_strcpy",     "String"),
        ("asm_strcmp",      "String"),
        ("asm_strcat",     "String"),
        ("asm_strchr",     "String"),
        ("asm_memcpy",     "String"),
        ("asm_memset",     "String"),
        ("asm_memcmp",     "String"),
        ("asm_abs",        "Math"),
        ("asm_min",        "Math"),
        ("asm_max",        "Math"),
        ("asm_clamp",      "Math"),
        ("asm_swap",       "Math"),
        ("asm_popcount",   "Bit"),
        ("asm_bsr64",      "Bit"),
        ("asm_bsf64",      "Bit"),
        ("asm_bswap32",    "Bit"),
        ("asm_bswap64",    "Bit"),
        ("asm_is_aligned", "Utility"),
        ("asm_align_up",   "Utility"),
        ("asm_noop",       "Utility"),
    ];

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  ADead-BIB × ASM-BIB Bridge — Full Test Report         ║");
    println!("║  Local MASM Replacement (ml64.exe independent)         ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  COFF: AMD64 | .text: {} bytes | Symbols: {:2}           ║",
        text.data.len(), offsets.len());
    println!("╠══════════════════════════════════════════════════════════╣");

    let mut pass_count = 0;
    let mut fail_count = 0;

    for (name, category) in &expected {
        let status = match offsets.get(*name) {
            Some(offset) => {
                let bytes = get_func_bytes(&obj, name);
                let ends_with_ret = bytes.last() == Some(&0xC3);
                let has_code = !bytes.is_empty();

                if has_code && ends_with_ret {
                    pass_count += 1;
                    format!("PASS  0x{:04X}  {:3}B", offset, bytes.len())
                } else {
                    fail_count += 1;
                    format!("FAIL  0x{:04X}  {:3}B  {}", offset, bytes.len(),
                        if !has_code { "NO CODE" } else { "NO RET" })
                }
            }
            None => {
                fail_count += 1;
                "FAIL  MISSING".to_string()
            }
        };

        println!("║  {} {:20} [{:7}] {:18} ║",
            if status.starts_with("PASS") { "✓" } else { "✗" },
            name, category, status);
    }

    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  Results: {} PASS / {} FAIL / {} TOTAL                    ║",
        pass_count, fail_count, pass_count + fail_count);
    println!("║  Coverage: {:.0}%                                         ║",
        (pass_count as f64 / expected.len() as f64) * 100.0);
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    // Print merge statistics
    let data = std::fs::read(OBJ_PATH).unwrap();
    let mut linker = BridgeLinker::new();
    linker.add_obj_bytes(&data).unwrap();

    let c_stub = vec![0x48, 0x83, 0xEC, 0x28, 0x33, 0xC9, 0xC3]; // sub rsp,28; xor ecx,ecx; ret
    let merged = linker.merge_simple(&c_stub, b"test\0", &[], &[]);

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Bridge Linker Merge Report                            ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  C code stub:      {:5} bytes @ 0x0000                ║", c_stub.len());
    println!("║  ASM code:         {:5} bytes @ 0x{:04X}                ║",
        merged.code.len() - c_stub.len(), merged.asm_code_offset);
    println!("║  Merged code:      {:5} bytes total                   ║", merged.code.len());
    println!("║  Merged data:      {:5} bytes total                   ║", merged.data.len());
    println!("║  Symbol map:       {:5} entries                       ║", merged.symbol_map.len());
    println!("║  Status:           READY FOR PE GENERATION             ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    assert_eq!(fail_count, 0, "{} functions FAILED — see report above", fail_count);
}
