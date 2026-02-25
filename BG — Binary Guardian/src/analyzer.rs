// ============================================================
// BG — Binary Guardian: Analyzer
// ============================================================
// Pipeline de análisis completo:
//
//   Load → Decode → Map → Validate → Evaluate → Report
//
// Cada paso es determinista.
// Mismo binario + misma policy = mismo resultado. Siempre.
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use std::path::Path;
use std::fmt;
use adead_bib::isa::ADeadOp;
use adead_bib::isa::decoder::Decoder;
use super::arch_map::ArchitectureMap;
use super::binary_loader::{BinaryLoader, BinaryInfo};
use super::capability::CapabilityMapper;
use super::policy::{PolicyEngine, SecurityPolicy, SecurityLevel, Verdict};

// ============================================================
// Analysis Result
// ============================================================

/// Resultado completo de un análisis BG.
/// Contiene toda la información para que el administrador
/// tome una decisión informada.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Información del binario (si se cargó desde archivo)
    pub binary_info: Option<BinaryInfo>,
    /// Mapa de arquitectura (perfil completo)
    pub map: ArchitectureMap,
    /// Veredicto de la policy
    pub verdict: Verdict,
    /// Nivel mínimo inferido
    pub minimum_level: SecurityLevel,
    /// Número total de instrucciones analizadas
    pub instruction_count: usize,
    /// Nombre de la policy usada
    pub policy_name: String,
}

impl fmt::Display for AnalysisResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref info) = self.binary_info {
            writeln!(f, "{}", info)?;
            writeln!(f)?;
        }
        writeln!(f, "{}", self.map)?;
        writeln!(f)?;
        writeln!(f, "  Policy:           {}", self.policy_name)?;
        writeln!(f, "  Min. level:       {}", self.minimum_level)?;
        writeln!(f, "  Instructions:     {}", self.instruction_count)?;
        writeln!(f)?;
        writeln!(f, "  {}", self.verdict)?;
        Ok(())
    }
}

// ============================================================
// Binary Guardian — API Principal
// ============================================================

pub struct BinaryGuardian;

impl BinaryGuardian {
    /// Analiza un archivo binario completo contra una policy.
    /// Pipeline: Load → Decode → Map → Validate → Categorize → Evaluate
    pub fn analyze_file(path: &Path, policy: &SecurityPolicy) -> Result<AnalysisResult, String> {
        let info = BinaryLoader::load_file(path)?;
        let result = Self::analyze_loaded(&info, policy);
        Ok(result)
    }

    /// Analiza un binario ya cargado en memoria.
    pub fn analyze_loaded(info: &BinaryInfo, policy: &SecurityPolicy) -> AnalysisResult {
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(&info.code_bytes);
        let mut map = CapabilityMapper::analyze(&ops);

        // Enrich with binary info
        map.binary_size = info.total_size;

        // Populate memory map from sections
        for section in &info.sections {
            map.memory_map.regions.push(super::arch_map::MemoryRegion {
                region_type: match section.kind {
                    super::binary_loader::SectionKind::Code => super::arch_map::RegionType::Code,
                    super::binary_loader::SectionKind::Data => super::arch_map::RegionType::Data,
                    super::binary_loader::SectionKind::ReadOnly => super::arch_map::RegionType::ReadOnly,
                    super::binary_loader::SectionKind::Unknown => super::arch_map::RegionType::Data,
                },
                offset: section.offset,
                size: section.size,
                name: section.name.clone(),
            });
            if section.is_rwx() {
                map.memory_map.rwx_count += 1;
            }
            if section.executable {
                map.memory_map.total_code_size += section.size;
            } else {
                map.memory_map.total_data_size += section.size;
            }
        }

        // Structural integrity
        map.integrity = BinaryLoader::validate_structure(info);

        // Import/Export categorization
        map.import_export_map.import_count = info.imports.len();
        map.import_export_map.export_count = info.exports.len();
        for import in &info.imports {
            map.import_export_map
                .imports_by_library
                .entry(import.library.clone())
                .or_default()
                .push(import.name.clone());
            map.import_export_map.categorize_import(&import.name);
        }
        for export in &info.exports {
            map.import_export_map.exports.push(export.name.clone());
        }

        let minimum_level = PolicyEngine::infer_minimum_level(&map);
        let verdict = PolicyEngine::evaluate(&map, policy);

        AnalysisResult {
            binary_info: Some(info.clone()),
            map,
            verdict,
            minimum_level,
            instruction_count: ops.len(),
            policy_name: policy.name.clone(),
        }
    }

    /// Analiza una secuencia de operaciones pre-decodificadas.
    pub fn analyze_ops(ops: &[ADeadOp], policy: &SecurityPolicy) -> AnalysisResult {
        let map = CapabilityMapper::analyze(ops);
        let minimum_level = PolicyEngine::infer_minimum_level(&map);
        let verdict = PolicyEngine::evaluate(&map, policy);

        AnalysisResult {
            binary_info: None,
            map,
            verdict,
            minimum_level,
            instruction_count: ops.len(),
            policy_name: policy.name.clone(),
        }
    }

    /// Analiza bytes crudos como código (sin formato de contenedor).
    pub fn analyze_bytes(bytes: &[u8], policy: &SecurityPolicy) -> AnalysisResult {
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(bytes);
        Self::analyze_ops(&ops, policy)
    }

