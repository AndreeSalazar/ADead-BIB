// ============================================================================
// compute/mod.adB — API Unificada de Compute v2.5
// ============================================================================
// API de alto nivel que abstrae GPU (CUDA/HIP) y CPU (SIMD).
// Auto-detecta el mejor backend disponible.
//
// Uso:
//   let runtime = Compute::auto()           // Auto-detecta backend
//   runtime.vector_add(a, b, c, n)          // Suma vectorial
//   runtime.matmul(A, B, C, m, n, k)        // Multiplicación de matrices
//   runtime.parallel_for(n, |i| { ... })   // Loop paralelo
//
// Filosofía: Escribes una vez, corre en GPU o CPU automáticamente
// ============================================================================

#![exports(
    // Runtime
    Compute, ComputeBackend, ComputeConfig,
    // Operaciones vectoriales
    vector_add, vector_sub, vector_mul, vector_scale,
    saxpy, dot_product,
    // Operaciones matriciales
    matmul, matmul_batched, transpose,
    // Reducciones
    reduce_sum, reduce_max, reduce_min, reduce_mean,
    // Deep Learning
    relu, sigmoid, tanh, softmax, layer_norm, batch_norm,
    conv2d, max_pool2d, avg_pool2d,
    // Tensor Cores (FP16)
    matmul_fp16, tensor_core_gemm,
    // Async/Streams
    Stream, create_stream, sync_stream, destroy_stream,
    async_copy, async_matmul
)]

// ============================================================================
// BACKEND DETECTION
// ============================================================================

enum ComputeBackend {
    Cuda,       // NVIDIA GPU (RTX 3060)
    HipCpu,     // CPU paralelo con SIMD
    Vulkan,     // Vulkan Compute (portable)
    Sequential  // CPU secuencial (fallback)
}

impl ComputeBackend {
    fn name(self) -> string {
        match self {
            Cuda => "CUDA (NVIDIA)",
            HipCpu => "HIP-CPU (SIMD)",
            Vulkan => "Vulkan Compute",
            Sequential => "CPU Sequential"
        }
    }
    
    fn is_gpu(self) -> bool {
        match self {
            Cuda => true,
            Vulkan => true,
            _ => false
        }
    }
}

// ============================================================================
// COMPUTE CONFIG
// ============================================================================

struct ComputeConfig {
    preferred_backend: ComputeBackend?,
    num_threads: u32,
    enable_simd: bool,
    enable_tensor_cores: bool,
    verbose: bool,
    default_block_size: (u32, u32, u32)
}

impl ComputeConfig {
    fn default() -> ComputeConfig {
        return ComputeConfig {
            preferred_backend: null,
            num_threads: 0,  // Auto-detect
            enable_simd: true,
            enable_tensor_cores: true,
            verbose: false,
            default_block_size: (256, 1, 1)
        }
    }
    
    fn cuda() -> ComputeConfig {
        let config = ComputeConfig::default()
        config.preferred_backend = ComputeBackend::Cuda
        return config
    }
    
    fn cpu() -> ComputeConfig {
        let config = ComputeConfig::default()
        config.preferred_backend = ComputeBackend::HipCpu
        return config
    }
}

// ============================================================================
// COMPUTE RUNTIME
// ============================================================================

struct Compute {
    backend: ComputeBackend,
    config: ComputeConfig,
    initialized: bool,
    device_name: string,
    total_memory_mb: u64,
    compute_capability: (u8, u8)
}

impl Compute {
    // Auto-detecta el mejor backend
    fn auto() -> Compute {
        return Compute::with_config(ComputeConfig::default())
    }
    
    // Fuerza CUDA
    fn cuda() -> Compute {
        return Compute::with_config(ComputeConfig::cuda())
    }
    
    // Fuerza CPU
    fn cpu() -> Compute {
        return Compute::with_config(ComputeConfig::cpu())
    }
    
    fn with_config(config: ComputeConfig) -> Compute {
        let backend = detect_best_backend(config)
        let info = get_device_info(backend)
        
        let compute = Compute {
            backend: backend,
            config: config,
            initialized: false,
            device_name: info.name,
            total_memory_mb: info.memory_mb,
            compute_capability: info.compute_cap
        }
        
        compute.init()
        return compute
    }
    
