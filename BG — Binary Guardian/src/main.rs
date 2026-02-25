// ============================================================
// BG — Binary Guardian: CLI Tool
// ============================================================
// Herramienta de línea de comandos para análisis de binarios.
//
// Uso:
//   bg analyze <binary> [--policy user|service|driver|kernel|sandbox]
//   bg inspect <binary>
//   bg check   <binary> --level user|service|driver|kernel
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================

use std::path::Path;
use std::env;
use bg::{BinaryGuardian, SecurityPolicy, SecurityLevel, BinaryLoader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        std::process::exit(1);
    }

    let command = &args[1];
    let target = &args[2];

    match command.as_str() {
        "analyze" => cmd_analyze(target, &args[3..]),
        "inspect" => cmd_inspect(target),
        "check" => cmd_check(target, &args[3..]),
        "info" => cmd_info(target),
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("═══════════════════════════════════════════════");
    eprintln!("  BG — Binary Guardian v0.1.0");
    eprintln!("  Deterministic ISA-Level Capability Guardian");
    eprintln!("═══════════════════════════════════════════════");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  bg analyze <binary> [--policy <level>]");
    eprintln!("  bg inspect <binary>");
    eprintln!("  bg check   <binary> [--level <level>]");
    eprintln!("  bg info    <binary>");
    eprintln!();
    eprintln!("Policies/Levels:");
    eprintln!("  kernel   Ring 0 — full hardware access");
    eprintln!("  driver   Ring 1 — IO + restricted ops");
    eprintln!("  service  Ring 2 — restricted, no direct IO");
    eprintln!("  user     Ring 3 — safe instructions only (default)");
    eprintln!("  sandbox  Ring 3 — strict, almost nothing allowed");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  bg analyze program.exe");
    eprintln!("  bg analyze kernel.bin --policy kernel");
    eprintln!("  bg check driver.sys --level driver");
    eprintln!("  bg inspect firmware.bin");
    eprintln!("  bg info program.exe");
}

/// Analiza un binario contra una policy y muestra el resultado completo.
fn cmd_analyze(target: &str, extra_args: &[String]) {
    let policy = parse_policy(extra_args, "--policy");

    let path = Path::new(target);
    match BinaryGuardian::analyze_file(path, &policy) {
        Ok(result) => {
            println!("{}", result);
            if result.verdict.is_denied() {
                std::process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Inspecciona un binario y muestra el Architecture Map sin policy check.
fn cmd_inspect(target: &str) {
    let path = Path::new(target);
    match BinaryLoader::load_file(path) {
        Ok(info) => {
            println!("{}", info);
            println!();

            let map = BinaryGuardian::inspect_bytes(&info.code_bytes);
            println!("{}", map);

            let level = bg::PolicyEngine::infer_minimum_level(&map);
            println!("  Inferred minimum level: {}", level);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Quick check: ¿puede ejecutarse al nivel dado?
fn cmd_check(target: &str, extra_args: &[String]) {
    let level = parse_level(extra_args, "--level");

    let path = Path::new(target);
    match BinaryLoader::load_file(path) {
        Ok(info) => {
            let can = BinaryGuardian::can_execute(&info.code_bytes, level);
            if can {
                println!("APPROVED — '{}' can execute at {}", target, level);
            } else {
                println!("DENIED — '{}' cannot execute at {}", target, level);
                std::process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Muestra información del binario (formato, secciones, etc.) sin análisis.
fn cmd_info(target: &str) {
    let path = Path::new(target);
    match BinaryLoader::load_file(path) {
        Ok(info) => {
            println!("{}", info);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_policy(args: &[String], flag: &str) -> SecurityPolicy {
    let level_str = find_flag_value(args, flag).unwrap_or_else(|| "user".to_string());
    match level_str.as_str() {
        "kernel" => SecurityPolicy::kernel(),
        "driver" => SecurityPolicy::driver(),
        "service" => SecurityPolicy::service(),
        "user" => SecurityPolicy::user(),
        "sandbox" => SecurityPolicy::sandbox(),
        other => {
            eprintln!("Unknown policy: '{}'. Using 'user'.", other);
            SecurityPolicy::user()
        }
    }
}

fn parse_level(args: &[String], flag: &str) -> SecurityLevel {
    let level_str = find_flag_value(args, flag).unwrap_or_else(|| "user".to_string());
    match level_str.as_str() {
        "kernel" => SecurityLevel::Kernel,
        "driver" => SecurityLevel::Driver,
        "service" => SecurityLevel::Service,
        "user" => SecurityLevel::User,
        other => {
            eprintln!("Unknown level: '{}'. Using 'user'.", other);
            SecurityLevel::User
        }
    }
}

fn find_flag_value(args: &[String], flag: &str) -> Option<String> {
    for (i, arg) in args.iter().enumerate() {
        if arg == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}
