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
use std::path::{Path, PathBuf};
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
                eprintln!("   Usage: adb cc <file.c> [-o output.exe]");
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
                eprintln!("   Usage: adb cxx <file.cpp> [-o output.exe]");
                std::process::exit(1);
            }
            compile_cpp_file(&args[2], &args)?;
        }

        // ============================================================
        // BUILD — Auto-detect by extension or adb.toml project
        // ============================================================
        "build" => {
            if args.len() < 3 {
                // No file argument — try adb.toml project
                if let Some(proj) = load_adb_toml(".") {
                    build_project(&proj, &args)?;
                } else {
                    eprintln!("❌ Error: No source file and no adb.toml found");
                    eprintln!("   Usage: adb build <file.c|file.cpp>");
                    eprintln!("   Or run from a project created with: adb create <name>");
                    std::process::exit(1);
                }
            } else {
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
        }

        // ============================================================
        // RUN — Build and execute
        // ============================================================
        "run" => {
            if args.len() < 3 {
                // No file argument — try adb.toml project
                if let Some(proj) = load_adb_toml(".") {
                    let output_file = build_project(&proj, &args)?;
                    println!("🚀 Running {}...\n", proj.name);
                    let exe_path = if cfg!(target_os = "windows") {
                        format!(".\\{}", output_file)
                    } else {
                        format!("./{}", output_file)
                    };
                    let status = Command::new(&exe_path).status()?;
                    if !status.success() {
                        eprintln!("\n⚠️  Program exited with status: {}", status);
                    }
                } else {
                    eprintln!("❌ Error: No source file and no adb.toml found");
                    eprintln!("   Usage: adb run <file.c|file.cpp>");
                    eprintln!("   Or run from a project created with: adb create <name>");
                    std::process::exit(1);
                }
            } else {
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
        // STEP — Step-by-step compilation visualization
        // ============================================================
        "step" => {
            if args.len() < 3 {
                eprintln!("Usage: adb step <file.c|file.cpp>");
                std::process::exit(1);
            }
            let input_file = &args[2];
            let ext = Path::new(input_file)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            match ext {
                "c" | "h" => step_compile_c(input_file)?,
                "cpp" | "cxx" | "cc" => step_compile_cpp(input_file)?,
                _ => {
                    eprintln!("Unsupported extension '.{}' for step mode", ext);
                    std::process::exit(1);
                }
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
        // CREATE — New project (like cargo new)
        // ============================================================
        "create" | "new" | "init" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing project name");
                eprintln!("   Usage: adb create <name> [--cpp|--c]");
                std::process::exit(1);
            }
            let name = &args[2];
            let is_cpp = args.iter().any(|a| a == "--cpp" || a == "--c++" || a == "--cxx");
            create_project(name, is_cpp)?;
        }

        // ============================================================
        // INSTALL — Copy headers to ~/.adead/include/
        // ============================================================
        "install" => {
            install_global_headers()?;
        }

        // ============================================================
        // INCLUDE — Show global include path
        // ============================================================
        "include" => {
            let include_dir = get_global_include_dir();
            println!("📂 ADead-BIB global include directory:");
            println!("   {}", include_dir.display());
            if include_dir.exists() {
                let count = fs::read_dir(&include_dir).map(|d| d.count()).unwrap_or(0);
                println!("   ✅ {} headers installed", count);
            } else {
                println!("   ⚠️  Not installed yet. Run: adb install");
            }
        }

        // ============================================================
        // HELP / VERSION
        // ============================================================
        "help" | "-h" | "--help" => {
            print_usage(&args[0]);
        }

        "version" | "-v" | "--version" => {
            println!("ADead-BIB v7.0.0 💀🦈 🇵🇪 — C/C++ Native Compiler");
            println!("Sin GCC, Sin LLVM, Sin Clang — 100% ADead-BIB");
            println!("Sin libc externa, Sin linker — Totalmente autosuficiente");
            println!();
            if let Ok(exe) = env::current_exe() {
                println!("Executable: {}", exe.display());
                if let Some(dir) = exe.parent() {
                    println!();
                    if cfg!(target_os = "windows") {
                        println!("  Agrega adb al PATH (Windows PowerShell):");
                        println!("  $env:Path += \";{}\"  ", dir.display());
                        println!();
                        println!("  Para agregar permanente (Admin):");
                        println!("  [Environment]::SetEnvironmentVariable('Path', $env:Path + ';{}', 'User')", dir.display());
                    } else {
                        println!("  Agrega adb al PATH (Linux/macOS):");
                        println!("  export PATH=\"$PATH:{}\"", dir.display());
                    }
                }
            }
            println!();
            println!("  Headers globales: {}", get_global_include_dir().display());
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
                    eprintln!("   Use 'adb help' for usage information.");
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
    print_path_hint();

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
    print_path_hint();

    Ok(())
}

// ============================================================
// UTILITIES
// ============================================================
fn print_path_hint() {
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            println!();
            if cfg!(target_os = "windows") {
                println!("   Para agregar adb al PATH en Windows:");
                println!("   $env:PATH += \";{}\"", dir.display());
            } else {
                println!("   Para agregar adb al PATH:");
                println!("   export PATH=\"$PATH:{}\"", dir.display());
            }
        }
    }
}

/// Returns the global include directory: ~/.adead/include/
fn get_global_include_dir() -> PathBuf {
    if let Some(home) = env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
    {
        PathBuf::from(home).join(".adead").join("include")
    } else {
        PathBuf::from(".adead").join("include")
    }
}

/// Install global headers to ~/.adead/include/
fn install_global_headers() -> Result<(), Box<dyn std::error::Error>> {
    let include_dir = get_global_include_dir();
    fs::create_dir_all(&include_dir)?;

    println!("📦 ADead-BIB — Instalando headers globales...");
    println!("   Destino: {}", include_dir.display());
    println!();

    let mut count = 0;

    // Write header_main.h
    let header_main_content = adead_bib::frontend::c::c_stdlib::get_header("header_main.h")
        .unwrap_or("// header_main.h\n");
    fs::write(include_dir.join("header_main.h"), header_main_content)?;
    println!("   ✅ header_main.h");
    count += 1;

    // Write all fastos_*.h headers
    let fastos_headers = [
        "fastos_stdio.h", "fastos_stdlib.h", "fastos_string.h",
        "fastos_math.h", "fastos_time.h", "fastos_assert.h",
        "fastos_errno.h", "fastos_limits.h", "fastos_types.h",
    ];
    for name in &fastos_headers {
        if let Some(content) = adead_bib::frontend::c::c_stdlib::get_header(name) {
            fs::write(include_dir.join(name), content)?;
            println!("   ✅ {}", name);
            count += 1;
        }
    }

    // Write standard C headers
    let std_headers = [
        "stdio.h", "stdlib.h", "string.h", "math.h", "time.h",
        "stdint.h", "stddef.h", "stdbool.h", "stdarg.h",
        "limits.h", "float.h", "errno.h", "assert.h",
        "signal.h", "ctype.h", "locale.h", "setjmp.h",
    ];
    for name in &std_headers {
        if let Some(content) = adead_bib::frontend::c::c_stdlib::get_header(name) {
            fs::write(include_dir.join(name), content)?;
            println!("   ✅ {}", name);
            count += 1;
        }
    }

    println!();
    println!("✅ {} headers instalados en {}", count, include_dir.display());
    println!();
    println!("   Ahora puedes usar desde cualquier carpeta:");
    println!("   #include <header_main.h>");
    println!();
    println!("   También puedes agregar tus propios headers en:");
    println!("   {}", include_dir.display());

    Ok(())
}

// ============================================================
// PROJECT SYSTEM (adb create / adb.toml)
// ============================================================

#[allow(dead_code)]
struct AdbProject {
    name: String,
    lang: String,       // "c" or "cpp"
    standard: String,   // "c99" or "cpp17"
    src_dir: String,    // "src/"
    include_dir: String,// "include/"
    output_dir: String, // "bin/"
}

/// Create a new project: adb create <name> [--cpp]
fn create_project(name: &str, is_cpp: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Validate project name
    if name.is_empty() || name.starts_with('-') || name.starts_with('.') {
        eprintln!("❌ Error: Invalid project name '{}'", name);
        std::process::exit(1);
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        eprintln!("❌ Error: Project name '{}' contains invalid characters", name);
        eprintln!("   Use only letters, numbers, _ and -");
        std::process::exit(1);
    }

    // Check if we're already inside a project with this name
    if let Ok(cwd) = env::current_dir() {
        if let Some(dir_name) = cwd.file_name().and_then(|n| n.to_str()) {
            if dir_name == name && cwd.join("adb.toml").exists() {
                eprintln!("❌ Error: You are already inside project '{}'", name);
                eprintln!("   Current directory is already the '{}' project.", name);
                eprintln!("   To recreate, go to the parent directory first.");
                std::process::exit(1);
            }
        }
    }

    // Check if an adb.toml exists in current directory (we're inside some project)
    if Path::new("adb.toml").exists() {
        eprintln!("⚠️  Warning: An adb.toml already exists in the current directory.");
        eprintln!("   Creating '{}' as a subdirectory project.", name);
    }

    let project_dir = Path::new(name);
    if project_dir.exists() {
        eprintln!("❌ Error: Directory '{}' already exists", name);
        std::process::exit(1);
    }

    let (lang, standard, ext) = if is_cpp {
        ("cpp", "cpp17", "cpp")
    } else {
        ("c", "c99", "c")
    };

    println!("📦 Creando proyecto ADead-BIB: {}", name);
    println!("   Lenguaje: {} ({})", lang.to_uppercase(), standard);
    println!();

    // Create directories
    fs::create_dir_all(project_dir.join("src"))?;
    fs::create_dir_all(project_dir.join("include"))?;
    fs::create_dir_all(project_dir.join("bin"))?;

    // Write adb.toml
    let toml_content = format!(
        "[project]\nname = \"{}\"\nversion = \"0.1.0\"\nlang = \"{}\"\nstandard = \"{}\"\n\n[build]\nsrc = \"src/\"\ninclude = \"include/\"\noutput = \"bin/\"\n",
        name, lang, standard
    );
    fs::write(project_dir.join("adb.toml"), &toml_content)?;
    println!("   ✅ adb.toml");

    // Copy header_main.h to include/
    let header_content = adead_bib::frontend::c::c_stdlib::get_header("header_main.h")
        .unwrap_or("// header_main.h — ADead-BIB\n");
    fs::write(project_dir.join("include").join("header_main.h"), header_content)?;
    println!("   ✅ include/header_main.h");

    // Write main source file
    let main_file = format!("src/main.{}", ext);
    let main_content = if is_cpp {
        format!(
            "#include <header_main.h>\n\nint main() {{\n    printf(\"Hola desde %s\\n\", \"{}\");\n    return 0;\n}}\n",
            name
        )
    } else {
        format!(
            "#include <header_main.h>\n\nint main() {{\n    printf(\"Hola desde %s\\n\", \"{}\");\n    return 0;\n}}\n",
            name
        )
    };
    fs::write(project_dir.join(&main_file), &main_content)?;
    println!("   ✅ {}", main_file);

    println!("   ✅ bin/");
    println!();
    println!("✅ Proyecto '{}' creado!", name);
    println!();
    println!("   Para compilar y ejecutar:");
    println!("   cd {}", name);
    println!("   adb run");
    println!();
    println!("   Estructura:");
    println!("   {}/", name);
    println!("   ├── adb.toml");
    println!("   ├── include/");
    println!("   │   └── header_main.h");
    println!("   ├── src/");
    println!("   │   └── main.{}", ext);
    println!("   └── bin/");

    Ok(())
}

/// Load adb.toml from a directory. Returns None if not found.
fn load_adb_toml(dir: &str) -> Option<AdbProject> {
    let toml_path = Path::new(dir).join("adb.toml");
    let content = fs::read_to_string(&toml_path).ok()?;

    let mut name = String::new();
    let mut lang = String::from("c");
    let mut standard = String::from("c99");
    let mut src_dir = String::from("src/");
    let mut include_dir = String::from("include/");
    let mut output_dir = String::from("bin/");

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("name") {
            if let Some(val) = extract_toml_value(line) { name = val; }
        } else if line.starts_with("lang") {
            if let Some(val) = extract_toml_value(line) { lang = val; }
        } else if line.starts_with("standard") {
            if let Some(val) = extract_toml_value(line) { standard = val; }
        } else if line.starts_with("src") {
            if let Some(val) = extract_toml_value(line) { src_dir = val; }
        } else if line.starts_with("include") && line.contains('=') {
            if let Some(val) = extract_toml_value(line) { include_dir = val; }
        } else if line.starts_with("output") {
            if let Some(val) = extract_toml_value(line) { output_dir = val; }
        }
    }

    if name.is_empty() { return None; }

    Some(AdbProject { name, lang, standard, src_dir, include_dir, output_dir })
}

