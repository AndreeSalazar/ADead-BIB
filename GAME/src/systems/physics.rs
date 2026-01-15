// ============================================================================
// Physics System - ADead-BIB Engine
// ============================================================================

use crate::ecs::World;

/// Sistema de física
pub struct PhysicsSystem {
    pub gravity: f32,
    pub max_velocity: f32,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            gravity: 980.0,  // pixels/s²
            max_velocity: 1000.0,
        }
    }
    
    pub fn with_gravity(mut self, gravity: f32) -> Self {
        self.gravity = gravity;
        self
    }
    
    /// Actualizar física de todas las entidades
    pub fn update(&self, world: &mut World, delta: f32) {
        // Obtener IDs de entidades con rigid body
        let entities: Vec<_> = world.rigid_bodies.keys().cloned().collect();
        
        for id in entities {
            // Obtener componentes
            let rb = match world.rigid_bodies.get(&id) {
                Some(rb) => *rb,
                None => continue,
            };
            
            if rb.is_kinematic {
                continue;
            }
            
            // Aplicar gravedad a velocidad
            if let Some(vel) = world.velocities.get_mut(&id) {
                // Gravedad
                vel.vy += self.gravity * rb.gravity_scale * delta;
                
                // Drag
                if rb.drag > 0.0 {
                    vel.vx *= 1.0 - rb.drag * delta;
                    vel.vy *= 1.0 - rb.drag * delta;
                }
                
                // Clamp velocidad
                vel.vx = vel.vx.clamp(-self.max_velocity, self.max_velocity);
                vel.vy = vel.vy.clamp(-self.max_velocity, self.max_velocity);
            }
            
            // Aplicar velocidad a posición
            if let (Some(vel), Some(transform)) = (
                world.velocities.get(&id),
                world.transforms.get_mut(&id)
            ) {
                transform.x += vel.vx * delta;
                transform.y += vel.vy * delta;
            }
        }
    }
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self::new()
    }
}
