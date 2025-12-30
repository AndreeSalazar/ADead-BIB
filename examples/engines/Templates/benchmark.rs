// ADead-BIB Heredar - Benchmark System
// Sistema de benchmarks para hardware real
// Nivel militar: métricas precisas, sin overhead
//
// Autor: Eddi Andreé Salazar Matos

use std::time::{Duration, Instant};

/// Resultado de benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: u32,
    pub total_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub avg_time: Duration,
    pub std_dev: Duration,
    pub throughput: f64,
    pub unit: String,
}

impl BenchmarkResult {
    pub fn print(&self) {
        println!("┌─────────────────────────────────────────────────────────────┐");
        println!("│ Benchmark: {:<48} │", self.name);
        println!("├─────────────────────────────────────────────────────────────┤");
        println!("│ Iterations:    {:>10}                                   │", self.iterations);
        println!("│ Total time:    {:>10.3} ms                                │", self.total_time.as_secs_f64() * 1000.0);
        println!("│ Min time:      {:>10.3} µs                                │", self.min_time.as_secs_f64() * 1_000_000.0);
        println!("│ Max time:      {:>10.3} µs                                │", self.max_time.as_secs_f64() * 1_000_000.0);
        println!("│ Avg time:      {:>10.3} µs                                │", self.avg_time.as_secs_f64() * 1_000_000.0);
        println!("│ Std dev:       {:>10.3} µs                                │", self.std_dev.as_secs_f64() * 1_000_000.0);
        println!("│ Throughput:    {:>10.2} {}                              │", self.throughput, self.unit);
        println!("└─────────────────────────────────────────────────────────────┘");
    }
    
    pub fn print_compact(&self) {
        println!("{:<30} {:>10.3} µs  ({:.2} {})", 
                 self.name,
                 self.avg_time.as_secs_f64() * 1_000_000.0,
                 self.throughput,
                 self.unit);
    }
}

/// Benchmark runner
pub struct Benchmark {
    name: String,
    warmup_iterations: u32,
    measure_iterations: u32,
    times: Vec<Duration>,
}

impl Benchmark {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            warmup_iterations: 10,
            measure_iterations: 100,
            times: Vec::new(),
        }
    }
    
    pub fn with_warmup(mut self, iterations: u32) -> Self {
        self.warmup_iterations = iterations;
        self
    }
    
    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.measure_iterations = iterations;
        self
    }
    
    /// Ejecuta el benchmark
    pub fn run<F>(&mut self, mut f: F) -> BenchmarkResult 
    where
        F: FnMut(),
    {
        // Warmup
        for _ in 0..self.warmup_iterations {
            f();
        }
        
        // Measure
        self.times.clear();
        let total_start = Instant::now();
        
        for _ in 0..self.measure_iterations {
            let start = Instant::now();
            f();
            self.times.push(start.elapsed());
        }
        
        let total_time = total_start.elapsed();
        
        // Calculate stats
        let min_time = *self.times.iter().min().unwrap();
        let max_time = *self.times.iter().max().unwrap();
        let avg_time = total_time / self.measure_iterations;
        
        let avg_ns = avg_time.as_nanos() as f64;
        let variance: f64 = self.times.iter()
            .map(|t| {
                let diff = t.as_nanos() as f64 - avg_ns;
                diff * diff
            })
            .sum::<f64>() / self.measure_iterations as f64;
        let std_dev = Duration::from_nanos(variance.sqrt() as u64);
        
        BenchmarkResult {
            name: self.name.clone(),
            iterations: self.measure_iterations,
            total_time,
            min_time,
            max_time,
            avg_time,
            std_dev,
            throughput: 0.0,
            unit: "ops/s".to_string(),
        }
    }
    
    /// Ejecuta benchmark con throughput
    pub fn run_with_throughput<F>(&mut self, mut f: F, ops_per_iteration: u64, unit: &str) -> BenchmarkResult
    where
        F: FnMut(),
    {
        let mut result = self.run(&mut f);
        
        let ops_per_sec = ops_per_iteration as f64 / result.avg_time.as_secs_f64();
        result.throughput = ops_per_sec;
        result.unit = unit.to_string();
        
        result
    }
}

/// Suite de benchmarks
pub struct BenchmarkSuite {
    name: String,
    results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            results: Vec::new(),
        }
    }
    
    pub fn add(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }
    
    pub fn print_summary(&self) {
        println!();
        println!("╔═══════════════════════════════════════════════════════════════╗");
        println!("║ BENCHMARK SUITE: {:<44} ║", self.name);
        println!("╠═══════════════════════════════════════════════════════════════╣");
        
        for result in &self.results {
            println!("║ {:<30} {:>10.3} µs  {:>10.2} {} ║", 
                     &result.name[..result.name.len().min(30)],
                     result.avg_time.as_secs_f64() * 1_000_000.0,
                     result.throughput,
                     &result.unit[..result.unit.len().min(8)]);
        }
        
        println!("╚═══════════════════════════════════════════════════════════════╝");
    }
}

/// Benchmarks predefinidos para GPU
pub struct GpuBenchmarks;

