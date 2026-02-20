// ============================================================
// FastOS — UEFI Type Definitions (Fase 8)
// ============================================================
// Core UEFI types matching the UEFI 2.10 specification.
// These are used by the UEFI boot application.
// ============================================================

#![allow(non_camel_case_types)]
#![allow(dead_code)]

/// UEFI Status codes
pub type EfiStatus = usize;
pub const EFI_SUCCESS: EfiStatus = 0;
pub const EFI_LOAD_ERROR: EfiStatus = 1;
pub const EFI_INVALID_PARAMETER: EfiStatus = 2;
pub const EFI_UNSUPPORTED: EfiStatus = 3;
pub const EFI_BUFFER_TOO_SMALL: EfiStatus = 5;
pub const EFI_NOT_FOUND: EfiStatus = 14;

/// UEFI Handle
pub type EfiHandle = *mut core::ffi::c_void;

/// UEFI Boolean
pub type EfiBool = u8;

/// UEFI GUID (128-bit)
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct EfiGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// EFI_SYSTEM_TABLE
#[repr(C)]
pub struct EfiSystemTable {
    pub hdr: EfiTableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    pub con_in: *mut EfiSimpleTextInputProtocol,
    pub console_out_handle: EfiHandle,
    pub con_out: *mut EfiSimpleTextOutputProtocol,
    pub standard_error_handle: EfiHandle,
    pub std_err: *mut EfiSimpleTextOutputProtocol,
    pub runtime_services: *mut EfiRuntimeServices,
    pub boot_services: *mut EfiBootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *mut EfiConfigurationTable,
}

/// EFI_TABLE_HEADER
#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

/// EFI_BOOT_SERVICES (partial — key functions only)
#[repr(C)]
pub struct EfiBootServices {
    pub hdr: EfiTableHeader,
    // Task Priority Services
    pub raise_tpl: usize,
    pub restore_tpl: usize,
    // Memory Services
    pub allocate_pages: unsafe extern "efiapi" fn(
        alloc_type: u32, mem_type: u32, pages: usize, memory: *mut u64
    ) -> EfiStatus,
    pub free_pages: unsafe extern "efiapi" fn(memory: u64, pages: usize) -> EfiStatus,
    pub get_memory_map: unsafe extern "efiapi" fn(
        map_size: *mut usize,
        map: *mut EfiMemoryDescriptor,
        map_key: *mut usize,
        descriptor_size: *mut usize,
        descriptor_version: *mut u32,
    ) -> EfiStatus,
    pub allocate_pool: unsafe extern "efiapi" fn(
        pool_type: u32, size: usize, buffer: *mut *mut u8
    ) -> EfiStatus,
    pub free_pool: unsafe extern "efiapi" fn(buffer: *mut u8) -> EfiStatus,
    // Event & Timer Services (6 entries)
    pub _event_padding: [usize; 6],
    // Protocol Handler Services
    pub _protocol_padding: [usize; 9],
    // Image Services
    pub _image_padding: [usize; 5],
    // Miscellaneous Services
    pub _misc_padding: [usize; 2],
    // LocateProtocol
    pub locate_protocol: unsafe extern "efiapi" fn(
        protocol: *const EfiGuid,
        registration: *mut core::ffi::c_void,
        interface: *mut *mut core::ffi::c_void,
    ) -> EfiStatus,
    // ... remaining services
    pub _remaining_padding: [usize; 5],
    pub exit_boot_services: unsafe extern "efiapi" fn(
        image_handle: EfiHandle, map_key: usize
    ) -> EfiStatus,
}

/// EFI_RUNTIME_SERVICES (opaque — not used during boot)
#[repr(C)]
pub struct EfiRuntimeServices {
    pub hdr: EfiTableHeader,
    // We don't need runtime services during boot
}

/// EFI_CONFIGURATION_TABLE
#[repr(C)]
pub struct EfiConfigurationTable {
    pub vendor_guid: EfiGuid,
    pub vendor_table: *mut core::ffi::c_void,
}

/// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: usize,
    pub output_string: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        string: *const u16,
    ) -> EfiStatus,
    pub test_string: usize,
    pub query_mode: usize,
    pub set_mode: usize,
    pub set_attribute: usize,
    pub clear_screen: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
    ) -> EfiStatus,
}

/// EFI_SIMPLE_TEXT_INPUT_PROTOCOL (opaque)
#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    _data: [u8; 0],
}

/// EFI_MEMORY_DESCRIPTOR
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EfiMemoryDescriptor {
    pub memory_type: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

/// Memory type constants
pub mod memory_type {
    pub const RESERVED: u32 = 0;
    pub const LOADER_CODE: u32 = 1;
    pub const LOADER_DATA: u32 = 2;
    pub const BOOT_SERVICES_CODE: u32 = 3;
    pub const BOOT_SERVICES_DATA: u32 = 4;
    pub const RUNTIME_SERVICES_CODE: u32 = 5;
    pub const RUNTIME_SERVICES_DATA: u32 = 6;
    pub const CONVENTIONAL_MEMORY: u32 = 7;
    pub const UNUSABLE_MEMORY: u32 = 8;
    pub const ACPI_RECLAIM: u32 = 9;
    pub const ACPI_NVS: u32 = 10;
    pub const MMIO: u32 = 11;
    pub const MMIO_PORT_SPACE: u32 = 12;
    pub const PAL_CODE: u32 = 13;
    pub const PERSISTENT_MEMORY: u32 = 14;
}

/// Allocation type for AllocatePages
pub const ALLOCATE_ANY_PAGES: u32 = 0;
pub const ALLOCATE_MAX_ADDRESS: u32 = 1;
pub const ALLOCATE_ADDRESS: u32 = 2;

/// Well-known GUIDs
pub const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data1: 0x9042A9DE,
    data2: 0x23DC,
    data3: 0x4A38,
    data4: [0x96, 0xFB, 0x7A, 0xDE, 0xD0, 0x80, 0x51, 0x6A],
};

pub const EFI_ACPI_20_TABLE_GUID: EfiGuid = EfiGuid {
    data1: 0x8868E871,
    data2: 0xE4F1,
    data3: 0x11D3,
    data4: [0xBC, 0x22, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
};
