// ============================================================
// ADead-BIB - GPU Backend
// ============================================================
// BINARY IS BINARY - Emitimos bytes GPU DIRECTAMENTE
// Sin GLSL. Sin HLSL. Código → SPIR-V/CUDA bytes → GPU
//
// Estructura:
// - hex/             : 🔥 CORE - Opcodes GPU directos (0xC0DA...)
// - bytecode_spirv.rs: ADead Bytecode → SPIR-V bytes
// - vulkan/          : Backend Vulkan (SPIR-V directo)
// - cuda.rs          : Backend CUDA (PTX directo)
// - gpu_detect.rs    : Detección de GPU
// - scheduler.rs     : Scheduler CPU↔GPU
// - memory.rs        : Memoria explícita (buffers)
// - metrics.rs       : Métricas reales
//
// Filosofía: "Bytes directos a la GPU. Sin shaders textuales."
// ============================================================

pub mod gpu_detect;
pub mod vulkan;
pub mod hex;
pub mod scheduler;
pub mod memory;
pub mod bytecode_spirv;
pub mod metrics;
pub mod vulkan_runtime;
pub mod cuda;
pub mod unified_pipeline;

// Re-exports principales
pub use gpu_detect::*;
pub use scheduler::{GpuScheduler, Dispatch, CommandBuffer};
pub use memory::{GpuAllocator, BufferUsage, MemoryType};
pub use bytecode_spirv::{BytecodeToSpirV, ADeadGpuOp};
pub use metrics::{GpuProfiler, GpuMetrics, PerformanceEstimator};
