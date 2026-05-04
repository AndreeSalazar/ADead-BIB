//! adB CLI v12.0 — Pipeline Completo
//! source → Lexer → Parser → IR → Optimizer → UB → Codegen → PE/ELF/DLL/SO/FLAT
#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use adeb_compiler::frontend::lexer::Lexer;
use adeb_compiler::frontend::parser::Parser;
use adeb_compiler::middle::ast_to_ir::ast_to_ir;
use adeb_compiler::middle::optimizer::Optimizer;
use adeb_compiler::middle::ub_detector::UbDetector;
use adeb_compiler::backend::codegen::Codegen;
use adeb_compiler::backend::pe::PeBuilder;
use adeb_compiler::backend::elf::ElfBuilder;

const VERSION: &str = "12.0.0";

#[derive(Default, Clone)]
struct Options {
    input: String,
    output: String,
    target: Target,
    strict: bool,
    step: bool,
}

#[derive(Default, Clone, Copy, PartialEq)]
enum Target {
    #[default]
    PeExe,
    PeDll,
    Elf,
    ElfSo,
    Flat,
    AdebOs,  // bare-metal target para futuro OS Rust
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { print_usage(); process::exit(1); }

    match args[1].as_str() {
        "cc"      => cmd_compile_c(&args[2..]),
        "cxx"     => cmd_compile_c(&args[2..]),  // C++ comparte pipeline en v12
        "run"     => cmd_run(&args[2..]),
        "step"    => cmd_step(&args[2..]),
        "create"  => cmd_create(&args[2..]),
        "build"   => cmd_build(&args[2..]),
        "gpu"     => cmd_gpu(&args[2..]),
        "spirv"   => cmd_spirv(&args[2..]),
        "version" | "-v" | "--version" => cmd_version(),
        arg if arg.ends_with(".c") => cmd_compile_c(&args[1..]),
        _ => { print_usage(); process::exit(1); }
    }
}

fn parse_options(args: &[String]) -> Options {
    if args.is_empty() { eprintln!("Error: No input"); process::exit(1); }

    let mut opts = Options::default();
    opts.input = args[0].clone();
    opts.output = args[0].replace(".c", ".exe");

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" if i + 1 < args.len() => { opts.output = args[i+1].clone(); i += 2; }
            "-Wstrict"     => { opts.strict = true; i += 1; }
            "-step"        => { opts.step = true; i += 1; }
            "--dll"        => { opts.target = Target::PeDll;  opts.output = opts.output.replace(".exe", ".dll");  i += 1; }
            "--so"         => { opts.target = Target::ElfSo;  opts.output = opts.output.replace(".exe", ".so");   i += 1; }
            "--elf"        => { opts.target = Target::Elf;    opts.output = opts.output.replace(".exe", ".elf");  i += 1; }
            "--flat"       => { opts.target = Target::Flat;   opts.output = opts.output.replace(".exe", ".bin");  i += 1; }
            "--target" if i + 1 < args.len() => {
                opts.target = match args[i+1].as_str() {
                    "pe" | "pe-exe" => Target::PeExe,
                    "pe-dll"        => Target::PeDll,
                    "elf"           => Target::Elf,
                    "elf-so"        => Target::ElfSo,
                    "flat"          => Target::Flat,
                    "adeb-os"       => Target::AdebOs,
                    "fastos256"     => Target::Flat,
                    other => { eprintln!("Unknown target: {}", other); process::exit(1); }
                };
                i += 2;
            }
            _ => i += 1,
        }
    }
    opts
}

fn cmd_compile_c(args: &[String]) {
    let opts = parse_options(args);
    println!("\x1b[36m[adB] C Compiler v{}\x1b[0m", VERSION);
    println!("  {} → {}", opts.input, opts.output);

    if !Path::new(&opts.input).exists() {
        eprintln!("Error: '{}' not found", opts.input);
        process::exit(1);
    }
    compile_pipeline(&opts);
}