/// Extract value from a TOML line like: key = "value"
fn extract_toml_value(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() == 2 {
        let val = parts[1].trim().trim_matches('"');
        Some(val.to_string())
    } else {
        None
    }
}

/// Build an adb.toml project. Returns the output filename.
fn build_project(proj: &AdbProject, args: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    // Find main source file
    let ext = if proj.lang == "cpp" { "cpp" } else { "c" };
    let main_src = Path::new(&proj.src_dir).join(format!("main.{}", ext));

    if !main_src.exists() {
        eprintln!("❌ Error: Source file not found: {}", main_src.display());
        eprintln!("   Expected: {}/main.{}", proj.src_dir, ext);
        std::process::exit(1);
    }

    // Ensure output directory exists
    fs::create_dir_all(&proj.output_dir).ok();

    // Build output path
    let exe_ext = if cfg!(target_os = "windows") { ".exe" } else { "" };
    let output_file = format!("{}{}{}", proj.output_dir, proj.name, exe_ext);

    // Build args with -o and include path
    let mut build_args = args.to_vec();
    // Add -o if not already specified
    if !build_args.iter().any(|a| a == "-o") {
        build_args.push("-o".to_string());
        build_args.push(output_file.clone());
    }

    let main_src_str = main_src.to_str().unwrap_or("src/main.c");

    if proj.lang == "cpp" {
        compile_cpp_file(main_src_str, &build_args)?;
    } else {
        compile_c_file(main_src_str, &build_args)?;
    }

    Ok(output_file)
}

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

