//! adB - CLI del compilador ADead-BIB

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("adB - Compilador C/C++ optimizado");
        println!("Uso: adB <archivo.c|archivo.cpp> [opciones]");
        println!();
        println!("Opciones:");
        println!("  -o <archivo>    Nombre de salida");
        println!("  -O2             Optimización nivel 2");
        println!("  -Wstrict        Modo estricto (UB = error)");
        println!("  -step           Mostrar pasos de compilación");
        process::exit(1);
    }
    
    println!("adB v1.0 - Compilador C/C++ optimizado");
    println!("Compilando: {}", args[1]);
    
    // TODO: Implementar pipeline completo
    println!("[TODO] Pipeline en desarrollo...");
}
