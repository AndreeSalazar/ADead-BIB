// ============================================================
// BG — Binary Guardian: Architecture Map
// ============================================================
// Perfil estructural completo de un binario.
// Derivado determinísticamente del ABIB IR (ADeadOp).
//
// No heurísticas. No firmas. Análisis matemático puro.
//
// Se genera una vez (O(n)), se consulta en O(1).
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use std::fmt;
use std::collections::HashMap;

// ============================================================
// Instruction Classification
// ============================================================

/// Clasificación determinista del nivel de privilegio de una instrucción.
/// Derivada estructuralmente del opcode — no de patrones ni heurísticas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionClass {
    /// Safe: sin efectos más allá de manipulación registro/memoria.
    Safe,
    /// Restricted: cruza frontera user/kernel o modifica contexto de ejecución.
    Restricted,
    /// Privileged: requiere Ring 0 — control de hardware, tablas de descriptores, MSRs.
    Privileged,
}

impl fmt::Display for InstructionClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionClass::Safe => write!(f, "SAFE"),
            InstructionClass::Restricted => write!(f, "RESTRICTED"),
            InstructionClass::Privileged => write!(f, "PRIVILEGED"),
        }
    }
}

// ============================================================
// Instruction Map
// ============================================================

#[derive(Debug, Clone)]
pub struct InstructionMap {
    pub total: usize,
    pub safe_count: usize,
    pub restricted_count: usize,
    pub privileged_count: usize,
    pub flagged: Vec<(usize, InstructionClass)>,
}

impl InstructionMap {
    pub fn new() -> Self {
        Self { total: 0, safe_count: 0, restricted_count: 0, privileged_count: 0, flagged: Vec::new() }
    }

    pub fn is_unprivileged(&self) -> bool { self.privileged_count == 0 }

    pub fn safe_ratio(&self) -> f64 {
        if self.total == 0 { return 1.0; }
        self.safe_count as f64 / self.total as f64
    }
}

// ============================================================
// Memory Map
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType { Code, Data, ReadOnly, RWX }

impl fmt::Display for RegionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegionType::Code => write!(f, "CODE"),
            RegionType::Data => write!(f, "DATA"),
            RegionType::ReadOnly => write!(f, "RODATA"),
            RegionType::RWX => write!(f, "RWX"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub region_type: RegionType,
    pub offset: usize,
    pub size: usize,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct MemoryMap {
    pub regions: Vec<MemoryRegion>,
    pub rwx_count: usize,
    pub self_modifying_code: bool,
    pub total_code_size: usize,
    pub total_data_size: usize,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self { regions: Vec::new(), rwx_count: 0, self_modifying_code: false, total_code_size: 0, total_data_size: 0 }
    }

    pub fn is_clean(&self) -> bool { self.rwx_count == 0 && !self.self_modifying_code }
}

// ============================================================
// Syscall Map
// ============================================================

#[derive(Debug, Clone)]
pub struct SyscallMap {
    pub syscall_count: usize,
    pub interrupt_vectors: Vec<u8>,
    pub uses_syscall_instruction: bool,
    pub call_sites: Vec<usize>,
}

impl SyscallMap {
    pub fn new() -> Self {
        Self { syscall_count: 0, interrupt_vectors: Vec::new(), uses_syscall_instruction: false, call_sites: Vec::new() }
    }

    pub fn has_syscalls(&self) -> bool { self.syscall_count > 0 || !self.interrupt_vectors.is_empty() }
}

// ============================================================
// IO Map
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IODirection { In, Out }

#[derive(Debug, Clone)]
pub struct IOAccess {
    pub port: Option<u16>,
    pub direction: IODirection,
    pub instruction_index: usize,
}

#[derive(Debug, Clone)]
pub struct IOMap {
    pub accesses: Vec<IOAccess>,
}

impl IOMap {
    pub fn new() -> Self { Self { accesses: Vec::new() } }

    pub fn has_io(&self) -> bool { !self.accesses.is_empty() }

    pub fn static_ports(&self) -> Vec<u16> { self.accesses.iter().filter_map(|a| a.port).collect() }

