// ============================================================
// ADead Universal Binary Backend — Target Registry
// ============================================================
// Each target implements the BinaryBackend trait.
// The translator selects the appropriate backend based on
// the desired output format.
//
// Supported targets:
//   - PE   (Windows .exe/.dll)
//   - ELF  (Linux, future)
//   - FsOS (FastOS native, future)
// ============================================================

pub mod pe;
pub mod elf;
pub mod fastos;

use crate::bib::format::BibModule;

// ============================================================
// Backend Trait — every target must implement this
// ============================================================

/// Output format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Windows PE executable (.exe)
    PeExe,
    /// Windows PE dynamic library (.dll)
    PeDll,
    /// Linux ELF executable
    ElfExe,
    /// Linux ELF shared object (.so)
    ElfSo,
    /// FastOS native executable
    FsOS,
}

/// Subsystem type (PE-specific, but useful for all targets)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subsystem {
    /// Console application (CUI)
    Console,
    /// Graphical application (GUI)
    Gui,
    /// Native / no subsystem (drivers, OS kernels)
    Native,
}

/// Configuration for the backend
#[derive(Debug, Clone)]
pub struct BackendConfig {
    /// Output format
    pub format: OutputFormat,
    /// Subsystem
    pub subsystem: Subsystem,
    /// Image base address (default: 0x140000000 for PE, 0x400000 for ELF)
    pub image_base: u64,
    /// Section alignment (default: 0x1000)
    pub section_alignment: u32,
    /// File alignment (default: 0x200 for PE, 1 for ELF)
    pub file_alignment: u32,
    /// Stack reserve size
    pub stack_reserve: u64,
    /// Stack commit size
    pub stack_commit: u64,
    /// Heap reserve size
    pub heap_reserve: u64,
    /// Heap commit size
    pub heap_commit: u64,
    /// Output file path
    pub output_path: String,
    /// Enable ASLR / dynamic base (PE: DllCharacteristics)
    pub dynamic_base: bool,
    /// Enable NX / DEP
    pub nx_compat: bool,
    /// Large address aware
    pub large_address_aware: bool,
}

impl Default for BackendConfig {
    fn default() -> Self {
        BackendConfig {
            format: OutputFormat::PeExe,
            subsystem: Subsystem::Console,
            image_base: 0x0000_0001_4000_0000,
            section_alignment: 0x1000,
            file_alignment: 0x200,
            stack_reserve: 0x100000,
            stack_commit: 0x1000,
            heap_reserve: 0x100000,
            heap_commit: 0x1000,
            output_path: String::from("output.exe"),
            dynamic_base: true,
            nx_compat: true,
            large_address_aware: true,
        }
    }
}

impl BackendConfig {
    /// Preset for Windows console application
    pub fn windows_console() -> Self {
        Self {
            format: OutputFormat::PeExe,
            subsystem: Subsystem::Console,
            ..Default::default()
        }
    }

    /// Preset for Windows GUI application (e.g. DirectX 12)
    pub fn windows_gui() -> Self {
        Self {
            format: OutputFormat::PeExe,
            subsystem: Subsystem::Gui,
            ..Default::default()
        }
    }

    /// Preset for Linux ELF executable
    pub fn linux_exe() -> Self {
        Self {
            format: OutputFormat::ElfExe,
            subsystem: Subsystem::Console,
            image_base: 0x400000,
            file_alignment: 1,
            ..Default::default()
        }
    }

    /// Preset for FastOS native executable
    pub fn fastos() -> Self {
        Self {
            format: OutputFormat::FsOS,
            subsystem: Subsystem::Native,
            image_base: 0x100000,
            section_alignment: 0x1000,
            file_alignment: 0x200,
            ..Default::default()
        }
    }
}

/// The universal binary backend trait
pub trait BinaryBackend {
    /// Name of this backend (e.g. "PE", "ELF", "FsOS")
    fn name(&self) -> &str;

    /// Supported output formats
    fn supported_formats(&self) -> &[OutputFormat];

    /// Translate a BibModule into a native binary
    fn translate(&self, module: &BibModule, config: &BackendConfig) -> Result<Vec<u8>, String>;

    /// Write the translated binary to a file
    fn write(&self, module: &BibModule, config: &BackendConfig) -> Result<(), String> {
        let data = self.translate(module, config)?;
        std::fs::write(&config.output_path, &data)
            .map_err(|e| format!("Failed to write '{}': {}", config.output_path, e))?;
        Ok(())
    }

    /// Validate that the module can be translated with this backend
    fn validate(&self, module: &BibModule, config: &BackendConfig) -> Result<(), String>;
}

/// Select the appropriate backend for a given output format
pub fn select_backend(format: OutputFormat) -> Box<dyn BinaryBackend> {
    match format {
        OutputFormat::PeExe | OutputFormat::PeDll => {
            Box::new(pe::PeBackend::new())
        }
        OutputFormat::ElfExe | OutputFormat::ElfSo => {
            Box::new(elf::ElfBackend::new())
        }
        OutputFormat::FsOS => {
            Box::new(fastos::FsOSBackend::new())
        }
    }
}
