// ============================================================================
// lifetime_determinista.rs — Ownership y Lifetime sin GC
// ============================================================================
//
// FILOSOFÍA: Drop timing 100% predecible. No hay pause-the-world.
// Orden de drop = orden inverso de declaración. Cero GC.
//
// Implementa un borrow checker simplificado para el compilador C:
//   - Tracking de ownership por scope
//   - Detección de use-after-free en compile-time
//   - Drop automático al salir del scope
//   - Reference counting determinista (para shared ownership)
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================================

use std::collections::HashMap;

/// Identificador único de un lifetime
pub type LifetimeId = u64;

/// Identificador único de un recurso owned
pub type ResourceId = u64;

/// Orden de drop de los recursos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropOrder {
    /// Drop en orden inverso de declaración (LIFO) — default, más seguro
    ReverseDeclaration,
    /// Drop en orden de declaración (FIFO)
    ForwardDeclaration,
    /// Drop explícito manual (el usuario controla cuándo)
    Manual,
}

/// Estado de un recurso tracked
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState {
    /// Recurso activo y accesible
    Alive,
    /// Recurso movido a otro owner (ya no accesible aquí)
    Moved,
    /// Recurso droppado (ya no existe)
    Dropped,
    /// Recurso prestado (borrow activo)
    Borrowed { mutable: bool },
}

/// Un recurso trackeado por el lifetime manager
#[derive(Debug, Clone)]
pub struct TrackedResource {
    /// ID único del recurso
    pub id: ResourceId,
    /// Nombre para diagnósticos
    pub name: String,
    /// Tipo del recurso (nombre del tipo C)
    pub type_name: String,
    /// Tamaño en bytes
    pub size: usize,
    /// Scope que lo posee
    pub owner_scope: LifetimeId,
    /// Estado actual
    pub state: ResourceState,
    /// Orden de declaración (para determinar orden de drop)
    pub declaration_order: u64,
    /// ¿Requiere drop especial? (e.g., tiene destructor custom)
    pub needs_destructor: bool,
    /// Nombre del destructor custom (si existe)
    pub destructor_name: Option<String>,
}

/// Un scope de lifetime (e.g., un bloque {}, una función, un if-body)
#[derive(Debug, Clone)]
pub struct LifetimeScope {
    /// ID único del scope
    pub id: LifetimeId,
    /// Scope padre (None para el scope global)
    pub parent: Option<LifetimeId>,
    /// Nombre del scope para diagnósticos
    pub name: String,
    /// Recursos owned por este scope
    pub resources: Vec<ResourceId>,
    /// Profundidad del scope (0 = global)
    pub depth: u32,
    /// Orden de drop configurado
    pub drop_order: DropOrder,
}

/// El Lifetime Manager — cerebro del ownership tracking
#[derive(Debug)]
pub struct Lifetime {
    /// Todos los scopes
    scopes: HashMap<LifetimeId, LifetimeScope>,
    /// Todos los recursos
    resources: HashMap<ResourceId, TrackedResource>,
    /// Stack de scopes activos (el último es el scope actual)
    scope_stack: Vec<LifetimeId>,
    /// Siguiente ID de scope
    next_scope_id: LifetimeId,
    /// Siguiente ID de recurso
    next_resource_id: ResourceId,
    /// Contador global de declaraciones (para drop order)
    declaration_counter: u64,
    /// Errores de lifetime detectados
    errors: Vec<LifetimeError>,
}

impl Lifetime {
    /// Crear un nuevo Lifetime Manager
    pub fn new() -> Self {
        let mut lm = Self {
            scopes: HashMap::new(),
            resources: HashMap::new(),
            scope_stack: Vec::new(),
            next_scope_id: 1,
            next_resource_id: 1,
            declaration_counter: 0,
            errors: Vec::new(),
        };

        // Crear scope global
        let global_scope = LifetimeScope {
            id: 0,
            parent: None,
            name: "global".to_string(),
            resources: Vec::new(),
            depth: 0,
            drop_order: DropOrder::ReverseDeclaration,
        };
        lm.scopes.insert(0, global_scope);
        lm.scope_stack.push(0);
        lm
    }

