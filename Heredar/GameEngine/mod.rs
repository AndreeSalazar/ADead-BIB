// ADead-BIB Heredar - Game Engine Template
// Base para crear Game Engines de alto rendimiento
// Nivel militar: máxima optimización, zero-copy, determinista
//
// Autor: Eddi Andreé Salazar Matos

use std::time::{Duration, Instant};

/// Configuración del Game Engine
#[derive(Debug, Clone)]
pub struct GameEngineConfig {
    /// Nombre del engine
    pub name: String,
    /// Resolución (width, height)
    pub resolution: (u32, u32),
    /// FPS objetivo
    pub target_fps: u32,
    /// VSync habilitado
    pub vsync: bool,
    /// Modo fullscreen
    pub fullscreen: bool,
    /// Workgroup size para compute
    pub compute_workgroup: (u32, u32, u32),
}

impl Default for GameEngineConfig {
    fn default() -> Self {
        Self {
            name: "ADead Game Engine".to_string(),
            resolution: (1920, 1080),
            target_fps: 60,
            vsync: true,
            fullscreen: false,
            compute_workgroup: (256, 1, 1),
        }
    }
}

impl GameEngineConfig {
    /// Configuración para RTX 3060
    pub fn rtx3060() -> Self {
        Self {
            compute_workgroup: (256, 1, 1), // 8 warps óptimo
            ..Default::default()
        }
    }
    
    /// Configuración para RTX 4090
    pub fn rtx4090() -> Self {
        Self {
            compute_workgroup: (256, 1, 1),
            target_fps: 144,
            ..Default::default()
        }
    }
    
    /// Configuración de alto rendimiento
    pub fn high_performance() -> Self {
        Self {
            vsync: false,
            target_fps: 0, // Sin límite
            ..Default::default()
        }
    }
}

/// Estado del frame actual
#[derive(Debug, Clone, Default)]
pub struct FrameState {
    /// Número de frame
    pub frame_number: u64,
    /// Delta time en segundos
    pub delta_time: f64,
    /// Tiempo total transcurrido
    pub total_time: f64,
    /// FPS actual
    pub current_fps: f32,
}

/// Game Engine base
pub struct GameEngine {
    pub config: GameEngineConfig,
    pub frame_state: FrameState,
    /// Timestamp del último frame
    last_frame: Instant,
    /// Acumulador para FPS
    fps_accumulator: f64,
    fps_frame_count: u32,
    /// Running
    running: bool,
}

impl GameEngine {
    pub fn new(config: GameEngineConfig) -> Self {
        Self {
            config,
            frame_state: FrameState::default(),
            last_frame: Instant::now(),
            fps_accumulator: 0.0,
            fps_frame_count: 0,
            running: false,
        }
    }
    
    /// Inicializa el engine
    pub fn init(&mut self) -> Result<(), &'static str> {
        println!("🎮 Initializing {}...", self.config.name);
        println!("   Resolution: {}x{}", self.config.resolution.0, self.config.resolution.1);
        println!("   Target FPS: {}", self.config.target_fps);
        println!("   VSync: {}", self.config.vsync);
        
        self.running = true;
        self.last_frame = Instant::now();
        
        Ok(())
    }
    
    /// Comienza un nuevo frame
    pub fn begin_frame(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;
        
        self.frame_state.delta_time = delta.as_secs_f64();
        self.frame_state.total_time += self.frame_state.delta_time;
        self.frame_state.frame_number += 1;
        
        // Calcular FPS
        self.fps_accumulator += self.frame_state.delta_time;
        self.fps_frame_count += 1;
        
        if self.fps_accumulator >= 1.0 {
            self.frame_state.current_fps = self.fps_frame_count as f32 / self.fps_accumulator as f32;
            self.fps_accumulator = 0.0;
            self.fps_frame_count = 0;
        }
    }
    
    /// Termina el frame actual
    pub fn end_frame(&mut self) {
        // Frame limiting si hay target FPS
        if self.config.target_fps > 0 {
            let target_frame_time = Duration::from_secs_f64(1.0 / self.config.target_fps as f64);
            let elapsed = self.last_frame.elapsed();
            
            if elapsed < target_frame_time {
                std::thread::sleep(target_frame_time - elapsed);
            }
        }
    }
    
    /// Verifica si el engine está corriendo
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    /// Detiene el engine
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    /// Obtiene delta time
    pub fn delta_time(&self) -> f64 {
        self.frame_state.delta_time
    }
    
    /// Obtiene FPS actual
    pub fn fps(&self) -> f32 {
        self.frame_state.current_fps
    }
}

/// Trait para sistemas del engine
pub trait EngineSystem {
    fn init(&mut self) -> Result<(), &'static str>;
    fn update(&mut self, delta_time: f64);
    fn shutdown(&mut self);
}

/// Sistema de física (placeholder para GPU compute)
pub struct PhysicsSystem {
    pub gravity: (f32, f32, f32),
    pub substeps: u32,
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self {
            gravity: (0.0, -9.81, 0.0),
            substeps: 4,
        }
    }
}

impl EngineSystem for PhysicsSystem {
    fn init(&mut self) -> Result<(), &'static str> {
        println!("   Physics system initialized");
        Ok(())
    }
    
    fn update(&mut self, _delta_time: f64) {
        // GPU compute dispatch para física
    }
    
    fn shutdown(&mut self) {
        println!("   Physics system shutdown");
    }
}

/// Sistema de renderizado
pub struct RenderSystem {
    pub clear_color: (f32, f32, f32, f32),
    pub draw_calls: u32,
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self {
            clear_color: (0.1, 0.1, 0.1, 1.0),
            draw_calls: 0,
        }
    }
}

impl EngineSystem for RenderSystem {
    fn init(&mut self) -> Result<(), &'static str> {
        println!("   Render system initialized");
        Ok(())
    }
    
    fn update(&mut self, _delta_time: f64) {
        self.draw_calls = 0;
        // GPU render dispatch
    }
    
    fn shutdown(&mut self) {
        println!("   Render system shutdown");
    }
}

/// Builder para Game Engine
pub struct GameEngineBuilder {
    config: GameEngineConfig,
}

impl GameEngineBuilder {
    pub fn new() -> Self {
        Self {
            config: GameEngineConfig::default(),
        }
    }
    
    pub fn with_name(mut self, name: &str) -> Self {
        self.config.name = name.to_string();
        self
    }
    
    pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
        self.config.resolution = (width, height);
        self
    }
    
    pub fn with_target_fps(mut self, fps: u32) -> Self {
        self.config.target_fps = fps;
        self
    }
    
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.config.vsync = vsync;
        self
    }
    
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.config.fullscreen = fullscreen;
        self
    }
    
    pub fn build(self) -> GameEngine {
        GameEngine::new(self.config)
    }
}

impl Default for GameEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_engine_creation() {
        let engine = GameEngineBuilder::new()
            .with_name("Test Engine")
            .with_resolution(1280, 720)
            .build();
        
        assert_eq!(engine.config.name, "Test Engine");
        assert_eq!(engine.config.resolution, (1280, 720));
    }
    
    #[test]
    fn test_frame_state() {
        let mut engine = GameEngine::new(GameEngineConfig::default());
        engine.init().unwrap();
        
        engine.begin_frame();
        assert_eq!(engine.frame_state.frame_number, 1);
    }
}
