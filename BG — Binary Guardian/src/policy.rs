// ============================================================
// BG — Binary Guardian: Policy Engine
// ============================================================
// Motor de evaluación de policies de seguridad.
//
// Determinista: misma ArchitectureMap + misma Policy = mismo Verdict.
//
// SecurityLevel mapea a anillos x86-64:
//   Kernel  = Ring 0 (todo permitido)
//   Driver  = Ring 1 (IO + restricted)
//   Service = Ring 2 (restricted, sin IO directo)
//   User    = Ring 3 (solo instrucciones safe)
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use std::fmt;
use super::arch_map::*;

// ============================================================
// Security Level
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    Kernel = 0,
    Driver = 1,
    Service = 2,
    User = 3,
}

impl fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityLevel::Kernel => write!(f, "Kernel (Ring 0)"),
            SecurityLevel::Driver => write!(f, "Driver (Ring 1)"),
            SecurityLevel::Service => write!(f, "Service (Ring 2)"),
            SecurityLevel::User => write!(f, "User (Ring 3)"),
        }
    }
}

// ============================================================
// Violation Types
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationType {
    // Instruction-level violations
    PrivilegedInstruction,
    UnauthorizedSyscall,
    UnauthorizedIO,
    ExcessiveIndirectControl,
    FarJumpNotAllowed,
    SelfModifyingCode,
    RWXMemory,
    // Structural violations
    InvalidEntryPoint,
    OverlappingSections,
    AnomalousPermissions,
    // Import-level violations
    ProcessInjectionImports,
    // Timing/debug violations
    TimingAttackCapability,
    // Hidden code violations
    HiddenEntryPoint,
    // Hardware access violations
    UnauthorizedHardwareAccess,
}

impl fmt::Display for ViolationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViolationType::PrivilegedInstruction => write!(f, "PRIVILEGED_INSTRUCTION"),
            ViolationType::UnauthorizedSyscall => write!(f, "UNAUTHORIZED_SYSCALL"),
            ViolationType::UnauthorizedIO => write!(f, "UNAUTHORIZED_IO"),
            ViolationType::ExcessiveIndirectControl => write!(f, "EXCESSIVE_INDIRECT_CONTROL"),
            ViolationType::FarJumpNotAllowed => write!(f, "FAR_JUMP_NOT_ALLOWED"),
            ViolationType::SelfModifyingCode => write!(f, "SELF_MODIFYING_CODE"),
            ViolationType::RWXMemory => write!(f, "RWX_MEMORY"),
            ViolationType::InvalidEntryPoint => write!(f, "INVALID_ENTRY_POINT"),
            ViolationType::OverlappingSections => write!(f, "OVERLAPPING_SECTIONS"),
            ViolationType::AnomalousPermissions => write!(f, "ANOMALOUS_PERMISSIONS"),
            ViolationType::ProcessInjectionImports => write!(f, "PROCESS_INJECTION_IMPORTS"),
            ViolationType::TimingAttackCapability => write!(f, "TIMING_ATTACK_CAPABILITY"),
            ViolationType::HiddenEntryPoint => write!(f, "HIDDEN_ENTRY_POINT"),
            ViolationType::UnauthorizedHardwareAccess => write!(f, "UNAUTHORIZED_HARDWARE_ACCESS"),
        }
    }
}

// ============================================================
// Violation
// ============================================================

#[derive(Debug, Clone)]
pub struct Violation {
    pub violation_type: ViolationType,
    pub description: String,
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationSeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl fmt::Display for ViolationSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViolationSeverity::Critical => write!(f, "CRITICAL"),
            ViolationSeverity::High => write!(f, "HIGH"),
            ViolationSeverity::Medium => write!(f, "MEDIUM"),
            ViolationSeverity::Low => write!(f, "LOW"),
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} — {}", self.severity, self.violation_type, self.description)
    }
}

// ============================================================
// Verdict
// ============================================================

#[derive(Debug, Clone)]
pub enum Verdict {
    Approved,
    Denied { violations: Vec<Violation> },
}

impl Verdict {
    pub fn is_approved(&self) -> bool { matches!(self, Verdict::Approved) }
    pub fn is_denied(&self) -> bool { matches!(self, Verdict::Denied { .. }) }
    pub fn violation_count(&self) -> usize {
        match self { Verdict::Approved => 0, Verdict::Denied { violations } => violations.len() }
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Verdict::Approved => write!(f, "✓ APPROVED"),
            Verdict::Denied { violations } => {
                writeln!(f, "✗ DENIED — {} violation(s):", violations.len())?;
                for v in violations {
                    writeln!(f, "  {}", v)?;
                }
                Ok(())
            }
        }
    }
}

