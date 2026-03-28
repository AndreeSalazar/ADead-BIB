use adeb_backend_x64::isa::isa_compiler::{IsaCompiler, Target};
use adeb_frontend_c::compile_c_to_program;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let code = match real_main(&args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            ExitCode::FAILURE
        }
    };

    if code == ExitCode::SUCCESS {
        println!("✅ Done (exit=0)");
    } else {
        eprintln!("⚠️  Done (exit!=0)");
    }
    code
}

fn real_main(args: &[String]) -> Result<ExitCode, Box<dyn std::error::Error>> {
    if args.len() < 2 {
        print_usage(&args[0]);
        return Ok(ExitCode::from(2));
    }

    match args[1].as_str() {
        "help" | "--help" | "-h" => {
            print_usage(&args[0]);
            Ok(ExitCode::SUCCESS)
        }
        "cc" | "c" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing C source file");
                eprintln!("   Usage: adb cc <file.c> [-o output.exe]");
                return Ok(ExitCode::from(2));
            }
            compile_c_file(&args[2], args)?;
            Ok(ExitCode::SUCCESS)
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Missing source file");
                eprintln!("   Usage: adb run <file.c> [-o output.exe]");
                return Ok(ExitCode::from(2));
            }
            let input = &args[2];
            compile_c_file(input, args)?;
            let output = get_output_filename(input, &args);
            let exe_path = if cfg!(target_os = "windows") {
                format!(".\\{}", output)
            } else {
                format!("./{}", output)
            };
            let status = Command::new(&exe_path).status()?;
            if status.success() {
                Ok(ExitCode::SUCCESS)
            } else {
                eprintln!("⚠️  Program exited with status: {}", status);
                let code = status.code().unwrap_or(1);
                let code_u8 = if (0..=255).contains(&code) {
                    code as u8
                } else {
                    1
                };
                Ok(ExitCode::from(code_u8))
            }
        }
        _ => {
            print_usage(&args[0]);
            Ok(ExitCode::from(2))
        }
    }
}

fn print_usage(bin: &str) {
    println!("Usage:");
    println!("  {} cc <file.c> [-o output.exe]", bin);
    println!("  {} run <file.c> [-o output.exe]", bin);
}

fn get_output_filename(input_file: &str, args: &[String]) -> String {
    if let Some(pos) = args.iter().position(|a| a == "-o") {
        if let Some(out) = args.get(pos + 1) {
            return out.clone();
        }
    }
    let base = Path::new(input_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("a");
    if cfg!(target_os = "windows") {
        format!("{}.exe", base)
    } else {
        base.to_string()
    }
}

fn compile_c_file(input_file: &str, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let output_file = get_output_filename(input_file, args);
    println!("🔨 ADead-BIB C Compiler");
    println!("   Source: {}", input_file);
    println!("   Target: {}", output_file);

    let source = fs::read_to_string(input_file)
        .map_err(|e| format!("Cannot read '{}': {}", input_file, e))?;

    println!("   Step 1: Parsing C...");
    let program = compile_c_to_program(&source).map_err(|e| format!("C parse error: {}", e))?;

    println!("   Step 2: Compiling to native code...");
    let mut compiler = IsaCompiler::new(Target::Windows);
    let (code, data, iat_offsets, string_offsets) = compiler.compile(&program);

    println!("   Step 3: Generating binary...");
    adeb_backend_x64::pe::generate_pe_with_offsets(
        &code,
        &data,
        &output_file,
        &iat_offsets,
        &string_offsets,
    )?;

    let meta = fs::metadata(&output_file)
        .map_err(|e| format!("Post-build: cannot stat output '{}': {}", output_file, e))?;
    if meta.len() == 0 {
        return Err(format!("Post-build: output '{}' is empty", output_file).into());
    }
    if cfg!(target_os = "windows") {
        let bytes = fs::read(&output_file)
            .map_err(|e| format!("Post-build: cannot read output '{}': {}", output_file, e))?;
        if bytes.len() < 2 || &bytes[0..2] != b"MZ" {
            return Err(format!("Post-build: output '{}' is not a PE (missing MZ)", output_file)
                .into());
        }
    }

    println!("✅ Build complete: {} ({} bytes)", output_file, meta.len());
    println!("✅ Post-build validation OK");
    Ok(())
}
