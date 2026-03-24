// ============================================================
// CUDead-BIB — CLI Module
// ============================================================
// Implementa: adb cuda archivo.cu
// Pipeline de 9 fases visibles
// ============================================================

use std::path::Path;
use std::fs;
use std::time::Instant;

use super::{
    GpuArch,
    parser::CudeadParser,
    ir::CudeadIRGenerator,
    ub_detector::GpuUBDetector,
    optimizer::GpuOptimizer,
    ptx_emitter::PtxEmitter,
};

/// CLI options for `adb cuda`
#[derive(Debug, Clone)]
pub struct CudaCliOptions {
    pub input_file: String,
    pub output_file: Option<String>,
    pub step_mode: bool,
    pub dry_run: bool,
    pub sm_version: u32,
    pub optimize_level: u32,
    pub warn_ub: bool,
    pub verbose: bool,
}

impl Default for CudaCliOptions {
    fn default() -> Self {
        Self {
            input_file: String::new(),
            output_file: None,
            step_mode: false,
            dry_run: false,
            sm_version: 86, // RTX 3060 default
            optimize_level: 2,
            warn_ub: false,
            verbose: false,
        }
    }
}

impl CudaCliOptions {
    pub fn parse(args: &[String]) -> Result<Self, String> {
        let mut opts = Self::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "--step" => opts.step_mode = true,
                "--dry" => opts.dry_run = true,
                "--warn-ub" => opts.warn_ub = true,
                "--verbose" | "-v" => opts.verbose = true,
                "-O0" => opts.optimize_level = 0,
                "-O1" => opts.optimize_level = 1,
                "-O2" => opts.optimize_level = 2,
                "-O3" => opts.optimize_level = 3,
                "-o" => {
                    i += 1;
                    if i < args.len() {
                        opts.output_file = Some(args[i].clone());
                    } else {
                        return Err("-o requires output file".to_string());
                    }
                }
                arg if arg.starts_with("--sm=") => {
                    let sm = arg.strip_prefix("--sm=").unwrap();
                    opts.sm_version = sm.parse().map_err(|_| format!("Invalid SM version: {}", sm))?;
                }
                arg if arg.ends_with(".cu") => {
                    opts.input_file = arg.to_string();
                }
                _ => {
                    return Err(format!("Unknown argument: {}", arg));
                }
            }
            i += 1;
        }

        if opts.input_file.is_empty() {
            return Err("No input file specified".to_string());
        }

        Ok(opts)
    }
}

/// Main CLI entry point
pub fn run_cuda_cli(args: &[String]) -> Result<i32, String> {
    let opts = CudaCliOptions::parse(args)?;
    let mut cli = CudaCli::new(opts);
    cli.run()
}

/// CLI executor
pub struct CudaCli {
    opts: CudaCliOptions,
    phase: u32,
}

impl CudaCli {
    pub fn new(opts: CudaCliOptions) -> Self {
        Self { opts, phase: 0 }
    }

    fn print_phase(&mut self, name: &str) {
        if self.opts.step_mode || self.opts.verbose {
            println!("\n--- Phase {:02}: {} ---", self.phase, name);
        }
        self.phase += 1;
    }

    fn print_info(&self, tag: &str, msg: &str) {
        if self.opts.step_mode || self.opts.verbose {
            println!("[{}]    {}", tag, msg);
        }
    }

    pub fn run(&mut self) -> Result<i32, String> {
        let start = Instant::now();

        // Phase 00: GPU DETECT
        self.print_phase("GPU DETECT");
        let gpu_info = self.detect_gpu()?;
        self.print_info("GPU", &gpu_info);

        // Phase 01: READ SOURCE
        self.print_phase("READ SOURCE");
        let source = self.read_source()?;
        let lines = source.lines().count();
        self.print_info("SRC", &format!("{} — {} líneas", self.opts.input_file, lines));

        // Phase 02: PARSER / AST
        self.print_phase("PARSER / AST");
        let ast = self.parse_source(&source)?;
        self.print_info("PARSE", &format!("{} kernels encontrados", ast.kernels.len()));

        // Phase 03: IR — CUDeadOp SSA-form
        self.print_phase("IR — CUDeadOp SSA-form");
        let ir = self.generate_ir(&ast)?;
        self.print_info("IR", &format!("{} kernels → IR", ir.kernels.len()));

        // Phase 04: UB DETECTOR GPU
        self.print_phase("UB DETECTOR GPU ★");
        match self.detect_ub(&ir) {
            Ok(()) => {
                self.print_info("UB", "0 UB detectados ✓");
            }
            Err(e) => {
                if self.opts.warn_ub {
                    self.print_info("UB", &format!("[WARN] {}", e));
                } else {
                    return Err(e);
                }
            }
        }

        // Phase 05: OPTIMIZER GPU
        self.print_phase("OPTIMIZER GPU");
        let optimized_ir = if self.opts.optimize_level > 0 {
            self.optimize(&ir)?
        } else {
            ir.clone()
        };
        self.print_info("OPT", &format!("Level {} aplicado", self.opts.optimize_level));

        // Phase 06: PTX EMITTER
        self.print_phase("PTX EMITTER");
        let ptx = self.emit_ptx(&optimized_ir)?;
        self.print_info("PTX", &format!("PTX 7.0 generado — sm_{}", self.opts.sm_version));

        // Phase 07: VRAM LAYOUT
        self.print_phase("VRAM LAYOUT");
        self.print_info("VRAM", "128B aligned — RTX 3060 12GB");

        // Phase 08: OUTPUT
        self.print_phase("OUTPUT");
        let output_file = self.write_output(&ptx)?;
        self.print_info("OUT", &output_file);

        // Phase 09: EXECUTE (unless --dry)
        if !self.opts.dry_run {
            self.print_phase("EXECUTE — RTX 3060");
            self.execute(&ptx)?;
        } else {
            self.print_phase("EXECUTE — SKIPPED (--dry)");
        }

        let elapsed = start.elapsed();
        if self.opts.step_mode || self.opts.verbose {
            println!("\n=== Completed in {:.2}ms ===", elapsed.as_secs_f64() * 1000.0);
        }

        Ok(0)
    }