    pub fn unique_ports(&self) -> Vec<u16> {
        let mut ports = self.static_ports();
        ports.sort();
        ports.dedup();
        ports
    }
}

// ============================================================
// Control Flow Map
// ============================================================

#[derive(Debug, Clone)]
pub struct ControlFlowMap {
    pub direct_jumps: usize,
    pub indirect_jumps: usize,
    pub direct_calls: usize,
    pub indirect_calls: usize,
    pub conditional_branches: usize,
    pub far_jumps: usize,
    pub indirect_sites: Vec<usize>,
}

impl ControlFlowMap {
    pub fn new() -> Self {
        Self { direct_jumps: 0, indirect_jumps: 0, direct_calls: 0, indirect_calls: 0,
               conditional_branches: 0, far_jumps: 0, indirect_sites: Vec::new() }
    }

    pub fn total_branches(&self) -> usize {
        self.direct_jumps + self.indirect_jumps + self.conditional_branches + self.far_jumps
    }

    pub fn has_indirect_control(&self) -> bool { self.indirect_jumps > 0 || self.indirect_calls > 0 }
}

// ============================================================
// Structural Integrity
// ============================================================

/// Perfil de integridad estructural del binario.
/// Análisis puramente determinista — detecta anomalías que
/// ningún binario legítimo debería presentar.
#[derive(Debug, Clone)]
pub struct StructuralIntegrity {
    /// ¿El entry point apunta a una sección de código válida?
    pub entry_point_valid: bool,
    /// ¿Se pudo validar el entry point?
    pub entry_point_checked: bool,
    /// Proporción código/datos
    pub code_to_data_ratio: f64,
    /// ¿Hay secciones que se solapan?
    pub overlapping_sections: bool,
    /// Número de secciones con permisos anormales
    pub anomalous_permissions: usize,
    /// ¿El entry point está al inicio de una sección?
    pub entry_at_section_start: bool,
    /// Header size / total size ratio
    pub header_ratio: f64,
    /// ¿Tiene TLS callbacks? (código que se ejecuta antes del entry point)
    pub has_tls_callbacks: bool,
    /// Número de TLS callbacks
    pub tls_callback_count: usize,
    /// ¿Tiene overlay data? (bytes después del último section end)
    pub has_overlay: bool,
    /// Tamaño del overlay en bytes
    pub overlay_size: usize,
}

impl StructuralIntegrity {
    pub fn new() -> Self {
        Self {
            entry_point_valid: true, entry_point_checked: false,
            code_to_data_ratio: 0.0, overlapping_sections: false,
            anomalous_permissions: 0, entry_at_section_start: true,
            header_ratio: 0.0,
            has_tls_callbacks: false, tls_callback_count: 0,
            has_overlay: false, overlay_size: 0,
        }
    }

    pub fn is_clean(&self) -> bool {
        (!self.entry_point_checked || self.entry_point_valid)
            && !self.overlapping_sections
            && self.anomalous_permissions == 0
            && !self.has_tls_callbacks
            && !self.has_overlay
    }
}

// ============================================================
// Import/Export Map
// ============================================================

/// Perfil de imports/exports — clasificación determinista de APIs.
/// No heurístico: categoriza por nombre exacto de API.
#[derive(Debug, Clone)]
pub struct ImportExportMap {
    pub import_count: usize,
    pub export_count: usize,
    pub imports_by_library: HashMap<String, Vec<String>>,
    pub exports: Vec<String>,
    pub memory_manipulation_apis: Vec<String>,
    pub network_apis: Vec<String>,
    pub filesystem_apis: Vec<String>,
    pub crypto_apis: Vec<String>,
    pub process_injection_apis: Vec<String>,
}

impl ImportExportMap {
    pub fn new() -> Self {
        Self {
            import_count: 0, export_count: 0,
            imports_by_library: HashMap::new(), exports: Vec::new(),
            memory_manipulation_apis: Vec::new(), network_apis: Vec::new(),
            filesystem_apis: Vec::new(), crypto_apis: Vec::new(),
            process_injection_apis: Vec::new(),
        }
    }

