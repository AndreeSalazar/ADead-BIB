// ============================================================================
// FastOS - Common Library
// ============================================================================
// Librería común para FastOS
// Tipos y utilidades compartidas entre kernel y userspace
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![no_std]

/// Versión de FastOS
pub const VERSION: &str = "0.1.0";

/// Nombre del sistema
pub const NAME: &str = "FastOS";

/// Syscall numbers
pub mod syscall {
    pub const SYS_EXIT: u64 = 0;
    pub const SYS_WRITE: u64 = 1;
    pub const SYS_READ: u64 = 2;
    pub const SYS_OPEN: u64 = 3;
    pub const SYS_CLOSE: u64 = 4;
    pub const SYS_MMAP: u64 = 5;
    pub const SYS_MUNMAP: u64 = 6;
}

/// Códigos de error
pub mod error {
    pub const OK: i32 = 0;
    pub const EINVAL: i32 = -1;
    pub const ENOMEM: i32 = -2;
    pub const ENOENT: i32 = -3;
    pub const EIO: i32 = -4;
    pub const EACCES: i32 = -5;
}

/// Tipos básicos
pub type Result<T> = core::result::Result<T, i32>;
