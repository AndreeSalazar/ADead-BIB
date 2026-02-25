// ============================================================
// BG — Binary Guardian: Policy Engine
// ============================================================
// Evaluates an ArchitectureMap against a SecurityPolicy.
//
// No heuristics. No scoring. No probabilities.
//
// Verdict = (ArchitectureMap ∩ AllowedCapabilities) ?
//   APPROVED : DENIED { violations }
//
// This is deterministic: same binary + same policy = same result.
// Always. Every time.
//
// Designed for FastOS: kernel loader uses this to gate execution.
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use std::fmt;
use super::arch_map::*;

// ============================================================
// Security Level (maps to CPU rings)
// ============================================================

/// Security level — maps directly to x86-64 privilege rings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// Ring 0 — Full hardware access. Kernel only.
    Kernel = 0,
    /// Ring 1 — IO + restricted ops. Drivers.
    Driver = 1,
    /// Ring 2 — Restricted ops, no direct IO. Services.
    Service = 2,
    /// Ring 3 — Safe instructions only. User applications.
    User = 3,
}

impl fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityLevel::Kernel => write!(f, "KERNEL (Ring 0)"),
            SecurityLevel::Driver => write!(f, "DRIVER (Ring 1)"),
            SecurityLevel::Service => write!(f, "SERVICE (Ring 2)"),
            SecurityLevel::User => write!(f, "USER (Ring 3)"),
        }
    }
}

// ============================================================
// Security Policy
// ============================================================

/// A security policy that defines what a binary is allowed to do.
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub level: SecurityLevel,
    /// Whitelist of allowed syscall vectors (None = all allowed for the level)
    pub allowed_syscall_vectors: Option<Vec<u8>>,
    /// Whitelist of allowed IO ports (None = all allowed for the level)
    pub allowed_io_ports: Option<Vec<u16>>,
    /// Maximum allowed indirect control flow sites (None = unlimited)
    pub max_indirect_sites: Option<usize>,
    /// Allow RWX memory regions
    pub allow_rwx: bool,
    /// Allow self-modifying code
    pub allow_self_modifying: bool,
    /// Allow far jumps (segment changes)
    pub allow_far_jumps: bool,
}

impl SecurityPolicy {
    /// Policy for kernel-mode code — everything allowed.
    pub fn kernel() -> Self {
        Self {
            level: SecurityLevel::Kernel,
            allowed_syscall_vectors: None,
            allowed_io_ports: None,
            max_indirect_sites: None,
            allow_rwx: true,
            allow_self_modifying: true,
            allow_far_jumps: true,
        }
    }

    /// Policy for driver code — IO + restricted, no CR/MSR/descriptor tables.
    pub fn driver() -> Self {
        Self {
            level: SecurityLevel::Driver,
            allowed_syscall_vectors: None,
            allowed_io_ports: None,
            max_indirect_sites: None,
            allow_rwx: false,
            allow_self_modifying: false,
            allow_far_jumps: false,
        }
    }

    /// Policy for service code — syscalls only, no direct hardware.
    pub fn service() -> Self {
        Self {
            level: SecurityLevel::Service,
            allowed_syscall_vectors: None,
            allowed_io_ports: Some(Vec::new()), // No IO allowed
            max_indirect_sites: Some(64),
            allow_rwx: false,
            allow_self_modifying: false,
            allow_far_jumps: false,
        }
    }

    /// Policy for user applications — safe instructions + syscalls only.
    pub fn user() -> Self {
        Self {
            level: SecurityLevel::User,
            allowed_syscall_vectors: None,
            allowed_io_ports: Some(Vec::new()),
            max_indirect_sites: Some(32),
            allow_rwx: false,
            allow_self_modifying: false,
            allow_far_jumps: false,
        }
    }

    /// Strict sandbox — almost nothing allowed.
    pub fn sandbox() -> Self {
        Self {
            level: SecurityLevel::User,
            allowed_syscall_vectors: Some(Vec::new()), // No syscalls
            allowed_io_ports: Some(Vec::new()),
            max_indirect_sites: Some(0),
            allow_rwx: false,
            allow_self_modifying: false,
            allow_far_jumps: false,
        }
    }
}

// ============================================================
// Violation
// ============================================================

