// ============================================================
// ADead-BIB - Unified Compute API
// ============================================================
// API unificada para computación paralela que abstrae:
// - Vulkan Compute (Portable - SPIR-V)
// - CPU Parallel (Rayon - fallback)
//
// Filosofía: SPIR-V portable para OpenGL/Vulkan, CPU fallback con Rayon
// ============================================================

use rayon::prelude::*;

/// Backend de compute seleccionado
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeBackend {
    /// Vulkan Compute (SPIR-V)
    Vulkan,
    /// CPU Parallel (Rayon fallback)
    CpuParallel,
}

impl ComputeBackend {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Vulkan => "Vulkan (SPIR-V)",
            Self::CpuParallel => "CPU (Rayon)",
        }
    }

    /// Detecta el mejor backend disponible
    pub fn detect_best() -> Self {
        // Por ahora, siempre usar CPU Parallel
        // TODO: Detectar Vulkan cuando esté implementado
        Self::CpuParallel
    }
}

/// Configuración del runtime de compute
#[derive(Debug, Clone)]
pub struct ComputeConfig {
    /// Backend preferido (None = auto-detect)
    pub preferred_backend: Option<ComputeBackend>,
    /// Número de threads para CPU (0 = auto)
    pub cpu_threads: usize,
    /// Verbose logging
    pub verbose: bool,
}

impl Default for ComputeConfig {
    fn default() -> Self {
        Self {
            preferred_backend: None,
            cpu_threads: 0, // Auto
            verbose: false,
        }
    }
}

/// Runtime unificado de compute
pub struct ComputeRuntime {
    backend: ComputeBackend,
    config: ComputeConfig,
}

impl ComputeRuntime {
    /// Crea un nuevo runtime con auto-detección
    pub fn new() -> Self {
        Self::with_config(ComputeConfig::default())
    }

    /// Crea un runtime con configuración específica
    pub fn with_config(config: ComputeConfig) -> Self {
        let backend = config
            .preferred_backend
            .unwrap_or_else(ComputeBackend::detect_best);

        if config.verbose {
            println!("[Compute] Backend: {}", backend.name());
        }

        Self {
            backend,
            config,
        }
    }

    /// Fuerza un backend específico
    pub fn with_backend(backend: ComputeBackend) -> Self {
        let mut config = ComputeConfig::default();
        config.preferred_backend = Some(backend);
        Self::with_config(config)
    }

    /// Obtiene el backend actual
    pub fn backend(&self) -> ComputeBackend {
        self.backend
    }

    // ========================================
    // API de Alto Nivel
    // ========================================

    /// Ejecuta una operación paralela sobre un rango usando Rayon
    pub fn parallel_for<F>(&self, n: usize, kernel: F)
    where
        F: Fn(usize) + Sync + Send,
    {
        (0..n).into_par_iter().for_each(kernel);
    }

    /// Lanza un kernel con dimensiones grid/block (simplificado para CPU)
    pub fn launch<F>(&self, grid: (u32, u32, u32), block: (u32, u32, u32), kernel: F)
    where
        F: Fn(usize) + Sync + Send,
    {
        let total = (grid.0 * block.0) as usize;
        self.parallel_for(total, kernel);
    }

    // ========================================
    // Operaciones Vectoriales
    // ========================================

