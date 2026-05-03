#!/usr/bin/env python3
"""Genera cli/main.rs con pipeline completo conectado"""

from pathlib import Path

CLI_PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\compiler\cli")

MAIN_RS = '''//! adB CLI - Pipeline Completo
//! source → Lexer → Parser → IR → Optimizer → UB → Codegen → PE
#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;
use crate::middle::ast_to_ir::ast_to_ir;
use crate::middle::optimizer::Optimizer;
use crate::middle::ub_detector::UbDetector;
use crate::backend::codegen::Codegen;
use crate::backend::pe::PeBuilder;

const VERSION: &str = "1.0.0";

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { print_usage(); process::exit(1); }
    
    match args[1].as_str() {
        "cc" => cmd_compile_c(&args[2..]),
        "run" => cmd_run(&args[2..]),
        "version" | "-v" => cmd_version(),
        arg if arg.ends_with(".c") => cmd_compile_c(&args[1..]),
        _ => { print_usage(); process::exit(1); }
    }
}

fn cmd_compile_c(args: &[String]) {
    if args.is_empty() { eprintln!("Error: No input"); process::exit(1); }
    
    let input = &args[0];
    let mut output = input.replace(".c", ".exe");
    
    for i in 0..args.len() {
        if args[i] == "-o" && i + 1 < args.len() {
            output = args[i + 1].clone();
        }
    }
    
    println!("\\x1b[36m[adB] C Compiler v{}\\x1b[0m", VERSION);
    println!("  {} → {}", input, output);
    
    if !Path::new(input).exists() {
        eprintln!("Error: '{}' not found", input);
        process::exit(1);
    }
    
    compile_pipeline(input, &output);
}

fn compile_pipeline(input: &str, output: &str) {
    // [1] SOURCE
    print!("  [1/7] Source... ");
    let source = fs::read_to_string(input).expect("read failed");
    println!("{} bytes", source.len());
    
    // [2] LEXER
    print!("  [2/7] Lexer... ");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    println!("{} tokens", tokens.len());
    
    // [3] PARSER
    print!("  [3/7] Parser... ");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("parse failed");
    println!("{} items", ast.items.len());
    
    // [4] AST → IR
    print!("  [4/7] IR... ");
    let ir = ast_to_ir(&ast);
    println!("{} functions", ir.functions.len());
    
    // [5] OPTIMIZER
    print!("  [5/7] Optimizer... ");
    let ir = Optimizer::run(ir);
    println!("optimized");
    
    // [6] UB DETECTOR
    print!("  [6/7] UB Check... ");
    let issues = UbDetector::check(&ir);
    if issues.is_empty() {
        println!("clean");
    } else {
        println!("{} warnings", issues.len());
        for issue in &issues {
            eprintln!("    ⚠ {:?}", issue);
        }
    }
    
    // [7] CODEGEN + PE
    print!("  [7/7] Codegen... ");
    let mut codegen = Codegen::new();
    let code = codegen.generate(&ir);
    
    let mut pe = PeBuilder::new().console();
    pe.code = code;
    pe.data = codegen.data_section;
    let exe = pe.build();
    
    fs::write(output, &exe).expect("write failed");
    println!("{} bytes", exe.len());
    
    println!("\\x1b[32m  ✓ {}\\x1b[0m", output);
}

fn cmd_run(args: &[String]) {
    if args.is_empty() { eprintln!("Error: No input"); process::exit(1); }
    let input = &args[0];
    let temp = "temp_run.exe";
    compile_pipeline(input, temp);
    
    println!("\\n\\x1b[32m[Running]\\x1b[0m {}\\n", temp);
    let status = process::Command::new(temp).status().expect("run failed");
    let _ = fs::remove_file(temp);
    process::exit(status.code().unwrap_or(1));
}

fn cmd_version() {
    println!("ADead-BIB Compiler v{}", VERSION);
    println!("  Pipeline: Lexer → Parser → IR → Opt → UB → Codegen → PE");
}

fn print_usage() {
    println!("adB cc <file.c> [-o out]");
    println!("adB run <file.c>");
}
'''

def main():
    print("🔧 Generando cli/main.rs (pipeline)...")
    path = CLI_PATH / "main.rs"
    path.write_text(MAIN_RS, encoding='utf-8')
    print(f"  ✅ main.rs → {MAIN_RS.count(chr(10))} líneas")

if __name__ == "__main__":
    main()