    pub fn is_clean(&self) -> bool { self.process_injection_apis.is_empty() }
    pub fn has_network(&self) -> bool { !self.network_apis.is_empty() }

    /// Categoriza determinísticamente un nombre de API importada.
    pub fn categorize_import(&mut self, api_name: &str) {
        let upper = api_name.to_uppercase();

        const MEMORY_APIS: &[&str] = &[
            "VIRTUALALLOC", "VIRTUALPROTECT", "VIRTUALFREE", "VIRTUALALLOCEX",
            "VIRTUALPROTECTEX", "HEAPALLOC", "HEAPFREE",
            "NTMAPVIEWOFSECTION", "NTUNMAPVIEWOFSECTION",
            "MMAP", "MPROTECT", "MUNMAP",
        ];
        for pat in MEMORY_APIS {
            if upper.contains(pat) { self.memory_manipulation_apis.push(api_name.to_string()); return; }
        }

        const INJECTION_APIS: &[&str] = &[
            "WRITEPROCESSMEMORY", "READPROCESSMEMORY",
            "CREATEREMOTETHREAD", "NTQUEUEAPCTHREAD",
            "SETWINDOWSHOOKEX", "SETTHREADCONTEXT",
            "NTWRITEVIRTUALMEMORY", "NTREADVIRTUALMEMORY", "PTRACE",
        ];
        for pat in INJECTION_APIS {
            if upper.contains(pat) { self.process_injection_apis.push(api_name.to_string()); return; }
        }

        const NETWORK_APIS: &[&str] = &[
            "WSASTARTUP", "SOCKET", "CONNECT", "SEND", "RECV",
            "BIND", "LISTEN", "ACCEPT", "GETADDRINFO",
            "INTERNETOPEN", "HTTPOPENREQUEST", "URLDOWNLOAD", "WINHTTP",
        ];
        for pat in NETWORK_APIS {
            if upper.contains(pat) { self.network_apis.push(api_name.to_string()); return; }
        }

        const FS_APIS: &[&str] = &[
            "CREATEFILE", "WRITEFILE", "READFILE", "DELETEFILE",
            "MOVEFILE", "COPYFILE", "FINDFIRSTFILE", "FINDNEXTFILE",
        ];
        for pat in FS_APIS {
            if upper.contains(pat) { self.filesystem_apis.push(api_name.to_string()); return; }
        }

        const CRYPTO_APIS: &[&str] = &[
            "CRYPTACQUIRECONTEXT", "CRYPTENCRYPT", "CRYPTDECRYPT",
            "CRYPTGENRANDOM", "BCRYPT", "NCRYPT",
        ];
        for pat in CRYPTO_APIS {
            if upper.contains(pat) { self.crypto_apis.push(api_name.to_string()); return; }
        }
    }
}

// ============================================================
// Hardware Access Map — NUEVO
// ============================================================

/// Dispositivo de hardware al que un binario accede via IO port.
/// Clasificación determinista: puerto → dispositivo, tabla fija.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HardwareDevice {
    DmaController,
    PicMaster,
    PicSlave,
    PitTimer,
    KeyboardPS2,
    CmosRtc,
    PostDiagnostic,
    FastA20Gate,
    IdeSecondary,
    IdePrimary,
    Com1,
    Com2,
    Com3,
    Com4,
    Lpt1,
    Lpt2,
    VgaController,
    FloppyController,
    PciConfigSpace,
    AcpiControl,
    Unknown(u16),
}

