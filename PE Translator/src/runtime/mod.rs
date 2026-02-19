// ============================================================
// ADead Runtime — Minimal startup code for translated binaries
// ============================================================
// When a BIB module has no explicit entry point or needs
// runtime initialization, this module provides the startup
// code that runs before main().
//
// Responsibilities:
//   - Stack setup (already done by OS loader)
//   - TLS initialization (future)
//   - Heap initialization (future)
//   - Call main()
//   - Call ExitProcess/exit()
//
// The runtime is injected as a code prefix before the user's
// code in the .text section.
// ============================================================

/// Generate x86-64 runtime init stub for Windows PE
/// This stub:
///   1. sub rsp, 40  (shadow space for Win64 ABI)
///   2. call [main_offset] (relative)
///   3. mov ecx, eax (exit code)
///   4. call [ExitProcess IAT entry] (indirect)
///   5. int3 (should never reach here)
pub fn windows_runtime_stub(main_offset: i32, exit_process_iat_rva: u32, _image_base_relative: bool) -> Vec<u8> {
    let mut code = Vec::new();

    // sub rsp, 40 (0x28) — shadow space
    code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);

    // call rel32 [main]
    code.push(0xE8);
    // The offset is relative to the next instruction
    // main_offset is from the END of this call instruction
    let call_offset = main_offset - 5; // adjust for the 5-byte call instruction itself
    code.extend_from_slice(&(call_offset as i32).to_le_bytes());

    // mov ecx, eax — pass return value as exit code
    code.extend_from_slice(&[0x89, 0xC1]);

    // call qword [rip + exit_process_offset]
    // This is an indirect call through the IAT
    // FF 15 xx xx xx xx = call [rip + disp32]
    code.extend_from_slice(&[0xFF, 0x15]);
    // The displacement is relative to the next instruction (RIP after this instruction)
    // We'll need the caller to fix this up based on actual layout
    code.extend_from_slice(&exit_process_iat_rva.to_le_bytes());

    // int3 — trap if ExitProcess returns (it shouldn't)
    code.push(0xCC);

    code
}

/// Generate x86-64 runtime init stub for Linux ELF
/// Uses syscall to exit
pub fn linux_runtime_stub(main_offset: i32) -> Vec<u8> {
    let mut code = Vec::new();

    // call rel32 [main]
    code.push(0xE8);
    let call_offset = main_offset - 5;
    code.extend_from_slice(&(call_offset as i32).to_le_bytes());

    // mov edi, eax — exit code
    code.extend_from_slice(&[0x89, 0xC7]);

    // mov eax, 60 — sys_exit
    code.extend_from_slice(&[0xB8, 0x3C, 0x00, 0x00, 0x00]);

    // syscall
    code.extend_from_slice(&[0x0F, 0x05]);

    code
}

/// Generate minimal stub for FastOS (just jump to main)
pub fn fastos_runtime_stub(main_offset: i32) -> Vec<u8> {
    let mut code = Vec::new();

    // call rel32 [main]
    code.push(0xE8);
    let call_offset = main_offset - 5;
    code.extend_from_slice(&(call_offset as i32).to_le_bytes());

    // hlt — halt after main returns
    code.push(0xF4);

    // jmp $ — infinite loop as safety net
    code.extend_from_slice(&[0xEB, 0xFE]);

    code
}

/// Size of the Windows runtime stub
pub const WINDOWS_STUB_SIZE: usize = 16;

/// Size of the Linux runtime stub
pub const LINUX_STUB_SIZE: usize = 12;

/// Size of the FastOS runtime stub
pub const FASTOS_STUB_SIZE: usize = 8;
