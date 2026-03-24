# CUDead-BIB — GPU Compiler Nativo

> Eddi Andreé Salazar Matos — Lima, Perú 🇵🇪
> ADead-BIB ecosystem — Binary Is Binary 💀🦈

---

## ¿Qué es CUDead-BIB?

CUDead-BIB es el compilador GPU nativo del ecosistema ADead-BIB que:

- Compila código GPU **sin CUDA oficial de NVIDIA**
- Genera drivers GPU mínimos (~100KB vs 500MB oficial)
- Conecta directamente al silicon RTX sin bloatware
- Sigue la filosofía Faggin-Scale: gradual, bits respetados, sin overhead

---

## Las 8 Primitivas Core

Las ÚNICAS abstracciones necesarias. Todo lo demás es bloatware.

### Kernel Definition
```rust
__cudead_kernel__   // define función GPU (reemplaza __global__)
__cudead_device__   // función auxiliar GPU (reemplaza __device__)
```

### Launch
```rust
cudead_launch()     // lanza grid de hilos al GPU
cudead_sync()       // sincroniza CPU↔GPU
```

### Memory
```rust
cudead_malloc()     // alloca VRAM explícita
cudead_free()       // libera VRAM explícita
cudead_push()       // CPU RAM → GPU VRAM (H2D)
cudead_pull()       // GPU VRAM → CPU RAM (D2H)
```

**= 8 primitivas TOTALES vs CUDA oficial: 13 librerías + 500MB**

---

## Arquitectura

```
código .cu personal
        │
        ▼
[ 01 PARSER ]
  cuda_parser.rs
  __cudead_kernel__ expansion
        │
        ▼
[ 02 IR — CudeadOp ]
  AST → GPU operations SSA-form
  thread index resolution
        │
        ▼
[ 03 UB DETECTOR GPU ]  ★ ÚNICO
  race conditions detectadas
  uncoalesced memory access
  bank conflicts
  ANTES del optimizer
        │
        ▼
[ 04 GPU OPTIMIZER ]
  warp efficiency analysis
  memory coalescing optimizer
  occupancy maximizer
        │
        ▼
[ 05 PTX EMITTER ]
  CudeadOp → PTX instructions
  sin NVCC — sin runtime oficial
        │
        ▼
[ 06 DRIVER MÍNIMO ]
  PCIe init          ~5KB
  VRAM allocator     ~10KB
  Kernel scheduler   ~20KB
  Sync CPU↔GPU       ~10KB
  = ~50-100KB total
```

---

## Estructura de Archivos

```
cudead/
├── mod.rs           # Módulo principal, CudeadCompiler
├── primitives.rs    # 8 primitivas core (Dim3, GpuPtr, KernelDef, etc.)
├── parser.rs        # Lexer + Parser para código .cu
├── ir.rs            # CudeadOp, KernelIR, CudeadIR
├── ptx_emitter.rs   # Genera código PTX desde IR
├── ub_detector.rs   # Detecta UB GPU (race conditions, etc.)
├── optimizer.rs     # Optimizaciones GPU (coalescing, occupancy)
├── driver.rs        # Driver mínimo (PCIe, VRAM, scheduler)
└── README.md        # Esta documentación
```

---

## Uso

```rust
use adead_bib::backend::gpu::cudead::{CudeadCompiler, CudeadConfig, GpuArch};

// Crear compilador
let config = CudeadConfig {
    arch: GpuArch::Ampere,  // RTX 3060
    ub_detection: true,
    optimize: true,
    step_mode: true,
    ..Default::default()
};
let compiler = CudeadCompiler::with_config(config);

// Compilar código CUDA
let source = r#"
    __cudead_kernel__ void vectorAdd(float *A, float *B, float *C, int n) {
        int i = blockIdx.x * blockDim.x + threadIdx.x;
        if (i < n) {
            C[i] = A[i] + B[i];
        }
    }
"#;

let output = compiler.compile(source)?;
println!("PTX:\n{}", output.ptx);
println!("Kernels: {:?}", output.kernels);
```

---

## UB GPU Detectado

CUDead-BIB detecta estos problemas ANTES de optimizar:

| UB Type | Severity | Description |
|---------|----------|-------------|
| RaceCondition | Error | Dos hilos escriben mismo addr |
| OutOfBounds | Error | idx sin boundary check |
| UninitializedShared | Error | __shared__ sin init |
| MissingSync | Error | escritura shared sin __syncthreads |
| ConditionalSync | Error | sync dentro de if (deadlock) |
| IndexOverflow | Warning | overflow en idx computation |
| MisalignedAccess | Warning | ptr no alineado 128B |
| UncoalescedAccess | Warning | acceso global no coalescido |
| WarpDivergence | Info | >50% hilos inactivos |
| BankConflict | Info | conflictos en shared memory |

---

## GPUs Soportadas

| Architecture | GPUs | SM Version |
|--------------|------|------------|
| Turing | RTX 20xx | sm_75 |
| Ampere | RTX 30xx | sm_86 |
| Ada Lovelace | RTX 40xx | sm_89 |
| Blackwell | RTX 50xx | sm_90 |

---

## Comparación

```
                CUDA oficial    CUDead-BIB
────────────────────────────────────────────
Instalación     500MB ❌        ~100KB ✅
Telemetría      SÍ ❌           NO ✅
Bloatware       SÍ ❌           NO ✅
Primitivas      13 librerías    8 funciones ✅
Sin NVCC        NO ❌           SÍ ✅
UB Detection    NO ❌           SÍ ✅
Bits respetados PARCIAL ❌      SIEMPRE ✅
FastOS compat   NO ❌           SÍ ✅
```

---

## Estado Actual

- ✅ Parser CUDA completo
- ✅ IR con operaciones GPU
- ✅ UB Detector GPU
- ✅ PTX Emitter
- ✅ Driver mínimo (simulado)
- ✅ Optimizer básico
- 🔨 Integración CLI pendiente
- ⏳ Driver real PCIe pendiente

---

*CUDead-BIB — GPU silicon directo — Sin NVIDIA bloatware — Binary Is Binary*
*Eddi Andreé Salazar Matos — Lima, Perú 🇵🇪 — 1 dev — 💀🦈*