/// Type of security violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationType {
    PrivilegedInstruction,
    UnauthorizedIO,
    UnauthorizedSyscall,
    RWXMemory,
    SelfModifyingCode,
    ExcessiveIndirectControl,
    UnauthorizedFarJump,
    ControlRegisterAccess,
    MSRAccess,
    DescriptorTableAccess,
    InterruptControl,
}

impl fmt::Display for ViolationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViolationType::PrivilegedInstruction => write!(f, "PRIVILEGED_INSTRUCTION"),
            ViolationType::UnauthorizedIO => write!(f, "UNAUTHORIZED_IO"),
            ViolationType::UnauthorizedSyscall => write!(f, "UNAUTHORIZED_SYSCALL"),
            ViolationType::RWXMemory => write!(f, "RWX_MEMORY"),
            ViolationType::SelfModifyingCode => write!(f, "SELF_MODIFYING_CODE"),
            ViolationType::ExcessiveIndirectControl => write!(f, "EXCESSIVE_INDIRECT_CONTROL"),
            ViolationType::UnauthorizedFarJump => write!(f, "UNAUTHORIZED_FAR_JUMP"),
            ViolationType::ControlRegisterAccess => write!(f, "CONTROL_REGISTER_ACCESS"),
            ViolationType::MSRAccess => write!(f, "MSR_ACCESS"),
            ViolationType::DescriptorTableAccess => write!(f, "DESCRIPTOR_TABLE_ACCESS"),
            ViolationType::InterruptControl => write!(f, "INTERRUPT_CONTROL"),
        }
    }
}

/// A specific security violation found during policy evaluation.
#[derive(Debug, Clone)]
pub struct Violation {
    pub kind: ViolationType,
    pub instruction_index: Option<usize>,
    pub description: String,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(idx) = self.instruction_index {
            write!(f, "[{}] @{}: {}", self.kind, idx, self.description)
        } else {
            write!(f, "[{}]: {}", self.kind, self.description)
        }
    }
}

// ============================================================
// Verdict
// ============================================================

/// Final verdict: APPROVED or DENIED.
#[derive(Debug, Clone)]
pub enum Verdict {
    Approved,
    Denied { violations: Vec<Violation> },
}

impl Verdict {
    pub fn is_approved(&self) -> bool {
        matches!(self, Verdict::Approved)
    }

    pub fn is_denied(&self) -> bool {
        matches!(self, Verdict::Denied { .. })
    }

    pub fn violations(&self) -> &[Violation] {
        match self {
            Verdict::Approved => &[],
            Verdict::Denied { violations } => violations,
        }
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Verdict::Approved => write!(f, "✅ APPROVED"),
            Verdict::Denied { violations } => {
                writeln!(f, "❌ DENIED — {} violation(s):", violations.len())?;
                for v in violations {
                    writeln!(f, "  • {}", v)?;
                }
                Ok(())
            }
        }
    }
}

// ============================================================
// Policy Engine
// ============================================================

/// Policy Engine — Evaluates an ArchitectureMap against a SecurityPolicy.
///
/// Deterministic. Same input → same output. Always.
pub struct PolicyEngine;

impl PolicyEngine {
    /// Evaluate a binary's architecture map against a security policy.
    /// Returns APPROVED or DENIED with a list of specific violations.
    pub fn evaluate(map: &ArchitectureMap, policy: &SecurityPolicy) -> Verdict {
        let mut violations = Vec::new();

        // Check privilege level constraints
        match policy.level {
            SecurityLevel::Kernel => {
                // Kernel can do anything — no checks needed
            }
            SecurityLevel::Driver => {
                // Drivers: no CR access, no MSR access, no descriptor tables
                Self::check_driver_violations(map, &mut violations);
            }
            SecurityLevel::Service => {
                // Services: no privileged instructions at all
                Self::check_service_violations(map, policy, &mut violations);
            }
            SecurityLevel::User => {
                // User: no privileged, no IO, limited syscalls
                Self::check_user_violations(map, policy, &mut violations);
            }
        }

        // Universal checks (apply to all levels except Kernel)
        if policy.level != SecurityLevel::Kernel {
            Self::check_universal(map, policy, &mut violations);
        }

        if violations.is_empty() {
            Verdict::Approved
        } else {
            Verdict::Denied { violations }
        }
    }

