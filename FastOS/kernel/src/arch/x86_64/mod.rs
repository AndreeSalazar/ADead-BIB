// ============================================================
// FastOS — Architecture: x86_64
// ============================================================
// Platform-specific code for AMD64 / Intel 64.
// Handles GDT, IDT, paging, I/O ports, CPU features.
// ============================================================

pub mod port;
pub mod cpu;
pub mod gdt;
pub mod idt;
pub mod paging;

/// Initialize all x86_64 architecture components
pub fn init() {
    gdt::init();
    idt::init();
}
