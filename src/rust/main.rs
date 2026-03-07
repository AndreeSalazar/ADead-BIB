// ============================================================
// ADead-BIB Compiler CLI v7.0
// C/C++ Native Compiler — Sin GCC, Sin LLVM, Sin Clang
// 100% Self-Sufficient — Sin libc, Sin linker externo
// ============================================================

use adead_bib::backend::gpu::gpu_detect::GPUFeatures;
use adead_bib::backend::gpu::vulkan::VulkanBackend;
use adead_bib::backend::gpu::vulkan_runtime;
use adead_bib::backend::microvm::{self, compile_microvm, MicroOp, MicroVM};
use adead_bib::backend::pe_tiny;
use adead_bib::frontend::c::compile_c_to_program;
use adead_bib::frontend::cpp::compile_cpp_to_program;
use adead_bib::isa::isa_compiler::Target;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        // ============================================================
        // C COMPILER — Primary command
        // ============================================================
        "cc" | "c" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing C source file");
                eprintln!("   Usage: adB cc <file.c> [-o output.exe]");
                std::process::exit(1);
            }
            compile_c_file(&args[2], &args)?;
        }

        // ============================================================
        // C++ COMPILER — Primary command
        // ============================================================
        "cxx" | "c++" | "cpp" | "g++" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing C++ source file");
                eprintln!("   Usage: adB cxx <file.cpp> [-o output.exe]");
                std::process::exit(1);
            }
            compile_cpp_file(&args[2], &args)?;
        }

        // ============================================================
        // BUILD — Auto-detect by extension
        // ============================================================
        "build" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing source file");
                eprintln!("   Usage: adB build <file.c|file.cpp> [-o output.exe]");
                std::process::exit(1);
            }
            let input_file = &args[2];
            let ext = Path::new(input_file)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            match ext {
                "c" | "h" => compile_c_file(input_file, &args)?,
                "cpp" | "cxx" | "cc" | "hpp" | "hxx" => compile_cpp_file(input_file, &args)?,
                _ => {
                    eprintln!("❌ Error: Unknown file extension '.{}'", ext);
                    eprintln!("   Supported: .c, .cpp, .cxx, .cc");
                    std::process::exit(1);
                }
            }
        }

        // ============================================================
        // RUN — Build and execute
        // ============================================================
        "run" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing source file");
                eprintln!("   Usage: adB run <file.c|file.cpp>");
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = get_output_filename(input_file, &args);
            let ext = Path::new(input_file)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            // Build
            match ext {
                "c" | "h" => compile_c_file(input_file, &args)?,
                "cpp" | "cxx" | "cc" | "hpp" | "hxx" => compile_cpp_file(input_file, &args)?,
                _ => {
                    eprintln!("❌ Error: Unknown file extension '.{}'", ext);
                    std::process::exit(1);
                }
            }

            // Run
            println!("🚀 Running {}...\n", input_file);
            let exe_path = if cfg!(target_os = "windows") {
                format!(".\\{}", output_file)
            } else {
                format!("./{}", output_file)
            };
            let status = Command::new(&exe_path).status()?;

            if !status.success() {
                eprintln!("\n⚠️  Program exited with status: {}", status);
            }
        }

        "--test-lexer" => {
            let file = &args[2];
            let source = std::fs::read_to_string(file).unwrap();
            let mut lexer = adead_bib::frontend::c::c_lexer::CLexer::new(&source);
            loop {
                let t = lexer.next_token();
                println!("line: {} token: {:?}", lexer.line, t);
                if t == adead_bib::frontend::c::c_lexer::CToken::Eof { break; }
            }
        }

        // ============================================================
        // NANO/MICRO/TINY — Minimal PE generators (no source needed)
        // ============================================================
        "nano" => {
            let output_file = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "nano.exe".to_string());
            let exit_code: u8 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);

            println!("🔬 Building NANO PE (x64)...");
            println!("   Target: Smallest valid Windows x64 executable");

            match pe_tiny::generate_pe_nano(exit_code, &output_file) {
                Ok(size) => {
                    println!("✅ Nano build complete: {} ({} bytes)", output_file, size);
                    println!("   🏆 Smallest valid Windows x64 PE!");
                }
                Err(e) => {
                    eprintln!("❌ Nano build failed: {}", e);
                    std::process::exit(1);
                }
            }
        }

        "micro" => {
            let output_file = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "micro.exe".to_string());
            let exit_code: u8 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);

            println!("🔬 Building MICRO PE (x86 32-bit)...");
            println!("   Target: Sub-256 byte Windows executable");

            match pe_tiny::generate_pe32_micro(exit_code, &output_file) {
                Ok(size) => {
                    println!("✅ Micro build complete: {} ({} bytes)", output_file, size);
                    if size < 256 {
                        println!("   🏆 SUB-256 BYTES ACHIEVED!");
                    }
                }
                Err(e) => {
                    eprintln!("❌ Micro build failed: {}", e);
                    std::process::exit(1);
                }
            }
        }

        // ============================================================
        // VM — MicroVM bytecode
        // ============================================================
        "vm" => {
            let output_file = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "program.adb".to_string());
            let exit_code: u8 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);

            println!("🔬 Building MicroVM bytecode...");
            println!("   Target: 4-bit instructions (1 byte = 2 ops)");

            let bytecode =
                compile_microvm(&[(MicroOp::Load, exit_code.min(15)), (MicroOp::Exit, 0)]);

            match microvm::save_bytecode(&bytecode, &output_file) {
                Ok(size) => {
                    println!("✅ MicroVM bytecode: {} ({} bytes)", output_file, size);
                    let mut vm = MicroVM::new(&bytecode);
                    let result = vm.run();
                    println!("   ▶️  Execution result: {}", result);
                }
                Err(e) => {
                    eprintln!("❌ MicroVM build failed: {}", e);
                    std::process::exit(1);
                }
            }
        }

        // ============================================================
        // GPU COMMANDS
        // ============================================================
        "gpu" => {
            let gpu = GPUFeatures::detect();
            gpu.print_summary();

            if gpu.available {
                println!();
                let mut backend = VulkanBackend::new();
                let spirv = backend.generate_optimized_shader(&gpu);
                let output_path = args
                    .get(2)
                    .cloned()
                    .unwrap_or_else(|| "builds/matmul.spv".to_string());

                match backend.save_spirv(&spirv, &output_path) {
                    Ok(size) => {
                        println!(
                            "✅ SPIR-V Shader generated: {} ({} bytes)",
                            output_path, size
                        );
                        println!("   Optimized for: {}", gpu.device_name);
                    }
                    Err(e) => eprintln!("❌ Failed to save shader: {}", e),
                }
            }
        }

        "spirv" => {
            let op = args.get(2).map(|s| s.as_str()).unwrap_or("matmul");
            let size: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1024);

            println!("🔬 SPIR-V Compute Shader Generator");
            println!("   Operation: {}", op);
            println!("   Size: {}x{}", size, size);
            println!();

            let mut backend = VulkanBackend::new();
            let spirv = match op {
                "matmul" => {
                    backend.set_workgroup_size(16, 16, 1);
                    backend.generate_matmul_shader(size, size, size)
                }
                _ => backend.generate_matmul_shader(size, size, size),
            };

            let output_path = format!("builds/{}_{}.spv", op, size);
            match backend.save_spirv(&spirv, &output_path) {
                Ok(sz) => {
                    println!("✅ SPIR-V generated: {} ({} bytes)", output_path, sz);
                    println!("   Workgroup: {:?}", backend.workgroup_size);
                }
                Err(e) => eprintln!("❌ Failed: {}", e),
            }
        }

        "vulkan" | "vk" => {
            println!("🔥 VULKAN RUNTIME - GPU Compute");
            println!();

            match unsafe { vulkan_runtime::VulkanRuntime::new() } {
                Ok(runtime) => {
                    runtime.print_device_info();
                    println!();
                    println!("✅ Vulkan runtime initialized successfully!");
                    let props = &runtime.device_props;
                    println!("🎯 GPU Capabilities:");
                    println!("   Max workgroup: {:?}", props.max_compute_workgroup_size);
                    println!(
                        "   Max invocations: {}",
                        props.max_compute_workgroup_invocations
                    );
                    println!(
                        "   Shared memory: {} KB",
                        props.max_compute_shared_memory / 1024
                    );
                }
                Err(e) => {
                    eprintln!("❌ Failed to initialize Vulkan: {}", e);
                    eprintln!("   Make sure Vulkan drivers are installed.");
                }
            }
        }

        "cuda" => {
            use adead_bib::backend::gpu::cuda;

            let op = args.get(2).map(|s| s.as_str()).unwrap_or("vectoradd");
            let size: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1024);

            println!("🔥 ADead-BIB + CUDA Code Generator");
            println!("   Operation: {}", op);
            println!("   Size: {}", size);
            println!();

            let code = match op {
                "matmul" => cuda::generate_matmul_benchmark(size),
                "benchmark" | "bench" => cuda::generate_full_benchmark(),
                _ => cuda::generate_adead_cuda_test(size),
            };

            let output_path = format!("CUDA/ADead_Generated/adead_{}.cu", op);
            fs::create_dir_all("CUDA/ADead_Generated").ok();
            match fs::write(&output_path, &code) {
                Ok(_) => {
                    println!("✅ CUDA code generated: {}", output_path);
                    println!("   Lines: {}", code.lines().count());
                    println!();
                    println!("📋 To compile: nvcc {} -o {}.exe", output_path, op);
                }
                Err(e) => eprintln!("❌ Failed to write CUDA code: {}", e),
            }
        }

        "unified" | "uni" => {
            use adead_bib::backend::gpu::unified_pipeline::{
                MathOperation, PipelineMode, UnifiedPipeline,
            };

            let op = args.get(2).map(|s| s.as_str()).unwrap_or("vectoradd");
            let size: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1000000);

            println!("🔥 ADead-BIB Unified Pipeline");
            println!("   Automatic CPU↔GPU decision");
            println!();

            let mode = if args.iter().any(|a| a == "--force-gpu") {
                PipelineMode::ForceGpu
            } else if args.iter().any(|a| a == "--cpu") {
                PipelineMode::CpuOnly
            } else {
                PipelineMode::Hybrid
            };

            let mut pipeline = UnifiedPipeline::with_mode(mode);

            let math_op = match op {
                "matmul" => {
                    let n = (size as f64).sqrt() as usize;
                    println!("   Operation: MatMul {}x{}", n, n);
                    MathOperation::MatMul { m: n, n, k: n }
                }
                "saxpy" => {
                    println!("   Operation: SAXPY ({} elements)", size);
                    MathOperation::Saxpy { size, alpha: 2.5 }
                }
                "reduce" => {
                    println!("   Operation: Reduction ({} elements)", size);
                    MathOperation::Reduction { size }
                }
                _ => {
                    println!("   Operation: VectorAdd ({} elements)", size);
                    MathOperation::VectorAdd { size }
                }
            };

            let result = pipeline.compile_math_op(math_op);
            println!();
            println!("📊 Compilation Result:");
            println!("   Target:  {:?}", result.target);
            println!("   Format:  {:?}", result.format);
            println!("   Size:    {} bytes", result.binary.len());
            println!();
            pipeline.print_summary();
        }

        // ============================================================
        // HELP / VERSION
        // ============================================================
        "help" | "-h" | "--help" => {
            print_usage(&args[0]);
        }

        "version" | "-v" | "--version" => {
            println!("ADead-BIB v7.0.0 — C/C++ Native Compiler");
            println!("Sin GCC, Sin LLVM, Sin Clang — 100% ADead-BIB");
            println!("Sin libc externa, Sin linker — Totalmente autosuficiente");
            println!();
            if let Ok(exe) = env::current_exe() {
                println!("Executable: {}", exe.display());
                if let Some(dir) = exe.parent() {
                    println!("\n  To add to PATH (Windows PowerShell):");
                    println!("  $env:Path += \";{}\"  ", dir.display());
                    println!("\n  To add permanently (run as Admin):");
                    println!("  [Environment]::SetEnvironmentVariable('Path', $env:Path + ';{}', 'User')", dir.display());
                }
            }
        }

        // ============================================================
        // AUTO-DETECT BY EXTENSION
        // ============================================================
        _ => {
            let input_file = command;
            let ext = Path::new(input_file)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            match ext {
                "c" | "h" => compile_c_file(input_file, &args)?,
                "cpp" | "cxx" | "cc" | "hpp" | "hxx" => compile_cpp_file(input_file, &args)?,
                _ => {
                    eprintln!("❌ Unknown command or file: {}", command);
                    eprintln!("   Use 'adB help' for usage information.");
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}

// ============================================================
// C COMPILATION
// ============================================================
fn compile_c_file(input_file: &str, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let output_file = get_output_filename(input_file, args);

    // Check for --flat flag (flat binary for bootloaders/OS)
    let is_flat = args.iter().any(|a| a == "--flat");
    let is_flat64 = args.iter().any(|a| a == "--flat64");
    let is_flat16 = args.iter().any(|a| a == "--flat16");
    let is_any_flat = is_flat || is_flat64 || is_flat16;
    let org_address = parse_org_address(args);
    let fixed_size = parse_fixed_size(args);

    if is_any_flat {
        let mode_str = if is_flat64 {
            "64-bit Long Mode"
        } else if is_flat16 {
            "16-bit Real Mode"
        } else {
            "64-bit Long Mode (default)"
        };
        println!("🔨 ADead-BIB C Compiler (Flat Binary Mode)");
        println!("   Source: {}", input_file);
        println!("   Target: {}", output_file);
        println!("   Mode:   {}", mode_str);
        println!("   Origin: 0x{:X}", org_address);
        if fixed_size > 0 {
            println!("   Size:   {} bytes (fixed)", fixed_size);
        }
    } else {
        println!("🔨 ADead-BIB C Compiler");
        println!("   Source: {}", input_file);
        println!("   Target: {}", output_file);
    }

    // 1. Read source
    let source = fs::read_to_string(input_file)
        .map_err(|e| format!("Cannot read '{}': {}", input_file, e))?;

    // 2. Parse C99
    println!("   Step 1: Parsing C99...");
    let program = compile_c_to_program(&source).map_err(|e| format!("C parse error: {}", e))?;

    println!(
        "   Step 2: {} functions, {} structs found",
        program.functions.len(),
        program.structs.len()
    );

    let warn_ub = args.iter().any(|a| a == "--warn-ub");
    let mut ub_detector = adead_bib::UBDetector::new().with_file(input_file.to_string());
    if warn_ub {
        ub_detector = ub_detector.with_warn_mode();
        println!("   ⚠️  UB_Detector: warning mode (avisa y continua)");
    } else {
        println!("   🛡️  UB_Detector: strict mode (se detiene en errores)");
    }

    ub_detector.analyze(&program);
    ub_detector.print_reports();
    if !warn_ub && ub_detector.has_errors() {
        eprintln!("❌ Error: Undefined Behavior detectado en modo estricto. Operación cancelada.");
        std::process::exit(1);
    }

    // 3. Compile to native code
    let target = if is_any_flat {
        Target::Raw
    } else {
        determine_target(args)
    };

    let mode_desc = if is_flat64 || is_flat {
        "x86-64 (64-bit long mode)"
    } else if is_flat16 {
        "x86 (16-bit real mode)"
    } else {
        "x86-64"
    };
    println!("   Step 3: Compiling to native {}...", mode_desc);

    // Create compiler with appropriate CPU mode
    let mut compiler = if is_flat16 {
        adead_bib::isa::isa_compiler::IsaCompiler::new_real16()
    } else if is_flat64 || is_flat {
        // 64-bit long mode for flat binaries (bare metal kernel)
        adead_bib::isa::isa_compiler::IsaCompiler::new_long64(Target::Raw)
    } else {
        adead_bib::isa::isa_compiler::IsaCompiler::new(target)
    };

    let (opcodes, data, iat_offsets, string_offsets) = compiler.compile(&program);

    // 4. Generate binary
    println!("   Step 4: Generating binary...");

    if is_fastos_target(args) {
        use adead_bib::output::po::PoOutput;
        let gen = PoOutput::new();
        match gen.generate(&opcodes, &data, &output_file) {
            Ok(s) => println!(
                "✅ FastOS binary: {} ({} bytes, v5.0 pipeline)",
                output_file, s
            ),
            Err(e) => {
                eprintln!("❌ FastOS generation failed: {}", e);
                std::process::exit(1);
            }
        }
    } else if is_any_flat {
        // Flat binary mode - no PE/ELF headers
        use adead_bib::backend::flat_binary::FlatBinaryGenerator;
        let mut gen = FlatBinaryGenerator::new(org_address);
        if fixed_size > 0 {
            gen.set_fixed_size(fixed_size);
        }
        let binary = gen.generate(&opcodes, &data);
        fs::write(&output_file, &binary)?;
        println!(
            "✅ Flat binary: {} ({} bytes, org=0x{:X})",
            output_file,
            binary.len(),
            org_address
        );
    } else {
        match target {
            Target::Windows => {
                adead_bib::backend::pe::generate_pe_with_offsets(
                    &opcodes,
                    &data,
                    &output_file,
                    &iat_offsets,
                    &string_offsets,
                )?;
            }
            Target::Linux => {
                adead_bib::backend::elf::generate_elf(&opcodes, &data, &output_file)?;
            }
            Target::Raw => {
                fs::write(&output_file, &opcodes)?;
            }
        }

        if let Ok(meta) = fs::metadata(&output_file) {
            println!(
                "✅ C compilation complete: {} ({} bytes)",
                output_file,
                meta.len()
            );
        } else {
            println!("✅ C compilation complete: {}", output_file);
        }
    }

    println!("   🏆 Sin GCC, sin LLVM, sin Clang — 100% ADead-BIB");

    Ok(())
}

/// Parse --org=0xNNNN argument
fn parse_org_address(args: &[String]) -> u64 {
    for arg in args {
        if arg.starts_with("--org=") {
            let addr_str = arg.trim_start_matches("--org=");
            if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
                return u64::from_str_radix(&addr_str[2..], 16).unwrap_or(0);
            } else {
                return addr_str.parse().unwrap_or(0);
            }
        }
    }
    0 // Default origin
}

/// Parse --size=NNNN argument for fixed size binaries
fn parse_fixed_size(args: &[String]) -> usize {
    for arg in args {
        if arg.starts_with("--size=") {
            let size_str = arg.trim_start_matches("--size=");
            if size_str.starts_with("0x") || size_str.starts_with("0X") {
                return usize::from_str_radix(&size_str[2..], 16).unwrap_or(0);
            } else {
                return size_str.parse().unwrap_or(0);
            }
        }
    }
    0 // No fixed size
}

// ============================================================
// C++ COMPILATION
// ============================================================
fn compile_cpp_file(input_file: &str, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let output_file = get_output_filename(input_file, args);

    println!("🔨 ADead-BIB C++ Compiler");
    println!("   Source: {}", input_file);
    println!("   Target: {}", output_file);

    // 1. Read source
    let source = fs::read_to_string(input_file)
        .map_err(|e| format!("Cannot read '{}': {}", input_file, e))?;

    // 2. Parse C++
    println!("   Step 1: Parsing C++...");
    let program = compile_cpp_to_program(&source).map_err(|e| format!("C++ parse error: {}", e))?;

    println!(
        "   Step 2: {} functions, {} structs, {} classes found",
        program.functions.len(),
        program.structs.len(),
        program.classes.len()
    );

    let warn_ub = args.iter().any(|a| a == "--warn-ub");
    let mut ub_detector = adead_bib::UBDetector::new().with_file(input_file.to_string());
    if warn_ub {
        ub_detector = ub_detector.with_warn_mode();
        println!("   ⚠️  UB_Detector: warning mode (avisa y continua)");
    } else {
        println!("   🛡️  UB_Detector: strict mode (se detiene en errores)");
    }

    ub_detector.analyze(&program);
    ub_detector.print_reports();
    if !warn_ub && ub_detector.has_errors() {
        eprintln!("❌ Error: Undefined Behavior detectado en modo estricto. Operación cancelada.");
        std::process::exit(1);
    }

    // 3. Compile to native x86-64
    println!("   Step 3: Compiling to native x86-64...");
    let target = determine_target(args);
    let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(target);
    let (opcodes, data, iat_offsets, string_offsets) = compiler.compile(&program);

    // 4. Generate binary
    println!("   Step 4: Generating binary...");
    if is_fastos_target(args) {
        use adead_bib::output::po::PoOutput;
        let gen = PoOutput::new();
        match gen.generate(&opcodes, &data, &output_file) {
            Ok(s) => println!(
                "✅ FastOS binary: {} ({} bytes, v5.0 pipeline)",
                output_file, s
            ),
            Err(e) => {
                eprintln!("❌ FastOS generation failed: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match target {
            Target::Windows => {
                adead_bib::backend::pe::generate_pe_with_offsets(
                    &opcodes,
                    &data,
                    &output_file,
                    &iat_offsets,
                    &string_offsets,
                )?;
            }
            Target::Linux => {
                adead_bib::backend::elf::generate_elf(&opcodes, &data, &output_file)?;
            }
            Target::Raw => {
                fs::write(&output_file, &opcodes)?;
            }
        }
    }

    if let Ok(meta) = fs::metadata(&output_file) {
        println!(
            "✅ C++ compilation complete: {} ({} bytes)",
            output_file,
            meta.len()
        );
    } else {
        println!("✅ C++ compilation complete: {}", output_file);
    }
    println!("   🏆 Sin GCC, sin LLVM, sin Clang — 100% ADead-BIB C++");

    Ok(())
}

// ============================================================
// UTILITIES
// ============================================================
fn is_fastos_target(args: &[String]) -> bool {
    for i in 0..args.len() {
        if args[i] == "--target" && i + 1 < args.len() {
            let t = &args[i + 1];
            if t == "fastos" || t == "po" {
                return true;
            }
        }
    }
    false
}

fn determine_target(args: &[String]) -> Target {
    for i in 0..args.len() {
        if args[i] == "--target" && i + 1 < args.len() {
            let t = &args[i + 1];
            if t == "fastos" || t == "po" || t == "raw" {
                return Target::Raw;
            }
            if t == "windows" || t == "pe" || t == "win" {
                return Target::Windows;
            }
            if t == "linux" || t == "elf" {
                return Target::Linux;
            }
        }
    }
    if cfg!(target_os = "windows") {
        Target::Windows
    } else if cfg!(target_os = "linux") {
        Target::Linux
    } else {
        Target::Raw
    }
}

fn get_output_filename(input: &str, args: &[String]) -> String {
    // Check if -o provided
    for i in 0..args.len() {
        if args[i] == "-o" && i + 1 < args.len() {
            return args[i + 1].clone();
        }
    }

    let ext = if is_fastos_target(args) {
        ".po"
    } else if determine_target(args) == Target::Linux {
        ""
    } else {
        ".exe"
    };

    // Default: input.exe
    Path::new(input)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        + ext
}

fn print_usage(_program: &str) {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         🔥 ADead-BIB v7.0.0 — C/C++ Compiler 🔥             ║");
    println!("║    Sin GCC, Sin LLVM, Sin Clang — 100% Self-Sufficient       ║");
    println!("║    Sin libc, Sin linker — header_main.h = TODO               ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("📋 COMPILAR C/C++:");
    println!("   adB cc <file.c> [-o output]     Compile C99/C11");
    println!("   adB cxx <file.cpp> [-o output]  Compile C++11/14/17/20");
    println!("     [--target fastos|windows|linux]");
    println!("     [--warn-ub] (Warning only, don't stop on UB)");
    println!("   adB build <file> [-o output]    Auto-detect by extension");
    println!("   adB run <file>                  Build and execute");
    println!("   adB <file.c|file.cpp>           Direct compilation");
    println!();
    println!("🚀 EXAMPLES:");
    println!("   adB cc hello.c                  Compile hello.c → hello.exe");
    println!("   adB cxx main.cpp -o app.exe     Compile main.cpp → app.exe");
    println!("   adB run test.c                  Compile and run test.c");
    println!("   adB hello.cpp                   Direct: hello.cpp → hello.exe");
    println!();
    println!("⚡ MINIMAL BINARIES:");
    println!("   adB nano [output] [exit_code]   Smallest valid x64 PE (~1KB)");
    println!("   adB micro [output] [exit_code]  Sub-256 byte x86 PE");
    println!("   adB vm [output] [exit_code]     MicroVM bytecode");
    println!();
    println!("🎮 GPU (Vulkan/CUDA):");
    println!("   adB gpu                         Detect GPU, generate shader");
    println!("   adB spirv [op] [size]           Generate SPIR-V compute shader");
    println!("   adB vulkan                      Initialize Vulkan runtime");
    println!("   adB cuda [op] [size]            Generate CUDA code (.cu)");
    println!("   adB unified [op] [size]         Auto CPU↔GPU decision");
    println!();
    println!("📝 SUPPORTED FEATURES:");
    println!("   C:   C99/C11, structs, pointers, arrays, printf, malloc");
    println!("   C++: C++98/11/14/17, classes, templates, namespaces, STL");
    println!("   header_main.h: Un solo #include — todo disponible");
    println!();
    println!("🎯 OUTPUT FORMATS:");
    println!("   Windows: PE executable (.exe)");
    println!("   Linux:   ELF executable");
    println!("   FastOS:  Po executable (.po)");
    println!();
    println!("🔧 PATH SETUP:");
    println!("   adB --version      Show path and setup instructions");
    println!();
}
