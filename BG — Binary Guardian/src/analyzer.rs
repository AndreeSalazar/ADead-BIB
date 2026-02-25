// ============================================================
// BG — Binary Guardian: Full Binary Analyzer
// ============================================================
// Pipeline completo: archivo → loader → decoder → map → verdict
//
//   External Binary (.exe / .elf / .bin)
//       ↓
//   Binary Loader (PE/ELF/Raw → secciones + code bytes)
//       ↓
//   ISA Decoder (bytes → ADeadOp)
//       ↓
//   Capability Mapper (ADeadOp → ArchitectureMap)
//       ↓
//   Policy Engine (ArchitectureMap × Policy → Verdict)
//       ↓
//   APPROVE / DENY
//
// Pre-execution, single-pass, determinista.
// Diseñado para integración con FastOS loader.
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use std::path::Path;
use adead_bib::isa::decoder::Decoder;
use adead_bib::isa::ADeadOp;
use crate::arch_map::ArchitectureMap;
use crate::binary_loader::{BinaryLoader, BinaryInfo};
use crate::capability::CapabilityMapper;
use crate::policy::{PolicyEngine, SecurityPolicy, SecurityLevel, Verdict};

/// Resultado de analizar un binario.
#[derive(Debug)]
pub struct AnalysisResult {
    /// Información del binario (formato, secciones, etc.)
    pub binary_info: Option<BinaryInfo>,
    /// El architecture map completo del binario
    pub map: ArchitectureMap,
    /// El veredicto de seguridad (APPROVED/DENIED)
    pub verdict: Verdict,
    /// El nivel de seguridad mínimo requerido inferido
    pub minimum_level: SecurityLevel,
    /// Número de instrucciones decodificadas
    pub instruction_count: usize,
    /// Policy usada para el análisis
    pub policy_name: String,
}

/// Binary Guardian — Deterministic ISA-Level Capability Guardian.
///
/// Analiza binarios antes de ejecutarlos. No es antivirus.
/// No es sandbox. Es una arquitectura de control estructural.
///
/// # Uso
/// ```rust,no_run
/// use bg::{BinaryGuardian, SecurityPolicy};
/// use std::path::Path;
///
/// // Analizar un archivo binario
/// let result = BinaryGuardian::analyze_file(
///     Path::new("program.exe"),
///     &SecurityPolicy::user(),
/// ).unwrap();
///
/// println!("{}", result);
/// ```
pub struct BinaryGuardian;

impl BinaryGuardian {
    /// Analiza un archivo binario (PE/ELF/raw) contra una security policy.
    ///
    /// Pipeline completo: Load → Decode → Map → Evaluate
    pub fn analyze_file(path: &Path, policy: &SecurityPolicy) -> Result<AnalysisResult, String> {
        let info = BinaryLoader::load_file(path)?;
        let result = Self::analyze_loaded(&info, policy);
        Ok(result)
    }

    /// Analiza un BinaryInfo ya cargado contra una security policy.
    pub fn analyze_loaded(info: &BinaryInfo, policy: &SecurityPolicy) -> AnalysisResult {
        // Decode
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(&info.code_bytes);

        // Map
        let mut map = CapabilityMapper::analyze(&ops);

        // Enriquecer el map con info del loader
        map.binary_name = Some(info.path.clone());
        map.binary_size = info.total_size;

        // Memory map desde secciones
        for sec in &info.sections {
            map.memory_map.regions.push(crate::arch_map::MemoryRegion {
                region_type: match sec.kind {
                    crate::binary_loader::SectionKind::Code => crate::arch_map::RegionType::Code,
                    crate::binary_loader::SectionKind::Data => crate::arch_map::RegionType::Data,
                    crate::binary_loader::SectionKind::ReadOnly => crate::arch_map::RegionType::ReadOnly,
                    crate::binary_loader::SectionKind::RWX => crate::arch_map::RegionType::RWX,
                    crate::binary_loader::SectionKind::Unknown => crate::arch_map::RegionType::Data,
                },
                offset: sec.offset,
                size: sec.size,
                name: sec.name.clone(),
            });

            if sec.executable {
                map.memory_map.total_code_size += sec.size;
            } else {
                map.memory_map.total_data_size += sec.size;
            }
        }
        map.memory_map.rwx_count = info.rwx_count;

        // Infer + Evaluate
        let minimum_level = PolicyEngine::infer_minimum_level(&map);
        let verdict = PolicyEngine::evaluate(&map, policy);

        AnalysisResult {
            instruction_count: map.instruction_map.total,
            binary_info: Some(info.clone()),
            map,
            verdict,
            minimum_level,
            policy_name: policy.name.clone(),
        }
    }