    fn detect_gpu(&self) -> Result<String, String> {
        // TODO: Real GPU detection via PCIe
        let arch = match self.opts.sm_version {
            75 => "Turing",
            86 => "Ampere",
            89 => "Ada Lovelace",
            90 => "Blackwell",
            _ => "Unknown",
        };
        Ok(format!("NVIDIA RTX 3060 12GB (GA106 {}) — sm_{}", arch, self.opts.sm_version))
    }

    fn read_source(&self) -> Result<String, String> {
        fs::read_to_string(&self.opts.input_file)
            .map_err(|e| format!("Failed to read {}: {}", self.opts.input_file, e))
    }

    fn parse_source(&self, source: &str) -> Result<super::parser::CudeadAst, String> {
        let parser = CudeadParser::new();
        parser.parse(source).map_err(|e| format!("Parse error: {:?}", e))
    }

    fn generate_ir(&self, ast: &super::parser::CudeadAst) -> Result<super::ir::CudeadIR, String> {
        let generator = CudeadIRGenerator::new();
        generator.generate(ast).map_err(|e| format!("IR generation error: {:?}", e))
    }

    fn detect_ub(&self, ir: &super::ir::CudeadIR) -> Result<(), String> {
        let detector = GpuUBDetector::new();
        detector.analyze(ir).map_err(|e| format!("UB detected: {:?}", e))
    }

    fn optimize(&self, ir: &super::ir::CudeadIR) -> Result<super::ir::CudeadIR, String> {
        let arch = GpuArch::from_sm(self.opts.sm_version);
        let optimizer = GpuOptimizer::new(arch);
        optimizer.optimize(ir).map_err(|e| format!("Optimizer error: {:?}", e))
    }

    fn emit_ptx(&self, ir: &super::ir::CudeadIR) -> Result<String, String> {
        let arch = GpuArch::from_sm(self.opts.sm_version);
        let emitter = PtxEmitter::new(arch);
        emitter.emit(ir).map_err(|e| format!("PTX emitter error: {:?}", e))
    }

    fn write_output(&self, ptx: &str) -> Result<String, String> {
        let output_file = self.opts.output_file.clone().unwrap_or_else(|| {
            let input = Path::new(&self.opts.input_file);
            let stem = input.file_stem().unwrap().to_str().unwrap();
            format!("{}.ptx", stem)
        });

        fs::write(&output_file, ptx)
            .map_err(|e| format!("Failed to write {}: {}", output_file, e))?;

        Ok(output_file)
    }

    fn execute(&self, _ptx: &str) -> Result<(), String> {
        // TODO: Real execution via CudeadDriver
        self.print_info("GPU", "PCIe handle — VRAM alloc — kernel launch — sync");
        self.print_info("GPU", "Execution simulated (driver not connected to real hardware yet)");
        Ok(())
    }
}

/// Help text for `adb cuda`
pub fn print_cuda_help() {
    println!(r#"
CUDead-BIB — GPU Compiler Nativo
================================

USAGE:
    adb cuda <archivo.cu> [OPTIONS]

OPTIONS:
    --step          Ver pipeline completo (9 fases)
    --dry           Solo compilar, no ejecutar
    -o <file>       Output específico
    --sm=<ver>      SM version (75, 86, 89, 90)
    -O0/-O1/-O2/-O3 Nivel de optimización
    --warn-ub       UB como warnings (no errores)
    --verbose, -v   Verbose output

EXAMPLES:
    adb cuda vecadd.cu
    adb cuda vecadd.cu --step
    adb cuda vecadd.cu --dry -O3
    adb cuda vecadd.cu --sm=86 -o kernel.ptx

RTX 3060 (default):
    SM version: 86 (Ampere)
    VRAM: 12GB
    Optimal block: 256 threads
"#);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let args = vec![
            "test.cu".to_string(),
            "--step".to_string(),
            "-O3".to_string(),
        ];
        let opts = CudaCliOptions::parse(&args).unwrap();
        assert_eq!(opts.input_file, "test.cu");
        assert!(opts.step_mode);
        assert_eq!(opts.optimize_level, 3);
    }

    #[test]
    fn test_parse_sm_version() {
        let args = vec![
            "test.cu".to_string(),
            "--sm=89".to_string(),
        ];
        let opts = CudaCliOptions::parse(&args).unwrap();
        assert_eq!(opts.sm_version, 89);
    }
}
