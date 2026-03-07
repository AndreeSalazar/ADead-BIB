// ============================================================
// ADead-BIB Output — Binary Generation
// ============================================================
// --target fastos  → .Po  (nativo, 24 bytes header)
// --target windows → .exe (PE optimizado)
// --target linux   → .elf (ELF estandar)
// ============================================================

pub mod elf;
pub mod pe;
pub mod po;

pub use elf::ElfOutput;
pub use pe::PeOutput;
pub use po::PoOutput;

/// Target output format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    /// FastOS nativo (.po) — 24 bytes header
    FastOS,
    /// Windows PE (.exe)
    WindowsPE,
    /// Linux ELF
    LinuxELF,
}

impl OutputFormat {
    pub fn extension(&self) -> &str {
        match self {
            OutputFormat::FastOS => ".po",
            OutputFormat::WindowsPE => ".exe",
            OutputFormat::LinuxELF => ".elf",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "fastos" | "po" => Some(OutputFormat::FastOS),
            "windows" | "pe" | "win" => Some(OutputFormat::WindowsPE),
            "linux" | "elf" => Some(OutputFormat::LinuxELF),
            _ => None,
        }
    }
}