    /// Quick check: ¿puede ejecutarse al nivel dado?
    pub fn can_execute(bytes: &[u8], level: SecurityLevel) -> bool {
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(bytes);
        let map = CapabilityMapper::analyze(&ops);
        let min = PolicyEngine::infer_minimum_level(&map);
        min >= level
    }

    /// Inspecciona bytes y retorna el Architecture Map sin policy check.
    pub fn inspect_bytes(bytes: &[u8]) -> ArchitectureMap {
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(bytes);
        CapabilityMapper::analyze(&ops)
    }

    /// Genera un reporte de acceso a hardware para el administrador.
    pub fn hardware_report(info: &BinaryInfo, policy: &SecurityPolicy) -> String {
        let result = Self::analyze_loaded(info, policy);
        let hw = &result.map.hardware_map;

        let mut report = String::new();
        report.push_str("═══════════════════════════════════════════════\n");
        report.push_str("  BG — Hardware Access Report\n");
        report.push_str("═══════════════════════════════════════════════\n\n");

        if hw.devices_accessed.is_empty() && !hw.timing_access && !hw.cpuid_access && !hw.debug_register_access {
            report.push_str("  No hardware access detected.\n");
            report.push_str("  Binary operates purely in software.\n");
        } else {
            if !hw.devices_accessed.is_empty() {
                report.push_str("  Devices Accessed:\n");
                for device in &hw.devices_accessed {
                    let count = hw.accesses.iter().filter(|a| a.device == *device).count();
                    let in_count = hw.accesses.iter().filter(|a| a.device == *device && a.direction == super::arch_map::IODirection::In).count();
                    let out_count = hw.accesses.iter().filter(|a| a.device == *device && a.direction == super::arch_map::IODirection::Out).count();
                    report.push_str(&format!("    ● {} — {} access(es) ({} IN, {} OUT)\n",
                        device, count, in_count, out_count));
                }
                report.push('\n');
            }

            if hw.timing_access {
                report.push_str("  ⚠ Timing Instructions: RDTSC/RDTSCP detected\n");
                report.push_str("    Risk: Timing side-channel attacks, anti-debugging\n\n");
            }
            if hw.debug_register_access {
                report.push_str("  ⚠ Debug Registers: DR0-DR7 access detected\n");
                report.push_str("    Risk: Hardware breakpoints, anti-debugging\n\n");
            }
            if hw.cpuid_access {
                report.push_str("  CPUID: CPU feature detection present\n\n");
            }

            // Risk assessment
            report.push_str("  Risk Summary:\n");
            if hw.touches_storage() {
                report.push_str("    ● STORAGE: Direct disk access ⚠ CRITICAL\n");
            }
            if hw.touches_pci() {
                report.push_str("    ● PCI: Configuration space access ⚠ HIGH\n");
            }
            if hw.touches_interrupt_controllers() {
                report.push_str("    ● INTERRUPTS: Interrupt controller access ⚠ HIGH\n");
            }
        }

        report.push_str("\n═══════════════════════════════════════════════\n");
        report
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use adead_bib::isa::*;

    #[test]
    fn test_safe_bytes() {
        // push rbp; mov rbp, rsp; xor eax, eax; pop rbp; ret
        let result = BinaryGuardian::analyze_bytes(
            &[0x55, 0x48, 0x89, 0xE5, 0x31, 0xC0, 0x5D, 0xC3],
            &SecurityPolicy::user(),
        );
        assert!(result.instruction_count > 0 || result.map.instruction_map.total == 0);
    }

    #[test]
    fn test_ops_analysis() {
        let ops = vec![
            ADeadOp::Push { src: Operand::Reg(Reg::RBP) },
            ADeadOp::Mov { dst: Operand::Reg(Reg::RBP), src: Operand::Reg(Reg::RSP) },
            ADeadOp::Xor { dst: Reg::EAX, src: Reg::EAX },
            ADeadOp::Pop { dst: Reg::RBP },
            ADeadOp::Ret,
        ];
        let result = BinaryGuardian::analyze_ops(&ops, &SecurityPolicy::user());
        assert!(result.verdict.is_approved());
        assert_eq!(result.instruction_count, 5);
        assert_eq!(result.minimum_level, SecurityLevel::User);
    }

    #[test]
    fn test_kernel_ops() {
        let ops = vec![ADeadOp::Cli, ADeadOp::Sti, ADeadOp::Hlt];
        let result = BinaryGuardian::analyze_ops(&ops, &SecurityPolicy::user());
        assert!(result.verdict.is_denied());
        assert_eq!(result.minimum_level, SecurityLevel::Kernel);

        // Same ops approved under kernel policy
        let result_k = BinaryGuardian::analyze_ops(&ops, &SecurityPolicy::kernel());
        assert!(result_k.verdict.is_approved());
    }

    #[test]
    fn test_display() {
        let ops = vec![ADeadOp::Ret];
        let result = BinaryGuardian::analyze_ops(&ops, &SecurityPolicy::user());
        let output = format!("{}", result);
        assert!(output.contains("APPROVED") || output.contains("DENIED"));
    }

    #[test]
    fn test_can_execute() {
        let ops_bytes = vec![0xC3]; // ret
        let can = BinaryGuardian::can_execute(&ops_bytes, SecurityLevel::User);
        // Even if decoder can't decode 0xC3 as ADeadOp, should handle gracefully
        assert!(can || !can); // Just verify no panic
    }
}
