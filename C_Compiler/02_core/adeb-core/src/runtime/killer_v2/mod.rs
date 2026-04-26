// ============================================================================
// ADead-BIB Runtime 2.0 Killer — Módulo Principal
// ============================================================================
//
// Arquitectura determinista de alto rendimiento para el compilador C nativo.
// Divide responsabilidades en 2 monolitos:
//
//   1. ARQUITECTURA — Infraestructura de memoria y ABI
//      - memoria_continua:      Arena allocator O(1), cero fragmentación
//      - stack_management:      Stack frames, shadow space, overflow guards
//      - lifetime_determinista: Ownership sin GC, drop en orden inverso
//      - abi_calling_convention: Win64/SysV ABI validado en compile-time
//
//   2. MONOLITO — Ejecución determinista
//      - ejecucion_matematica:  IEEE 754 strict, NaN/overflow = error explícito
//      - scheduler_determinista: Cooperative scheduling, orden reproducible
//      - error_como_valor:      Result<T, KillerError>, cero excepciones
//
// Regla fundamental: ZERO sorpresas en runtime.
// Todo comportamiento es predecible en compile-time.
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com
// ============================================================================

pub mod arquitectura;
pub mod monolito;

// Re-exports principales para acceso rápido
pub use arquitectura::memoria_continua::{Arena, ArenaConfig, ArenaStats};
pub use arquitectura::stack_management::{StackFrame, StackConfig, StackGuard};
pub use arquitectura::lifetime_determinista::{Lifetime, LifetimeScope, DropOrder};
pub use arquitectura::abi_calling_convention::{CallingConvention, AbiValidator, RegisterSlot};

pub use monolito::ejecucion_matematica::{StrictMath, MathError, MathMode};
pub use monolito::scheduler_determinista::{DeterministicScheduler, Task, TaskId, SchedulerConfig};
pub use monolito::error_como_valor::{KillerError, KillerResult, ErrorKind};

/// Versión del Runtime Killer
pub const KILLER_VERSION: &str = "2.0.0";

/// Configuración global del runtime determinista
#[derive(Debug, Clone)]
pub struct KillerConfig {
    /// Configuración de arena allocator
    pub arena: ArenaConfig,
    /// Configuración de stack
    pub stack: StackConfig,
    /// Modo matemático (strict por defecto)
    pub math_mode: MathMode,
    /// Configuración del scheduler
    pub scheduler: SchedulerConfig,
}

impl Default for KillerConfig {
    fn default() -> Self {
        Self {
            arena: ArenaConfig::default(),
            stack: StackConfig::default(),
            math_mode: MathMode::Strict,
            scheduler: SchedulerConfig::default(),
        }
    }
}

/// Runtime Killer 2.0 — Punto de entrada unificado
#[derive(Debug)]
pub struct KillerRuntime {
    pub config: KillerConfig,
    arena: Arena,
    scheduler: DeterministicScheduler,
}

impl KillerRuntime {
    /// Crear un nuevo runtime con configuración por defecto
    pub fn new() -> Self {
        Self::with_config(KillerConfig::default())
    }

    /// Crear un nuevo runtime con configuración custom
    pub fn with_config(config: KillerConfig) -> Self {
        let arena = Arena::new(config.arena.clone());
        let scheduler = DeterministicScheduler::new(config.scheduler.clone());
        Self { config, arena, scheduler }
    }

    /// Reset completo del runtime (O(1) — solo resetea el bump pointer)
    pub fn reset(&mut self) {
        self.arena.reset();
        self.scheduler.reset();
    }

    /// Obtener estadísticas de uso del arena
    pub fn stats(&self) -> ArenaStats {
        self.arena.stats()
    }
}
