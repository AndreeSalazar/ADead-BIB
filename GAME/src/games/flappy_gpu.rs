// ============================================================================
// Flappy Bird Game - GPU Version
// ============================================================================
// Versión del juego que usa GPU directo para renderizado
// ADead-BIB + Rust + wgpu (Vulkan/DX12/Metal)
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

use crate::engine::gpu_renderer::{GpuRenderer, GpuColor};
use crate::engine::input::{Input, KeyCode};

/// Estado del pájaro
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub velocity: f32,
}

/// Estado de un pipe
pub struct Pipe {
    pub x: f32,
    pub gap_y: f32,
    pub gap_size: f32,
    pub width: f32,
    pub passed: bool,
}

/// Juego Flappy Bird con GPU
pub struct FlappyGpu {
    pub bird: Bird,
    pub pipes: Vec<Pipe>,
    pub score: u32,
    pub game_over: bool,
    pub started: bool,
    pub frame: u64,
    
    // Configuración
    pub screen_width: f32,
    pub screen_height: f32,
    pub gravity: f32,
    pub flap_force: f32,
    pub pipe_speed: f32,
    pub pipe_spawn_interval: u64,
    pub gap_size: f32,
}

impl FlappyGpu {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            bird: Bird {
                x: 100.0,
                y: height / 2.0,
                width: 34.0,
                height: 24.0,
                velocity: 0.0,
            },
            pipes: Vec::new(),
            score: 0,
            game_over: false,
            started: false,
            frame: 0,
            screen_width: width,
            screen_height: height,
            gravity: 600.0,
            flap_force: -320.0,
            pipe_speed: 150.0,
            pipe_spawn_interval: 120,
            gap_size: 180.0,
        }
    }
    
    pub fn update(&mut self, delta: f32, input: &Input) {
        if self.game_over {
            if input.is_key_just_pressed(KeyCode::Space) || input.is_key_just_pressed(KeyCode::R) {
                self.reset();
            }
            return;
        }
        
        if !self.started {
            if input.is_key_just_pressed(KeyCode::Space) {
                self.started = true;
                self.bird.velocity = self.flap_force;
            }
            return;
        }
        
        self.frame += 1;
        
        // Input
        if input.is_key_just_pressed(KeyCode::Space) {
            self.bird.velocity = self.flap_force;
        }
        
        // Física del pájaro
        self.bird.velocity += self.gravity * delta;
        self.bird.y += self.bird.velocity * delta;
        
        // Límites
        if self.bird.y < 0.0 {
            self.bird.y = 0.0;
            self.bird.velocity = 0.0;
        }
        
        let ground_y = self.screen_height - 50.0;
        if self.bird.y + self.bird.height > ground_y {
            self.game_over = true;
        }
        
        // Mover pipes
        for pipe in &mut self.pipes {
            pipe.x -= self.pipe_speed * delta;
            
            // Score
            if !pipe.passed && pipe.x + pipe.width < self.bird.x {
                pipe.passed = true;
                self.score += 1;
                println!("🎉 Score: {}", self.score);
            }
        }
        
        // Eliminar pipes fuera de pantalla
        self.pipes.retain(|p| p.x > -100.0);
        
        // Spawn pipes
        if self.frame % self.pipe_spawn_interval == 0 {
            self.spawn_pipe();
        }
        
        // Colisiones
        self.check_collisions();
    }
    
    fn spawn_pipe(&mut self) {
        let max_gap_y = (self.screen_height - self.gap_size - 150.0).max(100.0);
        let gap_y = 80.0 + ((self.frame * 17 + self.score as u64 * 31) % max_gap_y as u64) as f32;
        
        self.pipes.push(Pipe {
            x: self.screen_width + 50.0,
            gap_y,
            gap_size: self.gap_size,
            width: 80.0,
            passed: false,
        });
    }
    
    fn check_collisions(&mut self) {
        let bird_left = self.bird.x;
        let bird_right = self.bird.x + self.bird.width;
        let bird_top = self.bird.y;
        let bird_bottom = self.bird.y + self.bird.height;
        
        for pipe in &self.pipes {
            let pipe_left = pipe.x;
            let pipe_right = pipe.x + pipe.width;
            
            // Solo verificar si el pájaro está en el rango X del pipe
            if bird_right > pipe_left && bird_left < pipe_right {
                // Verificar colisión con pipe superior
                if bird_top < pipe.gap_y {
                    self.game_over = true;
                    return;
                }
                // Verificar colisión con pipe inferior
                if bird_bottom > pipe.gap_y + pipe.gap_size {
                    self.game_over = true;
                    return;
                }
            }
        }
    }
    
    pub fn render(&self, renderer: &mut GpuRenderer) {
        renderer.clear();
        
        // Suelo
        let ground_y = self.screen_height as i32 - 50;
        renderer.draw_rect(0, ground_y, renderer.width, 10, GpuColor::GRASS.to_array());
        renderer.draw_rect(0, ground_y + 10, renderer.width, 40, GpuColor::BROWN.to_array());
        
        // Pipes
        for pipe in &self.pipes {
            let px = pipe.x as i32;
            let pw = pipe.width as u32;
            
            // Pipe superior
            let top_height = pipe.gap_y as u32;
            if top_height > 0 {
                renderer.draw_rect(px, 0, pw, top_height, GpuColor::GREEN.to_array());
                // Borde
                renderer.draw_rect(px, 0, 4, top_height, GpuColor::DARK_GREEN.to_array());
                renderer.draw_rect(px + pw as i32 - 4, 0, 4, top_height, GpuColor::DARK_GREEN.to_array());
            }
            
            // Pipe inferior
            let bottom_y = (pipe.gap_y + pipe.gap_size) as i32;
            let bottom_height = (self.screen_height as i32 - bottom_y - 50).max(0) as u32;
            if bottom_height > 0 {
                renderer.draw_rect(px, bottom_y, pw, bottom_height, GpuColor::GREEN.to_array());
                // Borde
                renderer.draw_rect(px, bottom_y, 4, bottom_height, GpuColor::DARK_GREEN.to_array());
                renderer.draw_rect(px + pw as i32 - 4, bottom_y, 4, bottom_height, GpuColor::DARK_GREEN.to_array());
            }
        }
        
        // Pájaro
        let bx = self.bird.x as i32;
        let by = self.bird.y as i32;
        let bw = self.bird.width as u32;
        let bh = self.bird.height as u32;
        renderer.draw_rect(bx, by, bw, bh, GpuColor::YELLOW.to_array());
        
        // Ojo
        renderer.draw_rect(bx + bw as i32 * 2 / 3, by + 4, 8, 8, GpuColor::WHITE.to_array());
        renderer.draw_rect(bx + bw as i32 * 2 / 3 + 3, by + 6, 4, 4, GpuColor::BLACK.to_array());
        
        // Pico
        renderer.draw_rect(bx + bw as i32, by + bh as i32 / 3, 12, 8, GpuColor::ORANGE.to_array());
        
        // Score
        self.draw_score(renderer);
        
        // Mensajes
        if self.game_over {
            self.draw_game_over(renderer);
        } else if !self.started {
            self.draw_start(renderer);
        }
    }
    
    fn draw_score(&self, renderer: &mut GpuRenderer) {
        // Fondo del score
        renderer.draw_rect(15, 15, 80, 40, [0.0, 0.0, 0.0, 0.7]);
        renderer.draw_rect(20, 20, 70, 30, GpuColor::WHITE.to_array());
        
        // Número simple (rectángulos por dígito)
        let score_str = format!("{}", self.score);
        for (i, _) in score_str.chars().enumerate() {
            renderer.draw_rect(25 + i as i32 * 15, 25, 10, 20, GpuColor::BLACK.to_array());
        }
    }
    
    fn draw_game_over(&self, renderer: &mut GpuRenderer) {
        let cx = (self.screen_width / 2.0) as i32;
        let cy = (self.screen_height / 2.0) as i32;
        
        renderer.draw_rect(cx - 120, cy - 40, 240, 80, GpuColor::RED.to_array());
        renderer.draw_rect(cx - 110, cy - 30, 220, 60, [0.5, 0.0, 0.0, 1.0]);
        renderer.draw_rect(cx - 80, cy - 5, 160, 10, GpuColor::WHITE.to_array());
    }
    
    fn draw_start(&self, renderer: &mut GpuRenderer) {
        let cx = (self.screen_width / 2.0) as i32;
        let cy = (self.screen_height / 2.0) as i32;
        
        renderer.draw_rect(cx - 100, cy - 30, 200, 60, [0.0, 0.0, 0.0, 0.8]);
        renderer.draw_rect(cx - 80, cy - 5, 160, 4, GpuColor::WHITE.to_array());
        renderer.draw_rect(cx - 80, cy + 5, 160, 4, GpuColor::WHITE.to_array());
    }
    
    pub fn resize(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
    }
    
    pub fn reset(&mut self) {
        self.bird.x = 100.0;
        self.bird.y = self.screen_height / 2.0;
        self.bird.velocity = 0.0;
        self.pipes.clear();
        self.score = 0;
        self.game_over = false;
        self.started = false;
        self.frame = 0;
    }
}
