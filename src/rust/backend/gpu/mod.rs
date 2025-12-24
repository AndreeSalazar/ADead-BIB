// ADead-BIB - GPU Backend
// Arquitectura completa para exprimir GPU al máximo
// Sin capas innecesarias: código → SPIR-V → GPU
//
// Estructura:
// - gpu_detect.rs    : Detección y análisis de GPU
// - vulkan/          : Backend Vulkan (SPIR-V directo)
// - hex/             : Binario HEX directo para GPU
// - scheduler.rs     : Scheduler CPU→GPU determinista
// - memory.rs        : Memoria explícita (buffers, zero-copy)
// - bytecode_spirv.rs: Puente ADead Bytecode → SPIR-V
// - metrics.rs       : Métricas reales (latencia, ocupación)
//
// Filosofía: "Detectar GPU → Analizar → Respetar → Exprimir"
//
// Las 4 piezas clave:
// 1. Scheduler CPU→GPU (quién decide cuándo ejecutar)
// 2. Memoria explícita (dónde viven los datos)
// 3. Bytecode→SPIR-V (código en bits → GPU)
// 4. Métricas reales (sin benchmarks fake)

pub mod gpu_detect;
pub mod vulkan;
pub mod hex;
pub mod scheduler;
pub mod memory;
pub mod bytecode_spirv;
pub mod metrics;
pub mod vulkan_runtime;

// Re-exports principales
pub use gpu_detect::*;
pub use scheduler::{GpuScheduler, Dispatch, CommandBuffer};
pub use memory::{GpuAllocator, BufferUsage, MemoryType};
pub use bytecode_spirv::{BytecodeToSpirV, ADeadGpuOp};
pub use metrics::{GpuProfiler, GpuMetrics, PerformanceEstimator};
