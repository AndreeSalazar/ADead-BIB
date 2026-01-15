// ============================================================================
// ADead-BIB Game Engine - Library
// ============================================================================
// Motor de juegos OOP: ADead-BIB + Rust + Vulkan
//
// Arquitectura:
//   - engine/   → Core del motor (window, renderer, input, time)
//   - ecs/      → Entity Component System (entidades, componentes, world)
//   - systems/  → Sistemas (physics, collision, render)
//   - games/    → Juegos de ejemplo
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

pub mod engine;
pub mod ecs;
pub mod systems;
pub mod games;

pub use engine::{EngineConfig, Window, Renderer, Input, KeyCode, Time};
pub use ecs::{World, Entity, EntityId};
pub use games::FlappyGame;
