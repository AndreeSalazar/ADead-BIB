// ADead-BIB Runtime - Módulo Principal
// Runtime determinista para exprimir CPU y GPU al máximo
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com
//
// Nota: gpu_detect movido a backend/gpu/gpu_detect.rs

pub mod cpu_detect;
pub mod dispatcher;

pub use cpu_detect::{CPUFeatures, ComputeBackend};
pub use dispatcher::{AutoDispatcher, SystemInfo, PerformanceEstimator};

// Re-export GPU detect desde backend
pub use crate::backend::gpu::gpu_detect::{GPUFeatures, GPUVendor, detect_vulkan_simple, detect_cuda_simple};
