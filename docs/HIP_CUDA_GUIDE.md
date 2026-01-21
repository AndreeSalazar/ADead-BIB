# ADead-BIB - Guía de Integración HIP-CPU + CUDA

## Visión General

ADead-BIB v2.5 introduce una **API de Compute Unificada** que permite escribir código una vez y ejecutarlo en múltiples backends:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    ADead-BIB Compute Backend v2.5                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                   API Unificada (compute::)                      │   │
│  │   compute::parallel_for(n, |i| { ... })                         │   │
│  │   compute::matmul(A, B, C)                                      │   │
│  │   compute::reduce(data, |a,b| a + b)                            │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                              │                                          │
│              ┌───────────────┼───────────────┐                         │
│              ▼               ▼               ▼                         │
│  ┌──────────────────┐ ┌──────────────┐ ┌──────────────┐                │
│  │   CUDA Backend   │ │  HIP-CPU     │ │   Vulkan     │                │
│  │   (RTX 3060)     │ │  (Fallback)  │ │   (Portable) │                │
│  │                  │ │              │ │              │                │
│  │  • PTX directo   │ │ • CPU SIMD   │ │ • SPIR-V     │                │
│  │  • cuBLAS        │ │ • Paralelo   │ │ • Compute    │                │
│  │  • Tensor Cores  │ │ • AVX2/512   │ │ • Shaders    │                │
│  └──────────────────┘ └──────────────┘ └──────────────┘                │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Componentes

### 1. HIP-CPU (`backend/gpu/hip/hip_cpu.rs`)

Ejecuta kernels estilo CUDA/HIP en CPU usando:
- **Paralelismo con threads** - Aprovecha todos los cores
- **SIMD** - AVX2/AVX512 cuando disponible
- **Fallback seguro** - Funciona sin GPU

```rust
use adead_bib::backend::gpu::HipCpuRuntime;

let runtime = HipCpuRuntime::with_default_config();

// Parallel for simple
runtime.parallel_for(1000, |i| {
    result[i] = a[i] + b[i];
});

// Operaciones vectoriales
runtime.vector_add(&a, &b, &mut c);
runtime.saxpy(2.5, &x, &mut y);

// Multiplicación de matrices
runtime.matmul(&a, &b, &mut c, m, n, k);
```

### 2. HIP Runtime (`backend/gpu/hip/hip_runtime.rs`)

Detección automática de backend y generación de código portable:

```rust
use adead_bib::backend::gpu::{detect_hip_backend, HipBackend};

let backend = detect_hip_backend();
match backend {
    HipBackend::Cuda => println!("NVIDIA GPU detectada"),
    HipBackend::Rocm => println!("AMD GPU detectada"),
    HipBackend::Cpu => println!("Usando CPU como fallback"),
}
```

### 3. CUDA to HIP Translator (`backend/gpu/hip/cuda_to_hip.rs`)

Traduce código CUDA a HIP para portabilidad:

```rust
use adead_bib::backend::gpu::translate_cuda_file;

let cuda_code = r#"
    cudaMalloc(&ptr, size);
    cudaMemcpy(dst, src, size, cudaMemcpyHostToDevice);
"#;

let hip_code = translate_cuda_file(cuda_code);
// Resultado: hipMalloc, hipMemcpy, etc.
```

### 4. Compute API Unificada (`backend/gpu/compute.rs`)

API de alto nivel que abstrae el backend:

```rust
use adead_bib::backend::gpu::ComputeRuntime;

// Auto-detecta el mejor backend
let runtime = ComputeRuntime::new();

// Operaciones vectoriales
runtime.vector_add(&a, &b, &mut c);
runtime.saxpy(alpha, &x, &mut y);
let dot = runtime.dot_product(&a, &b);

// Matrices
runtime.matmul(&a, &b, &mut c, m, n, k);
runtime.transpose(&a, &mut b, rows, cols);

// Reducciones
let sum = runtime.reduce_sum(&data);
let max = runtime.reduce_max(&data);
let min = runtime.reduce_min(&data);

// Parallel for personalizado
runtime.parallel_for(n, |i| {
    // Tu código aquí
});
```

## Uso Básico

### Ejemplo 1: Vector Add

