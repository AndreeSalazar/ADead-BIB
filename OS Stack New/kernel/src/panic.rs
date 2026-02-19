// ============================================================
// ADead-OS — Panic Handler (Rust)
// ============================================================
// Required for #![no_std] bare-metal Rust.
// Displays panic message on VGA and halts.
// ============================================================

use core::panic::PanicInfo;
use crate::vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = vga::VgaWriter::new();

    // Red background for panic
    writer.set_color(vga::Color::White, vga::Color::Red);
    writer.write_str("\n!!! KERNEL PANIC !!!\n");

    writer.set_color(vga::Color::Yellow, vga::Color::Red);

    if let Some(location) = info.location() {
        writer.write_str("  at ");
        writer.write_str(location.file());
        writer.write_str(":");
        writer.write_dec(location.line() as u64);
        writer.write_str("\n");
    }

    if let Some(message) = info.message().as_str() {
        writer.write_str("  ");
        writer.write_str(message);
        writer.write_str("\n");
    }

    writer.set_color(vga::Color::White, vga::Color::Red);
    writer.write_str("System halted.\n");

    // Halt forever
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
