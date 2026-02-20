// ============================================================
// FastOS — UEFI GOP (Graphics Output Protocol) (Fase 8)
// ============================================================
// Locates and configures the GOP framebuffer for graphical output.
// Replaces VBE (INT 10h) used in BIOS boot path.
// ============================================================

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use super::types::*;

/// GOP Pixel Format
#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum EfiGraphicsPixelFormat {
    PixelRedGreenBlueReserved8BitPerColor = 0,
    PixelBlueGreenRedReserved8BitPerColor = 1,
    PixelBitMask = 2,
    PixelBltOnly = 3,
}

/// GOP Pixel Bitmask
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EfiPixelBitmask {
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub reserved_mask: u32,
}

/// GOP Mode Information
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EfiGraphicsOutputModeInformation {
    pub version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: EfiGraphicsPixelFormat,
    pub pixel_information: EfiPixelBitmask,
    pub pixels_per_scan_line: u32,
}

/// GOP Mode
#[repr(C)]
pub struct EfiGraphicsOutputProtocolMode {
    pub max_mode: u32,
    pub mode: u32,
    pub info: *mut EfiGraphicsOutputModeInformation,
    pub size_of_info: usize,
    pub frame_buffer_base: u64,
    pub frame_buffer_size: usize,
}

/// EFI_GRAPHICS_OUTPUT_PROTOCOL
#[repr(C)]
pub struct EfiGraphicsOutputProtocol {
    pub query_mode: unsafe extern "efiapi" fn(
        this: *mut EfiGraphicsOutputProtocol,
        mode_number: u32,
        size_of_info: *mut usize,
        info: *mut *mut EfiGraphicsOutputModeInformation,
    ) -> EfiStatus,
    pub set_mode: unsafe extern "efiapi" fn(
        this: *mut EfiGraphicsOutputProtocol,
        mode_number: u32,
    ) -> EfiStatus,
    pub blt: usize, // Not used — we write directly to framebuffer
    pub mode: *mut EfiGraphicsOutputProtocolMode,
}

/// Framebuffer info extracted from GOP
#[derive(Clone, Copy)]
pub struct GopFramebuffer {
    pub base_addr: u64,
    pub width: u32,
    pub height: u32,
    pub pixels_per_line: u32,
    pub pixel_format: EfiGraphicsPixelFormat,
    pub size: usize,
}

/// Locate GOP and find the best video mode (prefer 1024x768 or highest available)
///
/// # Safety
/// Requires valid UEFI boot services pointer.
pub unsafe fn init_gop(boot_services: *mut EfiBootServices) -> Result<GopFramebuffer, EfiStatus> {
    let mut gop_ptr: *mut core::ffi::c_void = core::ptr::null_mut();

    let status = ((*boot_services).locate_protocol)(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID as *const EfiGuid,
        core::ptr::null_mut(),
        &mut gop_ptr,
    );

    if status != EFI_SUCCESS || gop_ptr.is_null() {
        return Err(EFI_NOT_FOUND);
    }

    let gop = gop_ptr as *mut EfiGraphicsOutputProtocol;
    let max_mode = (*(*gop).mode).max_mode;

    // Find best mode: prefer 1024x768, fallback to highest resolution
    let mut best_mode: u32 = (*(*gop).mode).mode; // Current mode as default
    let mut best_width: u32 = 0;
    let mut best_height: u32 = 0;
    let mut found_preferred = false;

    for mode_num in 0..max_mode {
        let mut info_size: usize = 0;
        let mut info_ptr: *mut EfiGraphicsOutputModeInformation = core::ptr::null_mut();

        let qs = ((*gop).query_mode)(gop, mode_num, &mut info_size, &mut info_ptr);
        if qs != EFI_SUCCESS || info_ptr.is_null() { continue; }

        let info = &*info_ptr;

        // Prefer 1024x768 (matches BIOS VBE mode)
        if info.horizontal_resolution == 1024 && info.vertical_resolution == 768 {
            best_mode = mode_num;
            best_width = 1024;
            best_height = 768;
            found_preferred = true;
            break;
        }

        // Track highest resolution as fallback
        let pixels = info.horizontal_resolution * info.vertical_resolution;
        if pixels > best_width * best_height && !found_preferred {
            best_mode = mode_num;
            best_width = info.horizontal_resolution;
            best_height = info.vertical_resolution;
        }
    }

    // Set the chosen mode
    if best_mode != (*(*gop).mode).mode {
        let _ = ((*gop).set_mode)(gop, best_mode);
    }

    // Read final mode info
    let mode = &*(*gop).mode;
    let info = &*mode.info;

    Ok(GopFramebuffer {
        base_addr: mode.frame_buffer_base,
        width: info.horizontal_resolution,
        height: info.vertical_resolution,
        pixels_per_line: info.pixels_per_scan_line,
        pixel_format: info.pixel_format,
        size: mode.frame_buffer_size,
    })
}
