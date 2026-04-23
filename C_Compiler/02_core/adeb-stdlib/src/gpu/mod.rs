// ============================================================
// ADead-BIB GPU Stdlib — C ABI
// ============================================================
// GPU support for C compilation → native
// OpenGL 1.0-4.6 + Vulkan 1.3 + SPIR-V
// ============================================================

pub mod fastos_gpu;
pub mod fastos_com;
pub mod opengl;
pub mod vulkan;

pub use fastos_gpu::*;
