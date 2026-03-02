//! FastOS Kernel — Rust Safety Layer
//! 
//! Puntos críticos de memoria protegidos con Rust:
//! - Memory Manager (VMM)
//! - Heap Allocator
//! - Page Tables
//! - Buffer Management
//! 
//! C domina el kernel, Rust protege la memoria.

#![no_std]
#![feature(alloc_error_handler)]

mod memory;
mod allocator;
mod page_table;
mod buffer;

pub use memory::*;
pub use allocator::*;
pub use page_table::*;
pub use buffer::*;

use core::panic::PanicInfo;

/// Panic handler para kernel — no puede usar std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // En kernel real: llamar a kernel_panic() de C
    loop {}
}

/// Alloc error handler
#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    loop {}
}
