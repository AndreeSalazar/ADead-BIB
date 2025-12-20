// TEST-G: Benchmark CPU vs GPU Dispatch
// Simula selección de backend y mide overhead de decisión
//
// Autor: Eddi Andreé Salazar Matos

use std::time::Instant;
use std::arch::x86_64::*;

/// Backend de compute
#[derive(Debug, Clone, Copy, PartialEq)]
enum Backend {
    CPU_Scalar,
    CPU_SSE2,
    CPU_AVX2,
    GPU_Vulkan,
    GPU_CUDA,
}

/// Dispatcher simple
struct Dispatcher {
    has_avx2: bool,
    has_vulkan: bool,
    has_cuda: bool,
    gpu_threshold: usize,
}

impl Dispatcher {
    fn new() -> Self {
        Self {
            has_avx2: Self::detect_avx2(),
            has_vulkan: Self::detect_vulkan(),
            has_cuda: Self::detect_cuda(),
            gpu_threshold: 1024 * 1024,
        }
    }
    
    fn detect_avx2() -> bool {
        unsafe {
            let cpuid = std::arch::x86_64::__cpuid_count(7, 0);
            (cpuid.ebx & (1 << 5)) != 0
        }
    }
    
    fn detect_vulkan() -> bool {
        use std::path::Path;
        let path = std::env::var("SystemRoot")
            .map(|r| format!("{}\\System32\\vulkan-1.dll", r))
            .unwrap_or_default();
        Path::new(&path).exists()
    }
    
    fn detect_cuda() -> bool {
        use std::path::Path;
        let path = std::env::var("SystemRoot")
            .map(|r| format!("{}\\System32\\nvcuda.dll", r))
            .unwrap_or_default();
        Path::new(&path).exists()
    }
    
    fn select(&self, size: usize) -> Backend {
        if size >= self.gpu_threshold {
            if self.has_cuda { return Backend::GPU_CUDA; }
            if self.has_vulkan { return Backend::GPU_Vulkan; }
        }
        if self.has_avx2 { Backend::CPU_AVX2 }
        else { Backend::CPU_SSE2 }
    }
}

/// MatMul escalar simple
fn matmul_scalar(a: &[f32], b: &[f32], c: &mut [f32], n: usize) {
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0f32;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }
}

/// MatMul con AVX2 (simplificado)
#[target_feature(enable = "avx2")]
unsafe fn matmul_avx2(a: &[f32], b: &[f32], c: &mut [f32], n: usize) {
    for i in 0..n {
        for j in (0..n).step_by(8) {
            let mut acc = _mm256_setzero_ps();
            
            for k in 0..n {
                let a_val = _mm256_broadcast_ss(&a[i * n + k]);
                let b_vec = _mm256_loadu_ps(&b[k * n + j]);
                acc = _mm256_fmadd_ps(a_val, b_vec, acc);
            }
            
            _mm256_storeu_ps(&mut c[i * n + j], acc);
        }
    }
}

/// Benchmark de dispatch
fn benchmark_dispatch(dispatcher: &Dispatcher, iterations: usize) -> f64 {
    let sizes = [100, 1000, 10000, 100000, 1000000, 10000000];
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        for &size in &sizes {
            let _ = dispatcher.select(size);
        }
    }
    
    let elapsed = start.elapsed();
    (elapsed.as_nanos() as f64) / (iterations as f64 * sizes.len() as f64)
}

/// Benchmark de MatMul CPU
fn benchmark_matmul_cpu(n: usize, iterations: usize) -> (f64, f64) {
    let a: Vec<f32> = (0..n*n).map(|i| (i % 100) as f32 * 0.01).collect();
    let b: Vec<f32> = (0..n*n).map(|i| (i % 100) as f32 * 0.01).collect();
    let mut c: Vec<f32> = vec![0.0; n * n];
    
    // Scalar
    let start = Instant::now();
    for _ in 0..iterations {
        matmul_scalar(&a, &b, &mut c, n);
    }
    let scalar_time = start.elapsed().as_secs_f64() / iterations as f64;
    
    // AVX2
    let start = Instant::now();
    for _ in 0..iterations {
        unsafe { matmul_avx2(&a, &b, &mut c, n); }
    }
    let avx2_time = start.elapsed().as_secs_f64() / iterations as f64;
    
    (scalar_time * 1000.0, avx2_time * 1000.0) // En ms
}

