// ============================================================
// FastOS — Kernel Core Subsystems
// ============================================================
// Memory management, interrupt handling, process scheduling.
// Named kernel_core to avoid shadowing Rust's core crate.
// ============================================================

pub mod memory;
pub mod interrupts;
pub mod process;
pub mod scheduler;

/// Initialize all core kernel subsystems
pub fn init() {
    memory::init();
    interrupts::init();
    scheduler::init(); // Also initializes process table
}
