// ============================================================
// FastOS — Hardware Drivers
// ============================================================
// All drivers written in Rust. Hardware access via I/O ports.
// ============================================================

pub mod framebuffer;
pub mod timer;
pub mod keyboard;
pub mod mouse;
pub mod disk;

// Fase 9 — Advanced drivers
pub mod ahci;
pub mod usb;
pub mod network;
pub mod audio;

// Fase 10 — GPU driver (NVIDIA/AMD via PCI + SPIR-V)
pub mod gpu;
pub mod spirv;
pub mod aexe;   // AEXE format (PE → FastOS native)
pub mod hdmi;   // HDMI output via GPU

/// Initialize all hardware drivers
pub fn init(boot_info: &crate::boot::BootInfo) {
    crate::serial_print("[FastOS]   framebuffer...\r\n");
    framebuffer::init(boot_info);
    crate::serial_print("[FastOS]   timer...\r\n");
    timer::init(1000);
    crate::serial_print("[FastOS]   keyboard...\r\n");
    keyboard::init();
    crate::serial_print("[FastOS]   mouse...\r\n");
    mouse::init(boot_info.framebuffer_width, boot_info.framebuffer_height);
    crate::serial_print("[FastOS]   disk...\r\n");
    disk::init();
    crate::serial_print("[FastOS]   basic drivers done\r\n");

    // Fase 10 — GPU driver
    crate::serial_print("[FastOS]   gpu...\r\n");
    if gpu::init() {
        crate::serial_print("[FastOS]   GPU detected!\r\n");
        // Test SPIR-V parser
        spirv::test_spirv();
        // Test AEXE format
        aexe::test_aexe();
        // Test HDMI output
        hdmi::test_hdmi();
    }

    // Fase 9 — Advanced drivers DISABLED for now (PCI scans crash in QEMU)
    // ahci::init();
    // usb::init();
    // network::init();
    // audio::init();
    crate::serial_print("[FastOS]   all drivers done\r\n");
}
