//! ADead-BIB GPU Backend Library
//! 
//! Generación de código GPU: PTX, SPIR-V, WGSL, HIP.

pub mod cudead;
pub mod spirv;
pub mod wgsl;
pub mod hip;

// Re-exports
pub use cudead::{CudaDriver, PtxEmitter, KernelDef};
pub use spirv::{SpirvEmitter, SpirvBytecode};
