// ============================================================
// FastOS — UEFI Memory Map (Fase 8)
// ============================================================
// Retrieves the UEFI memory map and converts it to the
// FastOS BootInfo E820-compatible format.
// ============================================================

#![allow(dead_code)]

use super::types::*;

/// Maximum memory map entries we support
const MAX_MMAP_ENTRIES: usize = 256;

/// Buffer for memory map (static to avoid allocation)
static mut MMAP_BUFFER: [u8; 8192] = [0u8; 8192];

/// Retrieve the UEFI memory map
///
/// Returns (map_key, descriptor_count, descriptor_size)
/// The map_key is needed for ExitBootServices.
///
/// # Safety
/// Requires valid UEFI boot services pointer.
pub unsafe fn get_memory_map(
    boot_services: *mut EfiBootServices,
) -> Result<(usize, usize, usize), EfiStatus> {
    let mut map_size: usize = MMAP_BUFFER.len();
    let mut map_key: usize = 0;
    let mut desc_size: usize = 0;
    let mut desc_version: u32 = 0;

    let status = ((*boot_services).get_memory_map)(
        &mut map_size,
        MMAP_BUFFER.as_mut_ptr() as *mut EfiMemoryDescriptor,
        &mut map_key,
        &mut desc_size,
        &mut desc_version,
    );

    if status != EFI_SUCCESS {
        return Err(status);
    }

    let desc_count = map_size / desc_size;
    Ok((map_key, desc_count, desc_size))
}

/// Convert UEFI memory map to E820-style entries for the kernel BootInfo
///
/// Maps UEFI memory types to E820 types:
///   ConventionalMemory → Usable (1)
///   LoaderCode/Data, BootServicesCode/Data → Usable (1) after ExitBootServices
///   ACPIReclaim → ACPI Reclaimable (3)
///   ACPI_NVS → ACPI NVS (4)
///   Everything else → Reserved (2)
pub unsafe fn convert_to_e820(
    desc_count: usize,
    desc_size: usize,
    out_entries: *mut E820Entry,
    max_entries: usize,
) -> usize {
    let mut count = 0usize;

    for i in 0..desc_count {
        if count >= max_entries { break; }

        let offset = i * desc_size;
        let desc = &*(MMAP_BUFFER.as_ptr().add(offset) as *const EfiMemoryDescriptor);

        let e820_type = match desc.memory_type {
            memory_type::CONVENTIONAL_MEMORY |
            memory_type::LOADER_CODE |
            memory_type::LOADER_DATA |
            memory_type::BOOT_SERVICES_CODE |
            memory_type::BOOT_SERVICES_DATA => 1, // Usable
            memory_type::ACPI_RECLAIM => 3,        // ACPI Reclaimable
            memory_type::ACPI_NVS => 4,            // ACPI NVS
            _ => 2,                                 // Reserved
        };

        let entry = &mut *out_entries.add(count);
        entry.base = desc.physical_start;
        entry.length = desc.number_of_pages * 4096;
        entry.entry_type = e820_type;
        entry.acpi_extended = 0;

        count += 1;
    }

    count
}

/// E820 entry (matches kernel's boot::E820Entry layout)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct E820Entry {
    pub base: u64,
    pub length: u64,
    pub entry_type: u32,
    pub acpi_extended: u32,
}

/// Exit boot services — must be called with the correct map_key
/// After this call, UEFI boot services are no longer available.
///
/// # Safety
/// This is a one-way operation. After calling this, only UEFI
/// runtime services remain available.
pub unsafe fn exit_boot_services(
    boot_services: *mut EfiBootServices,
    image_handle: EfiHandle,
    map_key: usize,
) -> Result<(), EfiStatus> {
    let status = ((*boot_services).exit_boot_services)(image_handle, map_key);
    if status != EFI_SUCCESS {
        return Err(status);
    }
    Ok(())
}