impl fmt::Display for HardwareDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HardwareDevice::DmaController => write!(f, "DMA Controller"),
            HardwareDevice::PicMaster => write!(f, "PIC Master (8259A)"),
            HardwareDevice::PicSlave => write!(f, "PIC Slave (8259A)"),
            HardwareDevice::PitTimer => write!(f, "PIT Timer (8254)"),
            HardwareDevice::KeyboardPS2 => write!(f, "Keyboard/PS2 (8042)"),
            HardwareDevice::CmosRtc => write!(f, "CMOS/RTC"),
            HardwareDevice::PostDiagnostic => write!(f, "POST Diagnostic"),
            HardwareDevice::FastA20Gate => write!(f, "Fast A20 Gate"),
            HardwareDevice::IdeSecondary => write!(f, "IDE Secondary"),
            HardwareDevice::IdePrimary => write!(f, "IDE Primary"),
            HardwareDevice::Com1 => write!(f, "COM1 Serial"),
            HardwareDevice::Com2 => write!(f, "COM2 Serial"),
            HardwareDevice::Com3 => write!(f, "COM3 Serial"),
            HardwareDevice::Com4 => write!(f, "COM4 Serial"),
            HardwareDevice::Lpt1 => write!(f, "LPT1 Parallel"),
            HardwareDevice::Lpt2 => write!(f, "LPT2 Parallel"),
            HardwareDevice::VgaController => write!(f, "VGA Controller"),
            HardwareDevice::FloppyController => write!(f, "Floppy Controller"),
            HardwareDevice::PciConfigSpace => write!(f, "PCI Config Space"),
            HardwareDevice::AcpiControl => write!(f, "ACPI Control"),
            HardwareDevice::Unknown(port) => write!(f, "Unknown (port 0x{:04X})", port),
        }
    }
}

/// Acceso a un dispositivo de hardware detectado en el binario.
#[derive(Debug, Clone)]
pub struct HardwareAccess {
    pub device: HardwareDevice,
    pub port: u16,
    pub direction: IODirection,
    pub instruction_index: usize,
}

/// Mapa completo de acceso a hardware del binario.
/// Determinista: mismos puertos IO → mismo mapa de hardware.
#[derive(Debug, Clone)]
pub struct HardwareAccessMap {
    /// Todos los accesos a hardware detectados
    pub accesses: Vec<HardwareAccess>,
    /// Dispositivos únicos accedidos
    pub devices_accessed: Vec<HardwareDevice>,
    /// ¿Accede a timing instructions? (RDTSC/RDTSCP)
    pub timing_access: bool,
    /// ¿Accede a debug registers?
    pub debug_register_access: bool,
    /// ¿Usa CPUID?
    pub cpuid_access: bool,
}

impl HardwareAccessMap {
    pub fn new() -> Self {
        Self {
            accesses: Vec::new(), devices_accessed: Vec::new(),
            timing_access: false, debug_register_access: false,
            cpuid_access: false,
        }
    }

    /// Clasifica un puerto IO a su dispositivo de hardware.
    /// Tabla determinista fija — no heurísticas.
    pub fn classify_port(port: u16) -> HardwareDevice {
        match port {
            0x00..=0x0F => HardwareDevice::DmaController,
            0x20..=0x21 => HardwareDevice::PicMaster,
            0x40..=0x43 => HardwareDevice::PitTimer,
            0x60..=0x64 => HardwareDevice::KeyboardPS2,
            0x70..=0x71 => HardwareDevice::CmosRtc,
            0x80 => HardwareDevice::PostDiagnostic,
            0x92 => HardwareDevice::FastA20Gate,
            0xA0..=0xA1 => HardwareDevice::PicSlave,
            0x170..=0x177 => HardwareDevice::IdeSecondary,
            0x1F0..=0x1F7 => HardwareDevice::IdePrimary,
            0x278..=0x27F => HardwareDevice::Lpt2,
            0x2E8..=0x2EF => HardwareDevice::Com4,
            0x2F8..=0x2FF => HardwareDevice::Com2,
            0x378..=0x37F => HardwareDevice::Lpt1,
            0x3B0..=0x3DF => HardwareDevice::VgaController,
            0x3E8..=0x3EF => HardwareDevice::Com3,
            0x3F0..=0x3F7 => HardwareDevice::FloppyController,
            0x3F8..=0x3FF => HardwareDevice::Com1,
            0xCF8..=0xCFF => HardwareDevice::PciConfigSpace,
            0xB000..=0xB03F => HardwareDevice::AcpiControl,
            _ => HardwareDevice::Unknown(port),
        }
    }

