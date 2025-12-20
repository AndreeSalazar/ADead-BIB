# 🚀 Ideas-6: ADead-BIB Universal Runtime

> **Autor:** Eddi Andreé Salazar Matos | **Hecho con ❤️ en Perú** 🇵🇪

---

## 🎯 Objetivo

Crear un **Runtime Universal Ultra Ligero** para ADead-BIB que sea:

1. **Compatible con TODOS los lenguajes** - C++, Rust, Python, C, Zig, etc.
2. **Determinista** - Comportamiento predecible y reproducible
3. **Ultra ligero** - Sin dependencias pesadas (NO como LLVM)
4. **Puro/Virgen** - Sin capas intermedias innecesarias
5. **Compatible con Vulkan** - Para gráficos y compute GPU

---

## 🏗️ Arquitectura del Runtime

```
┌─────────────────────────────────────────────────────────────────────┐
│                     ADead-BIB Universal Runtime                      │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │   C++ FFI   │  │  Rust FFI   │  │ Python FFI  │  │   C FFI     │ │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘ │
│         │                │                │                │        │
│         └────────────────┼────────────────┼────────────────┘        │
│                          ▼                                          │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │                    RUNTIME CORE (Determinista)                │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐              │   │
│  │  │  Memory    │  │  Opcodes   │  │  Scheduler │              │   │
│  │  │  Manager   │  │  Engine    │  │  (Tasks)   │              │   │
│  │  └────────────┘  └────────────┘  └────────────┘              │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                          │                                          │
│         ┌────────────────┼────────────────┐                        │
│         ▼                ▼                ▼                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                 │
│  │   CPU       │  │   GPU       │  │   Vulkan    │                 │
│  │  Backend    │  │  Backend    │  │  Backend    │                 │
│  │  (x86-64)   │  │  (CUDA)     │  │  (Compute)  │                 │
│  └─────────────┘  └─────────────┘  └─────────────┘                 │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 📁 Estructura de Carpetas Propuesta

```
ADead-BIB/
├── src/
│   ├── rust/                    # Compilador principal (existente)
│   │   ├── frontend/            # Lexer, Parser, AST
│   │   └── backend/             # Codegen, PE
│   └── cpp/                     # Emitter C++ (existente)
│       └── emitter/
│
├── runtime/                     # 🆕 NUEVO: Runtime Universal
│   ├── core/                    # Núcleo del runtime
│   │   ├── memory.h             # Gestor de memoria determinista
│   │   ├── memory.cpp
│   │   ├── opcodes.h            # Motor de opcodes
│   │   ├── opcodes.cpp
│   │   ├── scheduler.h          # Scheduler de tareas
│   │   ├── scheduler.cpp
│   │   ├── types.h              # Tipos universales
│   │   └── runtime.h            # API principal del runtime
│   │
│   ├── backends/                # Backends de ejecución
│   │   ├── cpu/                 # Backend CPU (x86-64, ARM)
│   │   │   ├── x86_64.h
│   │   │   ├── x86_64.cpp
│   │   │   ├── arm64.h
│   │   │   └── arm64.cpp
│   │   │
│   │   ├── gpu/                 # Backend GPU (CUDA)
│   │   │   ├── cuda_backend.h
│   │   │   ├── cuda_backend.cpp
│   │   │   └── cuda_kernels.cu
│   │   │
│   │   └── vulkan/              # Backend Vulkan
│   │       ├── vulkan_backend.h
│   │       ├── vulkan_backend.cpp
│   │       ├── vulkan_compute.h
│   │       ├── vulkan_compute.cpp
│   │       └── shaders/
│   │           ├── matmul.comp
│   │           ├── attention.comp
│   │           └── softmax.comp
│   │
│   ├── ffi/                     # Foreign Function Interfaces
│   │   ├── cpp/                 # FFI para C++
│   │   │   ├── adead_cpp.h
│   │   │   └── adead_cpp.cpp
│   │   │
│   │   ├── rust/                # FFI para Rust
│   │   │   ├── lib.rs
│   │   │   └── Cargo.toml
│   │   │
│   │   ├── python/              # FFI para Python
│   │   │   ├── adead_py.cpp
│   │   │   └── setup.py
│   │   │
│   │   ├── c/                   # FFI para C puro
│   │   │   ├── adead_c.h
│   │   │   └── adead_c.c
│   │   │
│   │   └── zig/                 # FFI para Zig
│   │       └── adead_zig.zig
│   │
│   └── tests/                   # Tests del runtime
│       ├── test_memory.cpp
│       ├── test_opcodes.cpp
│       └── test_vulkan.cpp
│
├── hex/                         # Opcodes GPU (existente)
├── python/                      # Scripts Python (existente)
└── docs/                        # Documentación
```

---

## 🔧 Runtime Core - Diseño

### 1. Memory Manager (Determinista)

```cpp
// runtime/core/memory.h
#pragma once
#include <cstdint>
#include <cstddef>

