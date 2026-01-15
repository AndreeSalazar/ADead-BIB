// ============================================================================
// Collision System - ADead-BIB Engine
// ============================================================================

use crate::ecs::{World, EntityId, Collider, Transform};

/// Resultado de colisión
#[derive(Debug, Clone)]
pub struct CollisionEvent {
    pub entity_a: EntityId,
    pub entity_b: EntityId,
    pub is_trigger: bool,
}

/// Sistema de colisiones
pub struct CollisionSystem {
    pub events: Vec<CollisionEvent>,
}

impl CollisionSystem {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    
    /// Detectar todas las colisiones
    pub fn update(&mut self, world: &World) {
        self.events.clear();
        
        // Obtener todas las entidades con collider
        let entities: Vec<_> = world.colliders.keys().cloned().collect();
        
        // Verificar colisiones entre pares
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let id_a = entities[i];
                let id_b = entities[j];
                
                // Obtener componentes
                let (col_a, col_b) = match (
                    world.colliders.get(&id_a),
                    world.colliders.get(&id_b)
                ) {
                    (Some(a), Some(b)) => (a, b),
                    _ => continue,
                };
                
                let (pos_a, pos_b) = match (
                    world.transforms.get(&id_a),
                    world.transforms.get(&id_b)
                ) {
                    (Some(a), Some(b)) => (a, b),
                    _ => continue,
                };
                
                // Verificar colisión
                if col_a.check_collision(pos_a, col_b, pos_b) {
                    self.events.push(CollisionEvent {
                        entity_a: id_a,
                        entity_b: id_b,
                        is_trigger: col_a.is_trigger || col_b.is_trigger,
                    });
                }
            }
        }
    }
    
    /// Verificar si una entidad colisionó con algo
    pub fn has_collision(&self, entity: EntityId) -> bool {
        self.events.iter().any(|e| e.entity_a == entity || e.entity_b == entity)
    }
    
    /// Obtener todas las colisiones de una entidad
    pub fn get_collisions(&self, entity: EntityId) -> Vec<EntityId> {
        self.events
            .iter()
            .filter_map(|e| {
                if e.entity_a == entity {
                    Some(e.entity_b)
                } else if e.entity_b == entity {
                    Some(e.entity_a)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for CollisionSystem {
    fn default() -> Self {
        Self::new()
    }
}
