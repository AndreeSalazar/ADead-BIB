// ============================================================
// FastOS — Interrupt Management (High-Level)
// ============================================================
// Wraps arch-specific IDT/PIC with kernel-level interrupt logic.
// ============================================================

use crate::arch::x86_64::{idt, cpu};

/// System tick counter (incremented by timer IRQ)
static mut SYSTEM_TICKS: u64 = 0;

/// Initialize interrupt system
pub fn init() {
    // IDT + PIC are initialized by arch::x86_64::init()
    // This module handles the kernel-level logic
}

/// Enable interrupts
pub fn enable() {
    cpu::sti();
}

/// Disable interrupts
pub fn disable() {
    cpu::cli();
}

/// Disable interrupts and return previous state
pub fn disable_save() -> bool {
    let flags: u64;
    unsafe {
        core::arch::asm!("pushfq; pop {}", out(reg) flags, options(nomem));
    }
    cpu::cli();
    flags & (1 << 9) != 0 // IF flag
}

/// Restore interrupt state
pub fn restore(was_enabled: bool) {
    if was_enabled {
        cpu::sti();
    }
}

/// Increment system tick (called from timer IRQ handler)
pub fn tick() {
    unsafe { SYSTEM_TICKS += 1; }
}

/// Get current system tick count
pub fn get_ticks() -> u64 {
    unsafe { SYSTEM_TICKS }
}

/// Get uptime in milliseconds (assumes 1000 Hz timer)
pub fn uptime_ms() -> u64 {
    get_ticks()
}

/// Send EOI for an IRQ
pub fn end_of_interrupt(irq: u8) {
    idt::send_eoi(irq);
}

/// Enable a specific IRQ line
pub fn enable_irq(irq: u8) {
    idt::enable_irq(irq);
}

/// Disable a specific IRQ line
pub fn disable_irq(irq: u8) {
    idt::disable_irq(irq);
}