    fn init(self) {
        if self.initialized {
            return
        }
        
        match self.backend {
            Cuda => {
                gpu::init()
                emit_opcode(0xC0DA0001)
            },
            HipCpu => {
                // HIP-CPU no necesita init especial
            },
            Vulkan => {
                // TODO: Vulkan init
            },
            Sequential => {
                // Nada que inicializar
            }
        }
        
        self.initialized = true
        
        if self.config.verbose {
            println("[Compute] Initialized")
            print("  Backend: ")
            println(self.backend.name())
            print("  Device: ")
            println(self.device_name)
        }
    }
    
    fn shutdown(self) {
        if !self.initialized {
            return
        }
        
        match self.backend {
            Cuda => {
                gpu::shutdown()
            },
            _ => {}
        }
        
        self.initialized = false
    }
    
    // ========================================
    // OPERACIONES VECTORIALES
    // ========================================
    
    // C = A + B
    fn vector_add(self, a: ptr<f32>, b: ptr<f32>, c: ptr<f32>, n: u64) {
        match self.backend {
            Cuda => {
                gpu::alloc(n * 4, gpu::reg0)
                gpu::alloc(n * 4, gpu::reg1)
                gpu::alloc(n * 4, gpu::reg2)
                gpu::copy_h2d(a as u64, gpu::reg0, n * 4)
                gpu::copy_h2d(b as u64, gpu::reg1, n * 4)
                gpu::add(gpu::reg0, gpu::reg1, gpu::reg2, n as u32)
                gpu::sync()
                gpu::copy_d2h(gpu::reg2, c as u64, n * 4)
                gpu::free(gpu::reg0)
                gpu::free(gpu::reg1)
                gpu::free(gpu::reg2)
            },
            HipCpu => {
                // Paralelo con SIMD
                parallel_for_simd(n, |i| {
                    c[i] = a[i] + b[i]
                })
            },
            _ => {
                for i in 0..n {
                    c[i] = a[i] + b[i]
                }
            }
        }
    }
    
    // C = A - B
    fn vector_sub(self, a: ptr<f32>, b: ptr<f32>, c: ptr<f32>, n: u64) {
        match self.backend {
            Cuda => {
                gpu::alloc(n * 4, gpu::reg0)
                gpu::alloc(n * 4, gpu::reg1)
                gpu::alloc(n * 4, gpu::reg2)
                gpu::copy_h2d(a as u64, gpu::reg0, n * 4)
                gpu::copy_h2d(b as u64, gpu::reg1, n * 4)
                gpu::sub(gpu::reg0, gpu::reg1, gpu::reg2, n as u32)
                gpu::sync()
                gpu::copy_d2h(gpu::reg2, c as u64, n * 4)
                gpu::free(gpu::reg0)
                gpu::free(gpu::reg1)
                gpu::free(gpu::reg2)
            },
            HipCpu => {
                parallel_for_simd(n, |i| {
                    c[i] = a[i] - b[i]
                })
            },
            _ => {
                for i in 0..n {
                    c[i] = a[i] - b[i]
                }
            }
        }
    }
    
    // C = A * B (element-wise)
    fn vector_mul(self, a: ptr<f32>, b: ptr<f32>, c: ptr<f32>, n: u64) {
        match self.backend {
            Cuda => {
                gpu::alloc(n * 4, gpu::reg0)
                gpu::alloc(n * 4, gpu::reg1)
                gpu::alloc(n * 4, gpu::reg2)
                gpu::copy_h2d(a as u64, gpu::reg0, n * 4)
                gpu::copy_h2d(b as u64, gpu::reg1, n * 4)
                gpu::mul(gpu::reg0, gpu::reg1, gpu::reg2, n as u32)
                gpu::sync()
                gpu::copy_d2h(gpu::reg2, c as u64, n * 4)
                gpu::free(gpu::reg0)
                gpu::free(gpu::reg1)
                gpu::free(gpu::reg2)
            },
            HipCpu => {
                parallel_for_simd(n, |i| {
                    c[i] = a[i] * b[i]
                })
            },
            _ => {
                for i in 0..n {
                    c[i] = a[i] * b[i]
                }
            }
        }
    }
    