    /// Vector Add: C = A + B
    pub fn vector_add(&self, a: &[f32], b: &[f32], c: &mut [f32]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), c.len());

        a.par_iter()
            .zip(b.par_iter())
            .zip(c.par_iter_mut())
            .for_each(|((&x, &y), z)| *z = x + y);
    }

    /// SAXPY: y = alpha * x + y
    pub fn saxpy(&self, alpha: f32, x: &[f32], y: &mut [f32]) {
        assert_eq!(x.len(), y.len());

        x.par_iter()
            .zip(y.par_iter_mut())
            .for_each(|(&xi, yi)| *yi = alpha * xi + *yi);
    }

    /// Vector Scale: y = alpha * x
    pub fn vector_scale(&self, alpha: f32, x: &[f32], y: &mut [f32]) {
        assert_eq!(x.len(), y.len());

        x.par_iter()
            .zip(y.par_iter_mut())
            .for_each(|(&xi, yi)| *yi = alpha * xi);
    }

    /// Dot Product: result = sum(a[i] * b[i])
    pub fn dot_product(&self, a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());

        a.par_iter()
            .zip(b.par_iter())
            .map(|(&x, &y)| x * y)
            .sum()
    }

    // ========================================
    // Operaciones de Matrices
    // ========================================

    /// Matrix Multiply: C = A * B (simplificado con Rayon)
    /// A: m x k, B: k x n, C: m x n
    pub fn matmul(&self, a: &[f32], b: &[f32], c: &mut [f32], m: usize, n: usize, k: usize) {
        assert_eq!(a.len(), m * k);
        assert_eq!(b.len(), k * n);
        assert_eq!(c.len(), m * n);

        (0..m).into_par_iter().for_each(|i| {
            for j in 0..n {
                let mut sum = 0.0f32;
                for l in 0..k {
                    sum += a[i * k + l] * b[l * n + j];
                }
                c[i * n + j] = sum;
            }
        });
    }

    /// Matrix Transpose: B = A^T
    pub fn transpose(&self, a: &[f32], b: &mut [f32], rows: usize, cols: usize) {
        assert_eq!(a.len(), rows * cols);
        assert_eq!(b.len(), rows * cols);

        (0..rows).into_par_iter().for_each(|row| {
            for col in 0..cols {
                b[col * rows + row] = a[row * cols + col];
            }
        });
    }

    // ========================================
    // Reducciones
    // ========================================

    /// Reduce Sum
    pub fn reduce_sum(&self, data: &[f32]) -> f32 {
        data.par_iter().sum()
    }

    /// Reduce Max
    pub fn reduce_max(&self, data: &[f32]) -> f32 {
        if data.is_empty() {
            return f32::NEG_INFINITY;
        }
        data.par_iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }

    /// Reduce Min
    pub fn reduce_min(&self, data: &[f32]) -> f32 {
        if data.is_empty() {
            return f32::INFINITY;
        }
        data.par_iter().cloned().fold(f32::INFINITY, f32::min)
    }

    // ========================================
    // Utilidades
    // ========================================

    /// Imprime información del runtime
    pub fn print_info(&self) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║              ADead-BIB Compute Runtime                       ║");
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ Backend:     {:<48}                                          ║", self.backend.name());
        println!("║ Threads:     {:<48}                                          ║", rayon::current_num_threads());
        println!("╚══════════════════════════════════════════════════════════════╝");
    }

    /// Benchmark simple
    pub fn benchmark(&self) -> BenchmarkResults {
        use std::time::Instant;

        let n = 1_000_000;
        let a: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..n).map(|i| (i * 2) as f32).collect();
        let mut c = vec![0.0f32; n];

        // Vector Add
        let start = Instant::now();
        for _ in 0..10 {
            self.vector_add(&a, &b, &mut c);
        }
        let vector_add_time = start.elapsed().as_secs_f64() / 10.0;

        // SAXPY
        let mut y = b.clone();
        let start = Instant::now();
        for _ in 0..10 {
            self.saxpy(2.5, &a, &mut y);
        }
        let saxpy_time = start.elapsed().as_secs_f64() / 10.0;

        // Reduce
        let start = Instant::now();
        let mut sum = 0.0f32;
        for _ in 0..10 {
            sum = self.reduce_sum(&a);
        }
        let reduce_time = start.elapsed().as_secs_f64() / 10.0;

        // MatMul (smaller)
        let m = 256;
        let mat_a = vec![1.0f32; m * m];
        let mat_b = vec![2.0f32; m * m];
        let mut mat_c = vec![0.0f32; m * m];

        let start = Instant::now();
        self.matmul(&mat_a, &mat_b, &mut mat_c, m, m, m);
        let matmul_time = start.elapsed().as_secs_f64();

        BenchmarkResults {
            backend: self.backend,
            vector_add_ms: vector_add_time * 1000.0,
            saxpy_ms: saxpy_time * 1000.0,
            reduce_ms: reduce_time * 1000.0,
            matmul_256_ms: matmul_time * 1000.0,
            elements: n,
        }
    }
}

impl Default for ComputeRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Resultados de benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub backend: ComputeBackend,
    pub vector_add_ms: f64,
    pub saxpy_ms: f64,
    pub reduce_ms: f64,
    pub matmul_256_ms: f64,
    pub elements: usize,
}

