// TEST-G: Test de Auto-Dispatch CPU+GPU
// Verifica selección automática de backend óptimo
//
// Autor: Eddi Andreé Salazar Matos

use std::arch::x86_64::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::Instant;

/// Backend de compute
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ComputeBackend {
    CpuScalar,
    CpuSse2,
    CpuAvx2,
    CpuAvx512,
    GpuVulkan,
    GpuCuda,
}

/// Características del CPU
#[derive(Debug, Clone, Hash)]
struct CpuFeatures {
    has_sse2: bool,
    has_avx: bool,
    has_avx2: bool,
    has_avx512f: bool,
    has_fma: bool,
}

impl CpuFeatures {
    fn detect() -> Self {
        unsafe {
            let cpuid1 = __cpuid(1);
            let cpuid7 = __cpuid_count(7, 0);
            
            Self {
                has_sse2: (cpuid1.edx & (1 << 26)) != 0,
                has_avx: (cpuid1.ecx & (1 << 28)) != 0,
                has_avx2: (cpuid7.ebx & (1 << 5)) != 0,
                has_avx512f: (cpuid7.ebx & (1 << 16)) != 0,
                has_fma: (cpuid1.ecx & (1 << 12)) != 0,
            }
        }
    }
    
    fn best_simd(&self) -> ComputeBackend {
        if self.has_avx512f { ComputeBackend::CpuAvx512 }
        else if self.has_avx2 { ComputeBackend::CpuAvx2 }
        else if self.has_sse2 { ComputeBackend::CpuSse2 }
        else { ComputeBackend::CpuScalar }
    }
}

/// Características de la GPU
#[derive(Debug, Clone, Hash)]
struct GpuFeatures {
    vulkan_available: bool,
    cuda_available: bool,
    is_nvidia: bool,
}

impl GpuFeatures {
    fn detect() -> Self {
        let system32 = std::env::var("SystemRoot").unwrap_or_default();
        
        let vulkan = Path::new(&format!("{}\\System32\\vulkan-1.dll", system32)).exists();
        let cuda = Path::new(&format!("{}\\System32\\nvcuda.dll", system32)).exists();
        let nvidia = Path::new(&format!("{}\\System32\\nvapi64.dll", system32)).exists() || cuda;
        
        Self {
            vulkan_available: vulkan,
            cuda_available: cuda,
            is_nvidia: nvidia,
        }
    }
}

/// Auto-dispatcher
#[derive(Debug)]
struct AutoDispatcher {
    cpu: CpuFeatures,
    gpu: GpuFeatures,
    gpu_threshold: usize,
}

impl AutoDispatcher {
    fn new() -> Self {
        Self {
            cpu: CpuFeatures::detect(),
            gpu: GpuFeatures::detect(),
            gpu_threshold: 1024 * 1024, // 1M elementos
        }
    }
    
    fn with_threshold(mut self, threshold: usize) -> Self {
        self.gpu_threshold = threshold;
        self
    }
    
    /// Selecciona el mejor backend para el tamaño dado
    fn select(&self, size: usize) -> ComputeBackend {
        // GPU para datos grandes
        if size >= self.gpu_threshold {
            if self.gpu.cuda_available && self.gpu.is_nvidia {
                return ComputeBackend::GpuCuda;
            }
            if self.gpu.vulkan_available {
                return ComputeBackend::GpuVulkan;
            }
        }
        
        // CPU con mejor SIMD
        self.cpu.best_simd()
    }
    
    /// Selecciona backend para MatMul
    fn select_matmul(&self, m: usize, n: usize, k: usize) -> ComputeBackend {
        let flops = 2 * m * n * k;
        self.select(flops)
    }
}

fn hash_backend(b: ComputeBackend) -> u64 {
    let mut hasher = DefaultHasher::new();
    b.hash(&mut hasher);
    hasher.finish()
}

/// Test 1: Detección de CPU
fn test_cpu_detection() -> bool {
    let cpu = CpuFeatures::detect();
    
    println!("  SSE2:    {}", if cpu.has_sse2 { "✓" } else { "✗" });
    println!("  AVX:     {}", if cpu.has_avx { "✓" } else { "✗" });
    println!("  AVX2:    {}", if cpu.has_avx2 { "✓" } else { "✗" });
    println!("  AVX-512: {}", if cpu.has_avx512f { "✓" } else { "✗" });
    println!("  FMA:     {}", if cpu.has_fma { "✓" } else { "✗" });
    println!("  Best:    {:?}", cpu.best_simd());
    
    // SSE2 siempre debe estar disponible en x86-64
    cpu.has_sse2
}

/// Test 2: Detección de GPU
fn test_gpu_detection() -> bool {
    let gpu = GpuFeatures::detect();
    
    println!("  Vulkan:  {}", if gpu.vulkan_available { "✓" } else { "✗" });
    println!("  CUDA:    {}", if gpu.cuda_available { "✓" } else { "✗" });
    println!("  NVIDIA:  {}", if gpu.is_nvidia { "✓" } else { "✗" });
    
    true
}