// ============================================================
// STEP COMPILER — Step-by-step visualization
// ============================================================
fn step_compile_c(input_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    use adead_bib::frontend::c::c_lexer::{CLexer, CToken};
    use adead_bib::frontend::c::c_preprocessor::CPreprocessor;
    use adead_bib::frontend::c::c_parser::CParser;
    use adead_bib::frontend::c::c_to_ir::CToIR;

    println!();
    println!("{}",  "=".repeat(64));
    println!("  ADead-BIB Step Compiler — {}", input_file);
    println!("  Cada paso del pipeline, visible.");
    println!("{}",  "=".repeat(64));
    println!();

    // 0. Read source
    let source = fs::read_to_string(input_file)
        .map_err(|e| format!("Cannot read '{}': {}", input_file, e))?;
    let source_lines: Vec<&str> = source.lines().collect();
    println!("[SOURCE]   {} lines, {} bytes", source_lines.len(), source.len());
    println!();

    // 1. PREPROCESSOR
    println!("--- Phase 1: PREPROCESSOR ---");
    let mut preprocessor = CPreprocessor::new();
    let preprocessed = preprocessor.process(&source);
    let pp_lines: Vec<&str> = preprocessed.lines().collect();
    println!("[PREPROC]  {} lines after preprocessing", pp_lines.len());
    // Show #include resolutions
    for line in &source_lines {
        let trimmed = line.trim();
        if trimmed.starts_with("#include") {
            println!("[PREPROC]  {} -> resolved internally", trimmed);
        }
    }
    println!();

    // 2. LEXER
    println!("--- Phase 2: LEXER ---");
    let (tokens, lines) = CLexer::new(&preprocessed).tokenize();
    println!("[LEXER]    {} tokens generated", tokens.len());
    // Show first N meaningful tokens with source context
    let max_show = 20;
    let mut shown = 0;
    for (i, tok) in tokens.iter().enumerate() {
        if *tok == CToken::Eof { break; }
        if shown >= max_show { break; }
        let line_num = if i < lines.len() { lines[i] } else { 0 };
        let tok_str = format!("{:?}", tok);
        // Shorten long token names
        let short = if tok_str.len() > 40 {
            format!("{}...", &tok_str[..37])
        } else {
            tok_str
        };
        println!("[LEXER]    {:>4}:{:<3}  {:<42} OK", line_num, i, short);
        shown += 1;
    }
    if tokens.len() > max_show + 1 {
        println!("[LEXER]    ... ({} more tokens)", tokens.len() - max_show - 1);
    }
    println!();

    // 3. PARSER
    println!("--- Phase 3: PARSER ---");
    let unit = CParser::new(tokens, lines).parse_translation_unit()
        .map_err(|e| format!("Parse error: {}", e))?;
    let mut func_count = 0;
    let mut struct_count = 0;
    let mut typedef_count = 0;
    let mut enum_count = 0;
    let mut global_count = 0;
    for decl in &unit.declarations {
        match decl {
            adead_bib::frontend::c::c_ast::CTopLevel::FunctionDef { name, params, body, .. } => {
                func_count += 1;
                println!("[PARSER]   function '{}' ({} params, {} stmts) OK", name, params.len(), body.len());
            }
            adead_bib::frontend::c::c_ast::CTopLevel::FunctionDecl { name, params, .. } => {
                println!("[PARSER]   declaration '{}' ({} params) OK", name, params.len());
            }
            adead_bib::frontend::c::c_ast::CTopLevel::StructDef { name, fields } => {
                struct_count += 1;
                println!("[PARSER]   struct '{}' ({} fields) OK", name, fields.len());
            }
            adead_bib::frontend::c::c_ast::CTopLevel::TypedefDecl { new_name, .. } => {
                typedef_count += 1;
                println!("[PARSER]   typedef '{}' OK", new_name);
            }
            adead_bib::frontend::c::c_ast::CTopLevel::EnumDef { name, values } => {
                enum_count += 1;
                println!("[PARSER]   enum '{}' ({} values) OK", name, values.len());
            }
            adead_bib::frontend::c::c_ast::CTopLevel::GlobalVar { declarators, .. } => {
                global_count += 1;
                for d in declarators {
                    println!("[PARSER]   global '{}' OK", d.name);
                }
            }
        }
    }
    println!("[PARSER]   Total: {} functions, {} structs, {} typedefs, {} enums, {} globals",
        func_count, struct_count, typedef_count, enum_count, global_count);
    println!();

    // 4. IR CONVERSION
    println!("--- Phase 4: IR (Intermediate Representation) ---");
    let mut converter = CToIR::new();
    let program = converter.convert(&unit)?;
    for func in &program.functions {
        println!("[IR]       function '{}' -> {} IR statements OK", func.name, func.body.len());
        // Show first few IR stmts
        let max_ir = 5;
        for (j, stmt) in func.body.iter().enumerate() {
            if j >= max_ir { break; }
            let ir_str = format!("{:?}", stmt);
            let short = if ir_str.len() > 70 { format!("{}...", &ir_str[..67]) } else { ir_str };
            println!("[IR]         {}", short);
        }
        if func.body.len() > max_ir {
            println!("[IR]         ... ({} more)", func.body.len() - max_ir);
        }
    }
    println!("[IR]       {} structs registered", program.structs.len());
    println!();

    // 5. UB DETECTOR
    println!("--- Phase 5: UB DETECTOR ---");
    let mut ub_detector = adead_bib::UBDetector::new().with_file(input_file.to_string());
    ub_detector = ub_detector.with_warn_mode();
    let reports = ub_detector.analyze(&program);
    if reports.is_empty() {
        println!("[UB]       No undefined behavior detected OK");
    } else {
        let errors = reports.iter().filter(|r| format!("{:?}", r.severity).contains("Error")).count();
        let warnings = reports.len() - errors;
        println!("[UB]       {} issues found ({} errors, {} warnings)", reports.len(), errors, warnings);
        for r in reports.iter().take(5) {
            let sev = format!("{:?}", r.severity);
            let msg_short = if r.message.len() > 60 {
                format!("{}...", &r.message[..57])
            } else {
                r.message.clone()
            };
            println!("[UB]       [{}] {}", sev, msg_short);
        }
        if reports.len() > 5 {
            println!("[UB]       ... ({} more)", reports.len() - 5);
        }
    }
    println!();

    // 6. CODEGEN
    println!("--- Phase 6: CODEGEN (x86-64) ---");
    let target = adead_bib::isa::isa_compiler::Target::Windows;
    let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(target);
    let (opcodes, data, iat_offsets, string_offsets) = compiler.compile(&program);
    println!("[CODEGEN]  {} bytes of machine code generated", opcodes.len());
    println!("[CODEGEN]  {} bytes of data section", data.len());
    println!("[CODEGEN]  {} IAT entries, {} string relocations", iat_offsets.len(), string_offsets.len());
    // Show first instructions as hex
    if !opcodes.is_empty() {
        let show_bytes = opcodes.len().min(32);
        let hex: Vec<String> = opcodes[..show_bytes].iter().map(|b| format!("{:02X}", b)).collect();
        println!("[CODEGEN]  First {} bytes:", show_bytes);
        // Show in rows of 16
        for chunk in hex.chunks(16) {
            println!("[CODEGEN]    {}", chunk.join(" "));
        }
        if opcodes.len() > 32 {
            println!("[CODEGEN]    ... ({} more bytes)", opcodes.len() - 32);
        }
    }
    // Show data section strings
    if !data.is_empty() {
        println!("[CODEGEN]  Data section strings:");
        let data_str = String::from_utf8_lossy(&data);
        for s in data_str.split('\0') {
            if !s.is_empty() && s.len() < 200 {
                println!("[CODEGEN]    \"{}\"", s.escape_default());
            }
        }
    }
    println!();

    // 7. OUTPUT SUMMARY
    println!("--- Phase 7: OUTPUT ---");
    let total_size = opcodes.len() + data.len() + 1024; // ~1KB PE overhead
    println!("[OUTPUT]   Target: Windows PE x86-64");
    println!("[OUTPUT]   Code:   {} bytes", opcodes.len());
    println!("[OUTPUT]   Data:   {} bytes", data.len());
    println!("[OUTPUT]   Est. binary: ~{} bytes", total_size);
    println!();
    println!("{}",  "=".repeat(64));
    println!("  Step compilation complete.");
    println!("  To build the actual binary: adb cc {} -o output.exe", input_file);
    println!("{}",  "=".repeat(64));

    Ok(())
}