    /// Entrar en un nuevo scope (e.g., abrir un bloque {})
    pub fn enter_scope(&mut self, name: &str) -> LifetimeId {
        let parent = *self.scope_stack.last().unwrap();
        let parent_depth = self.scopes[&parent].depth;

        let id = self.next_scope_id;
        self.next_scope_id += 1;

        let scope = LifetimeScope {
            id,
            parent: Some(parent),
            name: name.to_string(),
            resources: Vec::new(),
            depth: parent_depth + 1,
            drop_order: DropOrder::ReverseDeclaration,
        };

        self.scopes.insert(id, scope);
        self.scope_stack.push(id);
        id
    }

    /// Salir del scope actual — ejecuta drops de todos los recursos owned
    ///
    /// Retorna la lista de drops que deben ejecutarse (en orden)
    pub fn exit_scope(&mut self) -> Vec<DropAction> {
        let scope_id = match self.scope_stack.pop() {
            Some(id) if id != 0 => id,
            _ => {
                self.errors.push(LifetimeError::ScopeUnderflow);
                return Vec::new();
            }
        };

        let scope = self.scopes.get(&scope_id).cloned().unwrap();
        let mut drops = Vec::new();

        // Determinar orden de drop
        let resource_ids: Vec<ResourceId> = match scope.drop_order {
            DropOrder::ReverseDeclaration => {
                let mut ids = scope.resources.clone();
                ids.reverse();
                ids
            }
            DropOrder::ForwardDeclaration => scope.resources.clone(),
            DropOrder::Manual => Vec::new(), // No auto-drop
        };

        // Ejecutar drops
        for res_id in resource_ids {
            if let Some(resource) = self.resources.get_mut(&res_id) {
                match resource.state {
                    ResourceState::Alive | ResourceState::Borrowed { .. } => {
                        // Drop it
                        drops.push(DropAction {
                            resource_id: res_id,
                            resource_name: resource.name.clone(),
                            type_name: resource.type_name.clone(),
                            destructor: resource.destructor_name.clone(),
                            size: resource.size,
                        });
                        resource.state = ResourceState::Dropped;
                    }
                    ResourceState::Moved => {
                        // Already moved, nothing to drop
                    }
                    ResourceState::Dropped => {
                        self.errors.push(LifetimeError::DoubleFree {
                            resource: resource.name.clone(),
                        });
                    }
                }
            }
        }

        drops
    }

    /// Declarar un nuevo recurso en el scope actual
    pub fn declare_resource(
        &mut self,
        name: &str,
        type_name: &str,
        size: usize,
        needs_destructor: bool,
        destructor_name: Option<&str>,
    ) -> ResourceId {
        let scope_id = *self.scope_stack.last().unwrap();
        let id = self.next_resource_id;
        self.next_resource_id += 1;
        self.declaration_counter += 1;

        let resource = TrackedResource {
            id,
            name: name.to_string(),
            type_name: type_name.to_string(),
            size,
            owner_scope: scope_id,
            state: ResourceState::Alive,
            declaration_order: self.declaration_counter,
            needs_destructor,
            destructor_name: destructor_name.map(|s| s.to_string()),
        };

        self.resources.insert(id, resource);
        self.scopes.get_mut(&scope_id).unwrap().resources.push(id);
        id
    }

    /// Mover un recurso a otro scope (transfer ownership)
    pub fn move_resource(&mut self, id: ResourceId) -> Result<(), LifetimeError> {
        let resource = self.resources.get_mut(&id)
            .ok_or(LifetimeError::ResourceNotFound(id))?;

        match resource.state {
            ResourceState::Alive => {
                resource.state = ResourceState::Moved;
                Ok(())
            }
            ResourceState::Moved => {
                Err(LifetimeError::UseAfterMove {
                    resource: resource.name.clone(),
                })
            }
            ResourceState::Dropped => {
                Err(LifetimeError::UseAfterFree {
                    resource: resource.name.clone(),
                })
            }
            ResourceState::Borrowed { .. } => {
                Err(LifetimeError::MoveWhileBorrowed {
                    resource: resource.name.clone(),
                })
            }
        }
    }

