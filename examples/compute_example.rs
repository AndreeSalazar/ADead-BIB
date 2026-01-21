// ============================================================
// ADead-BIB - Compute API Example
// ============================================================
// Ejemplo de uso de la API unificada de compute
// Funciona con CUDA (RTX 3060) o HIP-CPU (fallback)
//
// Ejecutar: cargo run --example compute_example
// ============================================================

use adead_bib::backend::gpu::{
    ComputeRuntime,
    print_hip_info,
    SendPtr,
};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          ADead-BIB Compute API - Example                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    
    // Mostrar info del sistema
    print_hip_info();
    println!();
    
    // Crear runtime con auto-detección
    let runtime = ComputeRuntime::new();
    runtime.print_info();
    println!();
    
    // ========================================
    // Ejemplo 1: Vector Add
    // ========================================
    println!("═══ Ejemplo 1: Vector Add ═══");
    
    let n = 100_000;
    let a: Vec<f32> = (0..n).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..n).map(|i| (i * 2) as f32).collect();
    let mut c = vec![0.0f32; n];
    
    let start = std::time::Instant::now();
    runtime.vector_add(&a, &b, &mut c);
    let elapsed = start.elapsed();
    
    println!("  n = {}", n);
    println!("  Tiempo: {:?}", elapsed);
    println!("  Verificación: c[0]={}, c[99999]={}", c[0], c[99999]);
    println!("  Esperado:     c[0]=0, c[99999]={}", 99999.0 + 99999.0 * 2.0);
    println!();
    
    // ========================================
    // Ejemplo 2: SAXPY (y = alpha * x + y)
    // ========================================
    println!("═══ Ejemplo 2: SAXPY ═══");
    
    let alpha = 2.5f32;
    let x: Vec<f32> = (0..n).map(|i| i as f32).collect();
    let mut y: Vec<f32> = (0..n).map(|i| i as f32 * 0.5).collect();
    
    let start = std::time::Instant::now();
    runtime.saxpy(alpha, &x, &mut y);
    let elapsed = start.elapsed();
    
    println!("  alpha = {}", alpha);
    println!("  n = {}", n);
    println!("  Tiempo: {:?}", elapsed);
    println!("  y[1000] = {} (esperado: {})", y[1000], alpha * 1000.0 + 1000.0 * 0.5);
    println!();
    
    // ========================================
    // Ejemplo 3: Dot Product
    // ========================================
    println!("═══ Ejemplo 3: Dot Product ═══");
    
    let a_small: Vec<f32> = (0..1000).map(|i| i as f32).collect();
    let b_small: Vec<f32> = vec![1.0f32; 1000];
    
    let start = std::time::Instant::now();
    let dot = runtime.dot_product(&a_small, &b_small);
    let elapsed = start.elapsed();
    
    let expected: f32 = (0..1000).map(|i| i as f32).sum();
    println!("  n = 1000");
    println!("  Tiempo: {:?}", elapsed);
    println!("  Resultado: {} (esperado: {})", dot, expected);
    println!();
    
    // ========================================
    // Ejemplo 4: Matrix Multiply
    // ========================================
    println!("═══ Ejemplo 4: Matrix Multiply ═══");
    
    let m = 128;
    let mat_a = vec![1.0f32; m * m];
    let mat_b = vec![2.0f32; m * m];
    let mut mat_c = vec![0.0f32; m * m];
    
    let start = std::time::Instant::now();
    runtime.matmul(&mat_a, &mat_b, &mut mat_c, m, m, m);
    let elapsed = start.elapsed();
    
    let expected_val = (m as f32) * 2.0;
    println!("  Tamaño: {}x{}", m, m);
    println!("  Tiempo: {:?}", elapsed);
    println!("  C[0][0] = {} (esperado: {})", mat_c[0], expected_val);
    println!("  C[64][64] = {} (esperado: {})", mat_c[64 * m + 64], expected_val);
    println!();
    
    // ========================================
    // Ejemplo 5: Reducciones
    // ========================================
    println!("═══ Ejemplo 5: Reducciones ═══");
    
    let data: Vec<f32> = (0..10000).map(|i| i as f32).collect();
    
    let sum = runtime.reduce_sum(&data);
    let max = runtime.reduce_max(&data);
    let min = runtime.reduce_min(&data);
    
    println!("  n = 10000");
    println!("  Sum: {} (esperado: {})", sum, (0..10000).sum::<i32>() as f32);
    println!("  Max: {} (esperado: 9999)", max);
    println!("  Min: {} (esperado: 0)", min);
    println!();
    
    // ========================================
    // Ejemplo 6: Parallel For personalizado
    // ========================================
    println!("═══ Ejemplo 6: Parallel For ═══");
    
    let n = 10000;
    let mut result = vec![0i32; n];
    let result_ptr = SendPtr::new(result.as_mut_ptr());
    
    let start = std::time::Instant::now();
    runtime.parallel_for(n, |i| {
        unsafe {
            result_ptr.write(i, (i * i) as i32);
        }
    });
    let elapsed = start.elapsed();
    
    println!("  Operación: result[i] = i²");
    println!("  n = {}", n);
    println!("  Tiempo: {:?}", elapsed);
    println!("  result[100] = {} (esperado: {})", result[100], 100 * 100);
    println!("  result[9999] = {} (esperado: {})", result[9999], 9999 * 9999);
    println!();
    
    // ========================================
    // Benchmark completo
    // ========================================
    println!("═══ Benchmark Completo ═══");
    let results = runtime.benchmark();
    println!("{}", results);
    
    println!("✅ Todos los ejemplos completados!");
}