// ============================================================
// Security Policy
// ============================================================

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub name: String,
    pub level: SecurityLevel,
    pub allowed_syscall_vectors: Option<Vec<u8>>,
    pub allowed_io_ports: Option<Vec<u16>>,
    pub max_indirect_sites: Option<usize>,
    pub allow_rwx: bool,
    pub allow_self_modifying: bool,
    pub allow_far_jumps: bool,
    /// Enforce structural integrity checks
    pub require_structural_integrity: bool,
    /// Allow process injection APIs
    pub allow_process_injection: bool,
    /// Allow timing instructions (RDTSC/RDTSCP)
    pub allow_timing_instructions: bool,
    /// Allow TLS callbacks / hidden entry points
    pub allow_hidden_entry_points: bool,
    /// Allowed hardware devices (None = all allowed for kernel)
    pub allowed_hardware_devices: Option<Vec<HardwareDevice>>,
}

impl SecurityPolicy {
    pub fn kernel() -> Self {
        Self {
            name: "Kernel".to_string(),
            level: SecurityLevel::Kernel,
            allowed_syscall_vectors: None,
            allowed_io_ports: None,
            max_indirect_sites: None,
            allow_rwx: true,
            allow_self_modifying: true,
            allow_far_jumps: true,
            require_structural_integrity: false,
            allow_process_injection: true,
            allow_timing_instructions: true,
            allow_hidden_entry_points: true,
            allowed_hardware_devices: None, // All allowed
        }
    }

    pub fn driver() -> Self {
        Self {
            name: "Driver".to_string(),
            level: SecurityLevel::Driver,
            allowed_syscall_vectors: None,
            allowed_io_ports: None, // Drivers can access IO
            max_indirect_sites: Some(50),
            allow_rwx: false,
            allow_self_modifying: false,
            allow_far_jumps: true,
            require_structural_integrity: true,
            allow_process_injection: false,
            allow_timing_instructions: true,
            allow_hidden_entry_points: false,
            allowed_hardware_devices: None, // Drivers can access hardware
        }
    }

    pub fn service() -> Self {
        Self {
            name: "Service".to_string(),
            level: SecurityLevel::Service,
            allowed_syscall_vectors: Some(vec![0x80]),
            allowed_io_ports: Some(Vec::new()), // No IO allowed
            max_indirect_sites: Some(20),
            allow_rwx: false,
            allow_self_modifying: false,
            allow_far_jumps: false,
            require_structural_integrity: true,
            allow_process_injection: false,
            allow_timing_instructions: false,
            allow_hidden_entry_points: false,
            allowed_hardware_devices: Some(Vec::new()), // No hardware
        }
    }

    pub fn user() -> Self {
        Self {
            name: "User".to_string(),
            level: SecurityLevel::User,
            allowed_syscall_vectors: Some(vec![0x80]),
            allowed_io_ports: Some(Vec::new()),
            max_indirect_sites: Some(10),
            allow_rwx: false,
            allow_self_modifying: false,
            allow_far_jumps: false,
            require_structural_integrity: true,
            allow_process_injection: false,
            allow_timing_instructions: false,
            allow_hidden_entry_points: false,
            allowed_hardware_devices: Some(Vec::new()),
        }
    }

    pub fn sandbox() -> Self {
        Self {
            name: "Sandbox".to_string(),
            level: SecurityLevel::User,
            allowed_syscall_vectors: Some(Vec::new()),
            allowed_io_ports: Some(Vec::new()),
            max_indirect_sites: Some(0),
            allow_rwx: false,
            allow_self_modifying: false,
            allow_far_jumps: false,
            require_structural_integrity: true,
            allow_process_injection: false,
            allow_timing_instructions: false,
            allow_hidden_entry_points: false,
            allowed_hardware_devices: Some(Vec::new()),
        }
    }
}

// ============================================================
// Policy Engine
// ============================================================

pub struct PolicyEngine;

