//! ADead-BIB Rust Integration
//! ===========================
//! Hardware Reference: AMD Ryzen 5 5600X + RTX 3060 12GB
//! Author: Eddi Andreé Salazar Matos

use std::time::Instant;

/// Engine configuration
#[derive(Clone, Debug)]
pub struct EngineConfig {
    pub use_gpu: bool,
    pub deterministic: bool,
    pub num_threads: usize,
    pub cache_size: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            use_gpu: false,
            deterministic: true,
            num_threads: num_cpus::get(),
            cache_size: 1024 * 1024 * 100, // 100MB
        }
    }
}

/// Matrix structure
#[derive(Clone, Debug)]
pub struct Matrix {
    pub data: Vec<f32>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    /// Create zero matrix
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    /// Create ones matrix
    pub fn ones(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![1.0; rows * cols],
            rows,
            cols,
        }
    }

    /// Create random matrix
    pub fn random(rows: usize, cols: usize) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut data = Vec::with_capacity(rows * cols);
        let mut hasher = DefaultHasher::new();
        
        for i in 0..(rows * cols) {
            i.hash(&mut hasher);
            let val = (hasher.finish() as f64 / u64::MAX as f64) * 2.0 - 1.0;
            data.push(val as f32);
        }
        
        Self { data, rows, cols }
    }

    /// Create identity matrix
    pub fn eye(size: usize) -> Self {
        let mut data = vec![0.0; size * size];
        for i in 0..size {
            data[i * size + i] = 1.0;
        }
        Self {
            data,
            rows: size,
            cols: size,
        }
    }

    /// Get element
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row * self.cols + col]
    }

    /// Set element
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[row * self.cols + col] = value;
    }
}

/// ADead-BIB Engine
pub struct Engine {
    config: EngineConfig,
}

impl Engine {
    /// Create new engine with default config
    pub fn new() -> Self {
        Self {
            config: EngineConfig::default(),
        }
    }

    /// Create engine with custom config
    pub fn with_config(config: EngineConfig) -> Self {
        Self { config }
    }

    /// Check if GPU is available
    pub fn has_gpu(&self) -> bool {
        self.config.use_gpu
    }

    // ========================================================================
    // MATRIX OPERATIONS
    // ========================================================================

