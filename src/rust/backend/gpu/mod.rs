// ============================================================
// ADead-BIB - GPU Backend
// ============================================================
// BINARY IS BINARY - Emitimos bytes GPU DIRECTAMENTE
// Sin GLSL. Sin HLSL. Código → Opcodes HEX → Backend → GPU
//
// Arquitectura de dos niveles:
// ┌─────────────────────────────────────────────────────────┐
// │ Nivel 1: Opcodes ADead-BIB (0xC0DA...)                  │
// │   - Tu contrato                                         │
// │   - Tu formato                                          │
// │   - Portable                                            │
// │   - Documentado                                         │
// ├─────────────────────────────────────────────────────────┤
// │ Nivel 2: Backend por target                             │
// │   - spirv/   → Vulkan/OpenCL (TODAS las GPUs)           │
// │   - cuda/    → NVIDIA (PTX directo)                     │
// │   - vulkan/  → Runtime Vulkan                           │
// └─────────────────────────────────────────────────────────┘
//
// Estructura:
// - hex/           : 🔥 CORE - Opcodes GPU directos (0xC0DA...)
// - spirv/         : Backend SPIR-V (Vulkan/OpenCL)
// - cuda/          : Backend CUDA (NVIDIA PTX)
// - vulkan/        : Runtime Vulkan
// - detect.rs      : Detección de GPU
// - scheduler.rs   : Scheduler CPU↔GPU
// - memory.rs      : Memoria explícita (buffers)
// - metrics.rs     : Métricas reales
//
// Filosofía: "Bytes directos a la GPU. Sin shaders textuales."
// ============================================================

// === CORE: Opcodes HEX directos ===
pub mod hex;

// === Backends por target ===
pub mod spirv;           // SPIR-V (Vulkan/OpenCL) - Todas las GPUs
pub mod cuda;            // CUDA/PTX - Solo NVIDIA
pub mod vulkan;          // Runtime Vulkan
pub mod hip;             // HIP (AMD ROCm + HIP-CPU fallback)

// === API Unificada ===
pub mod compute;         // API unificada: compute::parallel_for, compute::matmul, etc.

// === Legacy (mantener compatibilidad) ===
pub mod vulkan_runtime;  // TODO: migrar a vulkan/

// === Infraestructura ===
pub mod gpu_detect;
pub mod scheduler;
pub mod memory;
pub mod metrics;
pub mod unified_pipeline;

// Re-exports principales
pub use gpu_detect::*;
pub use scheduler::{GpuScheduler, Dispatch, CommandBuffer};
pub use memory::{GpuAllocator, BufferUsage, MemoryType};
pub use spirv::bytecode::{BytecodeToSpirV, ADeadGpuOp};
pub use metrics::{GpuProfiler, GpuMetrics, PerformanceEstimator};

// Re-exports HIP + Compute API
pub use hip::{HipCpuRuntime, HipCpuConfig, Dim3, ThreadIdx, SendPtr};
pub use hip::{HipBackend, detect_hip_backend, get_device_info, HipDeviceInfo};
pub use hip::{HipCodeGen, HipKernel, print_hip_info};
pub use hip::cuda_to_hip::{CudaToHipTranslator, translate_cuda_file};
pub use compute::{ComputeRuntime, ComputeBackend, ComputeConfig, BenchmarkResults};