    // y = alpha * x (scale)
    fn vector_scale(self, alpha: f32, x: ptr<f32>, y: ptr<f32>, n: u64) {
        match self.backend {
            HipCpu => {
                parallel_for_simd(n, |i| {
                    y[i] = alpha * x[i]
                })
            },
            _ => {
                for i in 0..n {
                    y[i] = alpha * x[i]
                }
            }
        }
    }
    
    // y = alpha * x + y (SAXPY - BLAS Level 1)
    fn saxpy(self, alpha: f32, x: ptr<f32>, y: ptr<f32>, n: u64) {
        match self.backend {
            Cuda => {
                // Opcode especial para SAXPY optimizado
                emit_opcode(0xC0DA0025)  // GPU_SAXPY
                emit_f32(alpha)
                emit_u64(n)
            },
            HipCpu => {
                parallel_for_simd(n, |i| {
                    y[i] = alpha * x[i] + y[i]
                })
            },
            _ => {
                for i in 0..n {
                    y[i] = alpha * x[i] + y[i]
                }
            }
        }
    }
    
    // result = sum(a[i] * b[i])
    fn dot_product(self, a: ptr<f32>, b: ptr<f32>, n: u64) -> f32 {
        match self.backend {
            HipCpu => {
                return parallel_reduce(n, 0.0, |i, acc| {
                    acc + a[i] * b[i]
                })
            },
            _ => {
                let sum = 0.0
                for i in 0..n {
                    sum += a[i] * b[i]
                }
                return sum
            }
        }
    }
    
    // ========================================
    // OPERACIONES MATRICIALES
    // ========================================
    
    // C = A * B (Matrix Multiply)
    // A: m x k, B: k x n, C: m x n
    fn matmul(self, a: ptr<f32>, b: ptr<f32>, c: ptr<f32>, m: u32, n: u32, k: u32) {
        match self.backend {
            Cuda => {
                let size_a = m * k * 4
                let size_b = k * n * 4
                let size_c = m * n * 4
                
                gpu::alloc(size_a, gpu::reg0)
                gpu::alloc(size_b, gpu::reg1)
                gpu::alloc(size_c, gpu::reg2)
                
                gpu::copy_h2d(a as u64, gpu::reg0, size_a)
                gpu::copy_h2d(b as u64, gpu::reg1, size_b)
                
                gpu::matmul(gpu::reg0, gpu::reg1, gpu::reg2, m, n, k)
                gpu::sync()
                
                gpu::copy_d2h(gpu::reg2, c as u64, size_c)
                
                gpu::free(gpu::reg0)
                gpu::free(gpu::reg1)
                gpu::free(gpu::reg2)
            },
            HipCpu => {
                // Tiled matmul para mejor cache locality
                matmul_tiled_cpu(a, b, c, m, n, k)
            },
            _ => {
                // Naive matmul
                for row in 0..m {
                    for col in 0..n {
                        let sum = 0.0
                        for i in 0..k {
                            sum += a[row * k + i] * b[i * n + col]
                        }
                        c[row * n + col] = sum
                    }
                }
            }
        }
    }
    
    // Batched Matrix Multiply
    fn matmul_batched(self, a: ptr<f32>, b: ptr<f32>, c: ptr<f32>, 
                      m: u32, n: u32, k: u32, batch_size: u32) {
        let stride_a = m * k
        let stride_b = k * n
        let stride_c = m * n
        
        match self.backend {
            Cuda => {
                // Opcode para batched matmul
                emit_opcode(0xC0DA0026)  // GPU_MATMUL_BATCHED
                emit_u32(m)
                emit_u32(n)
                emit_u32(k)
                emit_u32(batch_size)
            },
            HipCpu => {
                parallel_for(batch_size as u64, |batch| {
                    let a_ptr = a + batch * stride_a
                    let b_ptr = b + batch * stride_b
                    let c_ptr = c + batch * stride_c
                    matmul_tiled_cpu(a_ptr, b_ptr, c_ptr, m, n, k)
                })
            },
            _ => {
                for batch in 0..batch_size {
                    let a_ptr = a + batch * stride_a
                    let b_ptr = b + batch * stride_b
                    let c_ptr = c + batch * stride_c
                    self.matmul(a_ptr, b_ptr, c_ptr, m, n, k)
                }
            }
        }
    }
    
