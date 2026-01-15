// ============================================================================
// ECS (Entity Component System) - ADead-BIB Engine
// ============================================================================
// Sistema de entidades y componentes para OOP limpio
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

pub mod entity;
pub mod components;
pub mod world;

pub use entity::{Entity, EntityId};
pub use components::*;
pub use world::World;
