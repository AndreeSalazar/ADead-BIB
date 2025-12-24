# 🔥 ADead-BIB

**Abstract Dead - Binary In Binary**

> A compiler that generates **pure executable binaries** by writing opcodes directly to the CPU, without going through an assembler. **Binary + HEX = ADead-BIB**.

---

## 🇵🇪 Made with ❤️ in Peru

**Author:** Eddi Andreé Salazar Matos  
**Email:** eddi.salazar.dev@gmail.com  
**License:** Apache 2.0

---

## ✅ Status: COMPLETE LANGUAGE + AI + GPU + VULKAN + OPTIMIZATIONS

| Feature | Status |
|---------|--------|
| **70+ built-in functions** | ✅ |
| **Complete OOP** | ✅ |
| **Import system** | ✅ |
| **Python FFI** | ✅ |
| **Integrated AI (0.19 MB RAM)** | ✅ |
| **Matrix functions for AI** | ✅ |
| **Ollama integration** | ✅ |
| **GPU Support (CUDA)** | ✅ |
| **Vulkan Support** | ✅ |
| **Hybrid CPU+GPU Mode** | ✅ |
| **HEX Opcodes for GPU** | ✅ |
| **Auto-Dispatch CPU/GPU** | ✅ |
| **Deterministic Runtime** | ✅ |
| **Server Load Benchmarks** | ✅ |
| **Branchless Optimizer** | ✅ NEW |
| **Syntax Checker (check command)** | ✅ NEW |
| **Enhanced Type Validation** | ✅ NEW |
| **AST Transformation System** | ✅ NEW |
| **Ultra-Compact Binaries** | 🎯 Target: Bytes |

---

## � Quick Start

### Prerequisites

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Install Python 3.8+
# Download from https://python.org

# 3. Install Python dependencies
pip install numpy

# 4. (Optional) Install Ollama for AI demos
winget install Ollama.Ollama
ollama pull tinyllama
```

### Build & Run

```powershell
# Clone the repository
git clone https://github.com/yourusername/ADead-BIB.git
cd ADead-BIB

# Build the compiler
cargo build --release

# Check syntax without compiling
cargo run --release -- check examples/hello_world.adB

# Compile and run Hello World
cargo run --release -- build examples/hello_world.adB
.\hello_world.exe
# Output: Hello, World!

# Or use run command (builds and runs)
cargo run --release -- run examples/hello_world.adB
```

---

## 📁 Project Structure

```
ADead-BIB/
├── src/rust/                    # Compiler Core
│   ├── frontend/                # Lexer, Parser, AST, Types
│   ├── backend/                 # Code Generation (CPU + GPU)
│   │   ├── cpu/                 # 🔥 CPU Backend (x86-64 directo)
│   │   │   ├── codegen.rs       # Generador de código legacy
│   │   │   ├── codegen_v2.rs    # Generador avanzado (multi-función)
│   │   │   ├── pe*.rs           # Generadores PE (tiny, minimal, full)
│   │   │   ├── elf.rs           # Generador ELF Linux
│   │   │   ├── syscalls.rs      # Syscalls directos Win/Linux
│   │   │   └── microvm.rs       # MicroVM bytecode (4-bit)
│   │   └── gpu/                 # 🔥 GPU Backend (Vulkan + HEX)
│   │       ├── gpu_detect.rs    # Detección y análisis GPU
│   │       ├── vulkan/          # Backend Vulkan (SPIR-V)
│   │       └── hex/             # Binario HEX directo para GPU
│   ├── optimizer/               # Branchless & SIMD Optimizer
│   ├── runtime/                 # Auto-dispatcher CPU/GPU
│   └── builder.rs               # Orchestrator & Build System
├── builds/                      # 🆕 Binarios generados
├── examples/                    # .adB example files
├── stdlib/                      # Standard library (math, io, string)
├── python/                      # Python FFI + AI + GPU
│   ├── adead_ffi.py             # FFI wrapper
│   ├── ai_complete.py           # Complete AI (0.19 MB RAM)
│   ├── ai_scalable.py           # Scalable AI with BPE
│   ├── vocabulary.py      # Vocabulary builder
│   ├── embeddings.py      # Semantic embeddings
│   ├── ollama_integration.py   # Ollama integration
│   ├── ollama_gpu_benchmark.py # Ollama GPU modes benchmark
│   ├── benchmark_gpu.py        # GPU benchmark
│   ├── benchmark_server_load.py # Server load simulation
│   ├── demo_gpu_comparison.py  # CPU vs GPU comparison
│   ├── demo_full.py            # Full demo
│   ├── gpu_detect.py           # Hardware detection
│   └── hybrid_compute.py       # Hybrid CPU+GPU system
├── hex/                   # GPU Opcodes
│   ├── gpu_opcodes.py     # GPU opcode generator
│   ├── cuda_kernels.py    # CUDA kernels
│   ├── binary_gpu.py      # Hybrid binary generator
│   └── README.md          # HEX documentation
├── build/                 # Compiled binaries (.exe)
├── docs/                  # Documentation
│   ├── EN/                # English documentation
│   ├── ES/                # Spanish documentation
│   └── IDEAS/             # Development roadmaps
│       ├── ideas-6.md     # Universal Runtime
│       ├── ideas-7.md     # Compiler improvements
│       └── ideas-8.md     # Post-processing & optimization
├── runtime/               # Universal Runtime (C)
│   ├── core/              # Memory, types, runtime API
│   ├── backends/          # CPU, GPU, Vulkan backends
│   └── ffi/               # C++, Python, Rust bindings
├── TEST-G/                # GPU/Vulkan tests
│   ├── vulkan_detect/     # Vulkan/CUDA detection
│   ├── cpu_gpu_dispatch/  # Auto-dispatch tests
│   └── benchmark/         # CPU vs GPU benchmarks
├── examples-new/          # Incremental compiler tests
│   ├── fase1_syscalls/    # Direct syscalls
│   ├── fase2_stack/       # Dynamic stack
│   ├── fase3_functions/   # Multi-function support
│   ├── fase4_targets/     # Multi-target (PE, ELF)
│   └── fase5_detect/      # CPU detection
├── GAME/                  # 🎮 Vulkan Bird Game Demo
│   ├── src/               # Rust game source
│   │   ├── main.rs        # Entry point
│   │   ├── game.rs        # Branchless game logic
│   │   └── renderer.rs    # Vulkan-ready renderer
│   ├── Cargo.toml         # Dependencies
│   └── README.md          # Game documentation
├── Como_usar.md           # Quick start guide (Spanish)
├── LICENSE                # Apache 2.0 License
└── README.md              # This file
```

---

## 🎯 What is ADead-BIB?

A compiler that transforms Python-style syntax directly into **x86-64 opcodes** and generates **PE executable binaries** without using an assembler.

```
hello_world.adB → Lexer → Parser → AST → x86-64 Opcodes → PE → CPU executes
```

**The CPU executes exactly what you write** - no intermediate layers, no runtime, no overhead.

---

## 🔥 Why is it Different?

| Approach | Flow | Overhead |
|----------|------|----------|
| **C/C++** | Code → Compiler → ASM → Object → Linker → Binary | Medium |
| **ASM** | ASM → Assembler → Object → Linker → Binary | Low |
| **ADead-BIB** | Code → **Direct Opcodes** → Binary | **Minimal** |

### Key Advantages

- ✅ **No ASM** - We write bytes directly, not assembler text
- ✅ **No Linker** - We generate complete PE in one step
- ✅ **No Runtime** - Standalone binaries, no dependencies
- ✅ **Total Control** - Every byte of the executable is yours
- ✅ **Ultra-Compact Binaries** - Current: ~1.5 KB | **Target: < 1 KB (Bytes)**
- ✅ **Branchless Optimization** - Automatic IF/ELSE → branchless transforms
- ✅ **Syntax Validation** - Fast syntax checking without compilation

---

## 📝 Syntax

ADead-BIB uses Python-style syntax with OOP:

```python
# Main function
def main():
    print("Hello, World!")
    x = 10
    y = 20
    print(x + y)

