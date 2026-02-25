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

// ============================================================
// Instruction Classification
// ============================================================

/// Clasificación determinista del nivel de privilegio de una instrucción.
/// Derivada estructuralmente del opcode — no de patrones ni heurísticas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionClass {
    /// Safe: sin efectos más allá de manipulación registro/memoria.
    /// mov, add, sub, xor, cmp, push, pop, jmp, jcc, call (directo), ret, nop
    Safe,
    /// Restricted: cruza frontera user/kernel o modifica contexto de ejecución.
    /// syscall, int N, sysret, iret
    Restricted,
    /// Privileged: requiere Ring 0 — control de hardware, tablas de descriptores, MSRs.
    /// cli, sti, hlt, lgdt, lidt, mov crN, rdmsr, wrmsr, invlpg, in, out
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

/// Clasifica cada instrucción del binario por nivel de privilegio.
#[derive(Debug, Clone)]
pub struct InstructionMap {
    pub total: usize,
    pub safe_count: usize,
    pub restricted_count: usize,
    pub privileged_count: usize,
    /// (instruction_index, class) solo para instrucciones non-safe
    pub flagged: Vec<(usize, InstructionClass)>,
}

impl InstructionMap {
    pub fn new() -> Self {
        Self {
            total: 0,
            safe_count: 0,
            restricted_count: 0,
            privileged_count: 0,
            flagged: Vec::new(),
        }
    }

    /// True si el binario contiene cero instrucciones privilegiadas.
    pub fn is_unprivileged(&self) -> bool {
        self.privileged_count == 0
    }

    /// Porcentaje de instrucciones safe.
    pub fn safe_ratio(&self) -> f64 {
        if self.total == 0 { return 1.0; }
        self.safe_count as f64 / self.total as f64
    }
}

// ============================================================
// Memory Map
// ============================================================

/// Tipo de región de memoria.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    /// Execute-only code
    Code,
    /// Read-write data
    Data,
    /// Read-only data
    ReadOnly,
    /// Read-Write-Execute — sospechoso, posible inyección de código
    RWX,
}

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

/// Una región de memoria detectada en el binario.
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub region_type: RegionType,
    pub offset: usize,
    pub size: usize,
    pub name: String,
}

/// Perfil de layout de memoria.
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
        Self {
            regions: Vec::new(),
            rwx_count: 0,
            self_modifying_code: false,
            total_code_size: 0,
            total_data_size: 0,
        }
    }

    /// True si no hay regiones RWX ni código auto-modificante.
    pub fn is_clean(&self) -> bool {
        self.rwx_count == 0 && !self.self_modifying_code
    }
}

// ============================================================
// Syscall Map
// ============================================================

/// Catálogo de uso de llamadas al sistema.
#[derive(Debug, Clone)]
pub struct SyscallMap {
    pub syscall_count: usize,
    pub interrupt_vectors: Vec<u8>,
    pub uses_syscall_instruction: bool,
    /// Índices de instrucciones donde ocurren syscalls/interrupciones
    pub call_sites: Vec<usize>,
}

impl SyscallMap {
    pub fn new() -> Self {
        Self {
            syscall_count: 0,
            interrupt_vectors: Vec::new(),
            uses_syscall_instruction: false,
            call_sites: Vec::new(),
        }
    }

    pub fn has_syscalls(&self) -> bool {
        self.syscall_count > 0 || !self.interrupt_vectors.is_empty()
    }
}

// ============================================================
// IO Map
// ============================================================

/// Dirección de una operación de puerto IO.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IODirection {
    In,
    Out,
}

/// Un acceso a puerto IO detectado.
#[derive(Debug, Clone)]
pub struct IOAccess {
    /// Número de puerto estático, o None si dinámico (vía registro DX)
    pub port: Option<u16>,
    pub direction: IODirection,
    pub instruction_index: usize,
}

/// Perfil de acceso a puertos IO.
#[derive(Debug, Clone)]
pub struct IOMap {
    pub accesses: Vec<IOAccess>,
}

