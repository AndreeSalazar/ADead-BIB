// ============================================================
// CUDead-BIB — GPU Compiler Nativo
// ============================================================
// Eddi Andreé Salazar Matos — Lima, Perú 🇵🇪
// ADead-BIB ecosystem — Binary Is Binary 💀🦈
//
// Compila código GPU SIN CUDA oficial de NVIDIA
// Genera drivers GPU mínimos (~100KB vs 500MB oficial)
// Conecta directamente al silicon RTX sin bloatware
//
// 8 PRIMITIVAS CORE:
// 1. __cudead_kernel__   → define función GPU
// 2. __cudead_device__   → función auxiliar GPU
// 3. cudead_launch()     → lanza grid de hilos
// 4. cudead_sync()       → sincroniza CPU↔GPU
// 5. cudead_malloc()     → alloca VRAM
// 6. cudead_free()       → libera VRAM
// 7. cudead_push()       → CPU RAM → GPU VRAM
// 8. cudead_pull()       → GPU VRAM → CPU RAM
//
// = 8 primitivas TOTALES vs CUDA oficial: 13 librerías + 500MB
// ============================================================

pub mod primitives;
pub mod ptx_emitter;
pub mod driver;
pub mod ub_detector;
pub mod parser;
pub mod ir;
pub mod optimizer;
pub mod cli;
pub mod cuda_driver;
pub mod runtime;

pub use primitives::*;
pub use ptx_emitter::*;
pub use driver::*;
pub use ub_detector::*;
pub use parser::*;
pub use ir::*;
pub use optimizer::*;

/// CUDead-BIB version
pub const CUDEAD_VERSION: &str = "1.0.0";

/// Target GPU architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuArch {
    /// NVIDIA Turing (RTX 20xx)
    Turing,
    /// NVIDIA Ampere (RTX 30xx)
    Ampere,
    /// NVIDIA Ada Lovelace (RTX 40xx)
    AdaLovelace,
    /// NVIDIA Blackwell (RTX 50xx)
    Blackwell,
    /// Generic SM (compute capability)
    Sm(u32, u32),
}

impl GpuArch {
    pub fn sm_version(&self) -> (u32, u32) {
        match self {
            GpuArch::Turing => (7, 5),
            GpuArch::Ampere => (8, 6),
            GpuArch::AdaLovelace => (8, 9),
            GpuArch::Blackwell => (9, 0),
            GpuArch::Sm(major, minor) => (*major, *minor),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            GpuArch::Turing => "Turing",
            GpuArch::Ampere => "Ampere",
            GpuArch::AdaLovelace => "Ada Lovelace",
            GpuArch::Blackwell => "Blackwell",
            GpuArch::Sm(_, _) => "Generic",
        }
    }

    /// Warp size (always 32 for NVIDIA)
    pub fn warp_size(&self) -> u32 {
        32
    }

    /// Max threads per block
    pub fn max_threads_per_block(&self) -> u32 {
        1024
    }

    /// Max shared memory per block (bytes)
    pub fn max_shared_memory(&self) -> u32 {
        match self {
            GpuArch::Turing => 64 * 1024,
            GpuArch::Ampere => 164 * 1024,
            GpuArch::AdaLovelace => 228 * 1024,
            GpuArch::Blackwell => 256 * 1024,
            GpuArch::Sm(major, _) if *major >= 8 => 164 * 1024,
            _ => 48 * 1024,
        }
    }

    /// Registers per SM
    pub fn registers_per_sm(&self) -> u32 {
        65536
    }

    /// Create from SM version number (e.g., 86 for sm_86)
    pub fn from_sm(sm: u32) -> Self {
        match sm {
            75 => GpuArch::Turing,
            80 | 86 | 87 => GpuArch::Ampere,
            89 => GpuArch::AdaLovelace,
            90 => GpuArch::Blackwell,
            _ => GpuArch::Sm(sm / 10, sm % 10),
        }
    }
}

/// CUDead-BIB compiler configuration
#[derive(Debug, Clone)]
pub struct CudeadConfig {
    /// Target GPU architecture
    pub arch: GpuArch,
    /// Enable UB detection
    pub ub_detection: bool,
    /// Enable optimizations
    pub optimize: bool,
    /// Verbose output
    pub verbose: bool,
    /// Step-by-step mode
    pub step_mode: bool,
    /// Output PTX (readable) instead of binary
    pub emit_ptx: bool,
}

impl Default for CudeadConfig {
    fn default() -> Self {
        Self {
            arch: GpuArch::Ampere, // RTX 3060 default
            ub_detection: true,
            optimize: true,
            verbose: false,
            step_mode: false,
            emit_ptx: false,
        }
    }
}

/// Main CUDead-BIB compiler
pub struct CudeadCompiler {
    config: CudeadConfig,
}

impl CudeadCompiler {
    pub fn new() -> Self {
        Self::with_config(CudeadConfig::default())
    }

    pub fn with_config(config: CudeadConfig) -> Self {
        Self { config }
    }