impl PolicyEngine {
    /// Evalúa un ArchitectureMap contra una SecurityPolicy.
    /// Determinista: mismo input → mismo output. Siempre.
    pub fn evaluate(map: &ArchitectureMap, policy: &SecurityPolicy) -> Verdict {
        let mut violations = Vec::new();

        match policy.level {
            SecurityLevel::Kernel => {} // Kernel can do anything
            SecurityLevel::Driver => Self::check_driver_violations(map, policy, &mut violations),
            SecurityLevel::Service => Self::check_service_violations(map, policy, &mut violations),
            SecurityLevel::User => Self::check_user_violations(map, policy, &mut violations),
        }

        // Universal checks (apply to non-kernel levels)
        if policy.level != SecurityLevel::Kernel {
            Self::check_universal(map, policy, &mut violations);
        }

        // Structural integrity checks
        if policy.require_structural_integrity {
            Self::check_structural_integrity(map, &mut violations);
        }

        // Import-based checks
        Self::check_import_violations(map, policy, &mut violations);

        // Timing attack checks
        Self::check_timing_violations(map, policy, &mut violations);

        // Hidden entry point checks
        Self::check_hidden_entry_points(map, policy, &mut violations);

        // Hardware access checks
        Self::check_hardware_access(map, policy, &mut violations);

        if violations.is_empty() {
            Verdict::Approved
        } else {
            Verdict::Denied { violations }
        }
    }

    // ============================================================
    // Level-specific checks
    // ============================================================

    fn check_driver_violations(map: &ArchitectureMap, _policy: &SecurityPolicy, violations: &mut Vec<Violation>) {
        // Drivers can use IO ports and interrupts, but not certain privileged ops
        if map.capabilities.msr_access {
            violations.push(Violation {
                violation_type: ViolationType::PrivilegedInstruction,
                description: "MSR access not permitted at driver level".to_string(),
                severity: ViolationSeverity::High,
            });
        }
        if map.capabilities.descriptor_table_access {
            violations.push(Violation {
                violation_type: ViolationType::PrivilegedInstruction,
                description: "Descriptor table modification not permitted at driver level".to_string(),
                severity: ViolationSeverity::High,
            });
        }
    }

    fn check_service_violations(map: &ArchitectureMap, policy: &SecurityPolicy, violations: &mut Vec<Violation>) {
        // Services: no direct IO, no privileged instructions
        if map.instruction_map.privileged_count > 0 {
            violations.push(Violation {
                violation_type: ViolationType::PrivilegedInstruction,
                description: format!("{} privileged instruction(s) at service level",
                    map.instruction_map.privileged_count),
                severity: ViolationSeverity::Critical,
            });
        }
        if map.io_map.has_io() {
            violations.push(Violation {
                violation_type: ViolationType::UnauthorizedIO,
                description: format!("IO port access ({} accesses) not allowed at service level",
                    map.io_map.accesses.len()),
                severity: ViolationSeverity::Critical,
            });
        }

        Self::check_syscall_vectors(map, policy, violations);
    }

    fn check_user_violations(map: &ArchitectureMap, policy: &SecurityPolicy, violations: &mut Vec<Violation>) {
        // User: most restrictive
        if map.instruction_map.privileged_count > 0 {
            violations.push(Violation {
                violation_type: ViolationType::PrivilegedInstruction,
                description: format!("{} privileged instruction(s) at user level",
                    map.instruction_map.privileged_count),
                severity: ViolationSeverity::Critical,
            });
        }
        if map.io_map.has_io() {
            violations.push(Violation {
                violation_type: ViolationType::UnauthorizedIO,
                description: format!("IO port access ({} accesses) forbidden at user level",
                    map.io_map.accesses.len()),
                severity: ViolationSeverity::Critical,
            });
        }

        Self::check_syscall_vectors(map, policy, violations);
    }

    // ============================================================
    // Universal checks
    // ============================================================

    fn check_universal(map: &ArchitectureMap, policy: &SecurityPolicy, violations: &mut Vec<Violation>) {
        // RWX memory
        if !policy.allow_rwx && map.memory_map.rwx_count > 0 {
            violations.push(Violation {
                violation_type: ViolationType::RWXMemory,
                description: format!("{} RWX memory region(s)", map.memory_map.rwx_count),
                severity: ViolationSeverity::High,
            });
        }

        // Self-modifying code
        if !policy.allow_self_modifying && map.memory_map.self_modifying_code {
            violations.push(Violation {
                violation_type: ViolationType::SelfModifyingCode,
                description: "Self-modifying code detected".to_string(),
                severity: ViolationSeverity::High,
            });
        }

        // Far jumps
        if !policy.allow_far_jumps && map.control_flow_map.far_jumps > 0 {
            violations.push(Violation {
                violation_type: ViolationType::FarJumpNotAllowed,
                description: format!("{} far jump(s) not permitted", map.control_flow_map.far_jumps),
                severity: ViolationSeverity::Medium,
            });
        }

        // Excessive indirect control flow
        if let Some(max) = policy.max_indirect_sites {
            let total = map.control_flow_map.indirect_calls + map.control_flow_map.indirect_jumps;
            if total > max {
                violations.push(Violation {
                    violation_type: ViolationType::ExcessiveIndirectControl,
                    description: format!("{} indirect sites (max: {})", total, max),
                    severity: ViolationSeverity::Medium,
                });
            }
        }
    }