namespace adead {

// Allocator determinista - sin fragmentación
class DeterministicAllocator {
public:
    static constexpr size_t BLOCK_SIZE = 4096;  // 4KB blocks
    static constexpr size_t MAX_BLOCKS = 65536; // 256MB max
    
    void* alloc(size_t size);
    void free(void* ptr);
    void reset();  // Libera todo de una vez
    
    size_t used() const;
    size_t available() const;
    
private:
    uint8_t* m_pool;
    size_t m_offset;
    size_t m_capacity;
};

// Arena allocator para operaciones temporales
class ArenaAllocator {
public:
    explicit ArenaAllocator(size_t size);
    ~ArenaAllocator();
    
    void* alloc(size_t size, size_t alignment = 8);
    void reset();
    
private:
    uint8_t* m_base;
    size_t m_offset;
    size_t m_capacity;
};

} // namespace adead
```

### 2. Opcodes Engine

```cpp
// runtime/core/opcodes.h
#pragma once
#include <cstdint>

namespace adead {

// Opcodes del runtime (compatibles con HEX existente)
enum class Opcode : uint32_t {
    // Control
    NOP         = 0x00000000,
    HALT        = 0x000000FF,
    
    // Memoria
    ALLOC       = 0x00010000,
    FREE        = 0x00010001,
    LOAD        = 0x00010010,
    STORE       = 0x00010011,
    
    // Aritmética
    ADD         = 0x00020000,
    SUB         = 0x00020001,
    MUL         = 0x00020002,
    DIV         = 0x00020003,
    
    // Matrices (AI)
    MATMUL      = 0x00030000,
    TRANSPOSE   = 0x00030001,
    SOFTMAX     = 0x00030002,
    RELU        = 0x00030003,
    
    // GPU
    GPU_INIT    = 0xC0DA0001,
    GPU_ALLOC   = 0xC0DA0010,
    GPU_MATMUL  = 0xC0DA0020,
    GPU_SYNC    = 0xC0DA00F0,
    
    // Vulkan
    VK_INIT     = 0xVK000001,
    VK_COMPUTE  = 0xVK000010,
    VK_SUBMIT   = 0xVK000020,
    VK_SYNC     = 0xVK0000F0,
};

// Instrucción del runtime
struct Instruction {
    Opcode opcode;
    uint32_t operands[4];
    
    static Instruction decode(const uint8_t* bytes);
    void encode(uint8_t* bytes) const;
};

// Motor de ejecución
class OpcodeEngine {
public:
    void execute(const Instruction& inst);
    void execute_batch(const Instruction* insts, size_t count);
    
    void set_backend(class Backend* backend);
    
private:
    Backend* m_backend;
    DeterministicAllocator m_alloc;
};

} // namespace adead
```

### 3. Backend Interface

```cpp
// runtime/core/backend.h
#pragma once

namespace adead {

// Interface para todos los backends
class Backend {
public:
    virtual ~Backend() = default;
    
    virtual bool init() = 0;
    virtual void shutdown() = 0;
    
    // Memoria
    virtual void* alloc(size_t size) = 0;
    virtual void free(void* ptr) = 0;
    
    // Operaciones
    virtual void matmul(const float* a, const float* b, float* c,
                        int m, int n, int k) = 0;
    virtual void softmax(const float* in, float* out, int rows, int cols) = 0;
    virtual void relu(const float* in, float* out, int size) = 0;
    
    // Sincronización
    virtual void sync() = 0;
    
