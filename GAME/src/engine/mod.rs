// ============================================================================
// ADead-BIB Game Engine - Core Module
// ============================================================================
// Motor de juegos OOP con Rust + Vulkan
// Arquitectura: ADead-BIB + Rust + Vulkan
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

pub mod window;
pub mod renderer;
pub mod gpu_renderer;
pub mod input;
pub mod time;

pub use window::Window;
pub use renderer::Renderer;
pub use gpu_renderer::{GpuRenderer, GpuColor, Vertex};
pub use input::{Input, KeyCode};
pub use time::Time;

/// Configuración del engine
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
    pub fullscreen: bool,
    pub target_fps: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            title: "ADead-BIB Game".to_string(),
            width: 800,
            height: 600,
            vsync: true,
            fullscreen: false,
            target_fps: 60,
        }
    }
}

impl EngineConfig {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            ..Default::default()
        }
    }
}