    // ============================================================
    // Structural integrity checks
    // ============================================================

    fn check_structural_integrity(map: &ArchitectureMap, violations: &mut Vec<Violation>) {
        if map.integrity.entry_point_checked && !map.integrity.entry_point_valid {
            violations.push(Violation {
                violation_type: ViolationType::InvalidEntryPoint,
                description: "Entry point does not reference a valid code section".to_string(),
                severity: ViolationSeverity::Critical,
            });
        }

        if map.integrity.overlapping_sections {
            violations.push(Violation {
                violation_type: ViolationType::OverlappingSections,
                description: "Binary contains overlapping sections — structural anomaly".to_string(),
                severity: ViolationSeverity::Critical,
            });
        }

        if map.integrity.anomalous_permissions > 0 {
            violations.push(Violation {
                violation_type: ViolationType::AnomalousPermissions,
                description: format!("{} section(s) with anomalous data+execute permissions",
                    map.integrity.anomalous_permissions),
                severity: ViolationSeverity::High,
            });
        }
    }

    // ============================================================
    // Import-based checks
    // ============================================================

    fn check_import_violations(map: &ArchitectureMap, policy: &SecurityPolicy, violations: &mut Vec<Violation>) {
        if !policy.allow_process_injection && !map.import_export_map.process_injection_apis.is_empty() {
            violations.push(Violation {
                violation_type: ViolationType::ProcessInjectionImports,
                description: format!("Process injection APIs detected: {:?}",
                    map.import_export_map.process_injection_apis),
                severity: ViolationSeverity::Critical,
            });
        }
    }

    // ============================================================
    // Timing / debug checks
    // ============================================================

    fn check_timing_violations(map: &ArchitectureMap, policy: &SecurityPolicy, violations: &mut Vec<Violation>) {
        if !policy.allow_timing_instructions && map.hardware_map.timing_access {
            violations.push(Violation {
                violation_type: ViolationType::TimingAttackCapability,
                description: "Timing instructions (RDTSC/RDTSCP) detected — potential timing side-channel".to_string(),
                severity: ViolationSeverity::Medium,
            });
        }
    }

    // ============================================================
    // Hidden entry point checks
    // ============================================================

    fn check_hidden_entry_points(map: &ArchitectureMap, policy: &SecurityPolicy, violations: &mut Vec<Violation>) {
        if !policy.allow_hidden_entry_points && map.integrity.has_tls_callbacks {
            violations.push(Violation {
                violation_type: ViolationType::HiddenEntryPoint,
                description: format!("{} TLS callback(s) — code executes before entry point",
                    map.integrity.tls_callback_count),
                severity: ViolationSeverity::High,
            });
        }
    }

    // ============================================================
    // Hardware access checks
    // ============================================================

    fn check_hardware_access(map: &ArchitectureMap, policy: &SecurityPolicy, violations: &mut Vec<Violation>) {
        if let Some(ref allowed) = policy.allowed_hardware_devices {
            for device in &map.hardware_map.devices_accessed {
                if !allowed.contains(device) {
                    violations.push(Violation {
                        violation_type: ViolationType::UnauthorizedHardwareAccess,
                        description: format!("Unauthorized access to {}", device),
                        severity: ViolationSeverity::Critical,
                    });
                }
            }
        }
    }

    // ============================================================
    // Helpers
    // ============================================================

    fn check_syscall_vectors(map: &ArchitectureMap, policy: &SecurityPolicy, violations: &mut Vec<Violation>) {
        if let Some(ref allowed) = policy.allowed_syscall_vectors {
            for vec in &map.syscall_map.interrupt_vectors {
                if !allowed.contains(vec) {
                    violations.push(Violation {
                        violation_type: ViolationType::UnauthorizedSyscall,
                        description: format!("INT vector 0x{:02X} not in allowed set", vec),
                        severity: ViolationSeverity::High,
                    });
                }
            }
        }
    }