impl std::fmt::Display for BenchmarkResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "╔══════════════════════════════════════════════════════════════╗"
        )?;
        writeln!(
            f,
            "║              ADead-BIB Compute Benchmark                      ║"
        )?;
        writeln!(
            f,
            "╠══════════════════════════════════════════════════════════════╣"
        )?;
        writeln!(f, "║ Backend:      {:<47} ║", self.backend.name())?;
        writeln!(f, "║ Elements:     {:<47} ║", self.elements)?;
        writeln!(
            f,
            "╠══════════════════════════════════════════════════════════════╣"
        )?;
        writeln!(
            f,
            "║ Vector Add:   {:>10.3} ms                                  ║",
            self.vector_add_ms
        )?;
        writeln!(
            f,
            "║ SAXPY:        {:>10.3} ms                                  ║",
            self.saxpy_ms
        )?;
        writeln!(
            f,
            "║ Reduce Sum:   {:>10.3} ms                                  ║",
            self.reduce_ms
        )?;
        writeln!(
            f,
            "║ MatMul 256²:  {:>10.3} ms                                  ║",
            self.matmul_256_ms
        )?;
        writeln!(
            f,
            "╚══════════════════════════════════════════════════════════════╝"
        )?;
        Ok(())
    }
}

// ========================================
// Funciones de conveniencia (API global)
// ========================================

/// Crea un runtime con auto-detección
pub fn create_runtime() -> ComputeRuntime {
    ComputeRuntime::new()
}

/// Crea un runtime forzando CPU Parallel
pub fn create_cpu_runtime() -> ComputeRuntime {
    ComputeRuntime::with_backend(ComputeBackend::CpuParallel)
}

/// Detecta el mejor backend disponible
pub fn detect_backend() -> ComputeBackend {
    ComputeBackend::detect_best()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_runtime() {
        let runtime = ComputeRuntime::new();
        assert_eq!(runtime.backend(), ComputeBackend::CpuParallel);
    }

    #[test]
    fn test_vector_add() {
        let runtime = ComputeRuntime::new();
        let n = 1000;

        let a: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..n).map(|i| i as f32 * 2.0).collect();
        let mut c = vec![0.0f32; n];

        runtime.vector_add(&a, &b, &mut c);

        for i in 0..n {
            assert!((c[i] - (a[i] + b[i])).abs() < 1e-6);
        }
    }

    #[test]
    fn test_saxpy() {
        let runtime = ComputeRuntime::new();
        let n = 1000;
        let alpha = 2.5f32;

        let x: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let mut y: Vec<f32> = (0..n).map(|i| i as f32 * 0.5).collect();
        let y_orig = y.clone();

        runtime.saxpy(alpha, &x, &mut y);

        for i in 0..n {
            let expected = alpha * x[i] + y_orig[i];
            assert!((y[i] - expected).abs() < 1e-5);
        }
    }

    #[test]
    fn test_dot_product() {
        let runtime = ComputeRuntime::new();

        let a = vec![1.0f32, 2.0, 3.0, 4.0];
        let b = vec![1.0f32, 1.0, 1.0, 1.0];

        let result = runtime.dot_product(&a, &b);
        assert!((result - 10.0).abs() < 1e-5);
    }

    #[test]
    fn test_reduce() {
        let runtime = ComputeRuntime::new();

        let data: Vec<f32> = (0..100).map(|i| i as f32).collect();

        let sum = runtime.reduce_sum(&data);
        let expected: f32 = (0..100).map(|i| i as f32).sum();
        assert!((sum - expected).abs() < 1e-2);

        let max = runtime.reduce_max(&data);
        assert!((max - 99.0).abs() < 1e-5);

        let min = runtime.reduce_min(&data);
        assert!((min - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_matmul() {
        let runtime = ComputeRuntime::new();
        let m = 32;

        let a = vec![1.0f32; m * m];
        let b = vec![2.0f32; m * m];
        let mut c = vec![0.0f32; m * m];

        runtime.matmul(&a, &b, &mut c, m, m, m);

        // Cada elemento debería ser m * 1.0 * 2.0 = 2m
        let expected = (m as f32) * 2.0;
        for val in &c {
            assert!((*val - expected).abs() < 1e-3);
        }
    }

    #[test]
    fn test_transpose() {
        let runtime = ComputeRuntime::new();

        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut b = vec![0.0f32; 6];

        runtime.transpose(&a, &mut b, 2, 3);

        // Transpuesta de 2x3 es 3x2
        assert!((b[0] - 1.0).abs() < 1e-5); // (0,0)
        assert!((b[1] - 4.0).abs() < 1e-5); // (1,0)
        assert!((b[2] - 2.0).abs() < 1e-5); // (0,1)
        assert!((b[3] - 5.0).abs() < 1e-5); // (1,1)
        assert!((b[4] - 3.0).abs() < 1e-5); // (0,2)
        assert!((b[5] - 6.0).abs() < 1e-5); // (1,2)
    }
}
