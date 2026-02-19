// ============================================================
// FastOS — Kernel Core Subsystems
// ============================================================
// Memory management, interrupt handling, process scheduling.
// ============================================================

pub mod memory;
pub mod interrupts;

/// Initialize all core kernel subsystems
pub fn init() {
    memory::init();
    interrupts::init();
}