    // Info
    virtual const char* name() const = 0;
    virtual size_t memory_available() const = 0;
};

// Backends concretos
class CPUBackend : public Backend { /* ... */ };
class CUDABackend : public Backend { /* ... */ };
class VulkanBackend : public Backend { /* ... */ };

} // namespace adead
```

---

## 🎮 Integración con Vulkan

### Vulkan Compute Backend

```cpp
// runtime/backends/vulkan/vulkan_backend.h
#pragma once
#include <vulkan/vulkan.h>
#include "../core/backend.h"

namespace adead {

class VulkanBackend : public Backend {
public:
    bool init() override;
    void shutdown() override;
    
    // Compute shaders
    void load_shader(const char* name, const uint32_t* spirv, size_t size);
    void dispatch_compute(const char* shader, uint32_t x, uint32_t y, uint32_t z);
    
    // Operaciones AI
    void matmul(const float* a, const float* b, float* c,
                int m, int n, int k) override;
    void attention(const float* q, const float* k, const float* v,
                   float* out, int seq_len, int dim, int heads);
    
private:
    VkInstance m_instance;
    VkPhysicalDevice m_physical_device;
    VkDevice m_device;
    VkQueue m_compute_queue;
    VkCommandPool m_command_pool;
    VkDescriptorPool m_descriptor_pool;
    
    // Shaders compilados
    std::unordered_map<std::string, VkPipeline> m_pipelines;
};

} // namespace adead
```

### Shader de Compute (GLSL → SPIR-V)

```glsl
// runtime/backends/vulkan/shaders/matmul.comp
#version 450

layout(local_size_x = 16, local_size_y = 16) in;

layout(set = 0, binding = 0) readonly buffer MatrixA {
    float a[];
};

layout(set = 0, binding = 1) readonly buffer MatrixB {
    float b[];
};

layout(set = 0, binding = 2) writeonly buffer MatrixC {
    float c[];
};

layout(push_constant) uniform Params {
    uint M;
    uint N;
    uint K;
};

void main() {
    uint row = gl_GlobalInvocationID.x;
    uint col = gl_GlobalInvocationID.y;
    
    if (row >= M || col >= N) return;
    
    float sum = 0.0;
    for (uint i = 0; i < K; i++) {
        sum += a[row * K + i] * b[i * N + col];
    }
    
    c[row * N + col] = sum;
}
```

---

## 🔗 FFI Universal

### C++ FFI

```cpp
// runtime/ffi/cpp/adead_cpp.h
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// Inicialización
int adead_init(int backend);  // 0=CPU, 1=CUDA, 2=Vulkan
void adead_shutdown();

// Memoria
void* adead_alloc(size_t size);
void adead_free(void* ptr);

// Operaciones
void adead_matmul(const float* a, const float* b, float* c,
                  int m, int n, int k);
void adead_softmax(const float* in, float* out, int rows, int cols);
void adead_relu(const float* in, float* out, int size);

// Vulkan específico
int adead_vulkan_init();
void adead_vulkan_compute(const char* shader, int x, int y, int z);

#ifdef __cplusplus
}
#endif
```

### Rust FFI

```rust
// runtime/ffi/rust/lib.rs
#[link(name = "adead_runtime")]
extern "C" {
    fn adead_init(backend: i32) -> i32;
    fn adead_shutdown();
    fn adead_alloc(size: usize) -> *mut u8;
    fn adead_free(ptr: *mut u8);
    fn adead_matmul(a: *const f32, b: *const f32, c: *mut f32,
                    m: i32, n: i32, k: i32);
}

pub struct ADeadRuntime {
    backend: i32,
}

impl ADeadRuntime {
    pub fn new(backend: Backend) -> Result<Self, &'static str> {
        let code = unsafe { adead_init(backend as i32) };
        if code == 0 {
            Ok(Self { backend: backend as i32 })
        } else {
            Err("Failed to initialize runtime")
        }
    }
    
    pub fn matmul(&self, a: &[f32], b: &[f32], m: usize, n: usize, k: usize) -> Vec<f32> {
        let mut c = vec![0.0f32; m * n];
        unsafe {
            adead_matmul(a.as_ptr(), b.as_ptr(), c.as_mut_ptr(),
                         m as i32, n as i32, k as i32);
        }
        c
    }
}