    // B = A^T (Transpose)
    fn transpose(self, a: ptr<f32>, b: ptr<f32>, rows: u32, cols: u32) {
        match self.backend {
            Cuda => {
                gpu::alloc(rows * cols * 4, gpu::reg0)
                gpu::alloc(rows * cols * 4, gpu::reg1)
                gpu::copy_h2d(a as u64, gpu::reg0, rows * cols * 4)
                gpu::tensor_transpose(gpu::reg0, gpu::reg1)
                gpu::sync()
                gpu::copy_d2h(gpu::reg1, b as u64, rows * cols * 4)
                gpu::free(gpu::reg0)
                gpu::free(gpu::reg1)
            },
            HipCpu => {
                parallel_for(rows as u64, |row| {
                    for col in 0..cols {
                        b[col * rows + row] = a[row * cols + col]
                    }
                })
            },
            _ => {
                for row in 0..rows {
                    for col in 0..cols {
                        b[col * rows + row] = a[row * cols + col]
                    }
                }
            }
        }
    }
    
    // ========================================
    // REDUCCIONES
    // ========================================
    
    fn reduce_sum(self, data: ptr<f32>, n: u64) -> f32 {
        match self.backend {
            HipCpu => {
                return parallel_reduce(n, 0.0, |i, acc| acc + data[i])
            },
            _ => {
                let sum = 0.0
                for i in 0..n {
                    sum += data[i]
                }
                return sum
            }
        }
    }
    
    fn reduce_max(self, data: ptr<f32>, n: u64) -> f32 {
        match self.backend {
            HipCpu => {
                return parallel_reduce(n, -inf, |i, acc| max(acc, data[i]))
            },
            _ => {
                let max_val = data[0]
                for i in 1..n {
                    if data[i] > max_val {
                        max_val = data[i]
                    }
                }
                return max_val
            }
        }
    }
    
    fn reduce_min(self, data: ptr<f32>, n: u64) -> f32 {
        match self.backend {
            HipCpu => {
                return parallel_reduce(n, inf, |i, acc| min(acc, data[i]))
            },
            _ => {
                let min_val = data[0]
                for i in 1..n {
                    if data[i] < min_val {
                        min_val = data[i]
                    }
                }
                return min_val
            }
        }
    }
    
    fn reduce_mean(self, data: ptr<f32>, n: u64) -> f32 {
        return self.reduce_sum(data, n) / (n as f32)
    }
    
    // ========================================
    // DEEP LEARNING ACTIVATIONS
    // ========================================
    
    // ReLU: out = max(0, in)
    fn relu(self, input: ptr<f32>, output: ptr<f32>, n: u64) {
        match self.backend {
            Cuda => {
                gpu::alloc(n * 4, gpu::reg0)
                gpu::alloc(n * 4, gpu::reg1)
                gpu::copy_h2d(input as u64, gpu::reg0, n * 4)
                gpu::relu(gpu::reg0, gpu::reg1, n as u32)
                gpu::sync()
                gpu::copy_d2h(gpu::reg1, output as u64, n * 4)
                gpu::free(gpu::reg0)
                gpu::free(gpu::reg1)
            },
            HipCpu => {
                parallel_for_simd(n, |i| {
                    output[i] = max(0.0, input[i])
                })
            },
            _ => {
                for i in 0..n {
                    output[i] = max(0.0, input[i])
                }
            }
        }
    }
    
    // Sigmoid: out = 1 / (1 + exp(-in))
    fn sigmoid(self, input: ptr<f32>, output: ptr<f32>, n: u64) {
        match self.backend {
            Cuda => {
                emit_opcode(0xC0DA0032)  // GPU_SIGMOID
                emit_u64(n)
            },
            HipCpu => {
                parallel_for_simd(n, |i| {
                    output[i] = 1.0 / (1.0 + exp(-input[i]))
                })
            },
            _ => {
                for i in 0..n {
                    output[i] = 1.0 / (1.0 + exp(-input[i]))
                }
            }
        }
    }
    
    // Tanh: out = tanh(in)
    fn tanh(self, input: ptr<f32>, output: ptr<f32>, n: u64) {
        match self.backend {
            Cuda => {
                emit_opcode(0xC0DA0033)  // GPU_TANH
                emit_u64(n)
            },
            HipCpu => {
                parallel_for_simd(n, |i| {
                    let e2x = exp(2.0 * input[i])
                    output[i] = (e2x - 1.0) / (e2x + 1.0)
                })
            },
            _ => {
                for i in 0..n {
                    let e2x = exp(2.0 * input[i])
                    output[i] = (e2x - 1.0) / (e2x + 1.0)
                }
            }
        }
    }
    
