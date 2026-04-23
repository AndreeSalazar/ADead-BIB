//! Architecture detection — stub

/// Detect current CPU architecture
pub fn detect_arch() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    return "x86_64";
    #[cfg(target_arch = "aarch64")]
    return "aarch64";
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return "unknown";
}