    /// Registra un acceso a puerto IO y clasifica el dispositivo.
    pub fn register_access(&mut self, port: u16, direction: IODirection, instruction_index: usize) {
        let device = Self::classify_port(port);
        if !self.devices_accessed.contains(&device) {
            self.devices_accessed.push(device.clone());
        }
        self.accesses.push(HardwareAccess { device, port, direction, instruction_index });
    }

    /// ¿El binario accede a algún dispositivo de almacenamiento directamente?
    pub fn touches_storage(&self) -> bool {
        self.devices_accessed.iter().any(|d| matches!(d,
            HardwareDevice::IdePrimary | HardwareDevice::IdeSecondary |
            HardwareDevice::FloppyController))
    }

    /// ¿El binario accede a controladores de interrupción?
    pub fn touches_interrupt_controllers(&self) -> bool {
        self.devices_accessed.iter().any(|d| matches!(d,
            HardwareDevice::PicMaster | HardwareDevice::PicSlave))
    }

    /// ¿El binario configura PCI?
    pub fn touches_pci(&self) -> bool {
        self.devices_accessed.iter().any(|d| matches!(d, HardwareDevice::PciConfigSpace))
    }
}

impl fmt::Display for HardwareAccessMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ┌─ Hardware Access Map ────────────────────┐")?;
        if self.devices_accessed.is_empty() && !self.timing_access && !self.cpuid_access {
            writeln!(f, "  │ No hardware access detected              │")?;
        } else {
            for device in &self.devices_accessed {
                let count = self.accesses.iter().filter(|a| a.device == *device).count();
                writeln!(f, "  │ {:22} × {:>3}              │", format!("{}", device), count)?;
            }
            if self.timing_access {
                writeln!(f, "  │ RDTSC/RDTSCP:    YES ⚠                  │")?;
            }
            if self.debug_register_access {
                writeln!(f, "  │ Debug Registers: YES ⚠                  │")?;
            }
            if self.cpuid_access {
                writeln!(f, "  │ CPUID:           yes                     │")?;
            }
        }
        writeln!(f, "  └────────────────────────────────────────┘")?;
        Ok(())
    }
}

// ============================================================
// Capability Flags
// ============================================================

#[derive(Debug, Clone)]
pub struct Capabilities {
    pub privileged_instructions: bool,
    pub io_port_access: bool,
    pub syscalls: bool,
    pub interrupts: bool,
    pub indirect_control_flow: bool,
    pub self_modifying_code: bool,
    pub control_register_access: bool,
    pub interrupt_control: bool,
    pub msr_access: bool,
    pub descriptor_table_access: bool,
    pub far_jumps: bool,
    /// RDTSC/RDTSCP — timing instructions
    pub timing_access: bool,
    /// Debug register access (DR0-DR7)
    pub debug_register_access: bool,
    /// CPUID usage
    pub cpuid_access: bool,
}

impl Capabilities {
    pub fn none() -> Self {
        Self {
            privileged_instructions: false, io_port_access: false,
            syscalls: false, interrupts: false, indirect_control_flow: false,
            self_modifying_code: false, control_register_access: false,
            interrupt_control: false, msr_access: false,
            descriptor_table_access: false, far_jumps: false,
            timing_access: false, debug_register_access: false,
            cpuid_access: false,
        }
    }

    pub fn requires_kernel(&self) -> bool {
        self.privileged_instructions || self.io_port_access
            || self.control_register_access || self.interrupt_control
            || self.msr_access || self.descriptor_table_access
    }

    pub fn is_pure_userspace(&self) -> bool {
        !self.requires_kernel() && !self.self_modifying_code && !self.far_jumps
    }

    pub fn active_count(&self) -> usize {
        let flags = [
            self.privileged_instructions, self.io_port_access, self.syscalls,
            self.interrupts, self.indirect_control_flow, self.self_modifying_code,
            self.control_register_access, self.interrupt_control, self.msr_access,
            self.descriptor_table_access, self.far_jumps,
            self.timing_access, self.debug_register_access, self.cpuid_access,
        ];
        flags.iter().filter(|&&f| f).count()
    }
}

