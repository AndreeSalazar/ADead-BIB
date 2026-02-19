// ============================================================
// FastOS — Ring 0/3 Separation
// ============================================================
// Kernel mode (Ring 0) vs User mode (Ring 3) enforcement.
// ============================================================

/// Privilege levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ring {
    Kernel = 0,  // Ring 0 — full hardware access
    User = 3,    // Ring 3 — restricted, must use syscalls
}

/// Check if current code is running in kernel mode
pub fn is_kernel_mode() -> bool {
    let cs: u16;
    unsafe {
        core::arch::asm!("mov {:x}, cs", out(reg) cs, options(nomem, nostack));
    }
    (cs & 3) == 0
}

/// Syscall numbers
pub mod syscall {
    pub const SYS_EXIT: u64    = 0;
    pub const SYS_WRITE: u64   = 1;
    pub const SYS_READ: u64    = 2;
    pub const SYS_OPEN: u64    = 3;
    pub const SYS_CLOSE: u64   = 4;
    pub const SYS_FORK: u64    = 5;
    pub const SYS_EXEC: u64    = 6;
    pub const SYS_WAIT: u64    = 7;
    pub const SYS_MMAP: u64    = 8;
    pub const SYS_MUNMAP: u64  = 9;
    pub const SYS_GETPID: u64  = 10;
    pub const SYS_YIELD: u64   = 11;
    pub const SYS_SLEEP: u64   = 12;
}

// TODO: Implement SYSCALL/SYSRET entry point
// TODO: Implement syscall dispatcher
// TODO: Set up MSRs for SYSCALL (IA32_STAR, IA32_LSTAR, IA32_FMASK)