    /// Infer the minimum security level required to run a binary.
    pub fn infer_minimum_level(map: &ArchitectureMap) -> SecurityLevel {
        if map.capabilities.requires_kernel() {
            if map.capabilities.control_register_access
                || map.capabilities.msr_access
                || map.capabilities.descriptor_table_access
                || map.capabilities.interrupt_control
            {
                SecurityLevel::Kernel
            } else {
                SecurityLevel::Driver
            }
        } else if map.capabilities.syscalls || map.capabilities.interrupts {
            SecurityLevel::Service
        } else {
            SecurityLevel::User
        }
    }

    // ---- Level-specific checks ----

    fn check_driver_violations(map: &ArchitectureMap, violations: &mut Vec<Violation>) {
        if map.capabilities.control_register_access {
            violations.push(Violation {
                kind: ViolationType::ControlRegisterAccess,
                instruction_index: None,
                description: "Driver cannot access control registers (CR0-CR4)".into(),
            });
        }
        if map.capabilities.msr_access {
            violations.push(Violation {
                kind: ViolationType::MSRAccess,
                instruction_index: None,
                description: "Driver cannot access MSRs (RDMSR/WRMSR)".into(),
            });
        }
        if map.capabilities.descriptor_table_access {
            violations.push(Violation {
                kind: ViolationType::DescriptorTableAccess,
                instruction_index: None,
                description: "Driver cannot modify GDT/IDT (LGDT/LIDT)".into(),
            });
        }
    }

    fn check_service_violations(
        map: &ArchitectureMap,
        policy: &SecurityPolicy,
        violations: &mut Vec<Violation>,
    ) {
        // Services cannot use ANY privileged instructions
        if map.capabilities.privileged_instructions {
            for (idx, class) in &map.instruction_map.flagged {
                if *class == InstructionClass::Privileged {
                    violations.push(Violation {
                        kind: ViolationType::PrivilegedInstruction,
                        instruction_index: Some(*idx),
                        description: "Service cannot use privileged instructions".into(),
                    });
                }
            }
        }

        // Check IO port restrictions
        if let Some(ref allowed) = policy.allowed_io_ports {
            for access in &map.io_map.accesses {
                let denied = match access.port {
                    Some(port) => !allowed.contains(&port),
                    None => true, // Dynamic port = always denied for services
                };
                if denied {
                    violations.push(Violation {
                        kind: ViolationType::UnauthorizedIO,
                        instruction_index: Some(access.instruction_index),
                        description: format!(
                            "Service cannot access IO port {:?}",
                            access.port
                        ),
                    });
                }
            }
        }
    }

    fn check_user_violations(
        map: &ArchitectureMap,
        policy: &SecurityPolicy,
        violations: &mut Vec<Violation>,
    ) {
        // User code: no privileged instructions
        if map.capabilities.privileged_instructions {
            for (idx, class) in &map.instruction_map.flagged {
                if *class == InstructionClass::Privileged {
                    violations.push(Violation {
                        kind: ViolationType::PrivilegedInstruction,
                        instruction_index: Some(*idx),
                        description: "User code cannot use privileged instructions".into(),
                    });
                }
            }
        }

        // User code: no IO access
        if let Some(ref allowed) = policy.allowed_io_ports {
            for access in &map.io_map.accesses {
                let denied = match access.port {
                    Some(port) => !allowed.contains(&port),
                    None => true,
                };
                if denied {
                    violations.push(Violation {
                        kind: ViolationType::UnauthorizedIO,
                        instruction_index: Some(access.instruction_index),
                        description: format!(
                            "User code cannot access IO port {:?}",
                            access.port
                        ),
                    });
                }
            }
        }

        // User code: check syscall whitelist
        if let Some(ref allowed_vectors) = policy.allowed_syscall_vectors {
            for vector in &map.syscall_map.interrupt_vectors {
                if !allowed_vectors.contains(vector) {
                    violations.push(Violation {
                        kind: ViolationType::UnauthorizedSyscall,
                        instruction_index: None,
                        description: format!(
                            "User code cannot use INT 0x{:02X}",
                            vector
                        ),
                    });
                }
            }
        }
    }

