// ============================================================
// PE Translator — ADead Universal Binary Backend CLI
// ============================================================
// Usage:
//   pe-translator <input.bib> [options]
//
// Options:
//   -o <output>       Output file path (default: output.exe)
//   -t <target>       Target: pe, elf, fastos (default: pe)
//   -s <subsystem>    Subsystem: console, gui, native (default: console)
//   --base <addr>     Image base address (hex, default: 0x140000000)
//   --info            Show BIB module info and exit
//   --demo            Generate a demo BIB and translate it
//
// Examples:
//   pe-translator program.bib -o program.exe -t pe -s gui
//   pe-translator program.bib -o program -t elf
//   pe-translator --demo -o demo.exe
//   pe-translator program.bib --info
// ============================================================

use pe_translator::bib;
use pe_translator::targets;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    // Parse arguments
    let mut input_path: Option<String> = None;
    let mut output_path = String::from("output.exe");
    let mut target = String::from("pe");
    let mut subsystem = String::from("console");
    let mut image_base: Option<u64> = None;
    let mut show_info = false;
    let mut demo_mode = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                i += 1;
                if i < args.len() { output_path = args[i].clone(); }
            }
            "-t" => {
                i += 1;
                if i < args.len() { target = args[i].clone(); }
            }
            "-s" => {
                i += 1;
                if i < args.len() { subsystem = args[i].clone(); }
            }
            "--base" => {
                i += 1;
                if i < args.len() {
                    let s = args[i].trim_start_matches("0x").trim_start_matches("0X");
                    image_base = u64::from_str_radix(s, 16).ok();
                }
            }
            "--info" => { show_info = true; }
            "--demo" => { demo_mode = true; }
            "-h" | "--help" => {
                print_usage();
                return;
            }
            other => {
                if input_path.is_none() && !other.starts_with('-') {
                    input_path = Some(other.to_string());
                } else {
                    eprintln!("Unknown argument: {}", other);
                    std::process::exit(1);
                }
            }
        }
        i += 1;
    }

    // ============================================================
    // Demo mode: generate a sample BIB and translate
    // ============================================================

    if demo_mode {
        println!("=== ADead PE Translator — Demo Mode ===");
        run_demo(&output_path, &target, &subsystem, image_base);
        return;
    }

    // ============================================================
    // Normal mode: read BIB file and translate
    // ============================================================

    let input = match input_path {
        Some(p) => p,
        None => {
            eprintln!("Error: No input file specified. Use --demo for a demo.");
            std::process::exit(1);
        }
    };

    println!("=== ADead PE Translator v1.0 ===");
    println!("  Input:  {}", input);

    let module = match bib::reader::read_file(&input) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error reading BIB: {}", e);
            std::process::exit(1);
        }
    };

    if show_info {
        println!("{}", module);
        return;
    }

    // Build config
    let format = match target.as_str() {
        "pe" => targets::OutputFormat::PeExe,
        "dll" => targets::OutputFormat::PeDll,
        "elf" => targets::OutputFormat::ElfExe,
        "so" => targets::OutputFormat::ElfSo,
        "fastos" => targets::OutputFormat::FsOS,
        _ => {
            eprintln!("Unknown target: {}. Use: pe, dll, elf, so, fastos", target);
            std::process::exit(1);
        }
    };

    let sub = match subsystem.as_str() {
        "console" => targets::Subsystem::Console,
        "gui" => targets::Subsystem::Gui,
        "native" => targets::Subsystem::Native,
        _ => {
            eprintln!("Unknown subsystem: {}. Use: console, gui, native", subsystem);
            std::process::exit(1);
        }
    };

    let mut config = targets::BackendConfig {
        format,
        subsystem: sub,
        output_path: output_path.clone(),
        ..targets::BackendConfig::default()
    };

    if let Some(base) = image_base {
        config.image_base = base;
    }

    // Select backend and translate
    let backend = targets::select_backend(format);
    println!("  Target: {} ({})", backend.name(), target);
    println!("  Output: {}", output_path);

    match backend.write(&module, &config) {
        Ok(()) => {
            let size = std::fs::metadata(&output_path)
                .map(|m| m.len())
                .unwrap_or(0);
            println!("  Size:   {} bytes", size);
            println!("  Status: OK — Binary generated successfully!");
        }
        Err(e) => {
            eprintln!("  Error:  {}", e);
            std::process::exit(1);
        }
    }
}

// ============================================================
// Demo: generate a "Hello World" PE from scratch
// ============================================================

