// ============================================================
// ADead-OS — Kernel Main (Rust)
// ============================================================
// Rust provides SECURITY: memory safety, type safety, no UB.
// ADead-BIB provides BASE: all hardware access, no ASM.
// C provides COMPATIBILITY: ABI headers, type contracts.
//
// This is a #![no_std] bare-metal kernel.
// ADead-BIB's stage2 jumps here after mode transition.
// ============================================================

#![no_std]
#![no_main]

mod vga;
mod panic;

// ============================================================
// External functions provided by ADead-BIB (the base layer)
// ADead-BIB generates raw x86-64 machine code for these.
// NO ASM. ADead-BIB IS the assembler.
// ============================================================
extern "C" {
    fn adead_outb(port: u16, value: u8);
    fn adead_inb(port: u16) -> u8;
    fn adead_cli();
    fn adead_sti();
    fn adead_hlt();
}

// ============================================================
// Kernel Entry Point
// ============================================================
// Called by ADead-BIB's stage2 after:
//   Real Mode → Protected Mode → Long Mode → here
// ============================================================

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Initialize VGA text mode
    let mut writer = vga::VgaWriter::new();
    writer.clear();

    // Print welcome banner
    writer.set_color(vga::Color::LightGreen, vga::Color::Black);
    writer.write_str("====================================\n");
    writer.write_str("  ADead-OS v0.1 — Kernel Active\n");
    writer.write_str("====================================\n");

    writer.set_color(vga::Color::White, vga::Color::Black);
    writer.write_str("\n");
    writer.write_str("3 Languages, Zero ASM:\n");

    writer.set_color(vga::Color::LightCyan, vga::Color::Black);
    writer.write_str("  [ADead-BIB] Base    - Boot, GDT, IDT, Hardware\n");

    writer.set_color(vga::Color::Yellow, vga::Color::Black);
    writer.write_str("  [Rust]      Security - Kernel logic, Memory safe\n");

    writer.set_color(vga::Color::LightRed, vga::Color::Black);
    writer.write_str("  [C]         Compat   - ABI headers, Interfaces\n");

    writer.set_color(vga::Color::White, vga::Color::Black);
    writer.write_str("\n");
    writer.write_str("ADead-BIB generates ALL machine code.\n");
    writer.write_str("No NASM. No GAS. No LLVM asm.\n");
    writer.write_str("\n");

    writer.set_color(vga::Color::LightGreen, vga::Color::Black);
    writer.write_str("Kernel initialized successfully.\n");

    writer.set_color(vga::Color::DarkGrey, vga::Color::Black);
    writer.write_str("Entering idle loop...\n");

    // Idle loop — halt until next interrupt
    loop {
        unsafe { adead_hlt(); }
    }
}

// ============================================================
// Hardware helpers (call ADead-BIB functions)
// ============================================================

/// Write a byte to an I/O port (via ADead-BIB)
pub fn outb(port: u16, value: u8) {
    unsafe { adead_outb(port, value); }
}

/// Read a byte from an I/O port (via ADead-BIB)
pub fn inb(port: u16) -> u8 {
    unsafe { adead_inb(port) }
}

/// Disable interrupts (via ADead-BIB)
pub fn cli() {
    unsafe { adead_cli(); }
}

/// Enable interrupts (via ADead-BIB)
pub fn sti() {
    unsafe { adead_sti(); }
}

/// Halt CPU until next interrupt (via ADead-BIB)
pub fn hlt() {
    unsafe { adead_hlt(); }
}

// ============================================================
// Interrupt handlers (Rust logic, ADead-BIB wraps with iretq)
// ============================================================

#[no_mangle]
pub extern "C" fn rust_timer_handler() {
    // Timer tick — will be used for scheduling later
}

#[no_mangle]
pub extern "C" fn rust_keyboard_handler() {
    // Read scancode from PS/2 controller
    let _scancode = inb(0x60);
    // TODO: Process keypress
}

#[no_mangle]
pub extern "C" fn rust_page_fault_handler(error_code: u64) {
    let mut writer = vga::VgaWriter::new();
    writer.set_color(vga::Color::White, vga::Color::Red);
    writer.write_str("PAGE FAULT! Error: 0x");
    writer.write_hex(error_code);
    writer.write_str("\n");
    loop { hlt(); }
}

#[no_mangle]
pub extern "C" fn rust_gpf_handler(error_code: u64) {
    let mut writer = vga::VgaWriter::new();
    writer.set_color(vga::Color::White, vga::Color::Red);
    writer.write_str("GENERAL PROTECTION FAULT! Error: 0x");
    writer.write_hex(error_code);
    writer.write_str("\n");
    loop { hlt(); }
}

#[no_mangle]
pub extern "C" fn rust_double_fault_handler() {
    let mut writer = vga::VgaWriter::new();
    writer.set_color(vga::Color::White, vga::Color::Red);
    writer.write_str("DOUBLE FAULT! System halted.\n");
    loop { hlt(); }
}