impl IOMap {
    pub fn new() -> Self {
        Self {
            accesses: Vec::new(),
        }
    }

    pub fn has_io(&self) -> bool {
        !self.accesses.is_empty()
    }

    pub fn static_ports(&self) -> Vec<u16> {
        self.accesses.iter().filter_map(|a| a.port).collect()
    }

    /// Puertos únicos usados.
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

/// Perfil estructural de control de flujo.
#[derive(Debug, Clone)]
pub struct ControlFlowMap {
    pub direct_jumps: usize,
    pub indirect_jumps: usize,
    pub direct_calls: usize,
    pub indirect_calls: usize,
    pub conditional_branches: usize,
    pub far_jumps: usize,
    /// Índices de instrucciones con jumps/calls indirectos (potenciales gadgets)
    pub indirect_sites: Vec<usize>,
}

impl ControlFlowMap {
    pub fn new() -> Self {
        Self {
            direct_jumps: 0,
            indirect_jumps: 0,
            direct_calls: 0,
            indirect_calls: 0,
            conditional_branches: 0,
            far_jumps: 0,
            indirect_sites: Vec::new(),
        }
    }

    pub fn total_branches(&self) -> usize {
        self.direct_jumps + self.indirect_jumps + self.conditional_branches + self.far_jumps
    }

    pub fn has_indirect_control(&self) -> bool {
        self.indirect_jumps > 0 || self.indirect_calls > 0
    }
}

// ============================================================
// Capability Flags
// ============================================================

/// Resumen compacto de capacidades — qué PUEDE hacer el binario.
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
}

impl Capabilities {
    pub fn none() -> Self {
        Self {
            privileged_instructions: false,
            io_port_access: false,
            syscalls: false,
            interrupts: false,
            indirect_control_flow: false,
            self_modifying_code: false,
            control_register_access: false,
            interrupt_control: false,
            msr_access: false,
            descriptor_table_access: false,
            far_jumps: false,
        }
    }

    /// True si el binario requiere Ring 0 para ejecutarse.
    pub fn requires_kernel(&self) -> bool {
        self.privileged_instructions
            || self.io_port_access
            || self.control_register_access
            || self.interrupt_control
            || self.msr_access
            || self.descriptor_table_access
    }

    /// True si el binario es puramente computacional (safe para Ring 3).
    pub fn is_pure_userspace(&self) -> bool {
        !self.requires_kernel()
            && !self.self_modifying_code
            && !self.far_jumps
    }

    /// Cuenta cuántas capacidades están activas.
    pub fn active_count(&self) -> usize {
        let flags = [
            self.privileged_instructions,
            self.io_port_access,
            self.syscalls,
            self.interrupts,
            self.indirect_control_flow,
            self.self_modifying_code,
            self.control_register_access,
            self.interrupt_control,
            self.msr_access,
            self.descriptor_table_access,
            self.far_jumps,
        ];
        flags.iter().filter(|&&f| f).count()
    }
}

// ============================================================
// Architecture Map — Perfil Completo del Binario
// ============================================================

