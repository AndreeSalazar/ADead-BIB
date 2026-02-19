// ============================================================
// FastOS Kernel — Main Entry Point (Rust)
// ============================================================
// The Rust security layer for FastOS.
// ADead-BIB handles hardware. Rust handles logic. C glues them.
//
// Boot flow:
//   1. Stage2 (ADead-BIB) loads kernel, sets VBE mode, writes BootInfo
//   2. kernel_main() reads BootInfo at 0x9000
//   3. Initialize: GDT → IDT → Memory → Drivers → Desktop
//   4. If no graphical FB → fallback to VGA text mode
//
// Format: FsOS (not PE, not ELF — our own)
// ============================================================

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// ---- New architecture modules ----
pub mod arch;
pub mod boot;
pub mod kernel_core;
pub mod drivers;

// ---- Desktop Engine (Fase 5) ----
pub mod desktop_engine;

// ---- Graphical Apps (Fase 7) ----
pub mod gfx_apps;

// ---- Legacy / existing modules ----
mod vga;
mod keyboard;
mod panic;
mod shell;
mod installer;
mod login;
pub mod desktop;
pub mod window;
pub mod startmenu;
pub mod apps;

use vga::{VgaWriter, Color};

// ============================================================
// Hardware functions — inline assembly (bare-metal, no C runtime)
// ============================================================

// ============================================================
// Kernel Entry Point
// ============================================================
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // ---- Phase 1: Early VGA output (always available) ----
    let mut vga = VgaWriter::new();
    vga.set_color(Color::LightGreen, Color::Black);
    vga.clear();

    vga.set_color(Color::White, Color::Black);
    vga.write_str("============================================\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("       FastOS v1.0 - 64-bit Kernel\n");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("============================================\n");
    vga.set_color(Color::LightCyan, Color::Black);
    vga.write_str("  Powered by: ADead-BIB + Rust + C\n");
    vga.write_str("  Format:     FsOS (not PE, not ELF)\n\n");

    // ---- Phase 2: Architecture initialization ----
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("  [INIT] GDT...");
    arch::x86_64::gdt::init();
    vga.write_str(" OK\n");

    vga.write_str("  [INIT] IDT + PIC...");
    arch::x86_64::idt::init();
    vga.write_str(" OK\n");

    // ---- Phase 3: Core subsystems ----
    vga.write_str("  [INIT] Memory manager...");
    kernel_core::memory::init();
    vga.write_str(" OK\n");

    vga.write_str("  [INIT] Interrupt manager...");
    kernel_core::interrupts::init();
    vga.write_str(" OK\n");

    // ---- Phase 4: Read BootInfo and init drivers ----
    let boot_info = unsafe { boot::BootInfo::from_address(0x9000) };

    if boot_info.is_valid() {
        vga.write_str("  [INIT] BootInfo valid (magic=0x");
        vga.write_hex(boot_info.magic as u64);
        vga.write_str(")\n");

        vga.write_str("  [INIT] Framebuffer: ");
        vga.write_dec(boot_info.framebuffer_width as u64);
        vga.write_str("x");
        vga.write_dec(boot_info.framebuffer_height as u64);
        vga.write_str("x");
        vga.write_dec(boot_info.framebuffer_bpp as u64);
        vga.write_str(" @ 0x");
        vga.write_hex(boot_info.framebuffer_addr);
        vga.write_str("\n");

        // Initialize framebuffer + timer
        drivers::init(boot_info);
        vga.write_str("  [INIT] Drivers loaded (framebuffer + timer)\n");
    } else {
        vga.set_color(Color::LightCyan, Color::Black);
        vga.write_str("  [INFO] No BootInfo — VGA text mode fallback\n");
    }

    // ---- Phase 5: Timer ----
    vga.write_str("  [INIT] PIT timer @ 1000 Hz...");
    drivers::timer::init(1000);
    vga.write_str(" OK\n");

    // ---- Phase 6: Enable interrupts ----
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("\n  [OK] All subsystems initialized.\n");
    vga.write_str("  [OK] Kernel loaded at 0x100000\n\n");

    // Brief pause so user sees boot info
    for _ in 0..3_000_000u64 {
        unsafe { core::hint::spin_loop(); }
    }

    // ---- Phase 7: User flow (installer → login → desktop) ----
    installer::run_installer(&mut vga);

    let logged_in = login::run_login(&mut vga);
    if !logged_in {
        vga.set_color(Color::Red, Color::Black);
        vga.clear();
        vga.write_str("\n  Login failed. Rebooting...\n");
        outb(0x64, 0xFE);
        loop { hlt(); }
    }

    desktop::run_desktop(&mut vga);

    // Should never reach here
    loop { hlt(); }
}

// ============================================================
// Hardware functions — inline assembly (bare-metal)
// ============================================================
#[inline(always)]
pub fn hlt() {
    unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
}

#[inline(always)]
pub fn outb(port: u16, value: u8) {
    unsafe { core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack)); }
}

#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe { core::arch::asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack)); }
    value
}

#[inline(always)]
pub fn cli() {
    unsafe { core::arch::asm!("cli", options(nomem, nostack)); }
}

#[inline(always)]
pub fn sti() {
    unsafe { core::arch::asm!("sti", options(nomem, nostack)); }
}