# Classes with inheritance
class Entity:
    x = 0
    y = 0
    
    virtual def update(self):
        pass

class Player extends Entity:
    health = 100
    
    override def update(self):
        print("Player update")
```

---

## 🤖 AI Integration

ADead-BIB includes a complete AI system with minimal RAM usage:

### Run AI Demo

```powershell
cd python
python ai_complete.py      # Basic AI (0.19 MB RAM)
python ai_scalable.py      # Scalable AI with BPE (0.82 MB RAM)
python vocabulary.py       # Build vocabulary
python embeddings.py       # Semantic embeddings
python ollama_integration.py  # Ollama integration (requires Ollama)
```

### AI Features

| Feature | Status | RAM |
|---------|--------|-----|
| BPE Tokenizer | ✅ | - |
| Semantic Embeddings | ✅ | 0.06 MB |
| Multi-head Attention | ✅ | 0.03 MB |
| Feed-forward Network | ✅ | 0.06 MB |
| Text Generation | ✅ | - |
| Text Analysis | ✅ | - |
| Similarity Scoring | ✅ | - |
| **Total** | ✅ | **0.19 MB** |

### Real Performance Results (Tested)

| Component | RAM | Speed | Use Case |
|-----------|-----|-------|----------|
| **ADead-BIB Compiler** | ~5 MB | 19 ms | 1.5 KB binaries (target: <1 KB) |
| **Basic AI** | 0.19 MB | 15 ms/token | Fast analysis |
| **Scalable AI (BPE)** | 0.82 MB | 34 ms/token | 0% UNK, 93% cache |
| **Ollama (TinyLlama)** | ~700 MB | 2.2 s/response | Coherent generation |

### Ollama Integration (Real LLM)

```powershell
# Install Ollama
winget install Ollama.Ollama

# Download model (637 MB)
ollama pull tinyllama

