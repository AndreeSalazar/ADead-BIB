// ============================================================================
// Entity - ADead-BIB ECS
// ============================================================================

/// ID único de entidad
pub type EntityId = u64;

/// Entidad base del juego
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub active: bool,
    pub tag: String,
}

impl Entity {
    /// Crear nueva entidad
    pub fn new(id: EntityId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            active: true,
            tag: String::new(),
        }
    }
    
    /// Crear con tag
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tag = tag.to_string();
        self
    }
    
    /// Activar/desactivar
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}
