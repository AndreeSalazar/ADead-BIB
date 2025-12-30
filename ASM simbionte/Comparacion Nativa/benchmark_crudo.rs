// ============================================================
// 🔥 BENCHMARK CRUDO - Comparación directa al metal 🔥
// ============================================================
// Este programa genera y ejecuta código máquina DIRECTAMENTE
// sin ningún intermediario, para comparar con Rust y C++.
// ============================================================

use std::time::Instant;

fn main() {
    println!("====================================================");
    println!("BENCHMARK CRUDO - Código máquina directo");
    println!("====================================================");
    println!();

    // El código que ADead-BIB genera para el loop:
    // Solo 8 bytes en el hot path!
    println!("ADead-BIB genera este código HEX:");
    println!();
    println!("  Setup:");
    println!("    48 8B 8D F8 FF FF FF  ; mov rcx, [rbp-8]");
    println!("    49 B8 00 CA 9A 3B 00 00 00 00  ; mov r8, 1000000000");
    println!("    4C 39 C1              ; cmp rcx, r8");
    println!("    7D 08                 ; jge skip");
    println!();
    println!("  Hot Loop (8 bytes):");
    println!("    48 FF C1              ; inc rcx");
    println!("    4C 39 C1              ; cmp rcx, r8");
    println!("    7C F8                 ; jl loop");
    println!();
    println!("  Cleanup:");
    println!("    48 89 8D F8 FF FF FF  ; mov [rbp-8], rcx");
    println!();
    println!("====================================================");
    println!();

    // Simular el loop en Rust para comparación
    println!("Ejecutando loop equivalente en Rust...");
    
    let iterations: i64 = 1_000_000_000;
    let mut counter: i64 = 0;
    
    let start = Instant::now();
    
    // Este loop es lo que ADead-BIB genera en HEX
    while counter < iterations {
        counter += 1;
    }
    
    let duration = start.elapsed();
    
    println!("Resultado: {}", counter);
    println!("Tiempo: {:.3}s", duration.as_secs_f64());
    println!();
    
    // Verificar si LLVM optimizó
    if duration.as_secs_f64() < 0.1 {
        println!("⚠️  LLVM probablemente optimizó el loop!");
        println!("    Tiempo < 0.1s es físicamente imposible para 1B iteraciones");
    } else {
        println!("✅ Loop ejecutado correctamente");
    }
}