fn compile_pipeline(opts: &Options) {
    // [1] SOURCE
    print!("  [1/7] Source... ");
    let source = fs::read_to_string(&opts.input).expect("read failed");
    println!("{} bytes", source.len());

    // [2] LEXER
    print!("  [2/7] Lexer... ");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    println!("{} tokens", tokens.len());
    if opts.step { for t in &tokens { eprintln!("    tok: {:?}", t); } }

    // [3] PARSER
    print!("  [3/7] Parser... ");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("parse failed");
    println!("{} items", ast.items.len());
    if opts.step { eprintln!("    AST: {} top-level items", ast.items.len()); }

    // [4] AST → IR
    print!("  [4/7] IR... ");
    let mut ir = ast_to_ir(&ast);
    println!("{} functions", ir.functions.len());

    // [5] OPTIMIZER
    print!("  [5/7] Optimizer... ");
    let mut opt = Optimizer::new();
    opt.run(&mut ir);
    println!("optimized");

    // [6] UB DETECTOR
    print!("  [6/7] UB Check... ");
    let mut ub = UbDetector::new();
    ub.analyze(&ir);
    if ub.reports.is_empty() {
        println!("clean");
    } else {
        println!("{} warnings", ub.reports.len());
        for issue in &ub.reports {
            eprintln!("    ⚠ {:?}", issue);
        }
        if opts.strict {
            eprintln!("\x1b[31m  FAIL: -Wstrict promotes UB warnings to errors\x1b[0m");
            process::exit(2);
        }
    }

    // [7] CODEGEN + WRAP
    print!("  [7/7] Codegen... ");
    let mut codegen = Codegen::new();
    let code = codegen.generate(&ir);

    let bin: Vec<u8> = match opts.target {
        Target::PeExe | Target::PeDll => {
            let mut pe = PeBuilder::new().console();
            pe.code = code;
            pe.data = codegen.data_section;
            pe.build()
        }
        Target::Elf | Target::ElfSo => {
            let mut elf = ElfBuilder::new();
            elf.code = code;
            elf.data = codegen.data_section;
            if opts.target == Target::ElfSo { elf = elf.pie(); }
            elf.build()
        }
        Target::Flat | Target::AdebOs => {
            // Bare-metal: solo el code, sin headers. Entry en offset 0.
            // Para el OS Rust del usuario.
            let mut bin = code.clone();
            bin.extend_from_slice(&codegen.data_section);
            bin
        }
    };

    fs::write(&opts.output, &bin).expect("write failed");
    println!("{} bytes", bin.len());
    println!("\x1b[32m  ✓ {}\x1b[0m", opts.output);
}

fn cmd_run(args: &[String]) {
    let mut opts = parse_options(args);
    opts.output = "temp_run.exe".to_string();
    opts.target = Target::PeExe;
    compile_pipeline(&opts);

    println!("\n\x1b[32m[Running]\x1b[0m {}\n", opts.output);
    let status = process::Command::new(&opts.output).status().expect("run failed");
    let _ = fs::remove_file(&opts.output);
    process::exit(status.code().unwrap_or(1));
}

fn cmd_step(args: &[String]) {
    let mut opts = parse_options(args);
    opts.step = true;
    println!("\x1b[36m[adB] Step Compiler v{}\x1b[0m", VERSION);
    compile_pipeline(&opts);
}

fn cmd_create(args: &[String]) {
    if args.is_empty() { eprintln!("Usage: adB create <project_name>"); process::exit(1); }
    let name = &args[0];
    let project_dir = Path::new(name);
    if project_dir.exists() {
        eprintln!("Error: '{}' already exists", name);
        process::exit(1);
    }
    fs::create_dir(project_dir).expect("create dir failed");
    fs::create_dir(project_dir.join("src")).expect("create src failed");
    fs::create_dir(project_dir.join("bin")).expect("create bin failed");
    let main_c = format!("// {} — generated by adB v{}\nint main() {{\n    return 0;\n}}\n", name, VERSION);
    fs::write(project_dir.join("src/main.c"), main_c).expect("write main.c failed");
    let toml = format!("[project]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"src/main.c\"\noutput = \"bin/{}.exe\"\n", name, name);
    fs::write(project_dir.join("adb.toml"), toml).expect("write adb.toml failed");
    println!("\x1b[32m  ✓ Created project: {}\x1b[0m", name);
    println!("    cd {} && adB run", name);
}

