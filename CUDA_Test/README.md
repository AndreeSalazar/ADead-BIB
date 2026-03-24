# CUDA_Test — CUDead-BIB Examples

> Eddi Andreé Salazar Matos — Lima, Perú 🇵🇪
> ADead-BIB ecosystem — Binary Is Binary 💀🦈

---

## Estructura

```
CUDA_Test/
├── include/
│   └── cudead.h        # Single header — 8 primitivas
├── examples/
│   ├── vecadd.cu       # Vector addition básico
│   ├── matmul.cu       # Matrix multiply con shared memory
│   └── reduction.cu    # Parallel reduction
└── README.md
```

---

## Las 8 Primitivas

```c
#include <cudead.h>

// MEMORY
GpuPtr d_data = cudead_malloc(bytes);    // Alloca VRAM
cudead_free(d_data);                      // Libera VRAM
cudead_push(host, gpu, bytes);            // CPU → GPU
cudead_pull(gpu, host, bytes);            // GPU → CPU

// LAUNCH
cudead_launch(kernel, grid, block, ...);  // Lanza kernel
cudead_sync();                            // CPU espera GPU

// DEVICE-SIDE
cudead_block_sync();                      // = __syncthreads
cudead_warp_sync();                       // = __syncwarp
```

---

## Compilar y Ejecutar

```bash
# Vector Addition
adb cuda examples/vecadd.cu

# Con pipeline visible (9 fases)
adb cuda examples/vecadd.cu --step

# Solo compilar, no ejecutar
adb cuda examples/vecadd.cu --dry

# Matrix Multiply
adb cuda examples/matmul.cu --step

# Parallel Reduction
adb cuda examples/reduction.cu --step
```

---

## Ejemplo: vecadd.cu

```c
#include <cudead.h>

__cudead_kernel__ void vectorAdd(
    const float* A,
    const float* B,
    float* C,
    int n
) {
    int idx = cudead_global_idx();
    if (idx < n) {
        C[idx] = A[idx] + B[idx];
    }
}

int main(void) {
    cudead_gpu_print();

    const int N = 1024 * 1024;
    const size_t bytes = N * sizeof(float);

    float* h_A = malloc(bytes);
    float* h_B = malloc(bytes);
    float* h_C = malloc(bytes);

    GpuPtr d_A = cudead_malloc(bytes);
    GpuPtr d_B = cudead_malloc(bytes);
    GpuPtr d_C = cudead_malloc(bytes);

    cudead_push(h_A, d_A, bytes);
    cudead_push(h_B, d_B, bytes);

    cudead_launch(vectorAdd, (N+255)/256, 256, d_A, d_B, d_C, N);
    cudead_sync();

    cudead_pull(d_C, h_C, bytes);

    cudead_free(d_A);
    cudead_free(d_B);
    cudead_free(d_C);

    return 0;
}
```

---

## Pipeline (9 Fases)

```
Phase 00: GPU DETECT      → RTX 3060 12GB sm_86
Phase 01: READ SOURCE     → archivo.cu
Phase 02: PARSER / AST    → kernels encontrados
Phase 03: IR              → CUDeadOp SSA-form
Phase 04: UB DETECTOR ★   → 10 tipos verificados
Phase 05: OPTIMIZER       → coalescing, occupancy
Phase 06: PTX EMITTER     → PTX 7.0 generado
Phase 07: VRAM LAYOUT     → 128B aligned
Phase 08: OUTPUT          → .ptx / .cubin
Phase 09: EXECUTE         → PCIe → VRAM → kernel → sync
```

---

## RTX 3060 Specs

```
GPU:            NVIDIA RTX 3060 12GB
Architecture:   GA106 — Ampere
Compute:        sm_86
SMs:            28
CUDA cores:     3584
VRAM:           12 GB GDDR6
Warp size:      32
Optimal block:  256 threads
```

---

## UB GPU Detectado

| UB Type | Severity | Description |
|---------|----------|-------------|
| RaceCondition | Error | Dos hilos escriben mismo addr |
| OutOfBounds | Error | idx sin boundary check |
| MissingSync | Error | shared sin cudead_block_sync |
| ConditionalSync | Error | sync dentro de if |
| IndexOverflow | Warning | overflow en idx |
| MisalignedAccess | Warning | ptr no alineado 128B |
| UncoalescedAccess | Warning | acceso no lineal |
| WarpDivergence | Info | >50% hilos inactivos |
| BankConflict | Info | shared memory conflict |

---

## Comparación

```
                CUDA oficial    CUDead-BIB
────────────────────────────────────────────
Instalación     500MB ❌        ~100KB ✅
Header          cuda_runtime.h  cudead.h ✅
Primitivas      13 librerías    8 funciones ✅
Sin NVCC        NO ❌           SÍ ✅
UB Detection    NO ❌           10 tipos ✅
```

---

*CUDead-BIB — RTX 3060 — PCIe raw — Sin NVCC — Binary Is Binary 💀🦈*
