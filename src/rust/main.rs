// ADead-BIB Compiler CLI
// Interfaz de línea de comandos robusta
// Soporta: build, run, check

use adead_bib::backend::gpu::gpu_detect::GPUFeatures;
use adead_bib::backend::gpu::vulkan::VulkanBackend;
use adead_bib::backend::gpu::vulkan_runtime;
use adead_bib::backend::microvm::{self, compile_microvm, MicroOp, MicroVM};
use adead_bib::backend::pe_tiny;
use adead_bib::builder::{BuildOptions, Builder};
use adead_bib::frontend::c::compile_c_to_program;
use adead_bib::frontend::cpp::compile_cpp_to_program;
use adead_bib::frontend::lexer::Lexer;
use adead_bib::frontend::parser::Parser;
use adead_bib::frontend::type_checker::TypeChecker;
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
        "build" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                print_usage(&args[0]);
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = get_output_filename(input_file, &args);

            // Check for optimization flags
            let size_opt = args.iter().any(|a| a == "--size" || a == "-s");
            let ultra_opt = args.iter().any(|a| a == "--ultra" || a == "-u");

            let opt_level = if ultra_opt {
                adead_bib::optimizer::OptLevel::Ultra
            } else if size_opt {
                adead_bib::optimizer::OptLevel::Aggressive
            } else {
                adead_bib::optimizer::OptLevel::Basic
            };

            println!("🔨 Building {}...", input_file);
            if size_opt || ultra_opt {
                println!("   Optimization: {:?}", opt_level);
            }

            let options = BuildOptions {
                target: determine_target(),
                optimize: true,
                output_path: output_file.clone(),
                verbose: true,
                opt_level,
                size_optimize: size_opt || ultra_opt,
            };

            Builder::build_file(input_file, options)?;
            println!("✅ Build complete: {}", output_file);
        }
        "opt" | "optimize" => {
            // Compilación con optimización máxima de tamaño
            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                eprintln!("   Uso: adB opt <archivo.adB>");
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = get_output_filename(input_file, &args);

            println!("🔧 Building with ULTRA optimization: {}...", input_file);
            println!("   Target: Smallest possible binary");

            let options = BuildOptions {
                target: determine_target(),
                optimize: true,
                output_path: output_file.clone(),
                verbose: true,
                opt_level: adead_bib::optimizer::OptLevel::Ultra,
                size_optimize: true,
            };

            Builder::build_file(input_file, options)?;

            // Show file size
            if let Ok(metadata) = std::fs::metadata(&output_file) {
                println!(
                    "✅ Optimized build complete: {} ({} bytes)",
                    output_file,
                    metadata.len()
                );
            } else {
                println!("✅ Optimized build complete: {}", output_file);
            }
        }
        "cc" => {
            // ============================================================
            // ADead-BIB C Compiler — Compila C99 nativo
            // Sin GCC. Sin LLVM. Sin Clang. Solo ADead-BIB.
            // ============================================================
            if args.len() < 3 {
                eprintln!("❌ Error: Missing C source file");
                eprintln!("   Usage: adB cc <file.c> [-o output.exe]");
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = get_output_filename(input_file, &args);

            println!("🔨 ADead-BIB C Compiler");
            println!("   Source: {}", input_file);
            println!("   Target: {}", output_file);

            // 1. Read C source
            let source = fs::read_to_string(input_file).map_err(|e| {
                format!("Cannot read '{}': {}", input_file, e)
            })?;

            // 2. C Frontend: CLexer → CParser → CTranslationUnit → Program
            println!("   Step 1: Parsing C99...");
            let program = compile_c_to_program(&source).map_err(|e| {
                format!("C parse error: {}", e)
            })?;

            println!("   Step 2: {} functions, {} structs found",
                program.functions.len(), program.structs.len());

            // 3. Compile via ISA Compiler → bytes
            println!("   Step 3: Compiling to native x86-64...");
            let target = determine_target();
            let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(target);
            let (opcodes, data) = compiler.compile(&program);

            // 4. Generate binary
            println!("   Step 4: Generating binary...");
            match target {
                Target::Windows => {
                    adead_bib::backend::pe::generate_pe(&opcodes, &data, &output_file)?;
                }
                Target::Linux => {
                    adead_bib::backend::elf::generate_elf(&opcodes, &data, &output_file)?;
                }
                Target::Raw => {
                    fs::write(&output_file, &opcodes)?;
                }
            }

            if let Ok(meta) = fs::metadata(&output_file) {
                println!("✅ C compilation complete: {} ({} bytes)", output_file, meta.len());
            } else {
                println!("✅ C compilation complete: {}", output_file);
            }
            println!("   🏆 Sin GCC, sin LLVM, sin Clang — 100% ADead-BIB");
        }
        "cxx" | "c++" | "cpp" => {
            // ============================================================
            // ADead-BIB C++ Compiler — Compila C++11/14/17/20 nativo
            // Sin GCC. Sin LLVM. Sin Clang. Solo ADead-BIB. 💀🦈
            // ============================================================
            if args.len() < 3 {
                eprintln!("❌ Error: Missing C++ source file");
                eprintln!("   Usage: adB cxx <file.cpp> [-o output.exe]");
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = get_output_filename(input_file, &args);

            println!("🔨 ADead-BIB C++ Compiler");
            println!("   Source: {}", input_file);
            println!("   Target: {}", output_file);

            // 1. Read C++ source
            let source = fs::read_to_string(input_file).map_err(|e| {
                format!("Cannot read '{}': {}", input_file, e)
            })?;

            // 2. C++ Frontend: CppLexer → CppParser → CppAST → Program
            println!("   Step 1: Parsing C++...");
            let program = compile_cpp_to_program(&source).map_err(|e| {
                format!("C++ parse error: {}", e)
            })?;

            println!("   Step 2: {} functions, {} structs, {} classes found",
                program.functions.len(), program.structs.len(), program.classes.len());

            // 3. Compile via ISA Compiler → bytes
            println!("   Step 3: Compiling to native x86-64...");
            let target = determine_target();
            let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(target);
            let (opcodes, data) = compiler.compile(&program);

            // 4. Generate binary
            println!("   Step 4: Generating binary...");
            match target {
                Target::Windows => {
                    adead_bib::backend::pe::generate_pe(&opcodes, &data, &output_file)?;
                }
                Target::Linux => {
                    adead_bib::backend::elf::generate_elf(&opcodes, &data, &output_file)?;
                }
                Target::Raw => {
                    fs::write(&output_file, &opcodes)?;
                }
            }

            if let Ok(meta) = fs::metadata(&output_file) {
                println!("✅ C++ compilation complete: {} ({} bytes)", output_file, meta.len());
            } else {
                println!("✅ C++ compilation complete: {}", output_file);
            }
            println!("   🏆 Sin GCC, sin LLVM, sin Clang — 100% ADead-BIB C++");
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                print_usage(&args[0]);
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = format!(
                "{}.exe",
                Path::new(input_file).file_stem().unwrap().to_str().unwrap()
            );

            // 1. Build
            let options = BuildOptions {
                target: determine_target(),
                optimize: true,
                output_path: output_file.clone(),
                verbose: false, // Quiet for run
                opt_level: adead_bib::optimizer::OptLevel::Basic,
                size_optimize: false,
            };

            if let Err(e) = Builder::build_file(input_file, options) {
                eprintln!("❌ Build failed: {}", e);
                std::process::exit(1);
            }

            // 2. Run
            println!("🚀 Running {}...\n", input_file);
            // Usar ruta relativa con ./ para Windows
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
        "check" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                print_usage(&args[0]);
                std::process::exit(1);
            }
            let input_file = &args[2];
            let json_output = args.iter().any(|a| a == "--json" || a == "-j");

            if json_output {
                // Salida JSON para extensión VS Code
                match check_syntax_json(input_file) {
                    Ok(json) => println!("{}", json),
                    Err(e) => {
                        let error_json = format!(
                            r#"{{"file":"{}","status":"error","errors":[{{"line":1,"column":1,"message":"{}"}}],"warnings":[]}}"#,
                            input_file,
                            e.to_string().replace('"', "\\\"")
                        );
                        println!("{}", error_json);
                        std::process::exit(1);
                    }
                }
            } else {
                println!("🔍 Checking syntax of {}...", input_file);

                match check_syntax(input_file) {
                    Ok(()) => {
                        println!("✅ Syntax check passed!");
                    }
                    Err(e) => {
                        eprintln!("❌ Syntax error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        "tiny" => {
            // Genera PE ultra-compacto (< 500 bytes)
            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                print_usage(&args[0]);
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = get_output_filename(input_file, &args);

            println!("🔬 Building TINY PE from {}...", input_file);
            println!("   Target: Ultra-compact binary (< 500 bytes)");

            // Leer y compilar
            let source = fs::read_to_string(input_file)?;
            let program = Parser::parse_program(&source)?;

            // Generar código mínimo via ISA Compiler
            let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(Target::Raw);
            let (opcodes, _data) = compiler.compile(&program);

            // Si el código es muy grande, usar exit simple
            let final_opcodes = if opcodes.len() > 200 {
                println!(
                    "   ⚠️  Code too large ({}b), using minimal exit",
                    opcodes.len()
                );
                pe_tiny::generate_exit_opcodes(0)
            } else {
                opcodes
            };

            // Generar PE tiny
            match pe_tiny::generate_pe_tiny(&final_opcodes, &output_file) {
                Ok(size) => {
                    println!("✅ Tiny build complete: {} ({} bytes)", output_file, size);
                    println!("   🎯 Goal: < 500 bytes | Achieved: {} bytes", size);
                }
                Err(e) => {
                    eprintln!("❌ Tiny build failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "nano" => {
            // Genera PE nano (el más pequeño posible)
            let output_file = if args.len() >= 3 {
                args[2].clone()
            } else {
                "nano.exe".to_string()
            };

            let exit_code: u8 = if args.len() >= 4 {
                args[3].parse().unwrap_or(0)
            } else {
                0
            };

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
            // Genera PE32 micro (< 256 bytes)
            let output_file = if args.len() >= 3 {
                args[2].clone()
            } else {
                "micro.exe".to_string()
            };

            let exit_code: u8 = if args.len() >= 4 {
                args[3].parse().unwrap_or(0)
            } else {
                0
            };

            println!("🔬 Building MICRO PE (x86 32-bit)...");
            println!("   Target: Sub-256 byte Windows executable");

            match pe_tiny::generate_pe32_micro(exit_code, &output_file) {
                Ok(size) => {
                    println!("✅ Micro build complete: {} ({} bytes)", output_file, size);
                    if size < 256 {
                        println!("   🏆 SUB-256 BYTES ACHIEVED!");
                    } else if size < 512 {
                        println!("   🎯 Sub-512 bytes achieved!");
                    }
                }
                Err(e) => {
                    eprintln!("❌ Micro build failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "flat" | "raw" => {
            // Genera flat binary desde código ADead-BIB (v3.1-OS)
            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                eprintln!("   Uso: adB flat <archivo.adB> [-o output.bin]");
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = if let Some(pos) = args.iter().position(|a| a == "-o") {
                args.get(pos + 1)
                    .cloned()
                    .unwrap_or_else(|| "flat.bin".to_string())
            } else {
                Path::new(input_file)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    + ".bin"
            };

            println!("🔧 Building FLAT binary from {}...", input_file);
            println!("   Target: Pure machine code, no headers");

            // Leer y compilar fuente ADead-BIB
            let source = fs::read_to_string(input_file)?;
            let program = Parser::parse_program(&source)?;

            // Generar código via ISA Compiler
            let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(Target::Raw);
            let (opcodes, data) = compiler.compile(&program);

            // Usar FlatBinaryGenerator
            let mut gen = adead_bib::backend::cpu::flat_binary::FlatBinaryGenerator::new(0x0000);
            let binary = gen.generate(&opcodes, &data);

            fs::write(&output_file, &binary)?;
            println!(
                "✅ Flat build complete: {} ({} bytes)",
                output_file,
                binary.len()
            );
            println!("   💎 Pure machine code — zero headers!");
        }
        "boot" => {
            // Genera boot sector (512 bytes con firma 0x55AA) desde ADead-BIB (v3.1-OS)
            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                eprintln!("   Uso: adB boot <archivo.adB> [-o boot.bin]");
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = if let Some(pos) = args.iter().position(|a| a == "-o") {
                args.get(pos + 1)
                    .cloned()
                    .unwrap_or_else(|| "boot.bin".to_string())
            } else {
                "boot.bin".to_string()
            };

            println!("🔧 Building BOOT SECTOR from {}...", input_file);
            println!("   Target: 512-byte boot sector (0x55AA signature)");
            println!("   Origin: 0x7C00 (BIOS load address)");

            // Leer y compilar fuente ADead-BIB
            let source = fs::read_to_string(input_file)?;
            let program = Parser::parse_program(&source)?;

            // Generar código via ISA Compiler
            let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(Target::Raw);
            let (opcodes, _data) = compiler.compile(&program);

            // Usar FlatBinaryGenerator para boot sector
            let mut gen = adead_bib::backend::cpu::flat_binary::FlatBinaryGenerator::new(0x7C00);
            let binary = gen.generate_boot_sector(&opcodes);

            fs::write(&output_file, &binary)?;
            println!(
                "✅ Boot sector complete: {} ({} bytes)",
                output_file,
                binary.len()
            );
            println!(
                "   🔥 Boot sector ready! Test with: qemu-system-x86_64 -drive format=raw,file={}",
                output_file
            );
        }
        "fastos" => {
            // Genera imagen de disco FastOS booteable desde ADead-BIB
            use adead_bib::isa::isa_compiler::CpuMode;

            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                eprintln!("   Uso: adB fastos <archivo.adB> [-o fastos.bin] [--run] [--mode real16|prot32|long64]");
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = if let Some(pos) = args.iter().position(|a| a == "-o") {
                args.get(pos + 1)
                    .cloned()
                    .unwrap_or_else(|| "fastos.bin".to_string())
            } else {
                "fastos.bin".to_string()
            };
            let run_qemu = args.iter().any(|a| a == "--run");

            // Parse CPU mode (default: long64)
            let cpu_mode = if let Some(pos) = args.iter().position(|a| a == "--mode") {
                match args.get(pos + 1).map(|s| s.as_str()) {
                    Some("real16") | Some("16") => CpuMode::Real16,
                    Some("prot32") | Some("32") => CpuMode::Protected32,
                    Some("long64") | Some("64") => CpuMode::Long64,
                    Some(other) => {
                        eprintln!("❌ Unknown CPU mode: {}. Use: real16, prot32, long64", other);
                        std::process::exit(1);
                    }
                    None => CpuMode::Long64,
                }
            } else {
                CpuMode::Long64 // Default: 64-bit
            };

            let mode_name = match cpu_mode {
                CpuMode::Real16 => "16-bit Real Mode",
                CpuMode::Protected32 => "32-bit Protected Mode",
                CpuMode::Long64 => "64-bit Long Mode (default)",
            };

            println!("🚀 Building FastOS image from {}...", input_file);
            println!("   Format: FastOS (magic: FsOS) — Alternative to PE/ELF");
            println!("   CPU Mode: {} ({}-bit)", mode_name, cpu_mode.operand_bits());
            println!("   Scaling: 16-bit → 32-bit → 64-bit (natural)");

            // Leer y compilar fuente ADead-BIB
            let source = fs::read_to_string(input_file)?;
            let program = Parser::parse_program(&source)?;

            // Generar código via ISA Compiler con CPU mode
            let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::with_cpu_mode(Target::Raw, cpu_mode);
            let (opcodes, _data) = compiler.compile(&program);

            // Generar boot sector con firma 0x55AA
            let mut gen = adead_bib::backend::cpu::flat_binary::FlatBinaryGenerator::new(0x7C00);
            let binary = gen.generate_boot_sector(&opcodes);

            fs::write(&output_file, &binary)?;
            println!(
                "✅ FastOS image complete: {} ({} bytes)",
                output_file,
                binary.len()
            );
            println!("   🔥 Magic: FsOS | Signature: 0x55AA | Mode: {}", mode_name);

            if run_qemu {
                println!("   🖥️  Launching QEMU...");
                let qemu = "C:\\Program Files\\qemu\\qemu-system-x86_64.exe";
                let status = std::process::Command::new(qemu)
                    .args(&["-drive", &format!("format=raw,file={}", output_file),
                            "-no-reboot", "-no-shutdown"])
                    .status();
                match status {
                    Ok(s) => println!("   QEMU exited: {}", s),
                    Err(e) => eprintln!("   ❌ QEMU failed: {}", e),
                }
            } else {
                println!(
                    "   Test: \"C:\\Program Files\\qemu\\qemu-system-x86_64.exe\" -drive format=raw,file={}",
                    output_file
                );
            }
        }
        "vm" => {
            // MicroVM: Bytecode ultra-compacto
            let output_file = if args.len() >= 3 {
                args[2].clone()
            } else {
                "program.adb".to_string()
            };

            let exit_code: u8 = if args.len() >= 4 {
                args[3].parse().unwrap_or(0)
            } else {
                0
            };

            println!("🔬 Building MicroVM bytecode...");
            println!("   Target: 4-bit instructions (1 byte = 2 ops)");

            // Generar bytecode: LOAD exit_code, EXIT
            let bytecode =
                compile_microvm(&[(MicroOp::Load, exit_code.min(15)), (MicroOp::Exit, 0)]);

            match microvm::save_bytecode(&bytecode, &output_file) {
                Ok(size) => {
                    println!("✅ MicroVM bytecode: {} ({} bytes)", output_file, size);
                    println!("   🧬 {} instructions in {} bytes", bytecode.len(), size);

                    // Ejecutar para verificar
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
        "bit" => {
            // 1 bit = 1 decisión
            let bit_value: bool = if args.len() >= 3 {
                args[2].parse::<u8>().unwrap_or(0) != 0
            } else {
                false
            };

            println!("🔬 1-BIT PROGRAM");
            println!("   The ultimate minimal computation");
            println!();
            println!("   Input bit: {}", if bit_value { "1" } else { "0" });
            println!(
                "   Decision:  {}",
                if bit_value {
                    "YES/TRUE/ON"
                } else {
                    "NO/FALSE/OFF"
                }
            );
            println!();
            println!("   📊 Theoretical size: 0.125 bytes (1 bit)");
            println!("   📊 Actual storage:   1 byte (8x overhead)");
            println!();

            let program = microvm::generate_1bit_program(bit_value);
            let min_size = microvm::theoretical_minimum(1);

            println!("   🧬 Program: {:08b} (binary)", program[0]);
            println!("   🎯 Minimum possible: {} bytes", min_size);
            println!();
            println!("   💡 With ADead runtime, this 1 bit executes as:");
            println!(
                "      [Runtime] + [1 bit] → exit({})",
                if bit_value { 1 } else { 0 }
            );
        }
        "gpu" => {
            // Detectar GPU y mostrar info completa
            let gpu = GPUFeatures::detect();

            // Mostrar resumen completo
            gpu.print_summary();

            if gpu.available {
                println!();

                // Generar shader optimizado
                let mut backend = VulkanBackend::new();
                let spirv = backend.generate_optimized_shader(&gpu);

                let output_path = if args.len() >= 3 {
                    args[2].clone()
                } else {
                    "builds/matmul.spv".to_string()
                };

                match backend.save_spirv(&spirv, &output_path) {
                    Ok(size) => {
                        println!(
                            "✅ SPIR-V Shader generated: {} ({} bytes)",
                            output_path, size
                        );
                        println!("   Optimized for: {}", gpu.device_name);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to save shader: {}", e);
                    }
                }
            }
        }
        "spirv" => {
            // Generar SPIR-V para operación específica
            let op = if args.len() >= 3 { &args[2] } else { "matmul" };
            let size: u32 = if args.len() >= 4 {
                args[3].parse().unwrap_or(1024)
            } else {
                1024
            };

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
                    println!("   Ready for Vulkan compute dispatch!");
                }
                Err(e) => {
                    eprintln!("❌ Failed: {}", e);
                }
            }
        }
        "vulkan" | "vk" => {
            // Inicializar Vulkan runtime REAL y exprimir GPU
            println!("🔥 VULKAN RUNTIME - EXPRIMIR GPU");
            println!();

            match unsafe { vulkan_runtime::VulkanRuntime::new() } {
                Ok(runtime) => {
                    runtime.print_device_info();
                    println!();
                    println!("✅ Vulkan runtime initialized successfully!");
                    println!("   Ready to dispatch compute shaders on your GPU.");
                    println!();

                    // Mostrar capacidades
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
            // Generar código CUDA desde ADead-BIB
            use adead_bib::backend::gpu::cuda;

            let op = if args.len() >= 3 {
                &args[2]
            } else {
                "vectoradd"
            };
            let size: usize = if args.len() >= 4 {
                args[3].parse().unwrap_or(1024)
            } else {
                1024
            };

            println!("🔥 ADead-BIB + CUDA Code Generator");
            println!("   Operation: {}", op);
            println!("   Size: {}", size);
            println!();

            let code = match op {
                "matmul" => {
                    println!("   Generating MatMul kernel {}x{}...", size, size);
                    cuda::generate_matmul_benchmark(size)
                }
                "benchmark" | "bench" => {
                    println!("   Generating Full Benchmark Suite (CPU vs GPU)...");
                    cuda::generate_full_benchmark()
                }
                "vectoradd" | _ => {
                    println!("   Generating VectorAdd kernel ({} elements)...", size);
                    cuda::generate_adead_cuda_test(size)
                }
            };

            // Guardar en CUDA/ADead_Generated/
            let output_path = format!("CUDA/ADead_Generated/adead_{}.cu", op);
            match fs::write(&output_path, &code) {
                Ok(_) => {
                    println!("✅ CUDA code generated: {}", output_path);
                    println!("   Lines: {}", code.lines().count());
                    println!();
                    println!("📋 To compile (requires CUDA Toolkit):");
                    println!("   nvcc {} -o {}.exe", output_path, op);
                    println!();
                    println!("🚀 To run:");
                    println!("   ./{}.exe", op);
                }
                Err(e) => {
                    eprintln!("❌ Failed to write CUDA code: {}", e);
                }
            }
        }
        "unified" | "uni" => {
            // Pipeline unificado: decisión automática CPU/GPU, elimina ruido
            use adead_bib::backend::gpu::unified_pipeline::{
                MathOperation, PipelineMode, UnifiedPipeline,
            };

            let op = if args.len() >= 3 {
                &args[2]
            } else {
                "vectoradd"
            };
            let size: usize = if args.len() >= 4 {
                args[3].parse().unwrap_or(1000000)
            } else {
                1000000
            };

            println!("🔥 ADead-BIB Unified Pipeline");
            println!("   Decisión automática CPU↔GPU | Elimina ruido");
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
                "reduce" | "reduction" => {
                    println!("   Operation: Reduction ({} elements)", size);
                    MathOperation::Reduction { size }
                }
                "vectoradd" | _ => {
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
        "play" | "repl" => {
            // Modo interactivo estilo Rust Playground / Jupyter
            run_playground()?;
        }
        "create" => {
            // adB create <nombre> - Crear nuevo proyecto (estilo Rust: cargo new)
            if args.len() < 3 {
                eprintln!("❌ Uso: adB create <nombre_proyecto>");
                eprintln!("   Ejemplo: adB create mi_juego");
                std::process::exit(1);
            }
            let project_name = &args[2];
            create_new_project(project_name)?;
        }
        "new" => {
            // Alias: adB new <nombre> = adB create <nombre>
            if args.len() < 3 {
                eprintln!("❌ Uso: adB new <nombre_proyecto>");
                std::process::exit(1);
            }
            let project_name = &args[2];
            create_new_project(project_name)?;
        }
        "init" => {
            // adB init - Inicializar proyecto en directorio actual
            let current_dir = std::env::current_dir()?;
            let project_name = current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("proyecto");
            create_new_project_in_place(project_name)?;
        }
        _ => {
            // Auto-detect by file extension: .c → cc, .cpp/.cxx/.cc → cxx, else → adB build
            let input_file = command;
            let ext = Path::new(input_file).extension().and_then(|e| e.to_str()).unwrap_or("");

            match ext {
                "c" | "h" => {
                    // Auto-route to C compiler
                    let output_file = get_output_filename(input_file, &args);
                    println!("🔨 ADead-BIB C Compiler (auto-detected .c)");
                    println!("   Source: {}", input_file);
                    println!("   Target: {}", output_file);

                    let source = fs::read_to_string(input_file).map_err(|e| {
                        format!("Cannot read '{}': {}", input_file, e)
                    })?;

                    println!("   Step 1: Parsing C99...");
                    let program = compile_c_to_program(&source).map_err(|e| {
                        format!("C parse error: {}", e)
                    })?;

                    println!("   Step 2: {} functions, {} structs found",
                        program.functions.len(), program.structs.len());

                    println!("   Step 3: Compiling to native x86-64...");
                    let target = determine_target();
                    let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(target);
                    let (opcodes, data) = compiler.compile(&program);

                    println!("   Step 4: Generating binary...");
                    match target {
                        Target::Windows => {
                            adead_bib::backend::pe::generate_pe(&opcodes, &data, &output_file)?;
                        }
                        Target::Linux => {
                            adead_bib::backend::elf::generate_elf(&opcodes, &data, &output_file)?;
                        }
                        Target::Raw => {
                            fs::write(&output_file, &opcodes)?;
                        }
                    }

                    if let Ok(meta) = fs::metadata(&output_file) {
                        println!("✅ C compilation complete: {} ({} bytes)", output_file, meta.len());
                    } else {
                        println!("✅ C compilation complete: {}", output_file);
                    }
                    println!("   🏆 Sin GCC, sin LLVM, sin Clang — 100% ADead-BIB");
                }
                "cpp" | "cxx" | "cc" | "hpp" => {
                    // Auto-route to C++ compiler
                    let output_file = get_output_filename(input_file, &args);
                    println!("🔨 ADead-BIB C++ Compiler (auto-detected .{})", ext);
                    println!("   Source: {}", input_file);
                    println!("   Target: {}", output_file);

                    let source = fs::read_to_string(input_file).map_err(|e| {
                        format!("Cannot read '{}': {}", input_file, e)
                    })?;

                    println!("   Step 1: Parsing C++...");
                    let program = compile_cpp_to_program(&source).map_err(|e| {
                        format!("C++ parse error: {}", e)
                    })?;

                    println!("   Step 2: {} functions, {} structs, {} classes found",
                        program.functions.len(), program.structs.len(), program.classes.len());

                    println!("   Step 3: Compiling to native x86-64...");
                    let target = determine_target();
                    let mut compiler = adead_bib::isa::isa_compiler::IsaCompiler::new(target);
                    let (opcodes, data) = compiler.compile(&program);

                    println!("   Step 4: Generating binary...");
                    match target {
                        Target::Windows => {
                            adead_bib::backend::pe::generate_pe(&opcodes, &data, &output_file)?;
                        }
                        Target::Linux => {
                            adead_bib::backend::elf::generate_elf(&opcodes, &data, &output_file)?;
                        }
                        Target::Raw => {
                            fs::write(&output_file, &opcodes)?;
                        }
                    }

                    if let Ok(meta) = fs::metadata(&output_file) {
                        println!("✅ C++ compilation complete: {} ({} bytes)", output_file, meta.len());
                    } else {
                        println!("✅ C++ compilation complete: {}", output_file);
                    }
                    println!("   🏆 Sin GCC, sin LLVM, sin Clang — 100% ADead-BIB C++");
                }
                _ => {
                    // Default: ADead-BIB language
                    let output_file = get_output_filename(input_file, &args);
                    let options = BuildOptions {
                        target: determine_target(),
                        optimize: true,
                        output_path: output_file.clone(),
                        verbose: true,
                        opt_level: adead_bib::optimizer::OptLevel::Basic,
                        size_optimize: false,
                    };
                    Builder::build_file(input_file, options)?;
                }
            }
        }
    }

    Ok(())
}

fn determine_target() -> Target {
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

    // Default: input.exe
    Path::new(input)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        + ".exe"
}

fn print_usage(_program: &str) {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           🔥 ADead-BIB v2.0.0 🔥                             ║");
    println!("║     OOP Puro + ASM Simbionte = El Nuevo Lenguaje            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("📋 CREAR PROYECTO (estilo Rust):");
    println!("   adB create <nombre>               - Crear nuevo proyecto");
    println!("   adB new <nombre>                  - Alias de create");
    println!("   adB init                          - Inicializar en directorio actual");
    println!();
    println!("📋 COMPILAR Y EJECUTAR:");
    println!("   adB run <archivo.adB>             - Compilar y ejecutar");
    println!("   adB build <archivo.adB>           - Compilar a ejecutable");
    println!("   adB check <archivo.adB>           - Verificar sintaxis");
    println!("   adB cc <archivo.c> [-o output]    - 🔥 Compilar C99 nativo");
    println!("   adB play                          - 🎮 Modo interactivo (REPL)");
    println!();
    println!("🚀 EJEMPLOS:");
    println!("   adB create hola                   - Crear proyecto 'hola'");
    println!("   adB run main.adB                  - Ejecutar main.adB");
    println!("   adB build main.adB -o app.exe     - Compilar a app.exe");
    println!();
    println!("⚡ MODOS AVANZADOS:");
    println!("   adB tiny <archivo.adB>            - PE ultra-compacto (< 500 bytes)");
    println!("   adB nano [output.exe] [exit_code] - PE más pequeño posible");
    println!("   adB micro [output.exe]            - PE32 sub-256 bytes");
    println!("   adB vm <output.adb>               - MicroVM bytecode");
    println!();
    println!("🎮 GPU (Vulkan/CUDA):");
    println!("   adB gpu                           - Detectar GPU y generar shader");
    println!("   adB spirv [op] [size]             - Generar SPIR-V compute shader");
    println!("   adB vulkan                        - Inicializar Vulkan runtime");
    println!("   adB cuda [op] [size]              - Generar código CUDA (.cu)");
    println!();
    println!("🚀 PIPELINE UNIFICADO (HEX + CUDA):");
    println!("   adB unified [op] [size]           - Decisión auto CPU↔GPU");
    println!("   adB unified matmul 1000000        - MatMul 1000x1000");
    println!("   Flags: --cpu (forzar CPU), --force-gpu (forzar GPU)");
    println!();
    println!("📝 SINTAXIS SOPORTADA:");
    println!("   • Python-style: def, print, if/elif/else, for, while");
    println!("   • Rust-style:   fn, let, mut, struct, impl, trait, match");
    println!("   • Scripts:      Código directo sin main() requerido");
    println!();
    println!("🎮 MODO PLAY (REPL):");
    println!("   adB play                          - Inicia playground interactivo");
    println!("   Escribe código ADead-BIB y presiona Enter para ejecutar");
    println!("   Comandos: :help, :clear, :exit, :run, :ast");
    println!();
    println!("🖥️  FASTOS / OS DEVELOPMENT:");
    println!("   adB fastos <archivo.adB> [-o out.bin]  - Build FastOS bootable image");
    println!("   adB fastos <archivo.adB> --run         - Build + launch QEMU");
    println!("   adB fastos <archivo.adB> --mode real16 - 16-bit real mode");
    println!("   adB fastos <archivo.adB> --mode prot32 - 32-bit protected mode");
    println!("   adB fastos <archivo.adB> --mode long64 - 64-bit long mode (default)");
    println!("   adB boot <archivo.adB> [-o boot.bin]   - Build 512-byte boot sector");
    println!("   adB flat <archivo.adB> [-o flat.bin]    - Build flat binary (no headers)");
    println!();
    println!("🎯 TAMAÑOS DE BINARIO:");
    println!("   Standard: ~1.5 KB  │  Tiny: < 500 bytes  │  Nano: ~1 KB");
}

fn check_syntax(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;

    // 1. Lexing
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    println!("   📝 Tokens: {}", tokens.len());

    // 2. Parsing
    let program = Parser::parse_program(&source)?;
    println!("   📦 Funciones: {}", program.functions.len());
    println!("   📦 Clases/Structs: {}", program.classes.len());
    println!("   📦 Statements top-level: {}", program.statements.len());

    // 3. Type checking
    let mut type_checker = TypeChecker::new();
    let _types = type_checker.check_program(&program);

    // 4. Validation - Scripts don't need main!
    if program.functions.is_empty() && program.statements.is_empty() {
        return Err("No code found in program".into());
    }

    // Info about main function
    let has_main = program.functions.iter().any(|f| f.name == "main");
    if has_main {
        println!("   ✅ Función main() encontrada");
    } else if !program.statements.is_empty() {
        println!(
            "   ✅ Script mode: {} statements top-level",
            program.statements.len()
        );
    }

    Ok(())
}

/// Check syntax y devuelve JSON para VS Code Extension
fn check_syntax_json(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;

    // 1. Lexing
    let mut lexer = Lexer::new(&source);
    let _tokens = lexer.tokenize();

    // 2. Parsing
    let program = Parser::parse_program(&source)?;

    // 3. Type checking
    let mut type_checker = TypeChecker::new();
    let _types = type_checker.check_program(&program);

    // 4. Detectar warnings y características
    let mut warnings: Vec<String> = Vec::new();
    let mut cpu_blocks = 0;
    let mut gpu_blocks = 0;
    let mut emit_calls = 0;
    let mut variables = 0;

    // Analizar código fuente para detectar patrones
    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        // Detectar emit![]
        if trimmed.contains("emit!") || trimmed.contains("emit![") {
            emit_calls += 1;
            warnings.push(format!(
                r#"{{"line":{},"column":1,"type":"raw_binary","severity":"warning","message":"emit![] usado - código binario directo"}}"#,
                line_num
            ));
        }

        // Detectar cpu::
        if trimmed.contains("cpu::") {
            cpu_blocks += 1;
            warnings.push(format!(
                r#"{{"line":{},"column":1,"type":"cpu_block","severity":"info","message":"Bloque cpu:: detectado"}}"#,
                line_num
            ));
        }

        // Detectar gpu::
        if trimmed.contains("gpu::") {
            gpu_blocks += 1;
            warnings.push(format!(
                r#"{{"line":{},"column":1,"type":"gpu_block","severity":"info","message":"Bloque gpu:: detectado"}}"#,
                line_num
            ));
        }

        // Detectar HEX literals
        if trimmed.contains("0x") && !trimmed.starts_with("//") {
            warnings.push(format!(
                r#"{{"line":{},"column":1,"type":"hex_literal","severity":"info","message":"Literal HEX detectado"}}"#,
                line_num
            ));
        }

        // Contar variables
        if trimmed.starts_with("let ") || trimmed.starts_with("const ") {
            variables += 1;
        }
    }

    // Construir JSON
    let json = format!(
        r#"{{"file":"{}","status":"ok","errors":[],"warnings":[{}],"diagnostics":{{"functions":{},"variables":{},"cpu_blocks":{},"gpu_blocks":{},"emit_calls":{}}}}}"#,
        file_path,
        warnings.join(","),
        program.functions.len(),
        variables,
        cpu_blocks,
        gpu_blocks,
        emit_calls
    );

    Ok(json)
}

/// Modo Playground interactivo estilo Rust Playground / Jupyter
/// Permite escribir y ejecutar código ADead-BIB de forma interactiva
fn run_playground() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        🎮 ADead-BIB Playground v0.2.0 🎮                     ║");
    println!("║     Modo interactivo - Escribe código y presiona Enter       ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("📝 Comandos disponibles:");
    println!("   :help     - Mostrar ayuda");
    println!("   :clear    - Limpiar buffer");
    println!("   :run      - Ejecutar buffer actual");
    println!("   :ast      - Mostrar AST del buffer");
    println!("   :tokens   - Mostrar tokens del buffer");
    println!("   :exit     - Salir del playground");
    println!("   :example  - Cargar ejemplo");
    println!();
    println!("💡 Tip: Escribe código directamente y presiona Enter dos veces para ejecutar");
    println!();

    let mut buffer = String::new();
    let mut line_number = 1;
    let mut variables: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

    loop {
        // Prompt
        if buffer.is_empty() {
            print!("adB[{}]> ", line_number);
        } else {
            print!("   ...> ");
        }
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // Comandos especiales
        if input.starts_with(':') {
            match input {
                ":help" | ":h" => {
                    print_playground_help();
                }
                ":clear" | ":c" => {
                    buffer.clear();
                    println!("🧹 Buffer limpiado");
                }
                ":exit" | ":quit" | ":q" => {
                    println!("👋 ¡Hasta luego!");
                    break;
                }
                ":run" | ":r" => {
                    if buffer.is_empty() {
                        println!("⚠️  Buffer vacío. Escribe código primero.");
                    } else {
                        execute_playground_code(&buffer, &mut variables);
                        line_number += 1;
                    }
                }
                ":ast" | ":a" => {
                    if buffer.is_empty() {
                        println!("⚠️  Buffer vacío.");
                    } else {
                        show_ast(&buffer);
                    }
                }
                ":tokens" | ":t" => {
                    if buffer.is_empty() {
                        println!("⚠️  Buffer vacío.");
                    } else {
                        show_tokens(&buffer);
                    }
                }
                ":example" | ":e" => {
                    buffer = r#"// Ejemplo ADead-BIB
print("Hola desde el Playground!")
let x = 42
let y = 10
print("Calculando...")
"#
                    .to_string();
                    println!("📝 Ejemplo cargado. Usa :run para ejecutar o :ast para ver el AST");
                }
                ":vars" | ":v" => {
                    if variables.is_empty() {
                        println!("📦 No hay variables definidas");
                    } else {
                        println!("📦 Variables:");
                        for (name, value) in &variables {
                            println!("   {} = {}", name, value);
                        }
                    }
                }
                _ => {
                    println!(
                        "❓ Comando desconocido: {}. Usa :help para ver comandos.",
                        input
                    );
                }
            }
            continue;
        }

        // Si línea vacía y hay buffer, ejecutar
        if input.is_empty() && !buffer.is_empty() {
            execute_playground_code(&buffer, &mut variables);
            buffer.clear();
            line_number += 1;
            continue;
        }

        // Añadir al buffer
        if !input.is_empty() {
            buffer.push_str(input);
            buffer.push('\n');

            // Si es una línea simple (print, let, etc.), ejecutar inmediatamente
            if is_complete_statement(input) && !input.ends_with(':') && !input.ends_with('{') {
                execute_playground_code(&buffer, &mut variables);
                buffer.clear();
                line_number += 1;
            }
        }
    }

    Ok(())
}

fn print_playground_help() {
    println!();
    println!("🎮 ADead-BIB Playground - Ayuda");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!("📝 COMANDOS:");
    println!("   :help, :h      - Mostrar esta ayuda");
    println!("   :clear, :c     - Limpiar el buffer de código");
    println!("   :run, :r       - Ejecutar el código en el buffer");
    println!("   :ast, :a       - Mostrar el AST del código");
    println!("   :tokens, :t    - Mostrar los tokens del código");
    println!("   :vars, :v      - Mostrar variables definidas");
    println!("   :example, :e   - Cargar código de ejemplo");
    println!("   :exit, :q      - Salir del playground");
    println!();
    println!("💡 SINTAXIS SOPORTADA:");
    println!("   • print(\"texto\")     - Imprimir texto");
    println!("   • let x = 42         - Definir variable (Rust-style)");
    println!("   • x = 42             - Asignar variable (Python-style)");
    println!("   • fn nombre() {{ }}   - Definir función (Rust-style)");
    println!("   • def nombre():      - Definir función (Python-style)");
    println!();
    println!("🚀 EJEMPLOS:");
    println!("   print(\"Hola mundo!\")");
    println!("   let x = 10 + 5");
    println!("   fn saludar() {{ print(\"Hola\") }}");
    println!();
}

fn execute_playground_code(code: &str, _variables: &mut std::collections::HashMap<String, i64>) {
    println!();
    println!("▶️  Ejecutando...");
    println!("───────────────────────────────────────");

    // Parse y ejecutar
    match Parser::parse_program(code) {
        Ok(program) => {
            // Mostrar qué se parseó
            if !program.functions.is_empty() {
                println!("📦 Funciones definidas: {}", program.functions.len());
                for f in &program.functions {
                    println!("   • fn {}()", f.name);
                }
            }

            if !program.statements.is_empty() {
                println!("📝 Statements: {}", program.statements.len());

                // Simular ejecución de statements
                for stmt in &program.statements {
                    match stmt {
                        adead_bib::frontend::ast::Stmt::Print(expr) => match expr {
                            adead_bib::frontend::ast::Expr::String(s) => {
                                println!("   → {}", s);
                            }
                            adead_bib::frontend::ast::Expr::Number(n) => {
                                println!("   → {}", n);
                            }
                            adead_bib::frontend::ast::Expr::Variable(v) => {
                                println!("   → [var: {}]", v);
                            }
                            _ => {
                                println!("   → [expresión]");
                            }
                        },
                        adead_bib::frontend::ast::Stmt::Assign { name, value } => match value {
                            adead_bib::frontend::ast::Expr::Number(n) => {
                                println!("   {} = {}", name, n);
                            }
                            adead_bib::frontend::ast::Expr::String(s) => {
                                println!("   {} = \"{}\"", name, s);
                            }
                            _ => {
                                println!("   {} = [expresión]", name);
                            }
                        },
                        _ => {}
                    }
                }
            }

            println!("───────────────────────────────────────");
            println!("✅ Ejecución completada");
        }
        Err(e) => {
            println!("❌ Error de sintaxis: {}", e);
        }
    }
    println!();
}

fn show_ast(code: &str) {
    println!();
    println!("🌳 AST (Abstract Syntax Tree):");
    println!("───────────────────────────────────────");

    match Parser::parse_program(code) {
        Ok(program) => {
            println!("{:#?}", program);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    println!();
}

fn show_tokens(code: &str) {
    println!();
    println!("🔤 Tokens:");
    println!("───────────────────────────────────────");

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    for (i, token) in tokens.iter().enumerate() {
        println!("   [{}] {:?}", i, token);
    }
    println!();
}

fn is_complete_statement(line: &str) -> bool {
    let line = line.trim();

    // Statements simples que se pueden ejecutar inmediatamente
    line.starts_with("print(")
        || line.starts_with("let ")
        || line.starts_with("const ")
        || (line.contains('=')
            && !line.contains("==")
            && !line.starts_with("fn ")
            && !line.starts_with("def "))
}

/// Crear un nuevo proyecto ADead-BIB con estructura estándar
/// Uso: adB create <nombre> o adB new <nombre>
fn create_new_project(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::path::Path;

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           🚀 ADead-BIB v2.0 - Nuevo Proyecto                ║");
    println!("║           OOP Puro + ASM Simbionte                          ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let project_path = Path::new(name);

    if project_path.exists() {
        eprintln!("❌ Error: El directorio '{}' ya existe", name);
        std::process::exit(1);
    }

    // Crear estructura
    create_project_structure(name, name)?;

    // Mostrar resultado
    print_project_created(name);

    Ok(())
}

/// Inicializar proyecto en el directorio actual
/// Uso: adB init
fn create_new_project_in_place(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           � ADead-BIB v2.0 - Inicializar Proyecto          ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Crear estructura en directorio actual
    create_project_structure(".", name)?;

    // Mostrar resultado
    print_project_created(name);

    Ok(())
}

/// Crear la estructura de archivos del proyecto
fn create_project_structure(base_path: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📁 Creando estructura de proyecto...");
    println!();

    // Crear directorios
    fs::create_dir_all(format!("{}/core", base_path))?;
    fs::create_dir_all(format!("{}/cpu", base_path))?;
    fs::create_dir_all(format!("{}/gpu", base_path))?;

    // ========================================================================
    // main.adB - Punto de entrada FUNCIONAL
    // ========================================================================
    let main_content = format!(
        r#"// ============================================================================
// {} - ADead-BIB Project
// ============================================================================
// Creado con: adB create {}
// Ejecutar:   adB run main.adB
// ============================================================================

fn main() {{
    println("========================================")
    println("     {} - ADead-BIB")
    println("     Binary Is Binary")
    println("========================================")
    println("")
    
    // Tu código aquí
    println("Hello, {}!")
    println("")
    
    // Variables
    let x = 42
    let y = 10
    let result = x + y
    
    print("Resultado: ")
    println(result)
    println("")
    
    println("========================================")
    println("     Proyecto listo!")
    println("========================================")
}}
"#,
        name, name, name, name
    );
    fs::write(format!("{}/main.adB", base_path), &main_content)?;
    println!("   ✅ main.adB          (punto de entrada)");

    // ========================================================================
    // call.adB - Lógica OOP (para proyectos más complejos)
    // ========================================================================
    let call_content = format!(
        r#"// ============================================================================
// {} - Lógica OOP Pura
// ============================================================================
// Este archivo es para lógica más compleja con OOP
// Importar desde main.adB con: #![imports(call: run)]
// ============================================================================

#![exports(run, Player)]

// Ejemplo de struct
struct Player {{
    name: string,
    x: i32,
    y: i32,
    hp: i32
}}

impl Player {{
    fn new(name: string) {{
        return Player {{
            name: name,
            x: 0,
            y: 0,
            hp: 100
        }}
    }}
    
    fn move_to(self, dx: i32, dy: i32) {{
        self.x = self.x + dx
        self.y = self.y + dy
    }}
    
    fn info(self) {{
        print("Player: ")
        println(self.name)
        print("Position: (")
        print(self.x)
        print(", ")
        print(self.y)
        println(")")
        print("HP: ")
        println(self.hp)
    }}
}}

// Función exportada
pub fn run() {{
    println("=== OOP Demo ===")
    
    let player = Player::new("Hero")
    player.info()
    
    player.move_to(5, 3)
    println("Moved!")
    player.info()
    
    println("=== Done ===")
}}
"#,
        name
    );
    fs::write(format!("{}/call.adB", base_path), &call_content)?;
    println!("   ✅ call.adB          (lógica OOP)");

    // ========================================================================
    // core/mod.adB - Intrínsecos del sistema
    // ========================================================================
    let core_content = r#"// ============================================================================
// core/mod.adB - Intrínsecos del Sistema
// ============================================================================

#![exports(init, shutdown)]

pub fn init() {
    // Inicialización del sistema
}

pub fn shutdown() {
    // Limpieza del sistema
}
"#;
    fs::write(format!("{}/core/mod.adB", base_path), core_content)?;
    println!("   ✅ core/mod.adB      (sistema)");

    // ========================================================================
    // cpu/mod.adB - Instrucciones CPU directas
    // ========================================================================
    let cpu_content = r#"// ============================================================================
// cpu/mod.adB - Instrucciones CPU Directas (x86-64)
// ============================================================================
// Usa cpu::mov, cpu::add, etc. para instrucciones directas
// Ejemplo: cpu::mov(cpu::rax, 42)
// ============================================================================

#![exports(rax, rbx, rcx, rdx, rsi, rdi)]

// Registros x86-64
pub const rax: u8 = 0
pub const rcx: u8 = 1
pub const rdx: u8 = 2
pub const rbx: u8 = 3
pub const rsi: u8 = 6
pub const rdi: u8 = 7
"#;
    fs::write(format!("{}/cpu/mod.adB", base_path), cpu_content)?;
    println!("   ✅ cpu/mod.adB       (instrucciones CPU)");

    // ========================================================================
    // gpu/mod.adB - Opcodes GPU directos
    // ========================================================================
    let gpu_content = r#"// ============================================================================
// gpu/mod.adB - Opcodes GPU Directos
// ============================================================================
// Usa gpu::init, gpu::matmul, etc. para operaciones GPU
// Opcodes: 0xC0DA0001 (init), 0xC0DA0020 (matmul), etc.
// ============================================================================

#![exports(init, shutdown, sync)]

pub fn init() {
    // GPU init: 0xC0DA0001
}

pub fn shutdown() {
    // GPU shutdown: 0xC0DA0002
}

pub fn sync() {
    // GPU sync: 0xC0DA00F0
}
"#;
    fs::write(format!("{}/gpu/mod.adB", base_path), gpu_content)?;
    println!("   ✅ gpu/mod.adB       (opcodes GPU)");

    // ========================================================================
    // build.adB - Configuración de build
    // ========================================================================
    let build_content = format!(
        r#"// ============================================================================
// build.adB - Configuración de Build
// ============================================================================

#![project("{}")]
#![version("1.0.0")]
#![main("main.adB")]
#![output("{}.exe")]

// Opciones de compilación
#![optimize(true)]
#![target("windows")]  // windows, linux, raw
"#,
        name, name
    );
    fs::write(format!("{}/build.adB", base_path), &build_content)?;
    println!("   ✅ build.adB         (configuración)");

    // ========================================================================
    // README.md
    // ========================================================================
    let readme_content = format!(
        r#"# {}

Proyecto ADead-BIB - OOP Puro + ASM Simbionte

## Ejecutar

```bash
adB run main.adB
```

## Compilar

```bash
adB build main.adB
```

## Estructura

```
{}/
├── main.adB      # Punto de entrada (EMPIEZA AQUÍ)
├── call.adB      # Lógica OOP pura
├── core/         # Intrínsecos del sistema
├── cpu/          # Instrucciones CPU directas
├── gpu/          # Opcodes GPU directos
└── build.adB     # Configuración
```

## Filosofía

> **ADead-BIB no abstrae la máquina, la domestica.**

| Nivel | Descripción | Ejemplo |
|-------|-------------|---------|
| Normal | OOP puro | `player.move(1, 0)` |
| Avanzado | Módulos cpu/gpu | `cpu::mov(rax, 42)` |
| Peligroso | Bytes directos | `emit![0x48, 0x31, 0xC0]` |

---

**Código → Bytes → Binario**
"#,
        name, name
    );
    fs::write(format!("{}/README.md", base_path), &readme_content)?;
    println!("   ✅ README.md         (documentación)");

    Ok(())
}

/// Mostrar mensaje de proyecto creado
fn print_project_created(name: &str) {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    ✅ Proyecto Creado                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("📂 Proyecto: {}", name);
    println!();
    println!("🚀 Comandos:");
    println!("   cd {}              # Entrar al proyecto", name);
    println!("   adB run main.adB   # Ejecutar");
    println!("   adB build main.adB # Compilar");
    println!("   adB check main.adB # Verificar sintaxis");
    println!();
    println!("📝 Edita main.adB para empezar a programar");
    println!();
    println!("💡 Tip: Usa call.adB para lógica OOP más compleja");
    println!();
}