fn step_compile_cpp(input_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    use adead_bib::frontend::cpp::cpp_lexer::{CppLexer, CppToken};

    println!();
    println!("{}",  "=".repeat(64));
    println!("  ADead-BIB Step Compiler (C++) — {}", input_file);
    println!("  Cada paso del pipeline, visible.");
    println!("{}",  "=".repeat(64));
    println!();

    // 0. Read source
    let source = fs::read_to_string(input_file)
        .map_err(|e| format!("Cannot read '{}': {}", input_file, e))?;
    let source_lines: Vec<&str> = source.lines().collect();
    println!("[SOURCE]   {} lines, {} bytes", source_lines.len(), source.len());
    println!();

    // 1. PREPROCESSOR
    println!("--- Phase 1: PREPROCESSOR ---");
    for line in &source_lines {
        let trimmed = line.trim();
        if trimmed.starts_with("#include") {
            println!("[PREPROC]  {} -> resolved internally", trimmed);
        }
    }
    println!();

    // 2. LEXER
    println!("--- Phase 2: LEXER ---");
    let (tokens, lines) = CppLexer::new(&source).tokenize();
    println!("[LEXER]    {} tokens generated", tokens.len());
    let max_show = 20;
    let mut shown = 0;
    for (i, tok) in tokens.iter().enumerate() {
        if *tok == CppToken::Eof { break; }
        if shown >= max_show { break; }
        let line_num = if i < lines.len() { lines[i] } else { 0 };
        let tok_str = format!("{:?}", tok);
        let short = if tok_str.len() > 40 {
            format!("{}...", &tok_str[..37])
        } else {
            tok_str
        };
        println!("[LEXER]    {:>4}:{:<3}  {:<42} OK", line_num, i, short);
        shown += 1;
    }
    if tokens.len() > max_show + 1 {
        println!("[LEXER]    ... ({} more tokens)", tokens.len() - max_show - 1);
    }
    println!();

    // 3. PARSER + IR
    println!("--- Phase 3: PARSER + IR ---");
    let program = adead_bib::frontend::cpp::compile_cpp_to_program(&source)
        .map_err(|e| format!("C++ parse error: {}", e))?;
    for func in &program.functions {
        println!("[PARSER]   function '{}' -> {} IR statements OK", func.name, func.body.len());
        let max_ir = 5;
        for (j, stmt) in func.body.iter().enumerate() {
            if j >= max_ir { break; }
            let ir_str = format!("{:?}", stmt);
            let short = if ir_str.len() > 70 { format!("{}...", &ir_str[..67]) } else { ir_str };
            println!("[IR]         {}", short);
        }
        if func.body.len() > max_ir {
            println!("[IR]         ... ({} more)", func.body.len() - max_ir);
        }
    }
    println!("[PARSER]   {} functions, {} structs/classes", program.functions.len(), program.structs.len());
    println!();

    // 4. UB DETECTOR
    println!("--- Phase 4: UB DETECTOR ---");
    let mut ub_detector = adead_bib::UBDetector::new().with_file(input_file.to_string());
    ub_detector = ub_detector.with_warn_mode();
    let reports = ub_detector.analyze(&program);
    if reports.is_empty() {
        println!("[UB]       No undefined behavior detected OK");
    } else {
        let errors = reports.iter().filter(|r| format!("{:?}", r.severity).contains("Error")).count();
        let warnings = reports.len() - errors;
        println!("[UB]       {} issues ({} errors, {} warnings)", reports.len(), errors, warnings);
        for r in reports.iter().take(5) {
            println!("[UB]       {:?}: {}",  r.severity, if r.message.len() > 60 { &r.message[..57] } else { &r.message });
        }
    }
    println!();

    // 5. CODEGEN
    println!("--- Phase 5: CODEGEN (x86-64) ---");
    let target = adead_bib::isa::isa_compiler::Target::Windows;
    let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(target);
    let (opcodes, data, iat_offsets, string_offsets) = compiler.compile(&program);
    println!("[CODEGEN]  {} bytes of machine code", opcodes.len());
    println!("[CODEGEN]  {} bytes of data section", data.len());
    println!("[CODEGEN]  {} IAT call sites, {} string relocations", iat_offsets.len(), string_offsets.len());
    // Show DLL imports from IAT registry
    {
        let dlls = adead_bib::backend::cpu::iat_registry::dll_names();
        println!("[CODEGEN]  IAT imports ({} DLLs):", dlls.len());
        for dll in &dlls {
            let entries = adead_bib::backend::cpu::iat_registry::entries_for_dll(dll);
            let names: Vec<&str> = entries.iter().map(|e| e.name).collect();
            println!("[CODEGEN]    {} ✅ ({})", dll, names.join(", "));
        }
    }
    // Detect API usage categories from function names in source
    {
        let src_lower = source.to_lowercase();
        let mut apis_used: Vec<&str> = Vec::new();
        if src_lower.contains("createwindow") || src_lower.contains("showwindow") {
            apis_used.push("🪟 Win32 Window");
        }
        if src_lower.contains("setpixel") || src_lower.contains("lineto") || src_lower.contains("createpen")
            || src_lower.contains("createsolidbrush") || src_lower.contains("moveto") {
            apis_used.push("🎨 GDI Rendering");
        }
        if src_lower.contains("fillgradienttriangle") || src_lower.contains("filltriangle")
            || src_lower.contains("drawtriangleoutline") {
            apis_used.push("🔺 Triangle Drawing");
        }
        if src_lower.contains("drawline") {
            apis_used.push("📏 Line Drawing");
        }
        if src_lower.contains("d3d12") || src_lower.contains("createdevice") {
            apis_used.push("🎮 DirectX 12");
        }
        if src_lower.contains("dxgi") || src_lower.contains("createdxgifactory") {
            apis_used.push("🖥️ DXGI");
        }
        if src_lower.contains("messageloop") || src_lower.contains("peekmessage") || src_lower.contains("getmessage") {
            apis_used.push("🔄 Message Loop");
        }
        if src_lower.contains("rgb(") {
            apis_used.push("🌈 RGB Colors");
        }
        if !apis_used.is_empty() {
            println!("[CODEGEN]  API categories detected:");
            for api in &apis_used {
                println!("[CODEGEN]    {}", api);
            }
        }
    }
    // DLLs actually used by the program (check which IAT slots are referenced)
    {
        let used_dlls: Vec<&str> = adead_bib::backend::cpu::iat_registry::dll_names()
            .into_iter()
            .filter(|dll| {
                let entries = adead_bib::backend::cpu::iat_registry::entries_for_dll(dll);
                entries.iter().any(|e| source.contains(e.name))
            })
            .collect();
        if !used_dlls.is_empty() {
            println!("[CODEGEN]  DLLs actually used by program: {}", used_dlls.join(", "));
        }
    }
    if !opcodes.is_empty() {
        let show_bytes = opcodes.len().min(32);
        let hex: Vec<String> = opcodes[..show_bytes].iter().map(|b| format!("{:02X}", b)).collect();
        println!("[CODEGEN]  First {} bytes:", show_bytes);
        for chunk in hex.chunks(16) {
            println!("[CODEGEN]    {}", chunk.join(" "));
        }
    }
    if !data.is_empty() {
        println!("[CODEGEN]  Data section strings:");
        let data_str = String::from_utf8_lossy(&data);
        for s in data_str.split('\0') {
            if !s.is_empty() && s.len() < 200 {
                println!("[CODEGEN]    \"{}\"", s.escape_default());
            }
        }
    }
    println!();

    println!("{}",  "=".repeat(64));
    println!("  Step compilation complete.");
    println!("  To build: adb cxx {} -o output.exe", input_file);
    println!("  To run:   adb run  (from project directory)");
    println!("{}",  "=".repeat(64));

    Ok(())
}

