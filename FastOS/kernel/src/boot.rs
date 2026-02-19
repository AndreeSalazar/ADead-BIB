// ============================================================
// FastOS — Boot Information
// ============================================================
// Struct passed from stage2 bootloader to kernel.
// Located at physical address 0x9000.
// ============================================================

/// Magic number: "FOS!" = 0x464F5321
pub const BOOT_INFO_MAGIC: u32 = 0x464F5321;

/// Boot information passed from stage2 to kernel
#[repr(C)]
pub struct BootInfo {
    pub magic: u32,
    pub framebuffer_addr: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u32,
    pub framebuffer_pitch: u32,
    pub memory_map_addr: u64,
    pub memory_map_count: u32,
    pub _reserved: [u8; 20],
}

impl BootInfo {
    /// Read boot info from the known physical address (0x9000)
    pub unsafe fn from_address(addr: u64) -> &'static BootInfo {
        &*(addr as *const BootInfo)
    }

    /// Validate the boot info magic number
    pub fn is_valid(&self) -> bool {
        self.magic == BOOT_INFO_MAGIC
    }

    /// Create a default boot info for VGA text mode fallback
    pub fn vga_fallback() -> BootInfo {
        BootInfo {
            magic: BOOT_INFO_MAGIC,
            framebuffer_addr: 0xB8000,
            framebuffer_width: 80,
            framebuffer_height: 25,
            framebuffer_bpp: 16,
            framebuffer_pitch: 160,
            memory_map_addr: 0,
            memory_map_count: 0,
            _reserved: [0; 20],
        }
    }
}

/// E820 memory map entry
#[repr(C)]
pub struct E820Entry {
    pub base: u64,
    pub length: u64,
    pub entry_type: u32,
    pub _reserved: u32,
}

/// E820 memory types
pub mod memory_type {
    pub const USABLE: u32 = 1;
    pub const RESERVED: u32 = 2;
    pub const ACPI_RECLAIMABLE: u32 = 3;
    pub const ACPI_NVS: u32 = 4;
    pub const BAD_MEMORY: u32 = 5;
}
