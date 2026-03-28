mod cli;

use crate::cli::term;
use adeb_backend_x64::isa::isa_compiler::{IsaCompiler, Target};
use adeb_core::ast::Program;
use adeb_frontend_c::ast::{CDeclarator, CDerivedType, CTopLevel, CTranslationUnit, CType};
use adeb_frontend_c::lower::to_ir::CToIR;
use adeb_frontend_c::parse::lexer::CToken;
use adeb_frontend_c::parse::parser::CParser;
use adeb_frontend_c::preprocessor::CPreprocessor;
use adeb_frontend_c::CLexer;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::ExitCode;

fn main() -> ExitCode {
    term::enable_ansi();
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
            let request = parse_request(args, CommandMode::Compile)?;
            compile_c_file(&request)?;
            Ok(ExitCode::SUCCESS)
        }
        "run" => {
            let request = parse_request(args, CommandMode::Run)?;
            compile_c_file(&request)?;
            let output = request.output_file;
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
        "step" => {
            let request = parse_request(args, CommandMode::Step)?;
            compile_c_file(&request)?;
            Ok(ExitCode::SUCCESS)
        }
        _ => {
            print_usage(&args[0]);
            Ok(ExitCode::from(2))
        }
    }
}

fn print_usage(bin: &str) {
    println!("Usage:");
    println!("  {} cc <file.c> [-o output.exe] [-step]", bin);
    println!("  {} run <file.c> [-o output.exe] [-step]", bin);
    println!("  {} step <file.c> [-o output.exe]", bin);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandMode {
    Compile,
    Run,
    Step,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompileRequest {
    input_file: String,
    output_file: String,
    step_mode: bool,
}

#[derive(Debug, Clone)]
struct CPipelineArtifacts {
    preprocessed: String,
    included_headers: Vec<String>,
    tokens: Vec<CToken>,
    token_lines: Vec<usize>,
    unit: CTranslationUnit,
    semantic: SemanticSnapshot,
    program: Program,
}

#[derive(Debug, Clone)]
struct SemanticSnapshot {
    entries: Vec<SymbolEntry>,
    duplicate_symbols: Vec<String>,
}

#[derive(Debug, Clone)]
struct SymbolEntry {
    kind: &'static str,
    name: String,
    detail: String,
}

fn parse_request(
    args: &[String],
    mode: CommandMode,
) -> Result<CompileRequest, Box<dyn std::error::Error>> {
    let command_name = args.get(1).map(String::as_str).unwrap_or("cc");
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut step_mode = mode == CommandMode::Step;
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
            flag if flag.starts_with('-') => {
                return Err(format!("Unknown option '{}' in '{}'", flag, command_name).into());
            }
            value => {
                if input_file.is_some() {
                    return Err(
                        format!("Unexpected extra positional argument '{}' in '{}'", value, command_name)
                            .into(),
                    );
                }
                input_file = Some(value.to_string());
                i += 1;
            }
        }
    }

    let input_file = input_file.ok_or_else(|| match mode {
        CommandMode::Compile => "Missing C source file for 'cc'".to_string(),
        CommandMode::Run => "Missing C source file for 'run'".to_string(),
        CommandMode::Step => "Missing C source file for 'step'".to_string(),
    })?;
    let output_file = output_file.unwrap_or_else(|| default_output_filename(&input_file));

    Ok(CompileRequest {
        input_file,
        output_file,
        step_mode,
    })
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

fn compile_c_pipeline(source: &str) -> Result<CPipelineArtifacts, String> {
    let mut preprocessor = CPreprocessor::new();
    let preprocessed = preprocessor.process(source);
    let mut included_headers: Vec<String> = preprocessor.included_headers().iter().cloned().collect();
    included_headers.sort();

    let (tokens, token_lines) = CLexer::new(&preprocessed).tokenize();
    let unit = CParser::new(tokens.clone(), token_lines.clone()).parse_translation_unit()?;
    let semantic = collect_semantic_snapshot(&unit);
    let mut lower = CToIR::new();
    let program = lower.convert(&unit)?;

    Ok(CPipelineArtifacts {
        preprocessed,
        included_headers,
        tokens,
        token_lines,
        unit,
        semantic,
        program,
    })
}

fn compile_c_file(request: &CompileRequest) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = &request.input_file;
    let output_file = &request.output_file;
    println!("🔨 ADead-BIB C Compiler");
    println!("   Source: {}", input_file);
    println!("   Target: {}", output_file);

    let source = fs::read_to_string(input_file)
        .map_err(|e| format!("Cannot read '{}': {}", input_file, e))?;

    let pipeline = compile_c_pipeline(&source).map_err(|e| format!("C pipeline error: {}", e))?;

    if request.step_mode {
        print_step_mode(input_file, &source, &pipeline);
    } else {
        println!("   Step 1: Parsing C...");
    }

    println!("   Step 2: Compiling to native code...");
    let mut compiler = IsaCompiler::new(Target::Windows);
    let (code, data, iat_offsets, string_offsets) = compiler.compile(&pipeline.program);

    if request.step_mode {
        print_backend_step(&code, &data, &iat_offsets, &string_offsets);
    }

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

fn collect_semantic_snapshot(unit: &CTranslationUnit) -> SemanticSnapshot {
    let mut entries = Vec::new();
    let mut counts = BTreeMap::<String, usize>::new();

    for declaration in &unit.declarations {
        match declaration {
            CTopLevel::FunctionDef {
                return_type,
                name,
                params,
                ..
            } => {
                entries.push(SymbolEntry {
                    kind: "function",
                    name: name.clone(),
                    detail: format!(
                        "{} ({}) [definition]",
                        render_ctype(return_type),
                        render_params(params)
                    ),
                });
                *counts.entry(name.clone()).or_default() += 1;
            }
            CTopLevel::FunctionDecl {
                return_type,
                name,
                params,
            } => {
                entries.push(SymbolEntry {
                    kind: "prototype",
                    name: name.clone(),
                    detail: format!("{} ({})", render_ctype(return_type), render_params(params)),
                });
                *counts.entry(name.clone()).or_default() += 1;
            }
            CTopLevel::GlobalVar {
                type_spec,
                declarators,
            } => {
                for declarator in declarators {
                    entries.push(SymbolEntry {
                        kind: "global",
                        name: declarator.name.clone(),
                        detail: render_declarator(type_spec, declarator),
                    });
                    *counts.entry(declarator.name.clone()).or_default() += 1;
                }
            }
            CTopLevel::StructDef { name, fields } => {
                entries.push(SymbolEntry {
                    kind: "struct",
                    name: name.clone(),
                    detail: format!("{} field(s)", fields.len()),
                });
                *counts.entry(name.clone()).or_default() += 1;
            }
            CTopLevel::EnumDef { name, values } => {
                entries.push(SymbolEntry {
                    kind: "enum",
                    name: name.clone(),
                    detail: format!("{} value(s)", values.len()),
                });
                *counts.entry(name.clone()).or_default() += 1;
            }
            CTopLevel::TypedefDecl { original, new_name } => {
                entries.push(SymbolEntry {
                    kind: "typedef",
                    name: new_name.clone(),
                    detail: render_ctype(original),
                });
                *counts.entry(new_name.clone()).or_default() += 1;
            }
        }
    }

    let duplicate_symbols = counts
        .into_iter()
        .filter_map(|(name, count)| (count > 1).then_some(name))
        .collect();

    SemanticSnapshot {
        entries,
        duplicate_symbols,
    }
}

fn render_params(params: &[adeb_frontend_c::ast::CParam]) -> String {
    if params.is_empty() {
        return "void".to_string();
    }

    params
        .iter()
        .map(|param| match &param.name {
            Some(name) => format!("{} {}", render_ctype(&param.param_type), name),
            None => render_ctype(&param.param_type),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_declarator(base: &CType, declarator: &CDeclarator) -> String {
    let final_type = match &declarator.derived_type {
        Some(derived) => render_derived_type(base, derived),
        None => render_ctype(base),
    };
    if declarator.initializer.is_some() {
        format!("{final_type} = <initializer>")
    } else {
        final_type
    }
}

fn render_derived_type(base: &CType, derived: &CDerivedType) -> String {
    match derived {
        CDerivedType::Pointer(next) => match next {
            Some(next) => format!("pointer to {}", render_derived_type(base, next)),
            None => format!("pointer to {}", render_ctype(base)),
        },
        CDerivedType::Array(size, next) => {
            let inner = match next {
                Some(next) => render_derived_type(base, next),
                None => render_ctype(base),
            };
            match size {
                Some(size) => format!("array[{size}] of {inner}"),
                None => format!("array[] of {inner}"),
            }
        }
    }
}

fn render_ctype(ty: &CType) -> String {
    match ty {
        CType::Void => "void".to_string(),
        CType::Char => "char".to_string(),
        CType::Short => "short".to_string(),
        CType::Int => "int".to_string(),
        CType::Long => "long".to_string(),
        CType::LongLong => "long long".to_string(),
        CType::Float => "float".to_string(),
        CType::Double => "double".to_string(),
        CType::Bool => "_Bool".to_string(),
        CType::Unsigned(inner) => format!("unsigned {}", render_ctype(inner)),
        CType::Signed(inner) => format!("signed {}", render_ctype(inner)),
        CType::Pointer(inner) => format!("{}*", render_ctype(inner)),
        CType::Array(inner, size) => match size {
            Some(size) => format!("{}[{}]", render_ctype(inner), size),
            None => format!("{}[]", render_ctype(inner)),
        },
        CType::Struct(name) => format!("struct {}", name),
        CType::Enum(name) => format!("enum {}", name),
        CType::Typedef(name) => name.clone(),
        CType::Function {
            return_type,
            params,
        } => format!(
            "{} fn({})",
            render_ctype(return_type),
            params.iter().map(render_ctype).collect::<Vec<_>>().join(", ")
        ),
        CType::Const(inner) => format!("const {}", render_ctype(inner)),
        CType::Volatile(inner) => format!("volatile {}", render_ctype(inner)),
        CType::Complex(inner) => format!("{}_Complex", render_ctype(inner)),
    }
}

fn print_step_mode(input_file: &str, source: &str, pipeline: &CPipelineArtifacts) {
    println!();
    println!("{}", term::phase_bar(0, "Preprocessor", "C"));
    println!(
        "{}",
        term::info(&format!(
            "Archivo: {} | líneas originales: {} | líneas preprocesadas: {}",
            input_file,
            source.lines().count(),
            pipeline.preprocessed.lines().count()
        ))
    );
    if pipeline.included_headers.is_empty() {
        println!("{}", term::warn("Headers resueltos: ninguno"));
    } else {
        println!(
            "{}",
            term::ok(&format!(
                "Headers resueltos ({}): {}",
                pipeline.included_headers.len(),
                pipeline.included_headers.join(", ")
            ))
        );
    }
    println!("{}", pipeline.preprocessed);

    println!();
    println!("{}", term::phase_bar(1, "Lexical Analysis", "C"));
    println!(
        "{}",
        term::info(&format!("Tokens generados: {}", pipeline.tokens.len()))
    );
    for (index, (token, line)) in pipeline
        .tokens
        .iter()
        .zip(pipeline.token_lines.iter())
        .enumerate()
    {
        println!(
            "{}",
            term::token_fmt(&format!("[{:04}] línea {:>4}  {:?}", index, line, token))
        );
    }

    println!();
    println!("{}", term::phase_bar(2, "Syntactic Analysis", "C"));
    println!(
        "{}",
        term::info(&format!(
            "Declaraciones de alto nivel: {}",
            pipeline.unit.declarations.len()
        ))
    );
    println!("{:#?}", pipeline.unit);

    println!();
    println!("{}", term::phase_bar(3, "Semantic Analysis", "C"));
    println!(
        "{}",
        term::info(&format!(
            "Símbolos recolectados: {}",
            pipeline.semantic.entries.len()
        ))
    );
    for entry in &pipeline.semantic.entries {
        println!(
            "{}",
            term::type_fmt(&format!(
                "{:<10} {:<24} {}",
                entry.kind, entry.name, entry.detail
            ))
        );
    }
    if pipeline.semantic.duplicate_symbols.is_empty() {
        println!("{}", term::ok("Símbolos duplicados: ninguno"));
    } else {
        println!(
            "{}",
            term::warn(&format!(
                "Símbolos duplicados detectados: {}",
                pipeline.semantic.duplicate_symbols.join(", ")
            ))
        );
    }

    println!();
    println!("{}", term::phase_bar(4, "IR Generation", "C"));
    println!(
        "{}",
        term::info(&format!(
            "IR: {} función(es), {} struct(s), {} statement(s) top-level",
            pipeline.program.functions.len(),
            pipeline.program.structs.len(),
            pipeline.program.statements.len()
        ))
    );
    println!("{:#?}", pipeline.program);
}

fn print_backend_step(
    code: &[u8],
    data: &[u8],
    iat_offsets: &[usize],
    string_offsets: &[usize],
) {
    println!();
    println!("{}", term::phase_bar(5, "Code Generation", "x64"));
    println!(
        "{}",
        term::info(&format!(
            "text={} bytes | data={} bytes | iat_offsets={} | string_offsets={}",
            code.len(),
            data.len(),
            iat_offsets.len(),
            string_offsets.len()
        ))
    );
    println!("CODE HEX  {}", bytes_to_hex(code, 64));
    println!("DATA HEX  {}", bytes_to_hex(data, 64));
    println!("IAT       {:?}", iat_offsets);
    println!("STRINGS   {:?}", string_offsets);
}

fn bytes_to_hex(bytes: &[u8], limit: usize) -> String {
    if bytes.is_empty() {
        return "<empty>".to_string();
    }

    let preview = bytes
        .iter()
        .take(limit)
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<_>>()
        .join(" ");

    if bytes.len() > limit {
        format!("{preview} ... ({} bytes total)", bytes.len())
    } else {
        preview
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn parse_request_enables_step_flag() {
        let args = str_args(&["adB", "cc", "demo.c", "-step", "-o", "demo.exe"]);
        let request = parse_request(&args, CommandMode::Compile).unwrap();
        assert_eq!(
            request,
            CompileRequest {
                input_file: "demo.c".to_string(),
                output_file: "demo.exe".to_string(),
                step_mode: true,
            }
        );
    }

    #[test]
    fn parse_request_supports_step_subcommand() {
        let args = str_args(&["adB", "step", "demo.c"]);
        let request = parse_request(&args, CommandMode::Step).unwrap();
        assert!(request.step_mode);
        assert_eq!(request.output_file, default_output_filename("demo.c"));
    }

    #[test]
    fn semantic_snapshot_collects_symbols() {
        let source = r#"
            typedef unsigned int u32;
            struct Vec2 { int x; int y; };
            int total;
            int add(int a, int b) { return a + b; }
        "#;
        let pipeline = compile_c_pipeline(source).unwrap();
        let names = pipeline
            .semantic
            .entries
            .iter()
            .map(|entry| entry.name.as_str())
            .collect::<Vec<_>>();
        assert!(names.contains(&"u32"));
        assert!(names.contains(&"Vec2"));
        assert!(names.contains(&"total"));
        assert!(names.contains(&"add"));
    }

    #[test]
    fn bytes_to_hex_truncates_preview() {
        let bytes = (0u8..100).collect::<Vec<_>>();
        let preview = bytes_to_hex(&bytes, 8);
        assert!(preview.contains("00 01 02 03 04 05 06 07"));
        assert!(preview.contains("100 bytes total"));
    }
}
