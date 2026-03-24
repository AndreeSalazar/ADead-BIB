// ============================================================
// cudead.h — CUDead-BIB Single Header
// ============================================================
// El ÚNICO include necesario para GPU programming
// Sin CUDA toolkit — Sin NVCC — PCIe raw directo
//
// Eddi Andreé Salazar Matos — Lima, Perú 🇵🇪
// ADead-BIB ecosystem — Binary Is Binary 💀🦈
// ============================================================

#ifndef CUDEAD_H
#define CUDEAD_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================
// TIPOS BASE
// ============================================================

// GpuPtr — puntero explícito a VRAM
typedef struct {
    uint64_t addr;   // dirección VRAM absoluta
    uint64_t size;   // bytes allocados
    uint64_t align;  // alineación real
} GpuPtr;

// LaunchConfig — configuración 3D de lanzamiento
typedef struct {
    uint32_t grid_x;
    uint32_t grid_y;
    uint32_t grid_z;
    uint32_t block_x;
    uint32_t block_y;
    uint32_t block_z;
    uint32_t shared_mem;
} LaunchConfig;

// GpuInfo — información del GPU detectado
typedef struct {
    char name[64];
    uint32_t sm_version;
    uint32_t sm_count;
    uint64_t vram_bytes;
    uint32_t max_threads_per_block;
    uint32_t max_shared_per_block;
    uint32_t warp_size;
} GpuInfo;

// ============================================================
// PRIMITIVA 1: cudead_malloc — Alloca VRAM
// ============================================================

// Alloca N bytes en VRAM — alineación 128B automática
GpuPtr cudead_malloc(size_t size);

// Con alineación explícita
GpuPtr cudead_malloc_aligned(size_t size, size_t alignment);

// ============================================================
// PRIMITIVA 2: cudead_free — Libera VRAM
// ============================================================

// Double-free → RECHAZADO por UB detector
// NULL → ignorado silencioso
void cudead_free(GpuPtr ptr);

// ============================================================
// PRIMITIVA 3: cudead_push — CPU → GPU (H2D)
// ============================================================

// PCIe transfer: RAM → VRAM
// Explícito siempre — el programador controla
void cudead_push(const void* host_ptr, GpuPtr gpu_ptr, size_t bytes);

// ============================================================
// PRIMITIVA 4: cudead_pull — GPU → CPU (D2H)
// ============================================================

// PCIe transfer: VRAM → RAM
// Explícito siempre
void cudead_pull(GpuPtr gpu_ptr, void* host_ptr, size_t bytes);

// ============================================================
// PRIMITIVA 5: cudead_launch — Lanza kernel
// ============================================================

