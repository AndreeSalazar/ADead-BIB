// ============================================================================
// Render System - ADead-BIB Engine
// ============================================================================

use crate::ecs::World;
use crate::engine::renderer::{Renderer, Color};

/// Sistema de renderizado
pub struct RenderSystem;

impl RenderSystem {
    pub fn new() -> Self {
        Self
    }
    
    /// Renderizar todas las entidades con sprite
    pub fn render(&self, world: &World, renderer: &mut Renderer) {
        // Obtener entidades con sprite y transform
        for (id, sprite) in &world.sprites {
            if !sprite.visible {
                continue;
            }
            
            let transform = match world.transforms.get(id) {
                Some(t) => t,
                None => continue,
            };
            
            // Dibujar sprite como rectángulo
            let color = Color::rgb(sprite.color.0, sprite.color.1, sprite.color.2);
            renderer.draw_rect(
                transform.x as i32,
                transform.y as i32,
                (sprite.width * transform.scale_x) as u32,
                (sprite.height * transform.scale_y) as u32,
                color,
            );
        }
    }
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self::new()
    }
}