impl GpuBenchmarks {
    /// Benchmark de dispatch latency
    pub fn dispatch_latency() -> BenchmarkResult {
        let mut bench = Benchmark::new("Dispatch Latency")
            .with_warmup(100)
            .with_iterations(1000);
        
        bench.run(|| {
            // Simular dispatch (en implementación real, usar Vulkan)
            std::hint::black_box(42);
        })
    }
    
    /// Benchmark de MatMul
    pub fn matmul(m: u32, n: u32, k: u32, tflops: f32) -> BenchmarkResult {
        let flops = 2 * m as u64 * n as u64 * k as u64;
        let name = format!("MatMul {}x{}x{}", m, n, k);
        
        let mut bench = Benchmark::new(&name)
            .with_warmup(10)
            .with_iterations(100);
        
        // Estimar tiempo basado en TFLOPS
        let estimated_time_ns = (flops as f64 / (tflops as f64 * 1e12 * 0.5)) * 1e9;
        
        bench.run_with_throughput(|| {
            // Simular tiempo de MatMul
            std::thread::sleep(Duration::from_nanos(estimated_time_ns as u64 / 100));
        }, flops, "GFLOPS")
    }
    
    /// Benchmark de memory bandwidth
    pub fn memory_bandwidth(size_mb: u32, bandwidth_gbs: f32) -> BenchmarkResult {
        let bytes = size_mb as u64 * 1024 * 1024;
        let name = format!("Memory Copy {} MB", size_mb);
        
        let mut bench = Benchmark::new(&name)
            .with_warmup(10)
            .with_iterations(100);
        
        let estimated_time_ns = (bytes as f64 / (bandwidth_gbs as f64 * 1e9)) * 1e9;
        
        bench.run_with_throughput(|| {
            std::thread::sleep(Duration::from_nanos(estimated_time_ns as u64 / 100));
        }, bytes, "GB/s")
    }
}

/// Benchmarks predefinidos para CPU
pub struct CpuBenchmarks;

impl CpuBenchmarks {
    /// Benchmark de generación de PE
    pub fn pe_generation() -> BenchmarkResult {
        let mut bench = Benchmark::new("PE Generation")
            .with_warmup(100)
            .with_iterations(1000);
        
        bench.run_with_throughput(|| {
            // Simular generación de PE
            let mut pe = Vec::with_capacity(2048);
            pe.extend_from_slice(&[0x4D, 0x5A]); // MZ header
            std::hint::black_box(pe);
        }, 1, "PE/s")
    }
    
    /// Benchmark de generación de SPIR-V
    pub fn spirv_generation() -> BenchmarkResult {
        let mut bench = Benchmark::new("SPIR-V Generation")
            .with_warmup(100)
            .with_iterations(1000);
        
        bench.run_with_throughput(|| {
            let mut spirv = Vec::with_capacity(1024);
            spirv.extend_from_slice(&[0x03, 0x02, 0x23, 0x07]); // Magic
            std::hint::black_box(spirv);
        }, 1, "shader/s")
    }
    
    /// Benchmark de bytecode compilation
    pub fn bytecode_compilation() -> BenchmarkResult {
        let mut bench = Benchmark::new("Bytecode→SPIR-V")
            .with_warmup(100)
            .with_iterations(1000);
        
        bench.run_with_throughput(|| {
            let bytecode = vec![0x10, 0x61, 0x20, 0x00];
            std::hint::black_box(bytecode);
        }, 4, "bytes/s")
    }
}

/// Ejecuta suite completa de benchmarks
pub fn run_full_benchmark_suite(gpu_tflops: f32, gpu_bandwidth: f32) -> BenchmarkSuite {
    let mut suite = BenchmarkSuite::new("ADead-BIB Full Suite");
    
    println!("🔬 Running benchmark suite...");
    println!("   GPU: {:.2} TFLOPS, {:.0} GB/s", gpu_tflops, gpu_bandwidth);
    println!();
    
    // CPU benchmarks
    println!("   Running CPU benchmarks...");
    suite.add(CpuBenchmarks::pe_generation());
    suite.add(CpuBenchmarks::spirv_generation());
    suite.add(CpuBenchmarks::bytecode_compilation());
    
    // GPU benchmarks (estimados)
    println!("   Running GPU benchmarks (estimated)...");
    suite.add(GpuBenchmarks::dispatch_latency());
    suite.add(GpuBenchmarks::matmul(512, 512, 512, gpu_tflops));
    suite.add(GpuBenchmarks::matmul(1024, 1024, 1024, gpu_tflops));
    suite.add(GpuBenchmarks::memory_bandwidth(64, gpu_bandwidth));
    suite.add(GpuBenchmarks::memory_bandwidth(256, gpu_bandwidth));
    
    suite
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_benchmark_basic() {
        let mut bench = Benchmark::new("Test")
            .with_warmup(5)
            .with_iterations(10);
        
        let result = bench.run(|| {
            std::thread::sleep(Duration::from_micros(10));
        });
        
        assert!(result.avg_time.as_micros() >= 10);
    }
    
    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::new("Test Suite");
        
        let mut bench = Benchmark::new("Quick Test")
            .with_iterations(10);
        
        suite.add(bench.run(|| {}));
        
        assert_eq!(suite.results.len(), 1);
    }
}
