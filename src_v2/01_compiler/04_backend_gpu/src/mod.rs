// ============================================================
// ADead-BIB - GPU Backend
// ============================================================
// SPIR-V + WGSL para OpenGL/Vulkan
//
// Arquitectura:
// ┌─────────────────────────────────────────────────────────┐
// │ Nivel 1: Opcodes ADead-BIB (0xC0DA...)                  │
// │   - Tu contrato                                         │
// │   - Tu formato                                          │
// │   - Portable                                            │
// │   - Documentado                                         │
// ├─────────────────────────────────────────────────────────┤
// │ Nivel 2: Backend por target                             │
// │   - spirv/   → Vulkan/OpenCL (OpenGL/Vulkan)            │
// │   - wgsl/    → WebGPU                                   │
// └─────────────────────────────────────────────────────────┘
//
// Estructura:
// - spirv/         : Backend SPIR-V (Vulkan/OpenCL/OpenGL)
// - wgsl/          : Backend WGSL (WebGPU)
// - compute/       : API unificada: compute::parallel_for, compute::matmul
// - scheduler.rs   : Scheduler CPU↔GPU
// - memory.rs      : Memoria explícita (buffers)
// - metrics.rs     : Métricas reales
//
// Filosofía: "SPIR-V portable para OpenGL/Vulkan"
// ============================================================

// === CORE: SPIR-V Backend ===
pub mod spirv;

// === WGSL Backend ===
pub mod wgsl;

// === API Unificada ===
pub mod compute;

// === Legacy (mantener compatibilidad) ===
pub mod vulkan_runtime;

// === Infraestructura ===
pub mod gpu_detect;
pub mod memory;
pub mod metrics;
pub mod scheduler;
pub mod unified_pipeline;

// Re-exports principales
pub use gpu_detect::*;
pub use memory::{BufferUsage, GpuAllocator, MemoryType};
pub use metrics::{GpuMetrics, GpuProfiler, PerformanceEstimator};
pub use scheduler::{CommandBuffer, Dispatch, GpuScheduler};
pub use spirv::bytecode::{ADeadGpuOp, BytecodeToSpirV};

// Re-exports Compute API
pub use compute::{BenchmarkResults, ComputeBackend, ComputeConfig, ComputeRuntime};