    /// Infiere el nivel mínimo de seguridad necesario para este binario.
    pub fn infer_minimum_level(map: &ArchitectureMap) -> SecurityLevel {
        if map.capabilities.requires_kernel() {
            SecurityLevel::Kernel
        } else if map.capabilities.far_jumps || map.capabilities.interrupts {
            SecurityLevel::Driver
        } else if map.capabilities.syscalls {
            SecurityLevel::Service
        } else {
            SecurityLevel::User
        }
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_safe_map() -> ArchitectureMap {
        let mut map = ArchitectureMap::new();
        map.instruction_map.total = 100;
        map.instruction_map.safe_count = 100;
        map
    }

    fn make_kernel_map() -> ArchitectureMap {
        let mut map = ArchitectureMap::new();
        map.instruction_map.total = 10;
        map.instruction_map.privileged_count = 5;
        map.capabilities.privileged_instructions = true;
        map.capabilities.io_port_access = true;
        map.capabilities.interrupt_control = true;
        map
    }

    #[test]
    fn test_safe_approved() {
        let map = make_safe_map();
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        assert!(verdict.is_approved());
    }

    #[test]
    fn test_kernel_approved_for_kernel() {
        let map = make_kernel_map();
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::kernel());
        assert!(verdict.is_approved());
    }

    #[test]
    fn test_kernel_denied_for_user() {
        let map = make_kernel_map();
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        assert!(verdict.is_denied());
    }

    #[test]
    fn test_structural_integrity_violation() {
        let mut map = make_safe_map();
        map.integrity.entry_point_checked = true;
        map.integrity.entry_point_valid = false;
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        assert!(verdict.is_denied());
        if let Verdict::Denied { violations } = verdict {
            assert!(violations.iter().any(|v| v.violation_type == ViolationType::InvalidEntryPoint));
        }
    }

    #[test]
    fn test_overlapping_sections_violation() {
        let mut map = make_safe_map();
        map.integrity.overlapping_sections = true;
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        assert!(verdict.is_denied());
    }

    #[test]
    fn test_injection_api_violation() {
        let mut map = make_safe_map();
        map.import_export_map.process_injection_apis.push("WriteProcessMemory".to_string());
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        assert!(verdict.is_denied());
        if let Verdict::Denied { violations } = verdict {
            assert!(violations.iter().any(|v| v.violation_type == ViolationType::ProcessInjectionImports));
        }
    }

    #[test]
    fn test_tls_callback_violation() {
        let mut map = make_safe_map();
        map.integrity.has_tls_callbacks = true;
        map.integrity.tls_callback_count = 2;
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        assert!(verdict.is_denied());
        if let Verdict::Denied { violations } = verdict {
            assert!(violations.iter().any(|v| v.violation_type == ViolationType::HiddenEntryPoint));
        }
    }

    #[test]
    fn test_timing_attack_violation() {
        let mut map = make_safe_map();
        map.hardware_map.timing_access = true;
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        assert!(verdict.is_denied());
        if let Verdict::Denied { violations } = verdict {
            assert!(violations.iter().any(|v| v.violation_type == ViolationType::TimingAttackCapability));
        }
    }

    #[test]
    fn test_hardware_access_violation() {
        let mut map = make_safe_map();
        map.hardware_map.devices_accessed.push(HardwareDevice::KeyboardPS2);
        let policy = SecurityPolicy::user(); // No hardware allowed
        let verdict = PolicyEngine::evaluate(&map, &policy);
        assert!(verdict.is_denied());
        if let Verdict::Denied { violations } = verdict {
            assert!(violations.iter().any(|v| v.violation_type == ViolationType::UnauthorizedHardwareAccess));
        }
    }

    #[test]
    fn test_kernel_allows_everything() {
        let mut map = make_kernel_map();
        map.integrity.has_tls_callbacks = true;
        map.import_export_map.process_injection_apis.push("WriteProcessMemory".to_string());
        map.hardware_map.timing_access = true;
        map.hardware_map.devices_accessed.push(HardwareDevice::IdePrimary);
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::kernel());
        assert!(verdict.is_approved());
    }

    #[test]
    fn test_infer_levels() {
        let safe = make_safe_map();
        assert_eq!(PolicyEngine::infer_minimum_level(&safe), SecurityLevel::User);

        let kernel = make_kernel_map();
        assert_eq!(PolicyEngine::infer_minimum_level(&kernel), SecurityLevel::Kernel);

        let mut syscall_map = make_safe_map();
        syscall_map.capabilities.syscalls = true;
        assert_eq!(PolicyEngine::infer_minimum_level(&syscall_map), SecurityLevel::Service);
    }

    #[test]
    fn test_violation_severity() {
        let mut map = make_safe_map();
        map.integrity.entry_point_checked = true;
        map.integrity.entry_point_valid = false;
        map.integrity.overlapping_sections = true;
        let verdict = PolicyEngine::evaluate(&map, &SecurityPolicy::user());
        if let Verdict::Denied { violations } = verdict {
            assert!(violations.iter().any(|v| v.severity == ViolationSeverity::Critical));
            assert!(violations.len() >= 2);
        }
    }
}
