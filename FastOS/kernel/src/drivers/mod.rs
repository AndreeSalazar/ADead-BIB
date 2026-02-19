// ============================================================
// FastOS — Hardware Drivers
// ============================================================
// All drivers written in Rust. Hardware access via I/O ports.
// ============================================================

pub mod framebuffer;
pub mod timer;

/// Initialize all hardware drivers
pub fn init(boot_info: &crate::boot::BootInfo) {
    framebuffer::init(boot_info);
    timer::init(1000); // 1000 Hz tick rate
}