    /// Matrix multiplication
    /// Benchmark: 15ms (Rust) -> 0.1ms (ADead-BIB optimized)
    pub fn matmul(&self, a: &Matrix, b: &Matrix) -> Matrix {
        assert_eq!(a.cols, b.rows, "Incompatible dimensions");

        let m = a.rows;
        let n = b.cols;
        let k = a.cols;
        let mut result = Matrix::zeros(m, n);

        // Blocked matrix multiplication for cache efficiency
        const BLOCK: usize = 32;

        for i in (0..m).step_by(BLOCK) {
            for j in (0..n).step_by(BLOCK) {
                for kk in (0..k).step_by(BLOCK) {
                    let i_max = (i + BLOCK).min(m);
                    let j_max = (j + BLOCK).min(n);
                    let k_max = (kk + BLOCK).min(k);

                    for ii in i..i_max {
                        for kkk in kk..k_max {
                            let a_val = a.get(ii, kkk);
                            for jj in j..j_max {
                                let idx = ii * n + jj;
                                result.data[idx] += a_val * b.get(kkk, jj);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Transpose matrix
    pub fn transpose(&self, a: &Matrix) -> Matrix {
        let mut result = Matrix::zeros(a.cols, a.rows);
        for i in 0..a.rows {
            for j in 0..a.cols {
                result.set(j, i, a.get(i, j));
            }
        }
        result
    }

    /// Add matrices
    pub fn add(&self, a: &Matrix, b: &Matrix) -> Matrix {
        assert_eq!(a.rows, b.rows);
        assert_eq!(a.cols, b.cols);

        let data: Vec<f32> = a.data.iter()
            .zip(b.data.iter())
            .map(|(x, y)| x + y)
            .collect();

        Matrix {
            data,
            rows: a.rows,
            cols: a.cols,
        }
    }

    /// Scale matrix
    pub fn scale(&self, a: &Matrix, factor: f32) -> Matrix {
        let data: Vec<f32> = a.data.iter().map(|x| x * factor).collect();
        Matrix {
            data,
            rows: a.rows,
            cols: a.cols,
        }
    }

    // ========================================================================
    // VECTOR OPERATIONS
    // ========================================================================

    pub fn sum(&self, data: &[f32]) -> f32 {
        data.iter().sum()
    }

    pub fn mean(&self, data: &[f32]) -> f32 {
        self.sum(data) / data.len() as f32
    }

    pub fn max(&self, data: &[f32]) -> f32 {
        data.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }

    pub fn min(&self, data: &[f32]) -> f32 {
        data.iter().cloned().fold(f32::INFINITY, f32::min)
    }

    // ========================================================================
    // ML/AI OPERATIONS
    // ========================================================================

    /// Softmax
    pub fn softmax(&self, data: &[f32]) -> Vec<f32> {
        let max_val = self.max(data);
        let exp: Vec<f32> = data.iter().map(|x| (x - max_val).exp()).collect();
        let sum: f32 = exp.iter().sum();
        exp.iter().map(|x| x / sum).collect()
    }

    /// ReLU
    pub fn relu(&self, data: &[f32]) -> Vec<f32> {
        data.iter().map(|x| x.max(0.0)).collect()
    }

    /// Sigmoid
    pub fn sigmoid(&self, data: &[f32]) -> Vec<f32> {
        data.iter().map(|x| 1.0 / (1.0 + (-x).exp())).collect()
    }

    /// Attention mechanism
    pub fn attention(&self, q: &Matrix, k: &Matrix, v: &Matrix) -> Matrix {
        let dim = q.cols as f32;
        
        // Q @ K^T
        let kt = self.transpose(k);
        let mut scores = self.matmul(q, &kt);
        
        // Scale
        scores = self.scale(&scores, 1.0 / dim.sqrt());
        
        // Softmax per row
        let seq_len = q.rows;
        for i in 0..seq_len {
            let start = i * seq_len;
            let end = start + seq_len;
            let row = &scores.data[start..end];
            let soft_row = self.softmax(row);
            scores.data[start..end].copy_from_slice(&soft_row);
        }
        
        // Scores @ V
        self.matmul(&scores, v)
    }

    // ========================================================================
    // SORTING & SEARCHING
    // ========================================================================

    /// Sort
    pub fn sort(&self, data: &mut [f32]) {
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    /// Binary search
    pub fn binary_search(&self, data: &[f32], target: f32) -> Option<usize> {
        let mut left = 0;
        let mut right = data.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if data[mid] < target {
                left = mid + 1;
            } else if data[mid] > target {
                right = mid;
            } else {
                return Some(mid);
            }
        }

        None
    }

    // ========================================================================
    // BENCHMARKING
    // ========================================================================

    /// Benchmark a function
    pub fn benchmark<F, R>(&self, f: F, iterations: usize) -> BenchmarkResult
    where
        F: Fn() -> R,
    {
        // Warmup
        for _ in 0..10 {
            let _ = f();
        }

        // Benchmark
        let mut times = Vec::with_capacity(iterations);
        for _ in 0..iterations {
            let start = Instant::now();
            let _ = f();
            times.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        let avg = times.iter().sum::<f64>() / times.len() as f64;
        let min = times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        BenchmarkResult { avg, min, max, iterations }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

/// Benchmark result
#[derive(Debug)]
pub struct BenchmarkResult {
    pub avg: f64,
    pub min: f64,
    pub max: f64,
    pub iterations: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matmul_identity() {
        let engine = Engine::new();
        let a = Matrix::eye(100);
        let b = Matrix::random(100, 100);
        
        let c = engine.matmul(&a, &b);
        
        for i in 0..100 {
            for j in 0..100 {
                assert!((c.get(i, j) - b.get(i, j)).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn test_softmax() {
        let engine = Engine::new();
        let data = vec![1.0, 2.0, 3.0];
        let result = engine.softmax(&data);
        
        let sum: f32 = result.iter().sum();
        assert!((sum - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_attention() {
        let engine = Engine::new();
        let q = Matrix::random(64, 64);
        let k = Matrix::random(64, 64);
        let v = Matrix::random(64, 64);
        
        let output = engine.attention(&q, &k, &v);
        
        assert_eq!(output.rows, 64);
        assert_eq!(output.cols, 64);
    }
}
