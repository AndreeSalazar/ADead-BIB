mod cli;
mod driver;

use crate::cli::term;
use crate::driver::c_driver;
use crate::driver::cpp_driver;
use crate::driver::cuda_driver;
use crate::driver::js_driver;
use std::env;
use std::path::Path;
use std::process::Command;
use std::process::ExitCode;

// ============================================================
// ADead-BIB — Unified Multi-Language Compiler CLI
// ============================================================
//   adB cc   <file.c>   [-o out] [-step]   C99/C11
//   adB cxx  <file.cpp> [-o out] [-step]   C++17/20
//   adB cuda <file.cu>  [-o out] [-step]   CUDA/PTX
//   adB js   <file.js>  [-o out] [-step]   JavaScript
//   adB run  <file.c>   [-o out] [-step]   Compile + Run
//   adB step <file.c>   [-o out]           Step mode (all phases)
// ============================================================

const VERSION: &str = "9.0";

fn main() -> ExitCode {
    term::enable_ansi();
    let args: Vec<String> = env::args().collect();
    let code = match real_main(&args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("  {}", term::error_text(&format!("Error: {}", e)));
            ExitCode::FAILURE
        }
    };

    if code == ExitCode::SUCCESS {
        println!("  {}", term::ok("Done (exit=0)"));
    } else {
        eprintln!("  {}", term::error_text("Done (exit!=0)"));
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

        // ── C Compiler ──────────────────────────────────
        "cc" | "c" => {
            let request = parse_request(args, Language::C)?;
            c_driver::compile_c_file(&request.input_file, &request.output_file, request.step_mode, request.strict)?;
            Ok(ExitCode::SUCCESS)
        }

        // ── C++ Compiler ────────────────────────────────
        "cxx" | "c++" | "cpp" => {
            let request = parse_request(args, Language::Cpp)?;
            cpp_driver::compile_cpp_file(&request.input_file, &request.output_file, request.step_mode, request.strict)?;
            Ok(ExitCode::SUCCESS)
        }

        // ── CUDA Compiler ───────────────────────────────
        "cuda" | "cu" => {
            let request = parse_request(args, Language::Cuda)?;
            cuda_driver::compile_cuda_file(&request.input_file, &request.output_file, request.step_mode)?;
            Ok(ExitCode::SUCCESS)
        }

        // ── JavaScript Compiler ─────────────────────────
        "js" | "javascript" => {
            let request = parse_request(args, Language::Js)?;
            js_driver::compile_js_file(&request.input_file, &request.output_file, request.step_mode)?;
            Ok(ExitCode::SUCCESS)
        }

        // ── Compile + Run (auto-detect language) ────────
        "run" => {
            let request = parse_request(args, Language::Auto)?;
            compile_by_language(&request)?;
            run_executable(&request.output_file)
        }

        // ── Step Mode (auto-detect language) ────────────
        "step" => {
            let mut request = parse_request_step(args)?;
            request.step_mode = true;
            compile_by_language(&request)?;
            Ok(ExitCode::SUCCESS)
        }

        // ── Version ─────────────────────────────────────
        "version" | "--version" | "-v" => {
            println!("{}", term::banner("Multi-Language", VERSION));
            println!("  Languages: C (complete), C++ (complete), CUDA (preview), JS (preview)");
            println!("  Target:    Windows PE x86-64");
            println!("  Backend:   ADead-BIB native (no LLVM, no GCC)");
            Ok(ExitCode::SUCCESS)
        }

        _ => {
            // Try auto-detection: if arg looks like a source file, infer and compile
            let first = &args[1];
            let lang = detect_language(first);
            if lang != Language::Auto || first.ends_with(".c") || first.ends_with(".h") {
                let request = CompileRequest {
                    input_file: first.clone(),
                    output_file: default_output_filename(first),
                    step_mode: args.iter().any(|a| a == "-step" || a == "--step"),
                    strict: args.iter().any(|a| a == "-Wstrict" || a == "--strict"),
                };
                compile_by_language(&request)?;
                Ok(ExitCode::SUCCESS)
            } else {
                print_usage(&args[0]);
                Ok(ExitCode::from(2))
            }
        }
    }
}

// ── CLI Types ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    C,
    Cpp,
    Cuda,
    Js,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompileRequest {
    input_file: String,
    output_file: String,
    step_mode: bool,
    strict: bool,
}

// ── Argument parsing ────────────────────────────────────────

