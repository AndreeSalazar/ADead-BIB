// ============================================================
// BG — Binary Guardian: Capability Mapper
// ============================================================
// Analyzes ADeadOp instructions and produces an ArchitectureMap.
//
// This is purely structural analysis — no heuristics.
// Each instruction is classified by what it IS, not by what
// it LOOKS LIKE.
//
// O(n) single-pass analysis over the instruction stream.
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use crate::isa::{ADeadOp, CallTarget, Operand};
use super::arch_map::*;

/// Capability Mapper — Analyzes ABIB IR and builds an Architecture Map.
pub struct CapabilityMapper;

impl CapabilityMapper {
    /// Analyze a sequence of ADeadOp instructions and produce a complete
    /// ArchitectureMap. Single-pass, O(n).
    pub fn analyze(ops: &[ADeadOp]) -> ArchitectureMap {
        let mut map = ArchitectureMap::new();

        for (i, op) in ops.iter().enumerate() {
            let class = Self::classify(op);

            // Instruction Map
            map.instruction_map.total += 1;
            match class {
                InstructionClass::Safe => map.instruction_map.safe_count += 1,
                InstructionClass::Restricted => {
                    map.instruction_map.restricted_count += 1;
                    map.instruction_map.flagged.push((i, class));
                }
                InstructionClass::Privileged => {
                    map.instruction_map.privileged_count += 1;
                    map.instruction_map.flagged.push((i, class));
                }
            }

            // Syscall Map
            match op {
                ADeadOp::Syscall => {
                    map.syscall_map.syscall_count += 1;
                    map.syscall_map.uses_syscall_instruction = true;
                    map.syscall_map.call_sites.push(i);
                }
                ADeadOp::Int { vector } => {
                    map.syscall_map.syscall_count += 1;
                    if !map.syscall_map.interrupt_vectors.contains(vector) {
                        map.syscall_map.interrupt_vectors.push(*vector);
                    }
                    map.syscall_map.call_sites.push(i);
                }
                _ => {}
            }

            // IO Map
            match op {
                ADeadOp::InByte { port } => {
                    map.io_map.accesses.push(IOAccess {
                        port: Self::extract_static_port(port),
                        direction: IODirection::In,
                        instruction_index: i,
                    });
                }
                ADeadOp::OutByte { port, .. } => {
                    map.io_map.accesses.push(IOAccess {
                        port: Self::extract_static_port(port),
                        direction: IODirection::Out,
                        instruction_index: i,
                    });
                }
                _ => {}
            }

            // Control Flow Map
            match op {
                ADeadOp::Jmp { .. } => {
                    map.control_flow_map.direct_jumps += 1;
                }
                ADeadOp::Jcc { .. } => {
                    map.control_flow_map.conditional_branches += 1;
                }
                ADeadOp::Call { target } => match target {
                    CallTarget::Relative(_) => {
                        map.control_flow_map.direct_calls += 1;
                    }
                    CallTarget::RipRelative(_) => {
                        map.control_flow_map.indirect_calls += 1;
                        map.control_flow_map.indirect_sites.push(i);
                    }
                    CallTarget::Name(_) => {
                        map.control_flow_map.direct_calls += 1;
                    }
                },
                ADeadOp::CallIAT { .. } => {
                    map.control_flow_map.indirect_calls += 1;
                    map.control_flow_map.indirect_sites.push(i);
                }
                ADeadOp::FarJmp { .. } => {
                    map.control_flow_map.far_jumps += 1;
                }
                _ => {}
            }

            // Capability detection for specific privileged ops
            match op {
                ADeadOp::Cli | ADeadOp::Sti => {
                    map.capabilities.interrupt_control = true;
                    map.capabilities.privileged_instructions = true;
                }
                ADeadOp::Hlt => {
                    map.capabilities.privileged_instructions = true;
                }
                ADeadOp::Lgdt { .. } | ADeadOp::Lidt { .. } => {
                    map.capabilities.descriptor_table_access = true;
                    map.capabilities.privileged_instructions = true;
                }
                ADeadOp::MovToCr { .. } | ADeadOp::MovFromCr { .. } => {
                    map.capabilities.control_register_access = true;
                    map.capabilities.privileged_instructions = true;
                }
                ADeadOp::Rdmsr | ADeadOp::Wrmsr => {
                    map.capabilities.msr_access = true;
                    map.capabilities.privileged_instructions = true;
                }
                ADeadOp::Invlpg { .. } => {
                    map.capabilities.privileged_instructions = true;
                }
                ADeadOp::InByte { .. } | ADeadOp::OutByte { .. } => {
                    map.capabilities.io_port_access = true;
                    map.capabilities.privileged_instructions = true;
                }
                ADeadOp::Syscall => {
                    map.capabilities.syscalls = true;
                }
                ADeadOp::Int { .. } => {
                    map.capabilities.interrupts = true;
                }
                ADeadOp::Iret => {
                    map.capabilities.privileged_instructions = true;
                }
                ADeadOp::FarJmp { .. } => {
                    map.capabilities.far_jumps = true;
                }
                ADeadOp::CallIAT { .. } => {
                    map.capabilities.indirect_control_flow = true;
                }
                ADeadOp::Call { target: CallTarget::RipRelative(_) } => {
                    map.capabilities.indirect_control_flow = true;
                }
                _ => {}
            }
        }

        map
    }

