// ============================================================
// BG — Binary Guardian: Architecture Map
// ============================================================
// Structural profile of a binary's behavior, derived
// deterministically from ABIB IR (ADeadOp).
//
// No heuristics. No signatures. Pure mathematical analysis.
//
// Pipeline: Binary → Decoder → ADeadIR → ArchitectureMap
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use std::fmt;

// ============================================================
// Instruction Classification
// ============================================================

/// Deterministic classification of an instruction's privilege level.
/// Derived structurally from the opcode — not from patterns or heuristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionClass {
    /// Safe: no side effects beyond register/memory manipulation.
    /// mov, add, sub, xor, cmp, push, pop, jmp, jcc, call (direct), ret, nop
    Safe,
    /// Restricted: crosses user/kernel boundary or modifies execution context.
    /// syscall, int N, sysret, iret
    Restricted,
    /// Privileged: requires Ring 0 — hardware control, descriptor tables, MSRs.
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

/// Classifies every instruction in the binary by privilege level.
#[derive(Debug, Clone)]
pub struct InstructionMap {
    pub total: usize,
    pub safe_count: usize,
    pub restricted_count: usize,
    pub privileged_count: usize,
    /// (instruction_index, class) for non-safe instructions only
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

    /// True if binary contains zero privileged instructions.
    pub fn is_unprivileged(&self) -> bool {
        self.privileged_count == 0
    }
}

// ============================================================
// Memory Map
// ============================================================

/// Type of memory region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    /// Execute-only code
    Code,
    /// Read-write data
    Data,
    /// Read-only data
    ReadOnly,
    /// Read-Write-Execute — suspicious, potential code injection
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

/// A detected memory region in the binary.
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub region_type: RegionType,
    pub offset: usize,
    pub size: usize,
}

/// Memory layout profile.
#[derive(Debug, Clone)]
pub struct MemoryMap {
    pub regions: Vec<MemoryRegion>,
    pub rwx_count: usize,
    pub self_modifying_code: bool,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            rwx_count: 0,
            self_modifying_code: false,
        }
    }

    /// True if no RWX regions and no self-modifying code detected.
    pub fn is_clean(&self) -> bool {
        self.rwx_count == 0 && !self.self_modifying_code
    }
}

// ============================================================
// Syscall Map
// ============================================================

/// Catalog of system call usage.
#[derive(Debug, Clone)]
pub struct SyscallMap {
    pub syscall_count: usize,
    pub interrupt_vectors: Vec<u8>,
    pub uses_syscall_instruction: bool,
    /// Instruction indices where syscalls/interrupts occur
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

/// Direction of an IO port operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IODirection {
    In,
    Out,
}

/// A detected IO port access.
#[derive(Debug, Clone)]
pub struct IOAccess {
    /// Static port number, or None if dynamic (via DX register)
    pub port: Option<u16>,
    pub direction: IODirection,
    pub instruction_index: usize,
}

/// IO port access profile.
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
}

// ============================================================
// Control Flow Map
// ============================================================

/// Control flow structural profile.
#[derive(Debug, Clone)]
pub struct ControlFlowMap {
    pub direct_jumps: usize,
    pub indirect_jumps: usize,
    pub direct_calls: usize,
    pub indirect_calls: usize,
    pub conditional_branches: usize,
    pub far_jumps: usize,
    /// Instruction indices of indirect jumps/calls (potential gadgets)
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

/// Compact capability summary — what the binary CAN do.
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

    /// True if the binary requires Ring 0 to execute.
    pub fn requires_kernel(&self) -> bool {
        self.privileged_instructions
            || self.io_port_access
            || self.control_register_access
            || self.interrupt_control
            || self.msr_access
            || self.descriptor_table_access
    }

    /// True if the binary is purely computational (safe for Ring 3).
    pub fn is_pure_userspace(&self) -> bool {
        !self.requires_kernel()
            && !self.self_modifying_code
            && !self.far_jumps
    }
}

// ============================================================
// Architecture Map — Complete Binary Profile
// ============================================================

/// Complete structural profile of a binary.
///
/// Generated once via static analysis. O(n) to build, O(1) to query.
/// This is the core output of BG's analysis pipeline.
#[derive(Debug, Clone)]
pub struct ArchitectureMap {
    pub instruction_map: InstructionMap,
    pub memory_map: MemoryMap,
    pub syscall_map: SyscallMap,
    pub io_map: IOMap,
    pub control_flow_map: ControlFlowMap,
    pub capabilities: Capabilities,
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
        }
    }
}

impl fmt::Display for ArchitectureMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "═══════════════════════════════════════")?;
        writeln!(f, "  BG — Binary Architecture Map")?;
        writeln!(f, "═══════════════════════════════════════")?;
        writeln!(f)?;
        writeln!(f, "  Instructions:     {}", self.instruction_map.total)?;
        writeln!(f, "    Safe:           {}", self.instruction_map.safe_count)?;
        writeln!(f, "    Restricted:     {}", self.instruction_map.restricted_count)?;
        writeln!(f, "    Privileged:     {}", self.instruction_map.privileged_count)?;
        writeln!(f)?;
        writeln!(f, "  Memory regions:   {}", self.memory_map.regions.len())?;
        writeln!(f, "    RWX regions:    {}", self.memory_map.rwx_count)?;
        writeln!(f, "    Self-modifying: {}", self.memory_map.self_modifying_code)?;
        writeln!(f)?;
        writeln!(f, "  Syscalls:         {}", self.syscall_map.syscall_count)?;
        writeln!(f, "    INT vectors:    {:?}", self.syscall_map.interrupt_vectors)?;
        writeln!(f, "    Uses SYSCALL:   {}", self.syscall_map.uses_syscall_instruction)?;
        writeln!(f)?;
        writeln!(f, "  IO ports:         {}", self.io_map.accesses.len())?;
        writeln!(f, "    Static ports:   {:?}", self.io_map.static_ports())?;
        writeln!(f)?;
        writeln!(f, "  Control flow:")?;
        writeln!(f, "    Direct jumps:   {}", self.control_flow_map.direct_jumps)?;
        writeln!(f, "    Indirect jumps: {}", self.control_flow_map.indirect_jumps)?;
        writeln!(f, "    Direct calls:   {}", self.control_flow_map.direct_calls)?;
        writeln!(f, "    Indirect calls: {}", self.control_flow_map.indirect_calls)?;
        writeln!(f, "    Conditionals:   {}", self.control_flow_map.conditional_branches)?;
        writeln!(f, "    Far jumps:      {}", self.control_flow_map.far_jumps)?;
        writeln!(f)?;
        writeln!(f, "  Capabilities:")?;
        writeln!(f, "    Requires kernel:  {}", self.capabilities.requires_kernel())?;
        writeln!(f, "    Pure userspace:   {}", self.capabilities.is_pure_userspace())?;
        writeln!(f, "    IO access:        {}", self.capabilities.io_port_access)?;
        writeln!(f, "    CR access:        {}", self.capabilities.control_register_access)?;
        writeln!(f, "    MSR access:       {}", self.capabilities.msr_access)?;
        writeln!(f, "    INT control:      {}", self.capabilities.interrupt_control)?;
        writeln!(f, "    Desc tables:      {}", self.capabilities.descriptor_table_access)?;
        writeln!(f, "═══════════════════════════════════════")?;
        Ok(())
    }
}