fn cmd_build(_args: &[String]) {
    let toml_path = Path::new("adb.toml");
    if !toml_path.exists() {
        eprintln!("Error: adb.toml not found in current directory");
        process::exit(1);
    }
    let toml = fs::read_to_string(toml_path).expect("read toml failed");
    let entry = extract_toml_value(&toml, "entry").unwrap_or_else(|| "src/main.c".into());
    let output = extract_toml_value(&toml, "output").unwrap_or_else(|| "bin/out.exe".into());
    if let Some(parent) = Path::new(&output).parent() { let _ = fs::create_dir_all(parent); }
    let opts = Options { input: entry, output, ..Default::default() };
    compile_pipeline(&opts);
}

fn extract_toml_value(toml: &str, key: &str) -> Option<String> {
    for line in toml.lines() {
        let line = line.trim();
        if line.starts_with(key) {
            if let Some(eq) = line.find('=') {
                let val = line[eq+1..].trim().trim_matches('"').to_string();
                return Some(val);
            }
        }
    }
    None
}

fn cmd_gpu(_args: &[String]) {
    println!("\x1b[36m[adB GPU]\x1b[0m v{}", VERSION);
    println!("  GPU detection: stub (full impl pending)");
    println!("  Vulkan: vulkan-1.dll");
    println!("  OpenGL: opengl32.dll (1.1) / wgl extensions (4.6)");
    println!("  DirectX: d3d12.dll, d3d11.dll, dxgi.dll");
}

fn cmd_spirv(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: adB spirv <kernel> [size]");
        process::exit(1);
    }
    let kernel = &args[0];
    let size = args.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(256);
    println!("\x1b[36m[adB SPIR-V]\x1b[0m {} (size {})", kernel, size);
    println!("  SPIR-V codegen: stub (full impl pending)");
}

fn cmd_version() {
    println!("\x1b[33m");
    println!("    █████╗ ██████╗ ███████╗ █████╗ ██████╗ ██████╗ ██╗██████╗");
    println!("   ██╔══██╗██╔══██╗██╔════╝██╔══██╗██╔══██╗██╔══██╗██║██╔══██╗");
    println!("   ███████║██║  ██║█████╗  ███████║██║  ██║██████╔╝██║██████╔╝");
    println!("   ██╔══██║██║  ██║██╔══╝  ██╔══██║██║  ██║██╔══██╗██║██╔══██╗");
    println!("   ██║  ██║██████╔╝███████╗██║  ██║██████╔╝██████╔╝██║██████╔╝");
    println!("   ╚═╝  ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═╝╚═════╝ ╚═════╝ ╚═╝╚═════╝");
    println!("\x1b[0m");
    println!("  ADead-BIB Compiler v{} 💀🦈 🇵🇪", VERSION);
    println!("  Pipeline: Lexer → Parser → IR → Opt → UB → Codegen → PE/ELF/Flat");
    println!("  Targets: pe-exe, pe-dll, elf, elf-so, flat, adeb-os");
}

fn print_usage() {
    println!("\x1b[36mADead-BIB Compiler v{}\x1b[0m", VERSION);
    println!("");
    println!("USAGE:");
    println!("  adB cc <file.c> [-o out] [--dll|--so|--elf|--flat] [-Wstrict] [-step]");
    println!("  adB cxx <file.cpp> [-o out]");
    println!("  adB run <file.c>            Compile and run");
    println!("  adB step <file.c>           Show pipeline stages");
    println!("  adB create <project>        Generate new C project");
    println!("  adB build                   Build project from adb.toml");
    println!("  adB gpu                     Detect GPU and generate shader");
    println!("  adB spirv <kernel> [size]   Generate SPIR-V compute shader");
    println!("  adB version                 Show ASCII banner and version");
    println!("");
    println!("TARGETS:");
    println!("  --target pe-exe     Windows PE executable (default)");
    println!("  --target pe-dll     Windows DLL");
    println!("  --target elf        Linux ELF executable");
    println!("  --target elf-so     Linux shared object (.so)");
    println!("  --target flat       Flat binary (no headers)");
    println!("  --target adeb-os    Bare-metal binary for ADead-OS Rust kernel");
}
