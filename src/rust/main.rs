// ADead-BIB Compiler CLI
// Interfaz de línea de comandos robusta
// Soporta: build, run, check

use std::env;
use std::process::Command;
use std::path::Path;
use adead_bib::builder::{Builder, BuildOptions};
use adead_bib::backend::codegen_v2::Target;

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
            // TODO: Implement Syntax Check only
            println!("Check not implemented yet");
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
    println!("  {} build <file.adB> [-o output.exe]", program);
    println!("  {} run <file.adB>", program);
    println!("  {} <file.adB> (Default: build)", program);
}