// ============================================================
// Architecture Map — Perfil Completo del Binario
// ============================================================

#[derive(Debug, Clone)]
pub struct ArchitectureMap {
    pub instruction_map: InstructionMap,
    pub memory_map: MemoryMap,
    pub syscall_map: SyscallMap,
    pub io_map: IOMap,
    pub control_flow_map: ControlFlowMap,
    pub capabilities: Capabilities,
    pub integrity: StructuralIntegrity,
    pub import_export_map: ImportExportMap,
    pub hardware_map: HardwareAccessMap,
    pub binary_name: Option<String>,
    pub binary_size: usize,
}

impl ArchitectureMap {
    pub fn new() -> Self {
        Self {
            instruction_map: InstructionMap::new(),
            memory_map: MemoryMap::new(),
            syscall_map: SyscallMap::new(),
            io_map: IOMap::new(),
            control_flow_map: ControlFlowMap::new(),
            capabilities: Capabilities::none(),
            integrity: StructuralIntegrity::new(),
            import_export_map: ImportExportMap::new(),
            hardware_map: HardwareAccessMap::new(),
            binary_name: None,
            binary_size: 0,
        }
    }
}

impl fmt::Display for ArchitectureMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "═══════════════════════════════════════════════")?;
        writeln!(f, "  BG — Binary Guardian: Architecture Map")?;
        writeln!(f, "═══════════════════════════════════════════════")?;
        if let Some(ref name) = self.binary_name {
            writeln!(f, "  Binary:           {}", name)?;
        }
        if self.binary_size > 0 {
            writeln!(f, "  Size:             {} bytes", self.binary_size)?;
        }
        writeln!(f)?;

        // Instruction Map
        writeln!(f, "  ┌─ Instruction Map ──────────────────────┐")?;
        writeln!(f, "  │ Total:          {:>8}                  │", self.instruction_map.total)?;
        writeln!(f, "  │ Safe:           {:>8}  ({:.1}%)        │",
            self.instruction_map.safe_count, self.instruction_map.safe_ratio() * 100.0)?;
        writeln!(f, "  │ Restricted:     {:>8}                  │", self.instruction_map.restricted_count)?;
        writeln!(f, "  │ Privileged:     {:>8}                  │", self.instruction_map.privileged_count)?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;

        // Memory Map
        writeln!(f, "  ┌─ Memory Map ────────────────────────────┐")?;
        writeln!(f, "  │ Regions:        {:>8}                   │", self.memory_map.regions.len())?;
        writeln!(f, "  │ Code size:      {:>8} bytes             │", self.memory_map.total_code_size)?;
        writeln!(f, "  │ Data size:      {:>8} bytes             │", self.memory_map.total_data_size)?;
        writeln!(f, "  │ RWX regions:    {:>8}                   │", self.memory_map.rwx_count)?;
        writeln!(f, "  │ Self-modifying: {:>8}                   │",
            if self.memory_map.self_modifying_code { "YES ⚠" } else { "no" })?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;

        // Syscall + IO
        writeln!(f, "  ┌─ Syscall Map ───────────────────────────┐")?;
        writeln!(f, "  │ Syscalls:       {:>8}                   │", self.syscall_map.syscall_count)?;
        writeln!(f, "  │ INT vectors:    {:?}", self.syscall_map.interrupt_vectors)?;
        writeln!(f, "  │ Uses SYSCALL:   {:>8}                   │",
            if self.syscall_map.uses_syscall_instruction { "yes" } else { "no" })?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;

        writeln!(f, "  ┌─ IO Map ────────────────────────────────┐")?;
        writeln!(f, "  │ Port accesses:  {:>8}                   │", self.io_map.accesses.len())?;
        writeln!(f, "  │ Unique ports:   {:?}", self.io_map.unique_ports())?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;

        // Control Flow
        writeln!(f, "  ┌─ Control Flow Map ──────────────────────┐")?;
        writeln!(f, "  │ Direct jumps:   {:>8}                   │", self.control_flow_map.direct_jumps)?;
        writeln!(f, "  │ Indirect jumps: {:>8}                   │", self.control_flow_map.indirect_jumps)?;
        writeln!(f, "  │ Direct calls:   {:>8}                   │", self.control_flow_map.direct_calls)?;
        writeln!(f, "  │ Indirect calls: {:>8}                   │", self.control_flow_map.indirect_calls)?;
        writeln!(f, "  │ Conditionals:   {:>8}                   │", self.control_flow_map.conditional_branches)?;
        writeln!(f, "  │ Far jumps:      {:>8}                   │", self.control_flow_map.far_jumps)?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;

        // Structural Integrity
        writeln!(f, "  ┌─ Structural Integrity ──────────────────┐")?;
        if self.integrity.entry_point_checked {
            writeln!(f, "  │ Entry valid:    {:>8}               │",
                if self.integrity.entry_point_valid { "yes ✓ " } else { "NO ⚠ " })?;
        }
        writeln!(f, "  │ Code/data:      {:>7.1}%                │", self.integrity.code_to_data_ratio * 100.0)?;
        writeln!(f, "  │ Overlapping:    {:>8}                   │",
            if self.integrity.overlapping_sections { "YES ⚠ " } else { "no    " })?;
        writeln!(f, "  │ Anomalous perms:{:>8}               │", self.integrity.anomalous_permissions)?;
        if self.integrity.has_tls_callbacks {
            writeln!(f, "  │ TLS callbacks:  {:>8} ⚠             │", self.integrity.tls_callback_count)?;
        }
        if self.integrity.has_overlay {
            writeln!(f, "  │ Overlay:        {:>8} bytes ⚠       │", self.integrity.overlay_size)?;
        }
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;

        // Import/Export
        if self.import_export_map.import_count > 0 || self.import_export_map.export_count > 0 {
            writeln!(f, "  ┌─ Import/Export Map ─────────────────────┐")?;
            writeln!(f, "  │ Imports:        {:>8}                   │", self.import_export_map.import_count)?;
            writeln!(f, "  │ Exports:        {:>8}                   │", self.import_export_map.export_count)?;
            writeln!(f, "  │ Libraries:      {:>8}                   │", self.import_export_map.imports_by_library.len())?;
            if !self.import_export_map.memory_manipulation_apis.is_empty() {
                writeln!(f, "  │ Memory APIs:    {:>8}               │", self.import_export_map.memory_manipulation_apis.len())?;
            }
            if !self.import_export_map.process_injection_apis.is_empty() {
                writeln!(f, "  │ Injection APIs: {:>8} ⚠             │", self.import_export_map.process_injection_apis.len())?;
            }
            if !self.import_export_map.network_apis.is_empty() {
                writeln!(f, "  │ Network APIs:   {:>8}               │", self.import_export_map.network_apis.len())?;
            }
            writeln!(f, "  └────────────────────────────────────────┘")?;
            writeln!(f)?;
        }

        // Hardware Access
        write!(f, "{}", self.hardware_map)?;
        writeln!(f)?;

        // Capabilities
        writeln!(f, "  ┌─ Capabilities ──────────────────────────┐")?;
        writeln!(f, "  │ Requires kernel:  {}                    │",
            if self.capabilities.requires_kernel() { "YES ⚠" } else { "no    " })?;
        writeln!(f, "  │ Pure userspace:   {}                    │",
            if self.capabilities.is_pure_userspace() { "yes ✓ " } else { "NO    " })?;
        writeln!(f, "  │ Struct clean:     {}                    │",
            if self.integrity.is_clean() { "yes ✓ " } else { "NO ⚠ " })?;
        writeln!(f, "  │ Import clean:     {}                    │",
            if self.import_export_map.is_clean() { "yes ✓ " } else { "NO ⚠ " })?;
        writeln!(f, "  │ Active caps:      {:>3}                    │", self.capabilities.active_count())?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f, "═══════════════════════════════════════════════")?;
        Ok(())
    }
}