    /// Analiza bytes crudos x86-64 contra una security policy.
    pub fn analyze_bytes(code: &[u8], policy: &SecurityPolicy) -> AnalysisResult {
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(code);
        Self::analyze_ops(&ops, policy)
    }

    /// Analiza instrucciones ADeadOp ya decodificadas.
    pub fn analyze_ops(ops: &[ADeadOp], policy: &SecurityPolicy) -> AnalysisResult {
        let map = CapabilityMapper::analyze(ops);
        let minimum_level = PolicyEngine::infer_minimum_level(&map);
        let verdict = PolicyEngine::evaluate(&map, policy);

        AnalysisResult {
            instruction_count: map.instruction_map.total,
            binary_info: None,
            map,
            verdict,
            minimum_level,
            policy_name: policy.name.clone(),
        }
    }

    /// Quick check: ¿puede este binario ejecutarse al nivel dado?
    pub fn can_execute(code: &[u8], level: SecurityLevel) -> bool {
        let policy = match level {
            SecurityLevel::Kernel => SecurityPolicy::kernel(),
            SecurityLevel::Driver => SecurityPolicy::driver(),
            SecurityLevel::Service => SecurityPolicy::service(),
            SecurityLevel::User => SecurityPolicy::user(),
        };
        Self::analyze_bytes(code, &policy).verdict.is_approved()
    }

    /// Inspecciona bytes y retorna solo el Architecture Map (sin policy check).
    pub fn inspect_bytes(code: &[u8]) -> ArchitectureMap {
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(code);
        CapabilityMapper::analyze(&ops)
    }

    /// Inspecciona IR del compilador (ADeadOp).
    /// Zero-cost path cuando BG está integrado en el pipeline del compilador.
    pub fn inspect_ir(ops: &[ADeadOp]) -> ArchitectureMap {
        CapabilityMapper::analyze(ops)
    }
}

impl std::fmt::Display for AnalysisResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref info) = self.binary_info {
            writeln!(f, "{}", info)?;
            writeln!(f)?;
        }
        writeln!(f, "{}", self.map)?;
        writeln!(f)?;
        writeln!(f, "  Policy:         {}", self.policy_name)?;
        writeln!(f, "  Minimum level:  {}", self.minimum_level)?;
        writeln!(f, "  Verdict:        {}", self.verdict)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_safe_prologue() {
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0x5D, 0xC3];
        let result = BinaryGuardian::analyze_bytes(&code, &SecurityPolicy::user());
        assert!(result.verdict.is_approved());
        assert_eq!(result.minimum_level, SecurityLevel::User);
    }

    #[test]
    fn test_quick_check() {
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
        assert!(BinaryGuardian::can_execute(&code, SecurityLevel::User));
    }

    #[test]
    fn test_inspect() {
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
        let map = BinaryGuardian::inspect_bytes(&code);
        assert!(map.capabilities.is_pure_userspace());
    }

    #[test]
    fn test_compiler_integration() {
        use adead_bib::isa::*;
        let ops = vec![
            ADeadOp::Push { src: Operand::Reg(Reg::RBP) },
            ADeadOp::Mov { dst: Operand::Reg(Reg::RBP), src: Operand::Reg(Reg::RSP) },
            ADeadOp::Ret,
        ];
        let result = BinaryGuardian::analyze_ops(&ops, &SecurityPolicy::user());
        assert!(result.verdict.is_approved());
    }

    #[test]
    fn test_display() {
        let code = vec![0x55, 0x48, 0x89, 0xE5, 0xC3];
        let result = BinaryGuardian::analyze_bytes(&code, &SecurityPolicy::user());
        let s = format!("{}", result);
        assert!(s.contains("APPROVED"));
    }
}
