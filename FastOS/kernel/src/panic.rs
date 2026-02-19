// ============================================================
// FastOS — Panic Handler (Rust)
// ============================================================
// Red screen of death for FastOS. Displays panic info and halts.
// ============================================================

use core::panic::PanicInfo;
use crate::vga::{VgaWriter, Color};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut vga = VgaWriter::new();

    // Red background panic screen
    vga.set_color(Color::White, Color::Red);
    vga.clear();

    vga.set_cursor(2, 0);
    vga.write_str("  !! FASTOS KERNEL PANIC !!\n\n");

    vga.set_color(Color::Yellow, Color::Red);
    if let Some(location) = info.location() {
        vga.write_str("  File: ");
        vga.write_str(location.file());
        vga.write_str("\n  Line: ");
        vga.write_dec(location.line() as u64);
        vga.write_str("\n\n");
    }

    vga.set_color(Color::White, Color::Red);
    vga.write_str("  System halted. Please reboot.\n");

    loop {
        unsafe {
            core::arch::asm!("cli");
            core::arch::asm!("hlt");
        }
    }
}
