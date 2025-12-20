// ADead-BIB Compiler
// Compilador principal

use std::env;
use std::fs;
use std::path::Path;

use adead_bib::frontend::parser;
use adead_bib::backend::pe;
use adead_bib::backend::codegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Uso: {} <archivo.adB>", args[0]);
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let output_file = if args.len() > 2 {
        args[2].clone()
    } else {
        Path::new(input_file)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string() + ".exe"
    };
    
    println!("Compilando: {} -> {}", input_file, output_file);
    
    // 1. Leer archivo fuente
    let source = fs::read_to_string(input_file)?;
    println!("✓ Archivo leído");
    
    // 2. Parsear (Rust)
    let ast = parser::parse(&source)?;
    println!("✓ Parseado exitoso");
    
    // 3. Emitir opcodes (Rust - directamente)
    println!("⚙️  Emitiendo opcodes...");
    let mut codegen = codegen::CodeGenerator::new(0x400000); // Base address
    let (opcodes, data) = codegen.generate(&ast);
    println!("✓ Opcodes generados: {} bytes", opcodes.len());
    
    // 4. Generar PE (Rust)
    println!("⚙️  Generando binario PE...");
    pe::generate_pe(&opcodes, &data, &output_file)?;
    println!("✓ Binario PE generado: {}", output_file);
    
    println!("\n✅ Compilación exitosa!");
    
    Ok(())
}