    /// Verificar que un recurso es accesible (no movido, no droppado)
    pub fn check_access(&self, id: ResourceId) -> Result<(), LifetimeError> {
        let resource = self.resources.get(&id)
            .ok_or(LifetimeError::ResourceNotFound(id))?;

        match resource.state {
            ResourceState::Alive | ResourceState::Borrowed { .. } => Ok(()),
            ResourceState::Moved => Err(LifetimeError::UseAfterMove {
                resource: resource.name.clone(),
            }),
            ResourceState::Dropped => Err(LifetimeError::UseAfterFree {
                resource: resource.name.clone(),
            }),
        }
    }

    /// Obtener todos los errores detectados
    pub fn errors(&self) -> &[LifetimeError] {
        &self.errors
    }

    /// ¿Hay errores?
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Scope actual
    pub fn current_scope(&self) -> LifetimeId {
        *self.scope_stack.last().unwrap()
    }

    /// Profundidad actual
    pub fn current_depth(&self) -> u32 {
        let id = self.current_scope();
        self.scopes[&id].depth
    }
}

/// Acción de drop a ejecutar
#[derive(Debug, Clone)]
pub struct DropAction {
    pub resource_id: ResourceId,
    pub resource_name: String,
    pub type_name: String,
    pub destructor: Option<String>,
    pub size: usize,
}

/// Errores de lifetime
#[derive(Debug, Clone)]
pub enum LifetimeError {
    /// Recurso no encontrado
    ResourceNotFound(ResourceId),
    /// Uso después de mover
    UseAfterMove { resource: String },
    /// Uso después de liberar
    UseAfterFree { resource: String },
    /// Mover mientras hay borrow activo
    MoveWhileBorrowed { resource: String },
    /// Double free
    DoubleFree { resource: String },
    /// Scope underflow (intentar salir del scope global)
    ScopeUnderflow,
}

impl std::fmt::Display for LifetimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifetimeError::ResourceNotFound(id) => write!(f, "Resource #{} not found", id),
            LifetimeError::UseAfterMove { resource } => write!(f, "Use after move: '{}'", resource),
            LifetimeError::UseAfterFree { resource } => write!(f, "Use after free: '{}'", resource),
            LifetimeError::MoveWhileBorrowed { resource } => write!(f, "Move while borrowed: '{}'", resource),
            LifetimeError::DoubleFree { resource } => write!(f, "Double free: '{}'", resource),
            LifetimeError::ScopeUnderflow => write!(f, "Scope underflow: cannot exit global scope"),
        }
    }
}

impl std::error::Error for LifetimeError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_scope_lifecycle() {
        let mut lm = Lifetime::new();
        let r1 = lm.declare_resource("x", "int", 4, false, None);
        let _scope = lm.enter_scope("if_block");
        let r2 = lm.declare_resource("y", "int", 4, false, None);

        // y should be dropped when exiting if_block
        let drops = lm.exit_scope();
        assert_eq!(drops.len(), 1);
        assert_eq!(drops[0].resource_name, "y");

        // x should still be alive
        assert!(lm.check_access(r1).is_ok());
        // y should be dropped
        assert!(lm.check_access(r2).is_err());
    }

    #[test]
    fn test_move_semantics() {
        let mut lm = Lifetime::new();
        let r1 = lm.declare_resource("buf", "Buffer", 1024, true, Some("buffer_destroy"));

        // Move it
        lm.move_resource(r1).unwrap();

        // Access after move should fail
        assert!(lm.check_access(r1).is_err());

        // Double move should fail
        assert!(lm.move_resource(r1).is_err());
    }

    #[test]
    fn test_reverse_drop_order() {
        let mut lm = Lifetime::new();
        let _scope = lm.enter_scope("block");
        lm.declare_resource("a", "int", 4, false, None);
        lm.declare_resource("b", "int", 4, false, None);
        lm.declare_resource("c", "int", 4, false, None);

        let drops = lm.exit_scope();
        // Drop order should be reverse: c, b, a
        assert_eq!(drops[0].resource_name, "c");
        assert_eq!(drops[1].resource_name, "b");
        assert_eq!(drops[2].resource_name, "a");
    }

    #[test]
    fn test_destructor_tracking() {
        let mut lm = Lifetime::new();
        let _scope = lm.enter_scope("block");
        lm.declare_resource("file", "FILE", 8, true, Some("fclose"));

        let drops = lm.exit_scope();
        assert_eq!(drops.len(), 1);
        assert_eq!(drops[0].destructor.as_deref(), Some("fclose"));
    }
}
