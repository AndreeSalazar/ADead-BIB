// ADead-BIB Heredar - Sistema de Herencia
// Facilita el uso de ADead-BIB para Game Engines, Graphics Engines y Compute
// Nivel militar: máxima optimización, zero-copy, determinista
//
// Autor: Eddi Andreé Salazar Matos

pub mod GameEngine;
pub mod GraphicsEngine;
pub mod ComputeEngine;
pub mod Templates;

// Re-exports para uso fácil
pub use GameEngine::{GameEngine, GameEngineBuilder, GameEngineConfig};
pub use GraphicsEngine::{GraphicsEngine, GraphicsEngineBuilder, GraphicsConfig};
pub use ComputeEngine::{ComputeEngine, ComputeEngineBuilder, ComputeConfig};
pub use Templates::gpu_context::GpuContext;
pub use Templates::benchmark::{Benchmark, BenchmarkSuite, BenchmarkResult};

/// Versión de Heredar
pub const VERSION: &str = "1.0.0";

/// Inicializa Heredar con configuración por defecto
pub fn init() {
    println!("🎮 ADead-BIB Heredar v{}", VERSION);
    println!("   Sistema de Herencia para Game/Graphics/Compute Engines");
    println!();
}

/// Crea un Game Engine preconfigurado para RTX 3060
pub fn game_engine_rtx3060() -> GameEngine::GameEngine {
    GameEngine::GameEngineBuilder::new()
        .with_name("ADead Game Engine")
        .with_resolution(1920, 1080)
        .with_target_fps(60)
        .build()
}

/// Crea un Graphics Engine preconfigurado para RTX 3060
pub fn graphics_engine_rtx3060() -> GraphicsEngine::GraphicsEngine {
    GraphicsEngine::GraphicsEngineBuilder::new()
        .with_backend(GraphicsEngine::RenderBackend::Vulkan)
        .with_resolution(1920, 1080)
        .with_ray_tracing(true)
        .build()
}

/// Crea un Compute Engine preconfigurado para RTX 3060
pub fn compute_engine_rtx3060() -> ComputeEngine::ComputeEngine {
    ComputeEngine::ComputeEngineBuilder::new()
        .with_workgroup(256, 1, 1)
        .with_scheduling(ComputeEngine::SchedulingMode::Deterministic)
        .build()
}

/// Crea un GPU Context para RTX 3060
pub fn gpu_context_rtx3060() -> Templates::gpu_context::GpuContext {
    Templates::gpu_context::GpuContext::rtx3060()
}