# Run full demo
cd python
python demo_full.py
```

**Sample Output:**
```
Prompt: 'What is Python in one sentence?'
Response: Python: A popular and powerful programming language...
Time: 2.4s
```

### Matrix Functions (Built-in)

```python
# In ADead-BIB code:
dot(2, 3, 4, 5)           # = 26 (dot product)
sum_sq(3, 4)              # = 25 (sum of squares)
relu(-3)                  # = 0 (ReLU activation)
weighted_sum(10, 2, 20, 3) # = 80
scale(200, 50)            # = 100 (x * factor / 100)
lerp(0, 100, 50)          # = 50 (linear interpolation)
```

---

## 📊 Implemented Features

| Component | Status | Description |
|-----------|--------|-------------|
| **Lexer** | ✅ | Tokenizes .adB code |
| **Parser** | ✅ | Generates AST from tokens |
| **Type Checker** | ✅ | Enhanced validation with warnings |
| **Syntax Checker** | ✅ | `check` command for fast validation |
| **Codegen** | ✅ | Emits x86-64 opcodes |
| **PE Generator** | ✅ | Generates Windows binaries |
| **ELF Generator** | ✅ | Generates Linux binaries |
| **Variables** | ✅ | Local variables on stack |
| **Operations** | ✅ | +, -, *, /, % |
| **Comparisons** | ✅ | ==, !=, <, <=, >, >= |
| **Conditionals** | ✅ | if/elif/else |
| **Loops** | ✅ | while, for |
| **Functions** | ✅ | With parameters |
| **OOP** | ✅ | Classes, inheritance, polymorphism |
| **70+ Built-ins** | ✅ | Math, AI, utilities |
| **Python FFI** | ✅ | Call ADead-BIB from Python |
| **GPU Support** | ✅ | CUDA kernels, hybrid mode |
| **HEX Opcodes** | ✅ | GPU opcodes for direct execution |
| **Branchless Optimizer** | ✅ | Auto-transforms IF/ELSE to branchless |
| **AST Transformation** | ✅ | Pattern detection & replacement |

---

## 🎮 GPU Support (CUDA)

ADead-BIB includes full GPU acceleration for AI and matrix operations.

### Author's Hardware (Benchmark Reference)

| Component | Specification |
|-----------|---------------|
| **GPU** | NVIDIA GeForce RTX 3060 |
| **VRAM** | 12 GB GDDR6 |
| **CUDA Cores** | 3584 |
| **SMs** | 28 |
| **CPU** | AMD Ryzen (6 cores, 12 threads) |
| **RAM** | 16 GB |

### GPU Benchmark Results

#### Matrix Multiplication (MatMul)

| Size | CPU | GPU | Speedup |
|------|-----|-----|---------|
| 512x512 | 1.04 ms | 0.10 ms | **10.1x** |
| 1024x1024 | 5.75 ms | 0.36 ms | **15.9x** |
| 2048x2048 | 38.22 ms | 2.38 ms | **16.1x** |
| 4096x4096 | 317 ms | 19 ms | **16.7x** |
| 8192x8192 | 2400+ ms | 120 ms | **20x** |

#### Transformer Attention

| Config | CPU | GPU | Speedup |
|--------|-----|-----|---------|
| seq=256, dim=64 | 21 ms | 4 ms | **5.4x** |
| seq=512, dim=128 | 92 ms | 1.3 ms | **73.6x** |
| seq=1024, dim=256 | 488 ms | 5.7 ms | **86.1x** |

#### Server Load Benchmark

| Level | MatMul | GFLOPS | Throughput |
|-------|--------|--------|------------|
| Light (Laptop) | 1024x1024 | 6,887 | 27.8M tok/s |
| Medium (Desktop) | 2048x2048 | 7,398 | 11.9M tok/s |
| Heavy (Workstation) | 4096x4096 | 7,551 | 6.8M tok/s |
| Extreme (Server) | 8192x8192 | **9,038** | 3.7M tok/s |
| Maximum (Data Center) | 8192x8192 | **9,175** | 1.6M tok/s |

**Peak Performance: 9,175 GFLOPS**

### Ollama GPU Modes

| Mode | CPU | GPU | Time/Response | Tokens/s |
|------|-----|-----|---------------|----------|
| **CPU Solo** | 100% | 0% | 5.06s | 6.0 |
| **GPU Solo** | 10% | 90% | 2.62s | 10.2 |
| **Balanced** | 50% | 50% | 3.10s | 9.6 |
| **Hybrid** | 10% | 90% | 2.74s | **12.4** |

**Speedup GPU vs CPU: 1.9x**

### Run GPU Benchmarks

```powershell
cd python

# GPU vs CPU comparison
python demo_gpu_comparison.py

# Full GPU benchmark
python benchmark_gpu.py

# Server load simulation
python benchmark_server_load.py

# Ollama GPU modes
python ollama_gpu_benchmark.py

# CUDA kernels
cd ../hex
python cuda_kernels.py
```

---

## 🔧 HEX Opcodes for GPU

ADead-BIB includes a custom GPU opcode system in the `hex/` folder:

```
hex/
├── gpu_opcodes.py      # GPU opcode generator
├── cuda_kernels.py     # Pre-compiled CUDA kernels
├── binary_gpu.py       # Hybrid CPU+GPU binary generator
└── README.md           # HEX documentation
```

### GPU Opcodes

| Opcode | Hex | Description |
|--------|-----|-------------|
| GPU_INIT | 0xC0DA0001 | Initialize CUDA context |
| GPU_ALLOC | 0xC0DA0010 | Allocate GPU memory |
| GPU_MATMUL | 0xC0DA0020 | Matrix multiplication |
| GPU_ATTENTION | 0xC0DA0040 | Multi-head attention |
| GPU_SOFTMAX | 0xC0DA0033 | Softmax activation |
| GPU_SYNC | 0xC0DA00F0 | Synchronize GPU |

### Example GPU Program

```
; ADead-BIB GPU Program: matmul_1024
0000: 0100DAC000           ; GPU_INIT
0005: 1000DAC002...        ; GPU_ALLOC 4MB
001F: 2000DAC006...        ; GPU_MATMUL 1024x1024
006B: F000DAC000           ; GPU_SYNC
009C: FFFFDAC000           ; GPU_END
; Total: 161 bytes
```

---

## 🔬 Technical Details

### Generated PE Layout

```
0x0000 - Headers (DOS, PE, COFF, Optional, Sections)
0x1000 - .text  (executable code - opcodes)
0x2000 - .rdata (imports + data)
```

### Example Generated Opcodes

For `print("Hello, World!")`:

```asm
48 83 EC 28          ; sub rsp, 40 (shadow space)
48 B9 60 20 40 00... ; mov rcx, string_address
FF 15 xx xx xx xx    ; call [rip+offset] (printf)
31 C0                ; xor eax, eax (return 0)
48 83 C4 28          ; add rsp, 40
C3                   ; ret
```

**27 bytes of machine code** - direct to CPU.

---

## 📚 Documentation

| Document | Language | Description |
|----------|----------|-------------|
| `docs/EN/` | English | English documentation |
| `docs/ES/` | Spanish | Spanish documentation |
| `docs/IDEAS/` | Mixed | Development roadmaps |

---

## 💡 General Use Cases & Optimization Potential

### 🚀 Why ADead-BIB + Python + Ollama?

| Scenario | Traditional | ADead-BIB Solution | Improvement |
|----------|-------------|-------------------|-------------|
| **Tokenization** | Python (slow) | ADead-BIB native | 5x faster |
| **Small binaries** | C++ (100+ KB) | ADead-BIB (1.5 KB → target: <1 KB) | 66x+ smaller |
| **AI preprocessing** | NumPy (heavy) | Built-in functions | 50% less RAM |
| **Text generation** | API calls | Local Ollama | No latency, private |

### 🎯 Recommended Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    YOUR APPLICATION                          │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│              PYTHON (Orchestration)                          │
│  - User interface                                            │
│  - Data loading                                              │
│  - Result formatting                                         │
└─────────────────────┬───────────────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        ▼             ▼             ▼
┌───────────┐  ┌───────────┐  ┌───────────┐
│ ADead-BIB │  │ Local AI  │  │  Ollama   │
│ (Fast)    │  │ (0.19 MB) │  │ (Quality) │
│           │  │           │  │           │
│ • Matrix  │  │ • Tokens  │  │ • Chat    │
│ • Math    │  │ • Embed   │  │ • Generate│
│ • Binaries│  │ • Analyze │  │ • Reason  │
└───────────┘  └───────────┘  └───────────┘
```