```rust
use adead_bib::backend::gpu::ComputeRuntime;

fn main() {
    let runtime = ComputeRuntime::new();
    
    let n = 100_000;
    let a: Vec<f32> = (0..n).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..n).map(|i| (i * 2) as f32).collect();
    let mut c = vec![0.0f32; n];
    
    runtime.vector_add(&a, &b, &mut c);
    
    println!("c[0] = {}", c[0]); // 0
    println!("c[99999] = {}", c[99999]); // 299997
}
```

### Ejemplo 2: Matrix Multiply

```rust
use adead_bib::backend::gpu::ComputeRuntime;

fn main() {
    let runtime = ComputeRuntime::new();
    
    let m = 256;
    let a = vec![1.0f32; m * m];
    let b = vec![2.0f32; m * m];
    let mut c = vec![0.0f32; m * m];
    
    runtime.matmul(&a, &b, &mut c, m, m, m);
    
    // Cada elemento de C = m * 1.0 * 2.0 = 512
    println!("c[0] = {}", c[0]);
}
```

### Ejemplo 3: Benchmark

```rust
use adead_bib::backend::gpu::ComputeRuntime;

fn main() {
    let runtime = ComputeRuntime::new();
    
    // Mostrar info del sistema
    runtime.print_info();
    
    // Ejecutar benchmark
    let results = runtime.benchmark();
    println!("{}", results);
}
```

## Configuración Avanzada

### Forzar Backend Específico

```rust
use adead_bib::backend::gpu::{ComputeRuntime, ComputeBackend};

// Forzar HIP-CPU (útil para debugging)
let runtime = ComputeRuntime::with_backend(ComputeBackend::HipCpu);

// Forzar CUDA
let runtime = ComputeRuntime::with_backend(ComputeBackend::Cuda);
```

### Configuración Personalizada

```rust
use adead_bib::backend::gpu::{ComputeRuntime, ComputeConfig};

let config = ComputeConfig {
    preferred_backend: None, // Auto-detect
    cpu_threads: 8,          // 8 threads
    enable_simd: true,       // Usar AVX2/512
    verbose: true,           // Logging
    default_block_size: (256, 1, 1),
};

let runtime = ComputeRuntime::with_config(config);
```

## Generación de Código GPU

### Generar Código CUDA/HIP Portable

```rust
use adead_bib::backend::gpu::{HipCodeGen, HipBackend};

let mut codegen = HipCodeGen::auto_detect();

// Añadir kernels
codegen.add_vector_add();
codegen.add_saxpy();
codegen.add_matmul();
codegen.add_matmul_shared(); // Con shared memory

// Generar código
let code = codegen.generate();
println!("{}", code);
```

## Arquitectura de Archivos

```
src/rust/backend/gpu/
├── hip/
│   ├── mod.rs           # Módulo principal
│   ├── hip_cpu.rs       # HIP-CPU runtime (fallback CPU)
│   ├── hip_runtime.rs   # Detección y codegen HIP
│   └── cuda_to_hip.rs   # Traductor CUDA → HIP
├── compute.rs           # API unificada de compute
├── cuda/
│   └── runtime.rs       # Backend CUDA nativo
└── ...
```

## Beneficios

| Característica | Descripción |
|----------------|-------------|
| **Auto-dispatch** | Detecta GPU → usa CUDA; sin GPU → usa HIP-CPU |
| **Código único** | Escribes una vez, corre en GPU o CPU |
| **Ultra-optimizado** | CUDA nativo para tu RTX 3060 |
| **Fallback robusto** | HIP-CPU usa SIMD (AVX2) en CPU |
| **Debugging fácil** | Depura en CPU, ejecuta en GPU |
| **Portable** | Mismo código funciona en NVIDIA y AMD |

## Rendimiento Esperado (RTX 3060)

| Operación | Tamaño | Tiempo Estimado |
|-----------|--------|-----------------|
| Vector Add | 1M elementos | ~0.5 ms |
| SAXPY | 1M elementos | ~0.5 ms |
| MatMul | 256x256 | ~1 ms |
| MatMul | 1024x1024 | ~10 ms |
| Reduce Sum | 1M elementos | ~0.3 ms |

## Próximos Pasos

1. **cuBLAS Integration** - Usar cuBLAS para MatMul ultra-optimizado
2. **Tensor Cores** - Aprovechar FP16 en RTX 3060
3. **Async Streams** - Overlap compute + transfer
4. **Multi-GPU** - Soporte para múltiples GPUs