fn parse_request(
    args: &[String],
    lang: Language,
) -> Result<CompileRequest, Box<dyn std::error::Error>> {
    let command_name = args.get(1).map(String::as_str).unwrap_or("cc");
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut step_mode = false;
    let mut strict = false;
    let mut i = 2;

    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                let out = args
                    .get(i + 1)
                    .ok_or_else(|| format!("Missing value after -o in '{}'", command_name))?;
                output_file = Some(out.clone());
                i += 2;
            }
            "-step" | "--step" => {
                step_mode = true;
                i += 1;
            }
            "-Wstrict" | "--strict" => {
                strict = true;
                i += 1;
            }
            flag if flag.starts_with('-') => {
                return Err(format!("Unknown option '{}' in '{}'", flag, command_name).into());
            }
            value => {
                if input_file.is_some() {
                    return Err(
                        format!("Unexpected extra argument '{}' in '{}'", value, command_name).into(),
                    );
                }
                input_file = Some(value.to_string());
                i += 1;
            }
        }
    }

    let input_file = input_file.ok_or_else(|| {
        format!("Missing source file for '{}'. Usage: adB {} <file>", command_name, command_name)
    })?;
    let output_file = output_file.unwrap_or_else(|| default_output_filename(&input_file));

    Ok(CompileRequest {
        input_file,
        output_file,
        step_mode,
        strict,
    })
}

fn parse_request_step(args: &[String]) -> Result<CompileRequest, Box<dyn std::error::Error>> {
    let mut req = parse_request(args, Language::Auto)?;
    req.step_mode = true;
    Ok(req)
}

fn default_output_filename(input_file: &str) -> String {
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

fn detect_language(input_file: &str) -> Language {
    match Path::new(input_file).extension().and_then(|e| e.to_str()) {
        Some("c") | Some("h") => Language::C,
        Some("cpp") | Some("cxx") | Some("cc") | Some("hpp") => Language::Cpp,
        Some("cu") | Some("cuh") => Language::Cuda,
        Some("js") | Some("mjs") => Language::Js,
        _ => Language::C, // default to C
    }
}

fn compile_by_language(request: &CompileRequest) -> Result<(), Box<dyn std::error::Error>> {
    let lang = detect_language(&request.input_file);
    match lang {
        Language::C | Language::Auto => {
            c_driver::compile_c_file(&request.input_file, &request.output_file, request.step_mode, request.strict)?;
        }
        Language::Cpp => {
            cpp_driver::compile_cpp_file(&request.input_file, &request.output_file, request.step_mode, request.strict)?;
        }
        Language::Cuda => {
            cuda_driver::compile_cuda_file(&request.input_file, &request.output_file, request.step_mode)?;
        }
        Language::Js => {
            js_driver::compile_js_file(&request.input_file, &request.output_file, request.step_mode)?;
        }
    }
    Ok(())
}

fn run_executable(output_file: &str) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let exe_path = if cfg!(target_os = "windows") {
        format!(".\\{}", output_file)
    } else {
        format!("./{}", output_file)
    };
    let status = Command::new(&exe_path).status()?;
    if status.success() {
        Ok(ExitCode::SUCCESS)
    } else {
        eprintln!("  Program exited with status: {}", status);
        let code = status.code().unwrap_or(1);
        let code_u8 = if (0..=255).contains(&code) { code as u8 } else { 1 };
        Ok(ExitCode::from(code_u8))
    }
}