impl Drop for ADeadRuntime {
    fn drop(&mut self) {
        unsafe { adead_shutdown(); }
    }
}

#[repr(i32)]
pub enum Backend {
    CPU = 0,
    CUDA = 1,
    Vulkan = 2,
}
```

### Python FFI

```python
# runtime/ffi/python/adead_py.py
import ctypes
from pathlib import Path

class ADeadRuntime:
    CPU = 0
    CUDA = 1
    VULKAN = 2
    
    def __init__(self, backend=CPU):
        lib_path = Path(__file__).parent / "libadead_runtime.so"
        self._lib = ctypes.CDLL(str(lib_path))
        
        # Configurar tipos
        self._lib.adead_init.argtypes = [ctypes.c_int]
        self._lib.adead_init.restype = ctypes.c_int
        
        self._lib.adead_matmul.argtypes = [
            ctypes.POINTER(ctypes.c_float),
            ctypes.POINTER(ctypes.c_float),
            ctypes.POINTER(ctypes.c_float),
            ctypes.c_int, ctypes.c_int, ctypes.c_int
        ]
        
        if self._lib.adead_init(backend) != 0:
            raise RuntimeError("Failed to initialize ADead runtime")
    
    def matmul(self, a, b):
        import numpy as np
        a = np.ascontiguousarray(a, dtype=np.float32)
        b = np.ascontiguousarray(b, dtype=np.float32)
        m, k = a.shape
        k2, n = b.shape
        assert k == k2
        
        c = np.zeros((m, n), dtype=np.float32)
        
        self._lib.adead_matmul(
            a.ctypes.data_as(ctypes.POINTER(ctypes.c_float)),
            b.ctypes.data_as(ctypes.POINTER(ctypes.c_float)),
            c.ctypes.data_as(ctypes.POINTER(ctypes.c_float)),
            m, n, k
        )
        
        return c
    
    def __del__(self):
        self._lib.adead_shutdown()
```

---

## 📊 Comparación: ADead-BIB vs LLVM

| Característica | LLVM | ADead-BIB Runtime |
|----------------|------|-------------------|
| **Tamaño** | ~100 MB | **< 1 MB** |
| **Dependencias** | Muchas | **Ninguna** |
| **Tiempo de compilación** | Minutos | **Milisegundos** |
| **Determinismo** | No garantizado | **100% determinista** |
| **GPU Support** | Limitado | **CUDA + Vulkan** |
| **Complejidad** | Alta | **Mínima** |
| **Lenguajes** | IR intermedio | **FFI directo** |

---

## 🚀 Fases de Implementación

### Fase 1: Core Runtime (Semana 1-2)
- [ ] Implementar `DeterministicAllocator`
- [ ] Implementar `ArenaAllocator`
- [ ] Implementar `OpcodeEngine`
- [ ] Tests unitarios

### Fase 2: CPU Backend (Semana 2-3)
- [ ] Implementar `CPUBackend`
- [ ] Optimizaciones SIMD (AVX2/AVX-512)
- [ ] MatMul optimizado
- [ ] Benchmarks

### Fase 3: Vulkan Backend (Semana 3-4)
- [ ] Implementar `VulkanBackend`
- [ ] Compute shaders (MatMul, Softmax, ReLU)
- [ ] Attention shader
- [ ] Integración con runtime

### Fase 4: FFI Universal (Semana 4-5)
- [ ] FFI C/C++
- [ ] FFI Rust
- [ ] FFI Python
- [ ] FFI Zig
- [ ] Documentación

### Fase 5: Integración (Semana 5-6)
- [ ] Integrar con compilador ADead-BIB
- [ ] Integrar con sistema HEX existente
- [ ] Benchmarks comparativos
- [ ] Documentación final

---

## 🎯 Beneficios

1. **Un runtime para todos los lenguajes** - C++, Rust, Python, C, Zig
2. **Ultra ligero** - < 1 MB vs 100+ MB de LLVM
3. **Determinista** - Mismo input = mismo output siempre
4. **GPU nativo** - CUDA y Vulkan sin capas intermedias
5. **Sin dependencias** - Compila en cualquier sistema
6. **Fácil de extender** - Agregar nuevos backends es simple

---

## 📝 Notas

- El runtime es **independiente** del compilador ADead-BIB
- Puede usarse como **librería standalone**
- Compatible con el sistema HEX existente
- Vulkan permite **compute sin CUDA** (AMD, Intel, etc.)

---

## 🆕 NUEVAS IDEAS - Diciembre 2024

### 🔥 Auto-Dispatch CPU+GPU Implementado

Se ha implementado un sistema de auto-dispatch que:

1. **Detecta hardware automáticamente**
   - CPU: CPUID para SSE2/AVX/AVX2/AVX-512/FMA
   - GPU: Vulkan y CUDA por verificación de DLLs

2. **Selecciona backend óptimo por tamaño**
   - < 1M elementos → CPU con mejor SIMD (AVX2)
   - ≥ 1M elementos → GPU (CUDA si NVIDIA, Vulkan si no)

3. **Overhead mínimo: ~5ns por decisión**
   - 200M+ decisiones por segundo
   - No afecta rendimiento

### 📊 Resultados de Benchmarks (TEST-G)

| Test | Resultado |
|------|-----------|
| Vulkan detectado | ✓ |
| CUDA detectado | ✓ |
| GPU Vendor | NVIDIA |
| Dispatch overhead | 4.59 ns |
| AVX2 speedup | 1.9x |
| Determinismo | 100% |

### 🚀 Nuevas Capacidades del Runtime

```rust
// Auto-dispatch en acción
let dispatcher = AutoDispatcher::new();