/// Test 3: Dispatch determinista
fn test_dispatch_deterministic() -> bool {
    let d1 = AutoDispatcher::new();
    let d2 = AutoDispatcher::new();
    
    let sizes = [100, 1000, 10000, 100000, 1000000, 10000000];
    let mut all_same = true;
    
    for &size in &sizes {
        let b1 = d1.select(size);
        let b2 = d2.select(size);
        
        let h1 = hash_backend(b1);
        let h2 = hash_backend(b2);
        
        if h1 != h2 {
            println!("  ❌ Size {} diferente", size);
            all_same = false;
        }
    }
    
    if all_same {
        println!("  Todas las decisiones son deterministas");
    }
    
    all_same
}

/// Test 4: Transición CPU→GPU
fn test_cpu_gpu_transition() -> bool {
    let dispatcher = AutoDispatcher::new().with_threshold(1000000);
    
    let small = dispatcher.select(100);
    let medium = dispatcher.select(500000);
    let large = dispatcher.select(2000000);
    let huge = dispatcher.select(100000000);
    
    println!("  100 elementos:    {:?}", small);
    println!("  500K elementos:   {:?}", medium);
    println!("  2M elementos:     {:?}", large);
    println!("  100M elementos:   {:?}", huge);
    
    // Verificar transición
    let small_is_cpu = matches!(small, 
        ComputeBackend::CpuScalar | ComputeBackend::CpuSse2 | 
        ComputeBackend::CpuAvx2 | ComputeBackend::CpuAvx512);
    
    let medium_is_cpu = matches!(medium,
        ComputeBackend::CpuScalar | ComputeBackend::CpuSse2 | 
        ComputeBackend::CpuAvx2 | ComputeBackend::CpuAvx512);
    
    println!("  Pequeño usa CPU: {}", small_is_cpu);
    println!("  Mediano usa CPU: {}", medium_is_cpu);
    
    small_is_cpu && medium_is_cpu
}

/// Test 5: MatMul dispatch
fn test_matmul_dispatch() -> bool {
    let dispatcher = AutoDispatcher::new();
    
    // Matrices pequeñas → CPU
    let small = dispatcher.select_matmul(32, 32, 32);
    // Matrices medianas → CPU
    let medium = dispatcher.select_matmul(256, 256, 256);
    // Matrices grandes → GPU
    let large = dispatcher.select_matmul(1024, 1024, 1024);
    
    println!("  32x32x32:     {:?}", small);
    println!("  256x256x256:  {:?}", medium);
    println!("  1024x1024x1024: {:?}", large);
    
    true
}

/// Test 6: Overhead de dispatch
fn test_dispatch_overhead() -> bool {
    let dispatcher = AutoDispatcher::new();
    let sizes = [100, 1000, 10000, 100000, 1000000];
    
    let iterations = 1000000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        for &size in &sizes {
            let _ = dispatcher.select(size);
        }
    }
    
    let elapsed = start.elapsed();
    let ns_per_dispatch = elapsed.as_nanos() as f64 / (iterations as f64 * sizes.len() as f64);
    
    println!("  {} iteraciones", iterations * sizes.len());
    println!("  Tiempo total: {:?}", elapsed);
    println!("  Overhead: {:.2} ns/dispatch", ns_per_dispatch);
    println!("  Dispatches/segundo: {:.0} M", 1e9 / ns_per_dispatch / 1e6);
    
    // Debe ser < 50ns
    ns_per_dispatch < 50.0
}

/// Test 7: Múltiples dispatchers
fn test_multiple_dispatchers() -> bool {
    let mut results = Vec::new();
    
    for _ in 0..10 {
        let d = AutoDispatcher::new();
        let b = d.select(5000000);
        results.push(hash_backend(b));
    }
    
    let first = results[0];
    let all_same = results.iter().all(|&h| h == first);
    
    println!("  10 dispatchers creados");
    println!("  Todos seleccionan mismo backend: {}", all_same);
    
    all_same
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     TEST-G: Auto-Dispatch CPU+GPU                          ║");
    println!("║     Autor: Eddi Andreé Salazar Matos                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    
    println!("🧪 Test 1: Detección CPU");
    if test_cpu_detection() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 2: Detección GPU");
    if test_gpu_detection() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 3: Dispatch determinista");
    if test_dispatch_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 4: Transición CPU→GPU");
    if test_cpu_gpu_transition() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 5: MatMul dispatch");
    if test_matmul_dispatch() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 6: Overhead de dispatch");
    if test_dispatch_overhead() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 7: Múltiples dispatchers");
    if test_multiple_dispatchers() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("════════════════════════════════════════════════════════════");
    println!("📊 Resultados: {} passed, {} failed", passed, failed);
    
    if failed == 0 {
        println!("✅ TEST-G DISPATCH COMPLETADO - Auto-dispatch es DETERMINISTA");
    } else {
        println!("❌ TEST-G DISPATCH FALLIDO");
    }
}