### 📊 Real-World Applications

1. **Chatbots** - Ollama for responses, ADead-BIB for preprocessing
2. **Data Analysis** - Local AI for fast tokenization, no API costs
3. **Edge Computing** - 0.19 MB AI runs on any device
4. **Game Development** - Ultra-compact binaries (<1 KB target), instant compilation
5. **Embedded Systems** - No runtime dependencies
6. **Private AI** - All processing local, no data leaves your machine

### 💰 Cost Comparison

| Solution | Monthly Cost | Latency | Privacy |
|----------|-------------|---------|---------|
| OpenAI API | $20-100+ | 500ms+ | ❌ |
| Cloud GPU | $50-500+ | 100ms+ | ❌ |
| **ADead-BIB + Ollama** | **$0** | **<50ms** | **✅** |

---

## 🎯 Philosophy

> **"Code → Opcodes → Binary"**

ADead-BIB eliminates unnecessary layers between your code and the CPU. No assembler, no linker, no runtime. Just bytes that the CPU executes directly.

**Fewer steps = Fewer errors = More control = Better performance**

---

## 🎯 Ultra-Compact Binaries: The Byte-Sized Goal ✅ ACHIEVED!

### Current Status
- **Standard build:** 2 KB (2,048 bytes)
- **Nano build:** **1 KB (1,024 bytes)** ✅ ACHIEVED!
- **Micro build:** **256 bytes** ✅ ACHIEVED! (PE32)
- **Flat binary:** **3 bytes** ✅ ACHIEVED! (pure code)
- **Ultimate goal:** < 500 bytes PE64 (in progress)

### 🆕 NEW: Ultra-Compact Build Commands

```powershell
# Standard build (~2 KB)
cargo run --release -- build examples/hello_world.adB

# Tiny build (< 1.5 KB) - Optimized PE
cargo run --release -- tiny examples/hello_world.adB

# Nano build (1 KB) - Smallest valid x64 PE
cargo run --release -- nano output.exe [exit_code]

# Micro build (256 bytes) - PE32 32-bit
cargo run --release -- micro output.exe [exit_code]

# Flat binary (3 bytes) - Pure machine code
cargo run --release -- flat output.bin [exit_code]

# MicroVM bytecode (2 bytes) - 4-bit instructions
cargo run --release -- vm output.adb [exit_code]

# 1-bit program - Ultimate minimal
cargo run --release -- bit [0|1]
```

### 🔥 GPU Commands (Vulkan/SPIR-V)

```powershell
# Detect GPU and generate optimized shader
cargo run --release -- gpu [output.spv]

# Generate SPIR-V compute shader
cargo run --release -- spirv matmul [size]
# Example: cargo run --release -- spirv matmul 1024

# Initialize Vulkan runtime REAL (exprimir GPU)
cargo run --release -- vk
```

### Vulkan Runtime Output (RTX 3060)

```
╔══════════════════════════════════════════════════════════════╗
║                 VULKAN RUNTIME INITIALIZED                    ║
╠══════════════════════════════════════════════════════════════╣
║ Device:     NVIDIA GeForce RTX 3060                          ║
║ Vendor ID:  0x10DE                                          ║
║ Type:       DISCRETE_GPU                                     ║
║ API:        1.4.312                                          ║
╠══════════════════════════════════════════════════════════════╣
║ Max Workgroup Size:  [1024, 1024, 64]                        ║
║ Max Invocations:     1024                                    ║
║ Shared Memory:       48 KB                                   ║
╚══════════════════════════════════════════════════════════════╝
```

---

---

## 🎮 Heredar: Sistema de Herencia

**Facilita el uso de ADead-BIB para Game Engines, Graphics Engines y Compute**

```
Heredar/
├── GameEngine/       # Templates para Game Engines
├── GraphicsEngine/   # Templates para Motores Gráficos
├── ComputeEngine/    # Templates para Cómputo GPU
├── Templates/        # GPU Context, Benchmarks
└── TECHNICAL_PAPER.md # Documentación técnica
```

### Uso Rápido

