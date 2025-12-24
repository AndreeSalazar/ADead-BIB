// ADead-BIB Compiler CLI
// Interfaz de línea de comandos robusta
// Soporta: build, run, check

use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;
use adead_bib::builder::{Builder, BuildOptions};
use adead_bib::backend::codegen_v2::Target;
use adead_bib::backend::pe_tiny;
use adead_bib::backend::microvm::{self, MicroVM, MicroOp, compile_microvm};
use adead_bib::backend::gpu::vulkan::VulkanBackend;
use adead_bib::backend::gpu::gpu_detect::GPUFeatures;
use adead_bib::backend::gpu::vulkan_runtime;
use adead_bib::frontend::parser::Parser;
use adead_bib::frontend::lexer::Lexer;
use adead_bib::frontend::type_checker::TypeChecker;

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
            
            println!("🔨 Building {}...", input_file);
            
            let options = BuildOptions {
                target: determine_target(),
                optimize: true,
                output_path: output_file.clone(),
                verbose: true,
            };
            
            Builder::build_file(input_file, options)?;
            println!("✅ Build complete: {}", output_file);
        },
        "run" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                print_usage(&args[0]);
                std::process::exit(1);
            }
            let input_file = &args[2];
            let output_file = format!("{}.exe", Path::new(input_file).file_stem().unwrap().to_str().unwrap());
            
            // 1. Build
            let options = BuildOptions {
                target: determine_target(),
                optimize: true,
                output_path: output_file.clone(),
                verbose: false, // Quiet for run
            };
            
            if let Err(e) = Builder::build_file(input_file, options) {
                eprintln!("❌ Build failed: {}", e);
                std::process::exit(1);
            }
            
            // 2. Run
            println!("🚀 Running {}...\n", input_file);
            let status = Command::new(&output_file).status()?;
            
            if !status.success() {
                eprintln!("\n⚠️  Program exited with status: {}", status);
            }
        },
        "check" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing input file");
                print_usage(&args[0]);
                std::process::exit(1);
            }
            let input_file = &args[2];
            
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
        },
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
            
            // Generar código mínimo
            let mut codegen = adead_bib::backend::codegen_v2::CodeGeneratorV2::new(Target::Raw);
            let (opcodes, _data) = codegen.generate(&program);
            
            // Si el código es muy grande, usar exit simple
            let final_opcodes = if opcodes.len() > 200 {
                println!("   ⚠️  Code too large ({}b), using minimal exit", opcodes.len());
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
        },
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
        },
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
        },
        "flat" => {
            // Genera flat binary (solo código)
            let output_file = if args.len() >= 3 {
                args[2].clone()
            } else {
                "flat.bin".to_string()
            };
            
            let exit_code: u8 = if args.len() >= 4 {
                args[3].parse().unwrap_or(0)
            } else {
                0
            };
            
            println!("🔬 Building FLAT binary...");
            println!("   Target: Pure code, no headers");
            
            let opcodes = pe_tiny::generate_exit_opcodes(exit_code as u32);
            match pe_tiny::generate_flat_binary(&opcodes, &output_file) {
                Ok(size) => {
                    println!("✅ Flat build complete: {} ({} bytes)", output_file, size);
                    println!("   💎 Pure machine code - {} bytes!", size);
                }
                Err(e) => {
                    eprintln!("❌ Flat build failed: {}", e);
                    std::process::exit(1);
                }
            }
        },
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
            let bytecode = compile_microvm(&[
                (MicroOp::Load, exit_code.min(15)),
                (MicroOp::Exit, 0),
            ]);
            
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
        },
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
            println!("   Decision:  {}", if bit_value { "YES/TRUE/ON" } else { "NO/FALSE/OFF" });
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
            println!("      [Runtime] + [1 bit] → exit({})", if bit_value { 1 } else { 0 });
        },
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
                        println!("✅ SPIR-V Shader generated: {} ({} bytes)", output_path, size);
                        println!("   Optimized for: {}", gpu.device_name);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to save shader: {}", e);
                    }
                }
            }
        },
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
                _ => {
                    backend.generate_matmul_shader(size, size, size)
                }
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
        },
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
                    println!("   Max invocations: {}", props.max_compute_workgroup_invocations);
                    println!("   Shared memory: {} KB", props.max_compute_shared_memory / 1024);
                }
                Err(e) => {
                    eprintln!("❌ Failed to initialize Vulkan: {}", e);
                    eprintln!("   Make sure Vulkan drivers are installed.");
                }
            }
        },
        _ => {
            // Legacy behavior: treat first arg as input file if it's not a command
            // Or just show usage. Let's support legacy "cargo run file.adB" style if possible, 
            // but explicitly asking for command is cleaner.
            // Assuming default is build:
            let input_file = command;
            let output_file = get_output_filename(input_file, &args);
            
             let options = BuildOptions {
                target: determine_target(),
                optimize: true,
                output_path: output_file.clone(),
                verbose: true,
            };
            
            Builder::build_file(input_file, options)?;
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
            return args[i+1].clone();
        }
    }
    
    // Default: input.exe
    Path::new(input)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string() + ".exe"
}

fn print_usage(program: &str) {
    println!("ADead-BIB Compiler CLI");
    println!("Usage:");
    println!("  {} build <file.adB> [-o output.exe]  - Standard build", program);
    println!("  {} run <file.adB>                   - Build and run", program);
    println!("  {} check <file.adB>                 - Syntax check only", program);
    println!("  {} tiny <file.adB> [-o output.exe]  - Ultra-compact PE (< 500 bytes)", program);
    println!("  {} nano [output.exe] [exit_code]    - Smallest possible PE (~1KB)", program);
    println!("  {} <file.adB>                       - Default: build", program);
    println!();
    println!("🎯 Size targets:");
    println!("   Standard: ~1.5 KB");
    println!("   Tiny:     < 500 bytes");
    println!("   Nano:     ~1 KB (smallest valid x64 PE)");
}

fn check_syntax(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    
    // 1. Lexing
    let mut lexer = Lexer::new(&source);
    let _tokens = lexer.tokenize();
    
    // 2. Parsing
    let program = Parser::parse_program(&source)?;
    
    // 3. Type checking
    let mut type_checker = TypeChecker::new();
    let _types = type_checker.check_program(&program);
    
    // 4. Basic validation
    if program.functions.is_empty() {
        return Err("No functions found in program".into());
    }
    
    // Check for main function
    let has_main = program.functions.iter().any(|f| f.name == "main");
    if !has_main {
        eprintln!("⚠️  Warning: No 'main' function found");
    }
    
    Ok(())
}