// 1D simple — variadic args
#define cudead_launch(kernel, grid, block, ...) \
    _cudead_launch_1d(#kernel, (void*)kernel, grid, block, __VA_ARGS__)

// 3D con config completa
#define cudead_launch_3d(kernel, cfg, ...) \
    _cudead_launch_3d(#kernel, (void*)kernel, cfg, __VA_ARGS__)

// Internal launch functions
void _cudead_launch_1d(const char* name, void* kernel, 
                       uint32_t grid, uint32_t block, ...);
void _cudead_launch_3d(const char* name, void* kernel, 
                       LaunchConfig cfg, ...);

// ============================================================
// PRIMITIVA 6: cudead_sync — CPU espera GPU
// ============================================================

// CPU espera que GPU termine TODOS los kernels
// Explícito siempre — nunca implícito
void cudead_sync(void);

// ============================================================
// PRIMITIVA 7: cudead_block_sync — Sync dentro del bloque
// ============================================================

// = __syncthreads() en CUDA
// SOLO usar dentro de __cudead_kernel__
#define cudead_block_sync() __cudead_block_sync()

// ============================================================
// PRIMITIVA 8: cudead_warp_sync — Sync dentro del warp
// ============================================================

// = __syncwarp() en CUDA
// SOLO usar dentro de __cudead_kernel__
#define cudead_warp_sync() __cudead_warp_sync()

// ============================================================
// KERNEL ATTRIBUTES
// ============================================================

// __cudead_kernel__ reemplaza __global__
#define __cudead_kernel__ __attribute__((cudead_kernel))

// __cudead_device__ reemplaza __device__
#define __cudead_device__ __attribute__((cudead_device))

// __shared__ — shared memory explícita
#define __shared__ __attribute__((cudead_shared))

// __restrict__ — hint de no-aliasing
#ifndef __restrict__
#define __restrict__ __restrict
#endif

// ============================================================
// ÍNDICES — EXPLÍCITOS SIEMPRE
// ============================================================

// Thread indices (dentro del bloque)
extern __cudead_device__ uint32_t threadIdx_x;
extern __cudead_device__ uint32_t threadIdx_y;
extern __cudead_device__ uint32_t threadIdx_z;

// Block indices (dentro del grid)
extern __cudead_device__ uint32_t blockIdx_x;
extern __cudead_device__ uint32_t blockIdx_y;
extern __cudead_device__ uint32_t blockIdx_z;

// Block dimensions
extern __cudead_device__ uint32_t blockDim_x;
extern __cudead_device__ uint32_t blockDim_y;
extern __cudead_device__ uint32_t blockDim_z;

// Grid dimensions
extern __cudead_device__ uint32_t gridDim_x;
extern __cudead_device__ uint32_t gridDim_y;
extern __cudead_device__ uint32_t gridDim_z;

// Convenience macros (CUDA-compatible)
#define threadIdx ((struct { uint32_t x, y, z; }){threadIdx_x, threadIdx_y, threadIdx_z})
#define blockIdx  ((struct { uint32_t x, y, z; }){blockIdx_x, blockIdx_y, blockIdx_z})
#define blockDim  ((struct { uint32_t x, y, z; }){blockDim_x, blockDim_y, blockDim_z})
#define gridDim   ((struct { uint32_t x, y, z; }){gridDim_x, gridDim_y, gridDim_z})

// Helper functions
static inline uint32_t cudead_global_idx(void) {
    return blockIdx_x * blockDim_x + threadIdx_x;
}

static inline uint32_t cudead_global_idx_2d_x(void) {
    return blockIdx_x * blockDim_x + threadIdx_x;
}

static inline uint32_t cudead_global_idx_2d_y(void) {
    return blockIdx_y * blockDim_y + threadIdx_y;
}

static inline uint32_t cudead_total_threads(void) {
    return gridDim_x * gridDim_y * gridDim_z * 
           blockDim_x * blockDim_y * blockDim_z;
}

// ============================================================
// GPU DETECTION & INFO
// ============================================================

// Detecta GPU disponible — retorna 0 si no hay
int cudead_gpu_detect(void);

// Obtiene info del GPU
GpuInfo cudead_gpu_info(void);

// Imprime info del GPU a stdout
void cudead_gpu_print(void);

// ============================================================
// MATH INTRINSICS (device-side)
// ============================================================

#ifdef __CUDEAD_DEVICE_CODE__

// Fast math
__cudead_device__ float  cudead_sqrtf(float x);
__cudead_device__ float  cudead_rsqrtf(float x);
__cudead_device__ float  cudead_sinf(float x);
__cudead_device__ float  cudead_cosf(float x);
__cudead_device__ float  cudead_expf(float x);
__cudead_device__ float  cudead_logf(float x);
__cudead_device__ float  cudead_powf(float x, float y);
__cudead_device__ float  cudead_fmaf(float a, float b, float c);

// Atomics
__cudead_device__ int    cudead_atomicAdd_i32(int* addr, int val);
__cudead_device__ float  cudead_atomicAdd_f32(float* addr, float val);
__cudead_device__ int    cudead_atomicMin_i32(int* addr, int val);
__cudead_device__ int    cudead_atomicMax_i32(int* addr, int val);
__cudead_device__ int    cudead_atomicCAS_i32(int* addr, int compare, int val);

// Warp primitives
__cudead_device__ int    cudead_warp_ballot(int predicate);
__cudead_device__ int    cudead_warp_any(int predicate);
__cudead_device__ int    cudead_warp_all(int predicate);
__cudead_device__ float  cudead_warp_shfl(float val, int src_lane);
__cudead_device__ float  cudead_warp_shfl_down(float val, int delta);
__cudead_device__ float  cudead_warp_shfl_up(float val, int delta);
__cudead_device__ float  cudead_warp_shfl_xor(float val, int lane_mask);

#endif // __CUDEAD_DEVICE_CODE__

// ============================================================
// ERROR HANDLING
// ============================================================

typedef enum {
    CUDEAD_SUCCESS = 0,
    CUDEAD_ERROR_NO_GPU = 1,
    CUDEAD_ERROR_OUT_OF_MEMORY = 2,
    CUDEAD_ERROR_INVALID_PTR = 3,
    CUDEAD_ERROR_LAUNCH_FAILED = 4,
    CUDEAD_ERROR_SYNC_FAILED = 5,
    CUDEAD_ERROR_PCIE_FAILED = 6,
} CudeadError;

// Get last error
CudeadError cudead_get_last_error(void);

// Get error string
const char* cudead_error_string(CudeadError err);

// ============================================================
// RTX 3060 CONSTANTS
// ============================================================

#define CUDEAD_RTX3060_SM_COUNT       28
#define CUDEAD_RTX3060_CUDA_CORES     3584
#define CUDEAD_RTX3060_VRAM_GB        12
#define CUDEAD_RTX3060_SM_VERSION     86
#define CUDEAD_RTX3060_WARP_SIZE      32
#define CUDEAD_RTX3060_MAX_THREADS    1024
#define CUDEAD_RTX3060_MAX_SHARED     49152
#define CUDEAD_RTX3060_OPTIMAL_BLOCK  256

#ifdef __cplusplus
}
#endif

#endif // CUDEAD_H
