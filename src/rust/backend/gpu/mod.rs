// ADead-BIB - GPU Backend
// Generación de código para GPU (Vulkan, CUDA, HEX binario)
// Elimina capas innecesarias: if, where, etc. → código GPU directo
//
// Estructura:
// - gpu_detect.rs  : Detección y análisis de GPU
// - vulkan/        : Backend Vulkan (SPIR-V)
// - hex/           : Binario HEX directo para GPU
//
// Filosofía: "Detectar GPU → Analizar → Respetar → Limpiar basura"

pub mod gpu_detect;
pub mod vulkan;
pub mod hex;

// Re-export principal
pub use gpu_detect::*;
