// ============================================================
// FastOS — UEFI Boot Flow (Fase 8)
// ============================================================
// Main UEFI entry point: efi_main
// 1. Print banner
// 2. Init GOP framebuffer
// 3. Get memory map
// 4. Write BootInfo at 0x9000
// 5. Exit boot services
// 6. Jump to kernel_main at 0x100000
// ============================================================

#![allow(dead_code)]

use super::types::*;
use super::gop;
use super::memory_map;

/// BootInfo address (same as BIOS boot path)
const BOOT_INFO_ADDR: u64 = 0x9000;
/// E820 map address
const E820_MAP_ADDR: u64 = 0x9100;
/// Kernel entry point
const KERNEL_ENTRY: u64 = 0x100000;
/// BootInfo magic
const BOOT_INFO_MAGIC: u32 = 0x464F5321; // "FOS!"

/// BootInfo struct (must match kernel/src/boot.rs layout exactly)
#[repr(C)]
pub struct BootInfo {
    pub magic: u32,
    pub framebuffer_addr: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_pitch: u32,
    pub framebuffer_bpp: u32,
    pub memory_map_addr: u64,
    pub memory_map_count: u32,
    pub boot_drive: u8,
    pub boot_mode: u8, // 0 = BIOS, 1 = UEFI
    pub _reserved: [u8; 2],
}

/// UEFI application entry point
///
/// Called by UEFI firmware with ImageHandle and SystemTable.
/// This function:
///   1. Prints a boot banner via ConOut
///   2. Locates GOP and sets video mode
///   3. Retrieves the UEFI memory map
///   4. Writes BootInfo at 0x9000 (compatible with BIOS path)
///   5. Calls ExitBootServices
///   6. Jumps to kernel_main at 0x100000
///
/// # Safety
/// This is a bare-metal UEFI entry point. All pointers come from firmware.
pub unsafe fn efi_main(image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
    let st = &*system_table;
    let con_out = &mut *st.con_out;
    let bs = &mut *st.boot_services;

    // 1. Clear screen and print banner
    (con_out.clear_screen)(st.con_out);
    print_ucs2(st.con_out, &ucs2_str("FastOS UEFI Boot v1.0\r\n"));
    print_ucs2(st.con_out, &ucs2_str("Initializing GOP framebuffer...\r\n"));

    // 2. Initialize GOP
    let fb = match gop::init_gop(st.boot_services) {
        Ok(fb) => {
            print_ucs2(st.con_out, &ucs2_str("  GOP: OK\r\n"));
            fb
        }
        Err(_) => {
            print_ucs2(st.con_out, &ucs2_str("  GOP: FAILED\r\n"));
            return EFI_UNSUPPORTED;
        }
    };

    // 3. Get memory map
    print_ucs2(st.con_out, &ucs2_str("Getting memory map...\r\n"));
    let (map_key, desc_count, desc_size) = match memory_map::get_memory_map(st.boot_services) {
        Ok(result) => {
            print_ucs2(st.con_out, &ucs2_str("  Memory map: OK\r\n"));
            result
        }
        Err(_) => {
            print_ucs2(st.con_out, &ucs2_str("  Memory map: FAILED\r\n"));
            return EFI_LOAD_ERROR;
        }
    };

    // 4. Convert memory map to E820 format and write BootInfo
    let e820_ptr = E820_MAP_ADDR as *mut memory_map::E820Entry;
    let e820_count = memory_map::convert_to_e820(desc_count, desc_size, e820_ptr, 128);

    let boot_info = &mut *(BOOT_INFO_ADDR as *mut BootInfo);
    boot_info.magic = BOOT_INFO_MAGIC;
    boot_info.framebuffer_addr = fb.base_addr;
    boot_info.framebuffer_width = fb.width;
    boot_info.framebuffer_height = fb.height;
    boot_info.framebuffer_pitch = fb.pixels_per_line * 4; // 32bpp = 4 bytes/pixel
    boot_info.framebuffer_bpp = 32;
    boot_info.memory_map_addr = E820_MAP_ADDR;
    boot_info.memory_map_count = e820_count as u32;
    boot_info.boot_drive = 0;
    boot_info.boot_mode = 1; // UEFI

    print_ucs2(st.con_out, &ucs2_str("Exiting boot services...\r\n"));

    // 5. Exit boot services (must re-get map_key since it may have changed)
    let (final_map_key, _, _) = match memory_map::get_memory_map(st.boot_services) {
        Ok(r) => r,
        Err(e) => return e,
    };

    if let Err(e) = memory_map::exit_boot_services(st.boot_services, image_handle, final_map_key) {
        return e;
    }

    // 6. Jump to kernel — we're now in a post-ExitBootServices environment
    // CPU is already in 64-bit long mode (UEFI guarantees this for x86_64)
    // Interrupts are disabled, paging is identity-mapped by firmware
    let kernel_entry: extern "C" fn() -> ! = core::mem::transmute(KERNEL_ENTRY as *const ());
    kernel_entry();
}

/// Print a UCS-2 string via ConOut
unsafe fn print_ucs2(con_out: *mut EfiSimpleTextOutputProtocol, s: &[u16]) {
    ((*con_out).output_string)(con_out, s.as_ptr());
}

/// Convert ASCII string to UCS-2 (null-terminated, max 128 chars)
fn ucs2_str(s: &str) -> [u16; 128] {
    let mut buf = [0u16; 128];
    for (i, b) in s.bytes().enumerate() {
        if i >= 127 { break; }
        buf[i] = b as u16;
    }
    buf
}