/// Perfil estructural completo de un binario.
///
/// Se genera una vez via análisis estático. O(n) para construir, O(1) para consultar.
/// Este es el output core del pipeline de análisis de BG.
#[derive(Debug, Clone)]
pub struct ArchitectureMap {
    pub instruction_map: InstructionMap,
    pub memory_map: MemoryMap,
    pub syscall_map: SyscallMap,
    pub io_map: IOMap,
    pub control_flow_map: ControlFlowMap,
    pub capabilities: Capabilities,
    /// Nombre/ruta del binario analizado (si disponible)
    pub binary_name: Option<String>,
    /// Tamaño total del binario en bytes
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
        writeln!(f, "  ┌─ Instruction Map ──────────────────────┐")?;
        writeln!(f, "  │ Total:          {:>8}               │", self.instruction_map.total)?;
        writeln!(f, "  │ Safe:           {:>8}  ({:.1}%)       │",
            self.instruction_map.safe_count,
            self.instruction_map.safe_ratio() * 100.0)?;
        writeln!(f, "  │ Restricted:     {:>8}               │", self.instruction_map.restricted_count)?;
        writeln!(f, "  │ Privileged:     {:>8}               │", self.instruction_map.privileged_count)?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;
        writeln!(f, "  ┌─ Memory Map ────────────────────────────┐")?;
        writeln!(f, "  │ Regions:        {:>8}               │", self.memory_map.regions.len())?;
        writeln!(f, "  │ Code size:      {:>8} bytes         │", self.memory_map.total_code_size)?;
        writeln!(f, "  │ Data size:      {:>8} bytes         │", self.memory_map.total_data_size)?;
        writeln!(f, "  │ RWX regions:    {:>8}               │", self.memory_map.rwx_count)?;
        writeln!(f, "  │ Self-modifying: {:>8}               │",
            if self.memory_map.self_modifying_code { "YES ⚠" } else { "no" })?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;
        writeln!(f, "  ┌─ Syscall Map ───────────────────────────┐")?;
        writeln!(f, "  │ Syscalls:       {:>8}               │", self.syscall_map.syscall_count)?;
        writeln!(f, "  │ INT vectors:    {:?}", self.syscall_map.interrupt_vectors)?;
        writeln!(f, "  │ Uses SYSCALL:   {:>8}               │",
            if self.syscall_map.uses_syscall_instruction { "yes" } else { "no" })?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;
        writeln!(f, "  ┌─ IO Map ────────────────────────────────┐")?;
        writeln!(f, "  │ Port accesses:  {:>8}               │", self.io_map.accesses.len())?;
        writeln!(f, "  │ Unique ports:   {:?}", self.io_map.unique_ports())?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;
        writeln!(f, "  ┌─ Control Flow Map ──────────────────────┐")?;
        writeln!(f, "  │ Direct jumps:   {:>8}               │", self.control_flow_map.direct_jumps)?;
        writeln!(f, "  │ Indirect jumps: {:>8}               │", self.control_flow_map.indirect_jumps)?;
        writeln!(f, "  │ Direct calls:   {:>8}               │", self.control_flow_map.direct_calls)?;
        writeln!(f, "  │ Indirect calls: {:>8}               │", self.control_flow_map.indirect_calls)?;
        writeln!(f, "  │ Conditionals:   {:>8}               │", self.control_flow_map.conditional_branches)?;
        writeln!(f, "  │ Far jumps:      {:>8}               │", self.control_flow_map.far_jumps)?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f)?;
        writeln!(f, "  ┌─ Capabilities ──────────────────────────┐")?;
        writeln!(f, "  │ Requires kernel:  {}                    │",
            if self.capabilities.requires_kernel() { "YES ⚠" } else { "no    " })?;
        writeln!(f, "  │ Pure userspace:   {}                    │",
            if self.capabilities.is_pure_userspace() { "yes ✓ " } else { "NO    " })?;
        writeln!(f, "  │ IO access:        {}                    │",
            if self.capabilities.io_port_access { "YES ⚠" } else { "no    " })?;
        writeln!(f, "  │ CR access:        {}                    │",
            if self.capabilities.control_register_access { "YES ⚠" } else { "no    " })?;
        writeln!(f, "  │ MSR access:       {}                    │",
            if self.capabilities.msr_access { "YES ⚠" } else { "no    " })?;
        writeln!(f, "  │ INT control:      {}                    │",
            if self.capabilities.interrupt_control { "YES ⚠" } else { "no    " })?;
        writeln!(f, "  │ Desc tables:      {}                    │",
            if self.capabilities.descriptor_table_access { "YES ⚠" } else { "no    " })?;
        writeln!(f, "  │ Active caps:      {:>3}                    │", self.capabilities.active_count())?;
        writeln!(f, "  └────────────────────────────────────────┘")?;
        writeln!(f, "═══════════════════════════════════════════════")?;
        Ok(())
    }
}
