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

/// Initialize all hardware drivers
pub fn init(boot_info: &crate::boot::BootInfo) {
    framebuffer::init(boot_info);
    timer::init(1000); // 1000 Hz tick rate
    keyboard::init();
    mouse::init(boot_info.framebuffer_width, boot_info.framebuffer_height);
    disk::init();
}