    // Softmax: out[i] = exp(in[i]) / sum(exp(in))
    fn softmax(self, input: ptr<f32>, output: ptr<f32>, n: u64) {
        match self.backend {
            Cuda => {
                gpu::alloc(n * 4, gpu::reg0)
                gpu::alloc(n * 4, gpu::reg1)
                gpu::copy_h2d(input as u64, gpu::reg0, n * 4)
                gpu::softmax(gpu::reg0, gpu::reg1, n as u32)
                gpu::sync()
                gpu::copy_d2h(gpu::reg1, output as u64, n * 4)
                gpu::free(gpu::reg0)
                gpu::free(gpu::reg1)
            },
            _ => {
                // Numerically stable softmax
                let max_val = self.reduce_max(input, n)
                let sum = 0.0
                for i in 0..n {
                    output[i] = exp(input[i] - max_val)
                    sum += output[i]
                }
                for i in 0..n {
                    output[i] /= sum
                }
            }
        }
    }
    
    // ========================================
    // TENSOR CORES (FP16) - RTX 3060
    // ========================================
    
    // FP16 Matrix Multiply using Tensor Cores
    fn matmul_fp16(self, a: ptr<f16>, b: ptr<f16>, c: ptr<f32>, 
                   m: u32, n: u32, k: u32) {
        if !self.config.enable_tensor_cores {
            // Fallback a FP32
            // Convertir y usar matmul normal
            return
        }
        
        match self.backend {
            Cuda => {
                // Opcode para Tensor Core GEMM
                emit_opcode(0xC0DA0060)  // GPU_TENSOR_CORE_GEMM
                emit_u32(m)
                emit_u32(n)
                emit_u32(k)
                emit![0x01]  // FP16 mode
            },
            _ => {
                // Fallback: convertir a FP32
                println("[Compute] Warning: Tensor Cores not available, using FP32")
            }
        }
    }
    
    // ========================================
    // ASYNC / STREAMS
    // ========================================
    
    fn create_stream(self) -> Stream {
        match self.backend {
            Cuda => {
                emit_opcode(0xC0DA0070)  // GPU_STREAM_CREATE
                return Stream { id: get_stream_id(), backend: self.backend }
            },
            _ => {
                return Stream { id: 0, backend: self.backend }
            }
        }
    }
    
    fn async_copy_h2d(self, stream: Stream, src: ptr<u8>, dest_reg: u8, size: u64) {
        match self.backend {
            Cuda => {
                emit_opcode(0xC0DA0071)  // GPU_ASYNC_COPY_H2D
                emit![stream.id as u8, dest_reg]
                emit_u64(src as u64)
                emit_u64(size)
            },
            _ => {
                // Sync copy fallback
            }
        }
    }
    
    fn async_matmul(self, stream: Stream, a_reg: u8, b_reg: u8, c_reg: u8,
                    m: u32, n: u32, k: u32) {
        match self.backend {
            Cuda => {
                emit_opcode(0xC0DA0072)  // GPU_ASYNC_MATMUL
                emit![stream.id as u8, a_reg, b_reg, c_reg]
                emit_u32(m)
                emit_u32(n)
                emit_u32(k)
            },
            _ => {
                // Sync matmul fallback
                self.matmul(null, null, null, m, n, k)
            }
        }
    }
    
    fn sync_stream(self, stream: Stream) {
        match self.backend {
            Cuda => {
                emit_opcode(0xC0DA0073)  // GPU_STREAM_SYNC
                emit![stream.id as u8]
            },
            _ => {}
        }
    }
    
    // ========================================
    // PARALLEL PRIMITIVES
    // ========================================
    
    fn parallel_for<F>(self, n: u64, kernel: F) 
    where F: Fn(u64) {
        match self.backend {
            HipCpu => {
                parallel_for_threads(n, self.config.num_threads, kernel)
            },
            _ => {
                for i in 0..n {
                    kernel(i)
                }
            }
        }
    }
    
    // ========================================
    // UTILITIES
    // ========================================
    