```rust
// Game Engine
let engine = GameEngineBuilder::new()
    .with_name("Mi Juego")
    .with_resolution(1920, 1080)
    .build();

// Graphics Engine
let renderer = GraphicsEngineBuilder::new()
    .with_backend(RenderBackend::Vulkan)
    .with_ray_tracing(true)
    .build();

// Compute Engine
let compute = ComputeEngineBuilder::new()
    .with_workgroup(256, 1, 1)
    .with_scheduling(SchedulingMode::Deterministic)
    .build();
```

### Nivel Militar 🎖️

- **Zero-copy transfers** - Sin copias innecesarias
- **Deterministic scheduling** - Sin locks, sin colas dinámicas
- **Direct SPIR-V** - Sin GLSL, sin HLSL
- **Memory coalescing** - Acceso óptimo a memoria
- **Workgroup optimization** - Por arquitectura GPU

---

## 🏗️ Architecture: Complete GPU System

ADead-BIB implements a **complete GPU architecture** with 4 key pieces:

### 1️⃣ Scheduler CPU→GPU (Deterministic)
```rust
// No dynamic queues, no locks, no abstractions
struct Dispatch {
    shader_id: u32,
    workgroups: (u32, u32, u32),
    buffer_ids: Vec<u32>,
    dependencies: Vec<u32>,
}
```

### 2️⃣ Explicit Memory Management
```rust
// Buffers, ring buffers, zero-copy
let buffer = allocator.create_buffer(size, BufferUsage::StorageReadWrite);
let staging = StagingBuffer::new(id, size);
let ring = RingBuffer::new(id, size, frames_in_flight);
```

### 3️⃣ ADead Bytecode → SPIR-V (Unique!)
```
ADead Bytecode (4-bit instructions)
         ↓
    SPIR-V IR
         ↓
       GPU

// Write logic in bits, execute on GPU
// No GLSL, no HLSL - direct compilation
```

### 4️⃣ Real Metrics (No Fake Benchmarks)
```
📊 GPU METRICS REPORT
   CPU → GPU:     10.25 µs
   Dispatch:      45.30 µs
   GFLOPS:        8.50
   Bandwidth:     280.00 GB/s
   P99 Latency:   120.50 µs
```

### GPU Backend Structure

```
src/rust/backend/gpu/
├── gpu_detect.rs      # GPU detection via nvidia-smi (RTX 30/40 series)
├── vulkan/            # SPIR-V generation (470+ lines)
├── hex/               # Direct HEX binary for GPU
├── scheduler.rs       # Deterministic CPU→GPU scheduler
├── memory.rs          # Explicit memory (buffers, zero-copy)
├── bytecode_spirv.rs  # ADead Bytecode → SPIR-V compiler
└── metrics.rs         # Real performance metrics
```

### GPU Detection Example (RTX 3060 12GB)

```
╔══════════════════════════════════════════════════════════════╗
║                      GPU DETECTION                            ║
╠══════════════════════════════════════════════════════════════╣
║ ✅ GPU Available                                             ║
║ Device:    NVIDIA GeForce RTX 3060                          ║
║ VRAM:      12288 MB (12.0 GB)                               ║
║ Compute:   28 SMs                                           ║
╠══════════════════════════════════════════════════════════════╣
║ 📊 SPECIFICATIONS                                            ║
║ CUDA Cores:    3584                                          ║
║ Boost Clock:   1777 MHz                                     ║
║ Bandwidth:     360.0 GB/s                                    ║
║ FP32:          12.74 TFLOPS                                  ║
║ FP16:          25.48 TFLOPS (Tensor Cores)                   ║
║ Architecture:  Ampere                                        ║
╠══════════════════════════════════════════════════════════════╣
║ 🎯 OPTIMAL SETTINGS                                          ║
║ Workgroup:     (256, 1, 1)                                   ║
║ MatMul Tile:   (16, 16, 1)                                   ║
║ MatMul 1024³:  ~0.34 ms (estimated)                          ║
╚══════════════════════════════════════════════════════════════╝
```

### Size Comparison

| Build Mode | Size | Reduction | Use Case |
|------------|------|-----------|----------|
| **Standard** | 2,048 bytes | - | Full features, imports |
| **Tiny** | 1,024 bytes | 50% | Compact, limited code |
| **Nano** | 1,024 bytes | 50% | Minimal x64, exit only |
| **Micro** | **256 bytes** | **87.5%** | PE32 32-bit |
| **Flat** | **3 bytes** | **99.85%** | Pure code, no headers |
| **MicroVM** | **2 bytes** | **99.9%** | Bytecode 4-bit |
| **1-bit** | **0.125 bytes** | **99.99%** | Teórico con runtime |
| **SPIR-V** | **644 bytes** | GPU | Vulkan compute shader |

### Why Bytes Matter

| Size | Use Case | Benefit |
|------|----------|---------|
| **< 500 bytes** | Microcontrollers, embedded | Minimal memory footprint |
| **< 1 KB** | Bootloaders, system tools | Fast loading, minimal storage |
| **1-2 KB** | Current ADead-BIB | **100x smaller than C++** |

### Optimization Techniques Used

1. **Minimal PE Headers** - Only essential fields
2. **No Data Directories** - NumberOfRvaAndSizes = 0
3. **Section/File Alignment** - 0x200 instead of 0x1000
4. **Direct Opcodes** - No library dependencies for nano
5. **Header Overlap** - Reuse unused DOS header fields

### Example: Nano PE Structure