    /// Compile a .cu file to GPU binary
    pub fn compile(&self, source: &str) -> Result<CudeadOutput, CudeadError> {
        if self.config.verbose {
            println!("[CUDead-BIB] Compiling {} bytes of source", source.len());
        }

        // Phase 1: Parse
        if self.config.step_mode {
            println!("[PHASE 1] PARSER");
        }
        let ast = self.parse(source)?;

        // Phase 2: Generate IR
        if self.config.step_mode {
            println!("[PHASE 2] IR GENERATION");
        }
        let ir = self.generate_ir(&ast)?;

        // Phase 3: UB Detection
        if self.config.ub_detection {
            if self.config.step_mode {
                println!("[PHASE 3] UB DETECTOR GPU");
            }
            self.detect_ub(&ir)?;
        }

        // Phase 4: Optimize
        let optimized_ir = if self.config.optimize {
            if self.config.step_mode {
                println!("[PHASE 4] GPU OPTIMIZER");
            }
            self.optimize(&ir)?
        } else {
            ir
        };

        // Phase 5: Emit PTX
        if self.config.step_mode {
            println!("[PHASE 5] PTX EMITTER");
        }
        let ptx = self.emit_ptx(&optimized_ir)?;

        // Phase 6: Assemble to binary (if not emit_ptx mode)
        let binary = if !self.config.emit_ptx {
            if self.config.step_mode {
                println!("[PHASE 6] PTX ASSEMBLER");
            }
            self.assemble_ptx(&ptx)?
        } else {
            Vec::new()
        };

        Ok(CudeadOutput {
            ptx,
            binary,
            kernels: optimized_ir.kernels.iter().map(|k| k.name.clone()).collect(),
        })
    }

    fn parse(&self, source: &str) -> Result<CudeadAst, CudeadError> {
        let parser = CudeadParser::new();
        parser.parse(source)
    }

    fn generate_ir(&self, ast: &CudeadAst) -> Result<CudeadIR, CudeadError> {
        let ir_gen = CudeadIRGenerator::new();
        ir_gen.generate(ast)
    }

    fn detect_ub(&self, ir: &CudeadIR) -> Result<(), CudeadError> {
        let detector = GpuUBDetector::new();
        detector.analyze(ir)
    }

    fn optimize(&self, ir: &CudeadIR) -> Result<CudeadIR, CudeadError> {
        let optimizer = GpuOptimizer::new(self.config.arch);
        optimizer.optimize(ir)
    }

    fn emit_ptx(&self, ir: &CudeadIR) -> Result<String, CudeadError> {
        let emitter = PtxEmitter::new(self.config.arch);
        emitter.emit(ir)
    }

    fn assemble_ptx(&self, ptx: &str) -> Result<Vec<u8>, CudeadError> {
        // For now, return PTX as bytes (real assembler would generate CUBIN)
        Ok(ptx.as_bytes().to_vec())
    }
}

impl Default for CudeadCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Output from CUDead-BIB compilation
#[derive(Debug, Clone)]
pub struct CudeadOutput {
    /// PTX assembly (readable)
    pub ptx: String,
    /// Binary GPU code
    pub binary: Vec<u8>,
    /// Kernel names found
    pub kernels: Vec<String>,
}

/// CUDead-BIB error types
#[derive(Debug, Clone)]
pub enum CudeadError {
    /// Parse error
    ParseError(String),
    /// IR generation error
    IRError(String),
    /// UB detected
    UBDetected(Vec<GpuUBIssue>),
    /// Optimization error
    OptimizeError(String),
    /// PTX emission error
    PtxError(String),
    /// Assembly error
    AssemblyError(String),
    /// Driver error
    DriverError(String),
}

impl std::fmt::Display for CudeadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CudeadError::ParseError(s) => write!(f, "Parse error: {}", s),
            CudeadError::IRError(s) => write!(f, "IR error: {}", s),
            CudeadError::UBDetected(issues) => {
                writeln!(f, "GPU UB detected ({} issues):", issues.len())?;
                for issue in issues {
                    writeln!(f, "  - {:?}", issue)?;
                }
                Ok(())
            }
            CudeadError::OptimizeError(s) => write!(f, "Optimize error: {}", s),
            CudeadError::PtxError(s) => write!(f, "PTX error: {}", s),
            CudeadError::AssemblyError(s) => write!(f, "Assembly error: {}", s),
            CudeadError::DriverError(s) => write!(f, "Driver error: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cudead_version() {
        assert_eq!(CUDEAD_VERSION, "1.0.0");
    }

    #[test]
    fn test_gpu_arch() {
        let arch = GpuArch::Ampere;
        assert_eq!(arch.sm_version(), (8, 6));
        assert_eq!(arch.warp_size(), 32);
        assert_eq!(arch.max_threads_per_block(), 1024);
    }

    #[test]
    fn test_cudead_config_default() {
        let config = CudeadConfig::default();
        assert!(config.ub_detection);
        assert!(config.optimize);
        assert!(!config.verbose);
    }
}
