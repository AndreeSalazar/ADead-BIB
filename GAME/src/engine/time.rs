// ============================================================================
// Time Manager - ADead-BIB Engine
// ============================================================================

use std::time::Instant;

/// Gestor de tiempo del juego
pub struct Time {
    start_time: Instant,
    last_frame: Instant,
    delta_time: f32,
    total_time: f32,
    frame_count: u64,
    fps: f32,
    fps_update_timer: f32,
    fps_frame_count: u32,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_frame: now,
            delta_time: 0.0,
            total_time: 0.0,
            frame_count: 0,
            fps: 0.0,
            fps_update_timer: 0.0,
            fps_frame_count: 0,
        }
    }
    
    /// Actualizar al inicio de cada frame
    pub fn update(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame);
        self.delta_time = elapsed.as_secs_f32();
        self.last_frame = now;
        self.total_time = now.duration_since(self.start_time).as_secs_f32();
        self.frame_count += 1;
        
        // Calcular FPS cada segundo
        self.fps_update_timer += self.delta_time;
        self.fps_frame_count += 1;
        if self.fps_update_timer >= 1.0 {
            self.fps = self.fps_frame_count as f32 / self.fps_update_timer;
            self.fps_update_timer = 0.0;
            self.fps_frame_count = 0;
        }
    }
    
    /// Delta time (segundos desde el último frame)
    #[inline]
    pub fn delta(&self) -> f32 {
        self.delta_time
    }
    
    /// Delta time en milisegundos
    #[inline]
    pub fn delta_ms(&self) -> f32 {
        self.delta_time * 1000.0
    }
    
    /// Tiempo total desde el inicio
    #[inline]
    pub fn total(&self) -> f32 {
        self.total_time
    }
    
    /// Número de frame actual
    #[inline]
    pub fn frame(&self) -> u64 {
        self.frame_count
    }
    
    /// FPS actual
    #[inline]
    pub fn fps(&self) -> f32 {
        self.fps
    }
    
    /// FPS como entero
    #[inline]
    pub fn fps_int(&self) -> u32 {
        self.fps as u32
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}
