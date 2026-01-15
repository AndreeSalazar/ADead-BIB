// ============================================================================
// FastOS GPU Drivers Module
// ============================================================================
// Hardware Abstraction Layer para GPUs
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

pub mod hal;
pub mod nvidia;
pub mod amd;
pub mod intel;
pub mod software;

pub use hal::*;