fn run_demo(output_path: &str, target: &str, subsystem: &str, image_base: Option<u64>) {
    use bib::format::*;
    use bib::builder::BibBuilder;

    // x86-64 machine code for a minimal program that calls ExitProcess(0)
    // This is a real, working Windows program:
    //   sub rsp, 0x28       ; shadow space
    //   xor ecx, ecx        ; exit code = 0
    //   call [rip + ???]     ; ExitProcess (will be resolved by IAT)
    //   int3                 ; trap
    //
    // For demo, we generate a simple "return 42" program:
    //   sub rsp, 0x28
    //   mov ecx, 0           ; ExitProcess(0)
    //   FF 15 xx xx xx xx    ; call [rip + IAT_offset] — patched by loader
    //   int3
    //
    // Since we can't know the IAT offset at BIB level, we emit a simpler
    // program that just returns via ret (the runtime stub handles exit):

    let code: Vec<u8> = vec![
        0x48, 0x83, 0xEC, 0x28,   // sub rsp, 0x28
        0xB8, 0x2A, 0x00, 0x00, 0x00, // mov eax, 42
        0x48, 0x83, 0xC4, 0x28,   // add rsp, 0x28
        0xC3,                       // ret
    ];

    let rodata: Vec<u8> = b"Hello from ADead-BIB!\0".to_vec();

    let module = BibBuilder::new(Arch::X86_64)
        .code(&code)
        .rodata(&rodata)
        .function("main", 0, code.len() as u64)
        .import("kernel32.dll", "ExitProcess", 0)
        .import("kernel32.dll", "GetStdHandle", 0)
        .import("kernel32.dll", "WriteConsoleA", 0)
        .import("msvcrt.dll", "printf", 0)
        .entry("main")
        .meta("compiler", "ADead-BIB v1.0")
        .meta("source", "demo")
        .build();

    println!("{}", module);

    // Save BIB file
    let bib_path = output_path.replace(".exe", ".bib")
        .replace(".elf", ".bib")
        .replace(".fsos", ".bib");
    match bib::writer::write_file(&module, &bib_path) {
        Ok(()) => println!("  BIB saved: {} ({} bytes)",
            bib_path, std::fs::metadata(&bib_path).map(|m| m.len()).unwrap_or(0)),
        Err(e) => eprintln!("  Warning: Could not save BIB: {}", e),
    }

    // Translate to target
    let format = match target {
        "elf" => targets::OutputFormat::ElfExe,
        "fastos" => targets::OutputFormat::FsOS,
        _ => targets::OutputFormat::PeExe,
    };

    let sub = match subsystem {
        "gui" => targets::Subsystem::Gui,
        "native" => targets::Subsystem::Native,
        _ => targets::Subsystem::Console,
    };

    let mut config = targets::BackendConfig {
        format,
        subsystem: sub,
        output_path: output_path.to_string(),
        ..targets::BackendConfig::default()
    };

    if let Some(base) = image_base {
        config.image_base = base;
    }

    let backend = targets::select_backend(format);
    println!("  Translating to {} ...", backend.name());

    match backend.write(&module, &config) {
        Ok(()) => {
            let size = std::fs::metadata(output_path)
                .map(|m| m.len())
                .unwrap_or(0);
            println!("  Output: {} ({} bytes)", output_path, size);
            println!("  Demo complete!");
        }
        Err(e) => {
            eprintln!("  Translation error: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("=== ADead PE Translator v1.0 ===");
    println!("Universal Binary Backend — ADead-BIB → PE/ELF/FsOS");
    println!();
    println!("Usage:");
    println!("  pe-translator <input.bib> [options]");
    println!();
    println!("Options:");
    println!("  -o <output>       Output file path (default: output.exe)");
    println!("  -t <target>       Target: pe, dll, elf, so, fastos (default: pe)");
    println!("  -s <subsystem>    Subsystem: console, gui, native (default: console)");
    println!("  --base <addr>     Image base address (hex)");
    println!("  --info            Show BIB module info and exit");
    println!("  --demo            Generate demo BIB and translate");
    println!("  -h, --help        Show this help");
    println!();
    println!("Examples:");
    println!("  pe-translator program.bib -o program.exe");
    println!("  pe-translator program.bib -o program -t elf");
    println!("  pe-translator program.bib -o app.fsos -t fastos");
    println!("  pe-translator --demo -o demo.exe");
    println!();
    println!("Pipeline:");
    println!("  ADead Source → Compiler → ADead-BIB → PE Translator → Native Binary");
}
