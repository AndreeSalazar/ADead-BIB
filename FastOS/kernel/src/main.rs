// ============================================================
// FastOS Kernel — Main Entry Point (Rust)
// ============================================================
// The Rust security layer for FastOS.
// ADead-BIB handles hardware. Rust handles logic. C glues them.
//
// This kernel provides:
//   - VGA text mode driver (80x25, 16 colors)
//   - Keyboard input handler
//   - Interactive installer screen
//   - Basic shell prompt
//
// Format: FsOS (not PE, not ELF — our own)
// ============================================================

#![no_std]
#![no_main]

mod vga;
mod keyboard;
mod panic;
mod shell;
mod installer;

use vga::{VgaWriter, Color};

// ============================================================
// ADead-BIB hardware functions (extern "C")
// ============================================================
extern "C" {
    fn fastos_cli();
    fn fastos_sti();
    fn fastos_hlt();
    fn fastos_outb(port: u16, value: u8);
    fn fastos_inb(port: u16) -> u8;
}

// ============================================================
// Kernel Entry Point
// ============================================================
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut vga = VgaWriter::new();

    // Clear screen with FastOS theme (green on black)
    vga.set_color(Color::LightGreen, Color::Black);
    vga.clear();

    // ---- FastOS Banner ----
    vga.set_color(Color::White, Color::Black);
    vga.write_str("============================================\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("       FastOS v1.0 - 64-bit Kernel\n");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("============================================\n");
    vga.set_color(Color::LightCyan, Color::Black);
    vga.write_str("  Powered by: ADead-BIB + Rust + C\n");
    vga.write_str("  Format:     FsOS (not PE, not ELF)\n");
    vga.write_str("  Mode:       64-bit Long Mode\n");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("============================================\n\n");

    // ---- System Info ----
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("[INFO] ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("Kernel loaded at 0x100000\n");

    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("[INFO] ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("VGA driver initialized (80x25)\n");

    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("[INFO] ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("Keyboard ready\n\n");

    // ---- Run Installer ----
    installer::run_installer(&mut vga);

    // ---- After installer: Shell ----
    shell::run_shell(&mut vga);

    // Should never reach here
    loop {
        unsafe { fastos_hlt(); }
    }
}

// ============================================================
// Safe wrappers for ADead-BIB hardware functions
// ============================================================
pub fn hlt() {
    unsafe { fastos_hlt(); }
}

pub fn outb(port: u16, value: u8) {
    unsafe { fastos_outb(port, value); }
}

pub fn inb(port: u16) -> u8 {
    unsafe { fastos_inb(port) }
}