fn print_usage(_program: &str) {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       🔥 ADead-BIB v7.0.0 💀🦈 — C/C++ Compiler 🔥        ║");
    println!("║    Sin GCC, Sin LLVM, Sin Clang — 100% Self-Sufficient      ║");
    println!("║    Sin libc, Sin linker — header_main.h = TODO              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("� PROYECTOS (como cargo):");
    println!("   adb create <name>               Nuevo proyecto C (adb.toml)");
    println!("   adb create <name> --cpp         Nuevo proyecto C++");
    println!("   adb build                       Compilar proyecto (lee adb.toml)");
    println!("   adb run                         Compilar y ejecutar proyecto");
    println!();
    println!("🔨 COMPILAR C/C++:");
    println!("   adb cc <file.c> [-o output]     Compile C99/C11");
    println!("   adb cxx <file.cpp> [-o output]  Compile C++11/14/17/20");
    println!("     [--target fastos|windows|linux]");
    println!("     [--warn-ub] (Warning only, don't stop on UB)");
    println!("   adb build <file> [-o output]    Auto-detect by extension");
    println!("   adb run <file>                  Build and execute");
    println!("   adb step <file>                 Step-by-step compilation view");
    println!("   adb <file.c|file.cpp>           Direct compilation");
    println!();
    println!("📦 HEADERS GLOBALES:");
    println!("   adb install                     Instala headers en ~/.adead/include/");
    println!("   adb include                     Muestra ruta de headers globales");
    println!();
    println!("🚀 EXAMPLES:");
    println!("   adb create hola                 Nuevo proyecto C");
    println!("   cd hola && adb run              Compilar y ejecutar");
    println!("   adb cc hello.c                  Compile hello.c → hello.exe");
    println!("   adb cxx main.cpp -o app.exe     Compile main.cpp → app.exe");
    println!("   adb run test.c                  Compile and run test.c");
    println!("   adb install                     Setup global headers");
    println!();
    println!("⚡ MINIMAL BINARIES:");
    println!("   adb nano [output] [exit_code]   Smallest valid x64 PE (~1KB)");
    println!("   adb micro [output] [exit_code]  Sub-256 byte x86 PE");
    println!("   adb vm [output] [exit_code]     MicroVM bytecode");
    println!();
    println!("🎮 GPU (Vulkan/CUDA):");
    println!("   adb gpu                         Detect GPU, generate shader");
    println!("   adb spirv [op] [size]           Generate SPIR-V compute shader");
    println!("   adb vulkan                      Initialize Vulkan runtime");
    println!("   adb cuda [op] [size]            Generate CUDA code (.cu)");
    println!("   adb unified [op] [size]         Auto CPU↔GPU decision");
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
    println!("🔧 SETUP:");
    println!("   adb --version      Show path and setup instructions");
    println!("   adb install        Install global headers");
    println!();
}