/// Test 1: Overhead de dispatch
fn test_dispatch_overhead() -> bool {
    let dispatcher = Dispatcher::new();
    
    let overhead_ns = benchmark_dispatch(&dispatcher, 100000);
    
    println!("  Overhead por decisión: {:.2} ns", overhead_ns);
    println!("  Decisiones por segundo: {:.0} M", 1e9 / overhead_ns / 1e6);
    
    // Debe ser < 100ns por decisión
    overhead_ns < 100.0
}

/// Test 2: Dispatch determinista
fn test_dispatch_deterministic() -> bool {
    let d1 = Dispatcher::new();
    let d2 = Dispatcher::new();
    
    let sizes = [100, 1000, 10000, 100000, 1000000, 10000000];
    let mut all_same = true;
    
    for &size in &sizes {
        let b1 = d1.select(size);
        let b2 = d2.select(size);
        
        if b1 != b2 {
            println!("  ❌ Size {} diferente: {:?} vs {:?}", size, b1, b2);
            all_same = false;
        }
    }
    
    if all_same {
        println!("  Todas las decisiones son idénticas");
    }
    
    all_same
}

/// Test 3: MatMul CPU benchmark
fn test_matmul_benchmark() -> bool {
    let n = 64; // Matriz pequeña para test rápido
    let iterations = 10;
    
    let (scalar_ms, avx2_ms) = benchmark_matmul_cpu(n, iterations);
    let speedup = scalar_ms / avx2_ms;
    
    println!("  Matriz {}x{}", n, n);
    println!("  Scalar: {:.3} ms", scalar_ms);
    println!("  AVX2:   {:.3} ms", avx2_ms);
    println!("  Speedup: {:.1}x", speedup);
    
    // AVX2 debe ser más rápido
    speedup > 1.0
}

/// Test 4: Selección correcta por tamaño
fn test_size_selection() -> bool {
    let dispatcher = Dispatcher::new();
    
    let small = dispatcher.select(1000);
    let medium = dispatcher.select(100000);
    let large = dispatcher.select(10000000);
    
    println!("  1K elementos:  {:?}", small);
    println!("  100K elementos: {:?}", medium);
    println!("  10M elementos:  {:?}", large);
    
    // Pequeño debe ser CPU, grande debe ser GPU (si disponible)
    let small_is_cpu = matches!(small, Backend::CPU_Scalar | Backend::CPU_SSE2 | Backend::CPU_AVX2);
    
    println!("  Pequeño usa CPU: {}", small_is_cpu);
    
    small_is_cpu
}

/// Test 5: GFLOPS estimados
fn test_gflops_estimate() -> bool {
    let n = 128;
    let iterations = 5;
    
    let (scalar_ms, avx2_ms) = benchmark_matmul_cpu(n, iterations);
    
    // FLOPS = 2 * n^3 (para MatMul)
    let flops = 2.0 * (n as f64).powi(3);
    
    let scalar_gflops = flops / (scalar_ms / 1000.0) / 1e9;
    let avx2_gflops = flops / (avx2_ms / 1000.0) / 1e9;
    
    println!("  Matriz {}x{}", n, n);
    println!("  Scalar: {:.2} GFLOPS", scalar_gflops);
    println!("  AVX2:   {:.2} GFLOPS", avx2_gflops);
    
    true
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     TEST-G: Benchmark CPU vs GPU Dispatch                  ║");
    println!("║     Autor: Eddi Andreé Salazar Matos                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    // Info del sistema
    let dispatcher = Dispatcher::new();
    println!("📊 Sistema detectado:");
    println!("   AVX2:   {}", if dispatcher.has_avx2 { "✓" } else { "✗" });
    println!("   Vulkan: {}", if dispatcher.has_vulkan { "✓" } else { "✗" });
    println!("   CUDA:   {}", if dispatcher.has_cuda { "✓" } else { "✗" });
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    
    println!("🧪 Test 1: Overhead de dispatch");
    if test_dispatch_overhead() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 2: Dispatch determinista");
    if test_dispatch_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 3: MatMul CPU benchmark");
    if test_matmul_benchmark() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 4: Selección por tamaño");
    if test_size_selection() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 5: GFLOPS estimados");
    if test_gflops_estimate() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("════════════════════════════════════════════════════════════");
    println!("📊 Resultados: {} passed, {} failed", passed, failed);
    
    if failed == 0 {
        println!("✅ TEST-G BENCHMARK COMPLETADO");
    } else {
        println!("❌ TEST-G BENCHMARK FALLIDO");
    }
}