    /// Classify a single instruction. Deterministic, O(1).
    pub fn classify(op: &ADeadOp) -> InstructionClass {
        match op {
            // ---- Privileged (Ring 0 required) ----
            ADeadOp::Cli
            | ADeadOp::Sti
            | ADeadOp::Hlt
            | ADeadOp::Iret
            | ADeadOp::Lgdt { .. }
            | ADeadOp::Lidt { .. }
            | ADeadOp::MovToCr { .. }
            | ADeadOp::MovFromCr { .. }
            | ADeadOp::Rdmsr
            | ADeadOp::Wrmsr
            | ADeadOp::Invlpg { .. }
            | ADeadOp::InByte { .. }
            | ADeadOp::OutByte { .. } => InstructionClass::Privileged,

            // ---- Restricted (crosses privilege boundary) ----
            ADeadOp::Syscall
            | ADeadOp::Int { .. }
            | ADeadOp::FarJmp { .. } => InstructionClass::Restricted,

            // ---- Safe (everything else) ----
            _ => InstructionClass::Safe,
        }
    }

    /// Extract a static port number from an operand, if possible.
    fn extract_static_port(operand: &Operand) -> Option<u16> {
        match operand {
            Operand::Imm8(v) => Some(*v as u16),
            Operand::Imm16(v) => Some(*v as u16),
            Operand::Imm32(v) => Some(*v as u16),
            _ => None, // Dynamic port (via DX register)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::*;

    #[test]
    fn test_safe_program() {
        let ops = vec![
            ADeadOp::Push { src: Operand::Reg(Reg::RBP) },
            ADeadOp::Mov { dst: Operand::Reg(Reg::RBP), src: Operand::Reg(Reg::RSP) },
            ADeadOp::Xor { dst: Reg::EAX, src: Reg::EAX },
            ADeadOp::Pop { dst: Reg::RBP },
            ADeadOp::Ret,
        ];
        let map = CapabilityMapper::analyze(&ops);
        assert_eq!(map.instruction_map.total, 5);
        assert_eq!(map.instruction_map.safe_count, 5);
        assert_eq!(map.instruction_map.privileged_count, 0);
        assert!(map.capabilities.is_pure_userspace());
    }

    #[test]
    fn test_kernel_program() {
        let ops = vec![
            ADeadOp::Cli,
            ADeadOp::Lgdt { src: Operand::Mem { base: Reg::RAX, disp: 0 } },
            ADeadOp::MovToCr { cr: 0, src: Reg::RAX },
            ADeadOp::Sti,
            ADeadOp::Hlt,
        ];
        let map = CapabilityMapper::analyze(&ops);
        assert_eq!(map.instruction_map.privileged_count, 5);
        assert!(map.capabilities.requires_kernel());
        assert!(map.capabilities.interrupt_control);
        assert!(map.capabilities.descriptor_table_access);
        assert!(map.capabilities.control_register_access);
    }

    #[test]
    fn test_syscall_detection() {
        let ops = vec![
            ADeadOp::Mov { dst: Operand::Reg(Reg::RAX), src: Operand::Imm64(1) },
            ADeadOp::Syscall,
            ADeadOp::Int { vector: 0x80 },
        ];
        let map = CapabilityMapper::analyze(&ops);
        assert_eq!(map.syscall_map.syscall_count, 2);
        assert!(map.syscall_map.uses_syscall_instruction);
        assert_eq!(map.syscall_map.interrupt_vectors, vec![0x80]);
    }

    #[test]
    fn test_io_port_detection() {
        let ops = vec![
            ADeadOp::InByte { port: Operand::Imm8(0x60) },
            ADeadOp::OutByte {
                port: Operand::Imm8(0x20),
                src: Operand::Reg(Reg::AL),
            },
            ADeadOp::InByte { port: Operand::Reg(Reg::DX) },
        ];
        let map = CapabilityMapper::analyze(&ops);
        assert_eq!(map.io_map.accesses.len(), 3);
        assert!(map.capabilities.io_port_access);
        let static_ports = map.io_map.static_ports();
        assert!(static_ports.contains(&0x60));
        assert!(static_ports.contains(&0x20));
    }

    #[test]
    fn test_control_flow_analysis() {
        let ops = vec![
            ADeadOp::Call { target: CallTarget::Relative(Label(0)) },
            ADeadOp::Call { target: CallTarget::RipRelative(0x100) },
            ADeadOp::CallIAT { iat_rva: 0x2040 },
            ADeadOp::Jmp { target: Label(1) },
            ADeadOp::Jcc { cond: Condition::Equal, target: Label(2) },
            ADeadOp::FarJmp { selector: 0x08, offset: 0x1000 },
        ];
        let map = CapabilityMapper::analyze(&ops);
        assert_eq!(map.control_flow_map.direct_calls, 1);
        assert_eq!(map.control_flow_map.indirect_calls, 2);
        assert_eq!(map.control_flow_map.direct_jumps, 1);
        assert_eq!(map.control_flow_map.conditional_branches, 1);
        assert_eq!(map.control_flow_map.far_jumps, 1);
        assert!(map.capabilities.indirect_control_flow);
    }
}