fn print_usage(bin: &str) {
    println!("{}", term::banner("Multi-Language", VERSION));
    println!();
    println!("  {}  {} <command> <file> [options]", term::phase_header("USAGE:"), bin);
    println!("  {}  {} <file.c|file.cpp>          {}", term::phase_header("SHORT:"), bin, term::dim("(auto-detect)"));
    println!();
    println!("  {}", term::phase_header("COMMANDS (Languages):"));
    println!("    {}   <file.c>     Compile C source (C99/C11)", term::ok("cc  "));
    println!("    {}   <file.cpp>   Compile C++ source (C++17/20)", term::ok("cxx "));
    println!("    {}   <file.cu>    Compile CUDA source (preview)", term::info("cuda"));
    println!("    {}   <file.js>    Compile JavaScript (preview)", term::info("js  "));
    println!();
    println!("  {}", term::phase_header("COMMANDS (Actions):"));
    println!("    {}   <file>       Compile + run (auto-detect language)", term::ok("run "));
    println!("    {}   <file>       Step mode: show every compiler phase", term::ok("step"));
    println!("    {}               Show compiler version", term::info("version"));
    println!("    {}               Show this help", term::info("help"));
    println!();
    println!("  {}", term::phase_header("OPTIONS:"));
    println!("    {}       Output file (default: <basename>.exe)", term::dim("-o <output>"));
    println!("    {}     Enable step mode (show all phases)", term::dim("-step, --step"));
    println!("    {}          Strict C mode: bit-widths enforced, all UB = error", term::dim("-Wstrict"));
    println!("    {}           C++ is always strict (implicit)", term::dim("(C++ note)"));
    println!();
    println!("  {}", term::phase_header("EXAMPLES:"));
    println!("    {} run hello.c                  {}", bin, term::dim("Compile + run C"));
    println!("    {} run app.cpp                  {}", bin, term::dim("Compile + run C++"));
    println!("    {} cc hello.c -o out.exe        {}", bin, term::dim("Custom output"));
    println!("    {} cxx app.cpp -step            {}", bin, term::dim("C++ step mode"));
    println!("    {} hello.c                      {}", bin, term::dim("Auto-detect C"));
    println!("    {} app.cpp                      {}", bin, term::dim("Auto-detect C++"));
    println!();
}

// ── Tests ───────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn str_args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn parse_request_cc_with_step() {
        let args = str_args(&["adB", "cc", "demo.c", "-step", "-o", "demo.exe"]);
        let request = parse_request(&args, Language::C).unwrap();
        assert_eq!(request.input_file, "demo.c");
        assert_eq!(request.output_file, "demo.exe");
        assert!(request.step_mode);
    }

    #[test]
    fn parse_request_step_subcommand() {
        let args = str_args(&["adB", "step", "demo.c"]);
        let request = parse_request_step(&args).unwrap();
        assert!(request.step_mode);
        assert_eq!(request.output_file, default_output_filename("demo.c"));
    }

    #[test]
    fn detect_language_from_extension() {
        assert_eq!(detect_language("hello.c"), Language::C);
        assert_eq!(detect_language("app.cpp"), Language::Cpp);
        assert_eq!(detect_language("kernel.cu"), Language::Cuda);
        assert_eq!(detect_language("app.js"), Language::Js);
        assert_eq!(detect_language("unknown.txt"), Language::C);
    }

    #[test]
    fn default_output_windows() {
        let out = default_output_filename("hello.c");
        if cfg!(target_os = "windows") {
            assert_eq!(out, "hello.exe");
        } else {
            assert_eq!(out, "hello");
        }
    }

    #[test]
    fn parse_request_cxx() {
        let args = str_args(&["adB", "cxx", "app.cpp", "-o", "app.exe"]);
        let request = parse_request(&args, Language::Cpp).unwrap();
        assert_eq!(request.input_file, "app.cpp");
        assert_eq!(request.output_file, "app.exe");
        assert!(!request.step_mode);
    }

    #[test]
    fn parse_request_cuda() {
        let args = str_args(&["adB", "cuda", "kernel.cu"]);
        let request = parse_request(&args, Language::Cuda).unwrap();
        assert_eq!(request.input_file, "kernel.cu");
    }

    #[test]
    fn parse_request_js() {
        let args = str_args(&["adB", "js", "app.js", "-step"]);
        let request = parse_request(&args, Language::Js).unwrap();
        assert_eq!(request.input_file, "app.js");
        assert!(request.step_mode);
    }

    #[test]
    fn c_pipeline_through_driver() {
        let result = c_driver::compile_c_pipeline("int main() { return 0; }", false);
        assert!(result.is_ok());
        let art = result.unwrap();
        assert_eq!(art.program.functions.len(), 1);
        assert!(!art.ub_report.has_warnings());
    }

    #[test]
    fn c_driver_ub_detects_div_zero() {
        let result = c_driver::compile_c_pipeline("int main() { int x = 10 / 0; return x; }", false).unwrap();
        assert!(result.ub_report.has_errors());
    }

    #[test]
    fn c_driver_ub_format_string() {
        let result = c_driver::compile_c_pipeline(r#"
            #include <stdio.h>
            int main() { printf("%d %d", 1); return 0; }
        "#, false).unwrap();
        assert!(result.ub_report.has_warnings());
    }

    #[test]
    fn parse_request_strict() {
        let args = str_args(&["adB", "cc", "demo.c", "-Wstrict"]);
        let request = parse_request(&args, Language::C).unwrap();
        assert!(request.strict);
    }
}