    fn check_universal(
        map: &ArchitectureMap,
        policy: &SecurityPolicy,
        violations: &mut Vec<Violation>,
    ) {
        // RWX memory check
        if !policy.allow_rwx && map.memory_map.rwx_count > 0 {
            violations.push(Violation {
                kind: ViolationType::RWXMemory,
                instruction_index: None,
                description: format!(
                    "{} RWX memory region(s) detected",
                    map.memory_map.rwx_count
                ),
            });
        }

        // Self-modifying code check
        if !policy.allow_self_modifying && map.memory_map.self_modifying_code {
            violations.push(Violation {
                kind: ViolationType::SelfModifyingCode,
                instruction_index: None,
                description: "Self-modifying code detected".into(),
            });
        }

        // Far jump check
        if !policy.allow_far_jumps && map.control_flow_map.far_jumps > 0 {
            violations.push(Violation {
                kind: ViolationType::UnauthorizedFarJump,
                instruction_index: None,
                description: format!(
                    "{} far jump(s) detected",
                    map.control_flow_map.far_jumps
                ),
            });
        }

        // Indirect control flow limit
        if let Some(max) = policy.max_indirect_sites {
            let total_indirect = map.control_flow_map.indirect_sites.len();
            if total_indirect > max {
                violations.push(Violation {
                    kind: ViolationType::ExcessiveIndirectControl,
                    instruction_index: None,
                    description: format!(
                        "{} indirect control flow sites (max: {})",
                        total_indirect, max
                    ),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::*;
    use crate::bg::capability::CapabilityMapper;

    #[test]
    fn test_safe_user_program_approved() {
        let ops = vec![
            ADeadOp::Push { src: Operand::Reg(Reg::RBP) },
            ADeadOp::Mov { dst: Operand::Reg(Reg::RBP), src: Operand::Reg(Reg::RSP) },
            ADeadOp::Xor { dst: Reg::EAX, src: Reg::EAX },
            ADeadOp::Ret,
        ];
        let map = CapabilityMapper::analyze(&ops);
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        assert!(verdict.is_approved());
    }

    #[test]
    fn test_kernel_code_denied_for_user() {
        let ops = vec![
            ADeadOp::Cli,
            ADeadOp::Lgdt { src: Operand::Mem { base: Reg::RAX, disp: 0 } },
            ADeadOp::Sti,
        ];
        let map = CapabilityMapper::analyze(&ops);
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        assert!(verdict.is_denied());
        assert!(verdict.violations().len() >= 3);
    }

    #[test]
    fn test_kernel_code_approved_for_kernel() {
        let ops = vec![
            ADeadOp::Cli,
            ADeadOp::MovToCr { cr: 0, src: Reg::RAX },
            ADeadOp::Wrmsr,
            ADeadOp::Sti,
        ];
        let map = CapabilityMapper::analyze(&ops);
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::kernel());
        assert!(verdict.is_approved());
    }

    #[test]
    fn test_io_denied_for_service() {
        let ops = vec![
            ADeadOp::InByte { port: Operand::Imm8(0x60) },
        ];
        let map = CapabilityMapper::analyze(&ops);
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::service());
        assert!(verdict.is_denied());
    }

    #[test]
    fn test_infer_minimum_level() {
        // Pure computation → User
        let user_ops = vec![ADeadOp::Add { dst: Operand::Reg(Reg::RAX), src: Operand::Imm8(1) }];
        let map = CapabilityMapper::analyze(&user_ops);
        assert_eq!(PolicyEngine::infer_minimum_level(&map), SecurityLevel::User);

        // Syscall → Service
        let svc_ops = vec![ADeadOp::Syscall];
        let map = CapabilityMapper::analyze(&svc_ops);
        assert_eq!(PolicyEngine::infer_minimum_level(&map), SecurityLevel::Service);

        // IO → Driver
        let drv_ops = vec![ADeadOp::InByte { port: Operand::Imm8(0x60) }];
        let map = CapabilityMapper::analyze(&drv_ops);
        assert_eq!(PolicyEngine::infer_minimum_level(&map), SecurityLevel::Driver);

        // CR access → Kernel
        let kern_ops = vec![ADeadOp::MovToCr { cr: 3, src: Reg::RAX }];
        let map = CapabilityMapper::analyze(&kern_ops);
        assert_eq!(PolicyEngine::infer_minimum_level(&map), SecurityLevel::Kernel);
    }
}