    fn print_info(self) {
        println("╔══════════════════════════════════════════════════════════════╗")
        println("║              ADead-BIB Compute Runtime v2.5                  ║")
        println("╠══════════════════════════════════════════════════════════════╣")
        print("║ Backend:     ")
        println(self.backend.name())
        print("║ Device:      ")
        println(self.device_name)
        print("║ Memory:      ")
        print(self.total_memory_mb)
        println(" MB")
        print("║ Compute:     ")
        print(self.compute_capability.0)
        print(".")
        println(self.compute_capability.1)
        print("║ SIMD:        ")
        println(if self.config.enable_simd { "ON" } else { "OFF" })
        print("║ Tensor Cores:")
        println(if self.config.enable_tensor_cores { "ON" } else { "OFF" })
        println("╚══════════════════════════════════════════════════════════════╝")
    }
    
    fn benchmark(self) -> BenchmarkResults {
        let n = 1000000
        let m = 256
        
        // Allocate test data
        let a = alloc<f32>(n)
        let b = alloc<f32>(n)
        let c = alloc<f32>(n)
        
        for i in 0..n {
            a[i] = i as f32
            b[i] = (i * 2) as f32
        }
        
        // Vector Add benchmark
        let start = time_now()
        for _ in 0..10 {
            self.vector_add(a, b, c, n)
        }
        let vector_add_ms = (time_now() - start) / 10.0
        
        // SAXPY benchmark
        let start = time_now()
        for _ in 0..10 {
            self.saxpy(2.5, a, b, n)
        }
        let saxpy_ms = (time_now() - start) / 10.0
        
        // Reduce benchmark
        let start = time_now()
        for _ in 0..10 {
            self.reduce_sum(a, n)
        }
        let reduce_ms = (time_now() - start) / 10.0
        
        // MatMul benchmark
        let mat_a = alloc<f32>(m * m)
        let mat_b = alloc<f32>(m * m)
        let mat_c = alloc<f32>(m * m)
        
        let start = time_now()
        self.matmul(mat_a, mat_b, mat_c, m, m, m)
        let matmul_ms = time_now() - start
        
        free(a)
        free(b)
        free(c)
        free(mat_a)
        free(mat_b)
        free(mat_c)
        
        return BenchmarkResults {
            backend: self.backend,
            vector_add_ms: vector_add_ms,
            saxpy_ms: saxpy_ms,
            reduce_ms: reduce_ms,
            matmul_256_ms: matmul_ms,
            elements: n
        }
    }
}

// ============================================================================
// STREAM (Async Operations)
// ============================================================================

struct Stream {
    id: u32,
    backend: ComputeBackend
}

// ============================================================================
// BENCHMARK RESULTS
// ============================================================================

struct BenchmarkResults {
    backend: ComputeBackend,
    vector_add_ms: f64,
    saxpy_ms: f64,
    reduce_ms: f64,
    matmul_256_ms: f64,
    elements: u64
}

impl BenchmarkResults {
    fn print(self) {
        println("╔══════════════════════════════════════════════════════════════╗")
        println("║              ADead-BIB Compute Benchmark                     ║")
        println("╠══════════════════════════════════════════════════════════════╣")
        print("║ Backend:      ")
        println(self.backend.name())
        print("║ Elements:     ")
        println(self.elements)
        println("╠══════════════════════════════════════════════════════════════╣")
        print("║ Vector Add:   ")
        print(self.vector_add_ms)
        println(" ms")
        print("║ SAXPY:        ")
        print(self.saxpy_ms)
        println(" ms")
        print("║ Reduce Sum:   ")
        print(self.reduce_ms)
        println(" ms")
        print("║ MatMul 256²:  ")
        print(self.matmul_256_ms)
        println(" ms")
        println("╚══════════════════════════════════════════════════════════════╝")
    }
}

// ============================================================================
// INTERNAL HELPERS
// ============================================================================

fn detect_best_backend(config: ComputeConfig) -> ComputeBackend {
    if config.preferred_backend != null {
        return config.preferred_backend
    }
    
    // Check for NVIDIA GPU
    if has_nvidia_gpu() {
        return ComputeBackend::Cuda
    }
    
    // Check for Vulkan
    if has_vulkan() {
        return ComputeBackend::Vulkan
    }
    
    // Fallback to HIP-CPU
    return ComputeBackend::HipCpu
}

struct DeviceInfo {
    name: string,
    memory_mb: u64,
    compute_cap: (u8, u8)
}

