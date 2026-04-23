//! ADead-BIB Platform Abstraction Layer
//!
//! Soporta: Windows, Linux, macOS, FastOS

pub mod os;
pub mod arch;

pub use os::*;
pub use arch::*;

/// Platform information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    FastOS,
}

/// Architecture information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    AArch64,
    Fast256, // FastOS 256-bit
}

/// Get current platform
pub fn current_platform() -> Platform {
    #[cfg(target_os = "windows")]
    return Platform::Windows;
    #[cfg(target_os = "linux")]
    return Platform::Linux;
    #[cfg(target_os = "macos")]
    return Platform::MacOS;
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    return Platform::Linux; // Default
}

/// Get current architecture
pub fn current_architecture() -> Architecture {
    #[cfg(target_arch = "x86_64")]
    return Architecture::X86_64;
    #[cfg(target_arch = "aarch64")]
    return Architecture::AArch64;
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return Architecture::X86_64; // Default
}