// Pequeño → CPU AVX2
dispatcher.select(1000)      // → CPU_AVX2

// Grande → GPU CUDA
dispatcher.select(10000000)  // → GPU_CUDA
```

### 📁 Archivos Implementados

| Archivo | Descripción |
|---------|-------------|
| `src/rust/runtime/cpu_detect.rs` | Detección CPU via CPUID |
| `src/rust/runtime/gpu_detect.rs` | Detección GPU Vulkan/CUDA |
| `src/rust/runtime/dispatcher.rs` | Auto-dispatch CPU+GPU |
| `TEST-G/vulkan_detect/` | Tests de detección GPU |
| `TEST-G/benchmark/` | Benchmarks CPU vs GPU |

### 🎯 Próximas Mejoras

1. **Vulkan Compute Shaders**
   - MatMul optimizado con tiling
   - Softmax con shared memory
   - Attention con flash attention

2. **SIMD Code Generation**
   - Emitir opcodes AVX2/AVX-512 directamente
   - Vectorización automática de loops
   - FMA (Fused Multiply-Add)

3. **Memory Pool GPU**
   - Pre-allocar buffers Vulkan
   - Reducir overhead de transferencia
   - Pinned memory para DMA

4. **Profiler Integrado**
   - RDTSC para ciclos CPU
   - Vulkan timestamps para GPU
   - Reporte automático de GFLOPS

### 💡 Ideas Adicionales

#### Tensor Unificado CPU/GPU
```rust
struct Tensor {
    data_cpu: Option<Vec<f32>>,
    data_gpu: Option<VulkanBuffer>,
    location: Location,
    dirty: bool,
}

impl Tensor {
    fn to_gpu(&mut self) { /* lazy copy */ }
    fn to_cpu(&mut self) { /* lazy copy */ }
    fn matmul(&self, other: &Tensor) -> Tensor {
        // Auto-dispatch basado en tamaño
    }
}
```

#### Pipeline de Operaciones Fusionadas
```rust
// En lugar de:
let a = matmul(x, w1);
let b = relu(a);
let c = matmul(b, w2);

// Fusionar en un solo kernel:
let c = pipeline()
    .matmul(x, w1)
    .relu()
    .matmul(w2)
    .execute();  // Un solo dispatch a GPU
```

#### Caché de Kernels Compilados
```rust
struct KernelCache {
    compiled: HashMap<String, VkPipeline>,
}

impl KernelCache {
    fn get_or_compile(&mut self, shader: &str) -> &VkPipeline {
        // Compilar una vez, reusar siempre
    }
}
```

---

**Creado por:** Eddi Andreé Salazar Matos  
**Email:** eddi.salazar.dev@gmail.com  
**Hecho con ❤️ en Perú** 🇵🇪
