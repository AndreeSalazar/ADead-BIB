// ============================================================================
// Window Manager - ADead-BIB Engine
// ============================================================================

use std::rc::Rc;
use winit::event_loop::EventLoop;
use winit::window::{WindowBuilder, Window as WinitWindow};
use super::EngineConfig;

/// Wrapper para la ventana del juego
pub struct Window {
    pub handle: Rc<WinitWindow>,
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl Window {
    /// Crear nueva ventana con configuración
    pub fn new(config: &EngineConfig, event_loop: &EventLoop<()>) -> Self {
        let handle = Rc::new(
            WindowBuilder::new()
                .with_title(&config.title)
                .with_inner_size(winit::dpi::LogicalSize::new(config.width, config.height))
                .with_resizable(false)
                .build(event_loop)
                .expect("Failed to create window")
        );
        
        Self {
            handle,
            width: config.width,
            height: config.height,
            title: config.title.clone(),
        }
    }
    
    /// Obtener dimensiones
    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    
    /// Obtener aspect ratio
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