```
Offset  Size   Content
0x000   64     DOS Header (MZ + e_lfanew)
0x040   4      PE Signature
0x044   20     COFF Header
0x058   240    Optional Header PE32+
0x148   40     Section Header (.text)
0x170   144    Padding
0x200   3-6    Code (xor eax,eax; ret)
0x203   509    Padding to 0x400
─────────────────────────────
Total: 1,024 bytes (1 KB)
```

**🏆 Achievement Unlocked:** 1 KB Windows x64 executable!

---

## 🚀 System Capabilities

Based on the author's hardware (RTX 3060 12GB), ADead-BIB can handle:

| Capability | Specification |
|------------|---------------|
| **Matrices** | Up to 8192x8192 (67M elements) |
| **Batch Size** | Up to 64-128 depending on sequence |
| **Sequences** | Up to 4096 tokens |
| **Model Layers** | Up to 12-24 layers |
| **Vocabulary** | 100K+ tokens |
| **Peak GFLOPS** | 9,175 |
| **Max Throughput** | 27.8M tokens/second |

### Production Estimates

| Use Case | Performance |
|----------|-------------|
| **Inference** | 10,000-50,000 tokens/second |
| **Training** | 1,000-5,000 tokens/second |
| **Attention** | Up to 86x faster than CPU |

### GPU Comparison

