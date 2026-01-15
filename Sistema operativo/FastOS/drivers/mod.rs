// ============================================================================
// FastOS - Drivers Module
// ============================================================================
// Módulo de drivers del kernel
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

pub mod display;
pub mod disk;

pub use display::{Display, DisplayMode, FramebufferInfo};
pub use disk::{AtaDisk, DiskInfo};
