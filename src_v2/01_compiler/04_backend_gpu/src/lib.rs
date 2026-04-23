//! ADead-BIB GPU Backend Library
//! 
//! Generación de código GPU: SPIR-V, WGSL (OpenGL/Vulkan)

pub mod spirv;
pub mod wgsl;

// Re-exports
pub use spirv::bytecode::{BytecodeToSpirV, ADeadGpuOp, ADeadGpuInstr};