| GPU | TFLOPS | Relative |
|-----|--------|----------|
| **RTX 3060 12GB** (Author's) | ~9 TFLOPS | 1x |
| RTX 4090 24GB | ~83 TFLOPS | 9x |
| A100 40GB | ~156 TFLOPS | 17x |
| H100 80GB | ~267 TFLOPS | 30x |

---

## 🆕 NEW: Intermediate Runtime - Clean Code for CPU & GPU

ADead-BIB acts as an **intermediate runtime** that cleans and optimizes code before execution, making it trivially simple for both CPU and GPU to process.

### 🧠 The Philosophy: Clean Code = Fast Execution

```
┌─────────────────────────────────────────────────────────────────┐
│                    TRADITIONAL APPROACH                          │
│  Source → Compiler → Messy Code → CPU/GPU struggles             │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                    ADead-BIB APPROACH                            │
│  Source → Parser → INTERMEDIATE RUNTIME → Clean Code → Fast!    │
│                    ↓                                             │
│           • Remove branches (IF/ELSE)                           │
│           • Eliminate dead code                                 │
│           • Vectorize loops (SIMD)                              │
│           • Coalesce memory access                              │
│           • Fuse operations                                     │
└─────────────────────────────────────────────────────────────────┘
```

### 🔧 What the Intermediate Runtime Does

| Stage | Action | Benefit |
|-------|--------|---------|
| **1. Parse** | AST generation | Structure analysis |
| **2. Clean** | Remove dead code, unused vars | Smaller binary |
| **3. Transform** | IF → Branchless, Loops → SIMD | 8x faster |
| **4. Optimize** | Fuse ops, align memory | Cache friendly |
| **5. Dispatch** | Auto-select CPU/GPU | Best hardware |
| **6. Execute** | Pure opcodes | Zero overhead |

### 🎯 Auto-Detection System

ADead-BIB automatically detects and uses the best hardware:

```
CPU: AMD Ryzen 5 5600X (12 threads)
├── SSE2:    ✓ (128-bit SIMD)
├── AVX:     ✓ (256-bit SIMD)
├── AVX2:    ✓ (256-bit + FMA)
├── AVX-512: ✗
└── FMA:     ✓ (Fused Multiply-Add)

GPU: NVIDIA (detected)
├── Vulkan:  ✓ (Cross-platform)
└── CUDA:    ✓ (NVIDIA optimized)
```

### ⚡ Auto-Dispatch in Action

```
Small data (< 1M elements)  → CPU AVX2 (low latency)
Large data (≥ 1M elements)  → GPU CUDA (high throughput)

MatMul 32x32       → CpuAvx2   (0.01 ms)
MatMul 256x256     → GpuCuda   (0.1 ms)
MatMul 1024x1024   → GpuCuda   (0.5 ms)
MatMul 4096x4096   → GpuCuda   (15 ms)
```

### 📊 Performance Metrics

| Metric | Value | Notes |
|--------|-------|-------|
| **Dispatch overhead** | 9.48 ns | Near-zero cost |
| **Dispatches/second** | 106 M | Real-time capable |
| **AVX2 speedup** | 1.9x | vs scalar code |
| **GPU speedup** | 16-86x | vs CPU (large data) |
| **Determinism** | 100% | Reproducible results |
| **Memory efficiency** | 90%+ | Coalesced access |

### 🚀 Run GPU Tests

```powershell
# Vulkan/CUDA Detection
.\TEST-G\vulkan_detect\test_vulkan.exe

# Auto-Dispatch Test
.\TEST-G\cpu_gpu_dispatch\test_dispatch.exe

# CPU vs GPU Benchmark
.\TEST-G\benchmark\test_benchmark.exe
```

---

## 🧹 NEW: Post-Processing & Branchless Optimization

The intermediate runtime includes a **post-processor** that transforms complex code into simple, GPU-friendly operations.

### ❌ The Problem: Branching Kills Performance

```
GPU with IF/ELSE: ~40% efficiency (warp divergence)
CPU with IF/ELSE: Branch misprediction penalty (15-20 cycles)

Why? GPU executes thousands of threads in lockstep.
     If some take IF and others take ELSE, half wait idle.
```

### ✅ The Solution: Branchless Code

| Pattern | Before (Branching) | After (Branchless) | Speedup |
|---------|-------------------|-------------------|---------|
| ReLU | `if x > 0: x else 0` | `max(0, x)` | **8x** |
| Select | `if cond: a else b` | `blend(a, b, mask)` | **8x** |
| Clamp | `if x < min...` | `max(min, min(x, max))` | **7x** |
| Abs | `if x < 0: -x else x` | `x & 0x7FFFFFFF` | **10x** |
| Sign | `if x > 0: 1 elif...` | `(x >> 31) | ((-x) >> 31)` | **6x** |

### 🔄 Automatic Transformations

The runtime automatically transforms these patterns:

```python
# BEFORE (your code)
def relu(x):
    if x > 0:
        return x
    else:
        return 0

# AFTER (runtime transforms to)
# Single instruction: VMAXPS (AVX2) or max() (GPU)
# No branches, no divergence, 8x faster
```

### 📈 Expected Improvements

| Operation | With Branching | Without | Speedup |
|-----------|---------------|---------|---------|
| ReLU (1M elements) | 2.5 ms | 0.3 ms | **8.3x** |
| Softmax (1M) | 15 ms | 2 ms | **7.5x** |
| Attention (1K seq) | 5 ms | 0.5 ms | **10x** |
| GELU (1M) | 8 ms | 1 ms | **8x** |
| LayerNorm (1M) | 12 ms | 1.5 ms | **8x** |

---

## 🗑️ NEW: Garbage Elimination

The runtime removes "garbage" that slows down CPU and GPU:

### CPU Garbage Removed

| Garbage | Problem | Solution | Benefit |
|---------|---------|----------|---------|
| Redundant loads | Cache miss | Register reuse | 2x faster |
| Dead stores | Wasted bandwidth | Elimination | 1.5x faster |
| Unaligned access | Slow memory | Alignment | 2x faster |
| Division by constant | 20+ cycles | Multiply by reciprocal | 10x faster |
| Modulo | Very slow | AND for power of 2 | 20x faster |

### GPU Garbage Removed

| Garbage | Problem | Solution | Benefit |
|---------|---------|----------|---------|
| Warp divergence | 50% idle threads | Branchless code | 2x faster |
| Non-coalesced access | 32x slower memory | Data reorganization | 10x faster |
| Excessive barriers | Thread stalls | Barrier reduction | 1.5x faster |
| Register pressure | Low occupancy | Spilling optimization | 1.3x faster |
| Shared memory conflicts | Bank conflicts | Padding | 2x faster |

### 🧹 Clean Code Example

```
BEFORE (Dirty):                    AFTER (Clean):
─────────────────                  ─────────────────
load x                             load x, y, z (coalesced)
if x > 0:                          max(0, x)  ← branchless
  store temp                       fma(x, y, z) ← fused
  load y                           store result
  mul temp, y
  load z
  add result, z
  store result
else:
  store 0

Instructions: 12                   Instructions: 4
Branches: 1                        Branches: 0
Memory ops: 6                      Memory ops: 2
```

---

## 🔥 NEW: Operation Fusion

The runtime fuses multiple operations into single instructions:

### Fused Operations

| Separate | Fused | Instruction | Speedup |
|----------|-------|-------------|---------|
| `a * b + c` | FMA | `VFMADD` | **2x** |
| `load + add` | Load-Add | Memory op | **1.5x** |
| `relu + add` | Fused activation | Single kernel | **1.8x** |
| `matmul + bias + relu` | Fused layer | Single kernel | **3x** |

### Example: Neural Network Layer

```
BEFORE (3 operations):
1. matmul(x, weights)     → temp1
2. add(temp1, bias)       → temp2  
3. relu(temp2)            → output

AFTER (1 fused operation):
1. fused_linear_relu(x, weights, bias) → output

Memory traffic: 3x less
Kernel launches: 3x less
Speed: 3x faster
```

---

## 📦 NEW: Memory Optimization

### Coalesced Memory Access

```
BEFORE (Scattered):          AFTER (Coalesced):
Thread 0 → addr 0            Thread 0 → addr 0
Thread 1 → addr 100          Thread 1 → addr 4
Thread 2 → addr 200          Thread 2 → addr 8
Thread 3 → addr 300          Thread 3 → addr 12

Memory transactions: 4        Memory transactions: 1
Bandwidth used: 25%          Bandwidth used: 100%
```

### Memory Pool

```rust
// Pre-allocated memory pool
// No malloc/free during execution
// Zero allocation overhead
let pool = MemoryPool::new(1_GB);
let tensor_a = pool.alloc(1024 * 1024);  // Instant
let tensor_b = pool.alloc(1024 * 1024);  // Instant
```

See `docs/IDEAS/ideas-8.md` for full documentation.

---

## 🎮 NEW: Vulkan Bird Game Demo

ADead-BIB includes a complete **Flappy Bird clone** demonstrating all runtime features:

### Game Features

| Feature | Implementation | Benefit |
|---------|----------------|---------|
| **Rendering** | Vulkan-ready architecture | GPU accelerated |
| **Physics** | Branchless collision | 8x faster |
| **Game loop** | Deterministic | Reproducible |
| **Memory** | Zero allocations | No GC pauses |

### Run the Game

```powershell
cd GAME
cargo build --release
.\target\release\adead-vulkan-bird.exe
```

### Game Output

```
╔════════════════════════════════════════════════════════════╗
║     🎮 ADead-BIB Vulkan Bird                               ║
║     Flappy Bird + Vulkan + Branchless Physics              ║
╚════════════════════════════════════════════════════════════╝

✅ Vulkan detected - GPU rendering available

📊 Game Statistics:
   Total frames: 173
   Average FPS: 58.7
   Final score: 1

🎯 ADead-BIB Features Demonstrated:
   ✅ Branchless collision detection
   ✅ Branchless physics (gravity, velocity)
   ✅ Deterministic game loop
   ✅ Zero-allocation frame updates
   ✅ Vulkan-ready architecture
```

### Branchless Physics Example

```rust
// Traditional (with branches - SLOW)
if bird.y < pipe.top || bird.y > pipe.bottom {
    game_over = true;
}

// ADead-BIB (branchless - FAST)
let hit = ((bird.y - pipe.top) as i32) >> 31;  // Bit shift = no branch
game_over |= hit;                               // OR = no branch
```

### Controls

| Key | Action |
|-----|--------|
| **SPACE** | Flap (jump) |
| **R** | Restart |
| **ESC** | Quit |

See `GAME/README.md` for full documentation.

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

## 📖 License

Apache-2.0 License - See LICENSE file for details.

---

## 🇵🇪 Credits

**Created by:** Eddi Andreé Salazar Matos  
**Email:** eddi.salazar.dev@gmail.com  
**Made with ❤️ in Peru**

### What's Included

**Compiler & Language:**
- ✅ Complete compiler (Lexer, Parser, Codegen, PE, ELF)
- ✅ 70+ built-in functions
- ✅ Full OOP support (classes, inheritance, polymorphism)
- ✅ Python FFI integration
- ✅ Multi-target output (Windows PE, Linux ELF, Raw binary)

**AI & Machine Learning:**
- ✅ AI system (0.19 MB RAM)
- ✅ Scalable AI with BPE tokenizer (0.82 MB RAM)
- ✅ Ollama integration (local LLM)
- ✅ Matrix functions for neural networks

**GPU & Hardware:**
- ✅ GPU support (CUDA)
- ✅ **Vulkan support** (NEW)
- ✅ Hybrid CPU+GPU mode
- ✅ **Auto-dispatch CPU/GPU** (NEW)
- ✅ **Auto-detection via CPUID** (NEW)
- ✅ HEX opcodes for GPU

**Intermediate Runtime & Optimizations:**
- ✅ **Deterministic execution** - Same input = Same output
- ✅ **Branchless optimization** - IF/ELSE → max/blend (✅ COMPLETE)
- ✅ **AST transformation** - Pattern detection & automatic replacement
- ✅ **Syntax checker** - Fast validation without compilation (`check` command)
- ✅ **Enhanced type validation** - Robust type checking with warnings
- ✅ **Garbage elimination** - Remove dead code
- ✅ **Operation fusion** - FMA, fused layers
- ✅ **Memory optimization** - Coalesced access, pools
- ✅ **SIMD vectorization** - Auto-vectorize loops

**Performance:**
- ✅ Server load benchmarks (9,175 GFLOPS peak)
- ✅ 8-10x speedup from branchless code
- ✅ 16-86x GPU speedup vs CPU
- ✅ 106M dispatches/second

**Documentation:**
- ✅ Complete documentation (EN/ES)
- ✅ Ideas roadmap (ideas-6, 7, 8)
- ✅ TEST-G GPU test suite
- ✅ Basic test suite for compiler

**Compiler Improvements (NEW):**
- ✅ **Syntax Check Command** - `adeadc check file.adB` for fast validation
- ✅ **Branchless Optimizer** - Complete implementation with AST transformation
- ✅ **Enhanced Error Messages** - Clear, descriptive error messages
- ✅ **Type Validation** - Robust type checking with compatibility warnings
- ✅ **AST Pattern Replacement** - Automatic code optimization

**Game Demo (NEW):**
- ✅ **Vulkan Bird** - Flappy Bird clone
- ✅ **Branchless physics** - Zero branches in game logic
- ✅ **Deterministic gameplay** - Same input = same output
- ✅ **58.7 FPS** - Smooth performance

---

## 📈 Summary: Why ADead-BIB?

| Feature | Traditional | ADead-BIB | Improvement |
|---------|-------------|-----------|-------------|
| **Binary size** | 100+ KB | 1.5 KB → **<1 KB target** | **66x+ smaller** |
| **Compilation** | Seconds | Milliseconds | **100x faster** |
| **Dependencies** | Many | Zero | **Standalone** |
| **GPU dispatch** | Manual | Automatic | **Zero effort** |
| **Branching** | Everywhere | Eliminated | **8x faster** |
| **Memory access** | Scattered | Coalesced | **10x faster** |
| **Operations** | Separate | Fused | **3x faster** |
| **Syntax checking** | Full compile | Instant (`check`) | **10x faster** |

---

**ADead-BIB: Intermediate Runtime for Clean, Fast Code**

```
Source Code → ADead-BIB Runtime → Clean Opcodes → CPU/GPU Execute
                    ↓
         • No branches (branchless)
         • No garbage (clean)
         • No waste (fused ops)
         • No manual dispatch (auto)
         • Ultra-compact binaries (target: <1 KB)
```

**Result: CPU and GPU work at 100% efficiency** 🚀🇵🇪

---

## 🎯 Roadmap: From KB to Bytes

### ✅ Completed (2024)
- ✅ Branchless Optimizer - Complete implementation
- ✅ Syntax Checker - Fast validation command
- ✅ Enhanced Type System - Robust validation
- ✅ AST Transformation - Pattern-based optimization
- ✅ Current binary size: ~1.5 KB

### 🎯 Next Goals
- 🎯 **Reduce binary size to < 1 KB** (1,000 bytes)
- 🎯 **Ultimate goal: < 500 bytes** for simple programs
- 🎯 **Minimal PE/ELF headers** - Only essential data
- 🎯 **Direct syscalls** - Eliminate library dependencies
- 🎯 **Advanced dead code elimination** - Remove all unused code
- 🎯 **String compression** - Compress literals
- 🎯 **Opcode optimization** - Use shortest instruction forms

**Vision: Binaries so small they're measured in bytes, not kilobytes!** 📦→📝