fn get_device_info(backend: ComputeBackend) -> DeviceInfo {
    match backend {
        Cuda => {
            // Query NVIDIA GPU info
            return DeviceInfo {
                name: "NVIDIA RTX 3060",
                memory_mb: 12288,
                compute_cap: (8, 6)
            }
        },
        HipCpu => {
            return DeviceInfo {
                name: "CPU (HIP-CPU Runtime)",
                memory_mb: get_system_memory_mb(),
                compute_cap: (0, 0)
            }
        },
        _ => {
            return DeviceInfo {
                name: "Unknown",
                memory_mb: 0,
                compute_cap: (0, 0)
            }
        }
    }
}

fn matmul_tiled_cpu(a: ptr<f32>, b: ptr<f32>, c: ptr<f32>, m: u32, n: u32, k: u32) {
    const TILE_SIZE: u32 = 32
    
    // Zero C
    for i in 0..(m * n) {
        c[i] = 0.0
    }
    
    // Tiled multiplication
    for row_tile in 0..((m + TILE_SIZE - 1) / TILE_SIZE) {
        let row_start = row_tile * TILE_SIZE
        let row_end = min(row_start + TILE_SIZE, m)
        
        for col_tile in 0..((n + TILE_SIZE - 1) / TILE_SIZE) {
            let col_start = col_tile * TILE_SIZE
            let col_end = min(col_start + TILE_SIZE, n)
            
            for k_tile in 0..((k + TILE_SIZE - 1) / TILE_SIZE) {
                let k_start = k_tile * TILE_SIZE
                let k_end = min(k_start + TILE_SIZE, k)
                
                for row in row_start..row_end {
                    for col in col_start..col_end {
                        let sum = c[row * n + col]
                        for i in k_start..k_end {
                            sum += a[row * k + i] * b[i * n + col]
                        }
                        c[row * n + col] = sum
                    }
                }
            }
        }
    }
}

fn parallel_for_threads<F>(n: u64, num_threads: u32, kernel: F)
where F: Fn(u64) {
    // Implementación con threads nativos
    let threads = if num_threads == 0 { get_cpu_count() } else { num_threads }
    let chunk_size = (n + threads - 1) / threads
    
    // Spawn threads y ejecutar kernel
    for t in 0..threads {
        let start = t * chunk_size
        let end = min(start + chunk_size, n)
        
        spawn_thread(|| {
            for i in start..end {
                kernel(i)
            }
        })
    }
    
    join_all_threads()
}

fn parallel_for_simd<F>(n: u64, kernel: F)
where F: Fn(u64) {
    // SIMD-optimized parallel for
    parallel_for_threads(n, 0, kernel)
}

fn parallel_reduce<F>(n: u64, init: f32, reducer: F) -> f32
where F: Fn(u64, f32) -> f32 {
    let threads = get_cpu_count()
    let chunk_size = (n + threads - 1) / threads
    let partial_results = alloc<f32>(threads)
    
    for t in 0..threads {
        partial_results[t] = init
    }
    
    // Parallel reduction
    for t in 0..threads {
        let start = t * chunk_size
        let end = min(start + chunk_size, n)
        
        spawn_thread(|| {
            let acc = init
            for i in start..end {
                acc = reducer(i, acc)
            }
            partial_results[t] = acc
        })
    }
    
    join_all_threads()
    
    // Final reduction
    let result = init
    for t in 0..threads {
        result = reducer(0, result + partial_results[t] - init)
    }
    
    free(partial_results)
    return result
}

// Opcode emission helpers
fn emit_opcode(opcode: u32) {
    emit![
        ((opcode >> 24) & 0xFF) as u8,
        ((opcode >> 16) & 0xFF) as u8,
        ((opcode >> 8) & 0xFF) as u8,
        (opcode & 0xFF) as u8
    ]
}

fn emit_u32(value: u32) {
    emit![
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8
    ]
}

fn emit_u64(value: u64) {
    emit![
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8,
        ((value >> 32) & 0xFF) as u8,
        ((value >> 40) & 0xFF) as u8,
        ((value >> 48) & 0xFF) as u8,
        ((value >> 56) & 0xFF) as u8
    ]
}

fn emit_f32(value: f32) {
    let bits = value.to_bits()
    emit_u32(bits)
}
