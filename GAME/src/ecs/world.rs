// ============================================================================
// World - ADead-BIB ECS
// ============================================================================
// Contenedor principal de entidades y sistemas

use super::entity::{Entity, EntityId};
use super::components::*;
use std::collections::HashMap;

/// Mundo del juego - contiene todas las entidades
pub struct World {
    next_id: EntityId,
    pub entities: HashMap<EntityId, Entity>,
    pub transforms: HashMap<EntityId, Transform>,
    pub velocities: HashMap<EntityId, Velocity>,
    pub sprites: HashMap<EntityId, Sprite>,
    pub colliders: HashMap<EntityId, Collider>,
    pub rigid_bodies: HashMap<EntityId, RigidBody>,
    pub healths: HashMap<EntityId, Health>,
    pub scores: HashMap<EntityId, Score>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            entities: HashMap::new(),
            transforms: HashMap::new(),
            velocities: HashMap::new(),
            sprites: HashMap::new(),
            colliders: HashMap::new(),
            rigid_bodies: HashMap::new(),
            healths: HashMap::new(),
            scores: HashMap::new(),
        }
    }
    
    /// Crear nueva entidad
    pub fn spawn(&mut self, name: &str) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        self.entities.insert(id, Entity::new(id, name));
        id
    }
    
    /// Crear entidad con transform
    pub fn spawn_at(&mut self, name: &str, x: f32, y: f32) -> EntityId {
        let id = self.spawn(name);
        self.transforms.insert(id, Transform::new(x, y));
        id
    }
    
    /// Destruir entidad
    pub fn destroy(&mut self, id: EntityId) {
        self.entities.remove(&id);
        self.transforms.remove(&id);
        self.velocities.remove(&id);
        self.sprites.remove(&id);
        self.colliders.remove(&id);
        self.rigid_bodies.remove(&id);
        self.healths.remove(&id);
        self.scores.remove(&id);
    }
    
    /// Agregar componente Transform
    pub fn add_transform(&mut self, id: EntityId, transform: Transform) -> &mut Self {
        self.transforms.insert(id, transform);
        self
    }
    
    /// Agregar componente Velocity
    pub fn add_velocity(&mut self, id: EntityId, velocity: Velocity) -> &mut Self {
        self.velocities.insert(id, velocity);
        self
    }
    
    /// Agregar componente Sprite
    pub fn add_sprite(&mut self, id: EntityId, sprite: Sprite) -> &mut Self {
        self.sprites.insert(id, sprite);
        self
    }
    
    /// Agregar componente Collider
    pub fn add_collider(&mut self, id: EntityId, collider: Collider) -> &mut Self {
        self.colliders.insert(id, collider);
        self
    }
    
    /// Agregar componente RigidBody
    pub fn add_rigid_body(&mut self, id: EntityId, rb: RigidBody) -> &mut Self {
        self.rigid_bodies.insert(id, rb);
        self
    }
    
    /// Agregar componente Health
    pub fn add_health(&mut self, id: EntityId, health: Health) -> &mut Self {
        self.healths.insert(id, health);
        self
    }
    
    /// Agregar componente Score
    pub fn add_score(&mut self, id: EntityId, score: Score) -> &mut Self {
        self.scores.insert(id, score);
        self
    }
    
    /// Obtener entidades con tag específico
    pub fn get_by_tag(&self, tag: &str) -> Vec<EntityId> {
        self.entities
            .iter()
            .filter(|(_, e)| e.tag == tag && e.active)
            .map(|(id, _)| *id)
            .collect()
    }
    
    /// Obtener todas las entidades activas
    pub fn get_active(&self) -> Vec<EntityId> {
        self.entities
            .iter()
            .filter(|(_, e)| e.active)
            .map(|(id, _)| *id)
            .collect()
    }
    
    /// Limpiar todas las entidades
    pub fn clear(&mut self) {
        self.entities.clear();
        self.transforms.clear();
        self.velocities.clear();
        self.sprites.clear();
        self.colliders.clear();
        self.rigid_bodies.clear();
        self.healths.clear();
        self.scores.clear();
        self.next_id = 1;
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
