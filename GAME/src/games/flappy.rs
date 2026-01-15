// ============================================================================
// Flappy Bird Game - ADead-BIB Engine
// ============================================================================
// Juego completo usando el sistema ECS
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

use crate::ecs::{World, EntityId, Transform, Velocity, Sprite, Collider, RigidBody, Score};
use crate::engine::renderer::{Renderer, Color};
use crate::engine::input::{Input, KeyCode};
use crate::systems::{PhysicsSystem, CollisionSystem, RenderSystem};

/// Estado del juego Flappy
pub struct FlappyGame {
    pub world: World,
    pub physics: PhysicsSystem,
    pub collision: CollisionSystem,
    pub render: RenderSystem,
    
    // Entidades principales
    pub bird_id: EntityId,
    pub pipe_ids: Vec<EntityId>,
    pub ground_id: EntityId,
    
    // Estado
    pub score: u32,
    pub game_over: bool,
    pub started: bool,
    pub frame: u64,
    
    // Configuración
    pub screen_width: f32,
    pub screen_height: f32,
    pub pipe_speed: f32,
    pub pipe_spawn_interval: u64,
    pub flap_force: f32,
    pub gap_size: f32,
    pub last_pipe_x: f32,  // Para controlar separación mínima
}

impl FlappyGame {
    pub fn new(width: f32, height: f32) -> Self {
        let mut world = World::new();
        let physics = PhysicsSystem::new().with_gravity(600.0);  // Gravedad más suave
        let collision = CollisionSystem::new();
        let render = RenderSystem::new();
        
        // Crear pájaro
        let bird_id = world.spawn_at("Bird", 100.0, height / 2.0);
        world.add_velocity(bird_id, Velocity::zero());
        world.add_sprite(bird_id, Sprite::new(34.0, 24.0, (255, 215, 0))); // Amarillo
        world.add_collider(bird_id, Collider::new(34.0, 24.0));
        world.add_rigid_body(bird_id, RigidBody::new(1.0));
        
        // Crear suelo
        let ground_id = world.spawn_at("Ground", 0.0, height - 50.0);
        world.add_sprite(ground_id, Sprite::new(width, 50.0, (139, 69, 19))); // Marrón
        world.add_collider(ground_id, Collider::new(width, 50.0));
        
        // Marcar entidades con tags
        if let Some(e) = world.entities.get_mut(&bird_id) {
            e.tag = "player".to_string();
        }
        if let Some(e) = world.entities.get_mut(&ground_id) {
            e.tag = "ground".to_string();
        }
        
        Self {
            world,
            physics,
            collision,
            render,
            bird_id,
            pipe_ids: Vec::new(),
            ground_id,
            score: 0,
            game_over: false,
            started: false,
            frame: 0,
            screen_width: width,
            screen_height: height,
            pipe_speed: 120.0,  // Velocidad más lenta
            pipe_spawn_interval: 150,  // Spawn más frecuente
            flap_force: -320.0,  // Salto más suave
            gap_size: 200.0,  // Gap aún más grande
            last_pipe_x: width + 50.0,  // Tracking del último pipe
        }
    }
    
    /// Actualizar juego
    pub fn update(&mut self, delta: f32, input: &Input) {
        if self.game_over {
            if input.is_key_just_pressed(KeyCode::Space) || input.is_key_just_pressed(KeyCode::R) {
                self.reset();
            }
            return;
        }
        
        // Iniciar juego con Space
        if !self.started {
            if input.is_key_just_pressed(KeyCode::Space) {
                self.started = true;
                self.flap();
            }
            return;
        }
        
        self.frame += 1;
        
        // Input: Flap
        if input.is_key_just_pressed(KeyCode::Space) {
            self.flap();
        }
        
        // Física
        self.physics.update(&mut self.world, delta);
        
        // Mover pipes
        self.update_pipes(delta);
        
        // Spawn pipes - cada N frames si hay espacio
        let min_pipe_distance = 350.0;  // Distancia mínima entre pipes
        let should_spawn = self.frame % self.pipe_spawn_interval == 0;
        
        // Spawnear si el último pipe ya se movió lo suficiente
        if should_spawn && (self.pipe_ids.is_empty() || self.last_pipe_x < self.screen_width - min_pipe_distance) {
            self.spawn_pipe();
            println!("🎯 Pipe spawned at frame {}", self.frame);
        }
        
        // Colisiones
        self.collision.update(&self.world);
        self.check_collisions();
        
        // Verificar límites
        self.check_bounds();
    }
    
    /// Hacer que el pájaro salte
    fn flap(&mut self) {
        if let Some(vel) = self.world.velocities.get_mut(&self.bird_id) {
            vel.vy = self.flap_force;
        }
    }
    
    /// Actualizar pipes
    fn update_pipes(&mut self, delta: f32) {
        let mut to_remove = Vec::new();
        let bird_x = self.world.transforms.get(&self.bird_id).map(|t| t.x).unwrap_or(0.0);
        let mut max_pipe_x: f32 = 0.0;
        
        for &pipe_id in &self.pipe_ids {
            // Mover pipe
            if let Some(transform) = self.world.transforms.get_mut(&pipe_id) {
                transform.x -= self.pipe_speed * delta;
                max_pipe_x = max_pipe_x.max(transform.x);
                
                // Verificar si pasó al pájaro (para score)
                // El pipe pasa cuando su borde derecho está a la izquierda del pájaro
                let pipe_right = transform.x + 80.0;
                if let Some(entity) = self.world.entities.get_mut(&pipe_id) {
                    // Solo contar pipe_top para evitar doble score
                    if entity.tag == "pipe_top" && entity.active && pipe_right < bird_x {
                        self.score += 1;
                        entity.active = false; // Marcar como contado
                        println!("🎉 Score! Total: {} (pipe_x={:.0}, bird_x={:.0})", self.score, transform.x, bird_x);
                    }
                }
                
                // Marcar para eliminar si salió de pantalla
                if transform.x < -100.0 {
                    to_remove.push(pipe_id);
                }
            }
        }
        
        // Eliminar pipes fuera de pantalla
        for id in to_remove {
            self.world.destroy(id);
            self.pipe_ids.retain(|&x| x != id);
        }
        
        // Actualizar tracking del último pipe
        self.last_pipe_x = max_pipe_x;
    }
    
    /// Spawn nuevo par de pipes
    fn spawn_pipe(&mut self) {
        // Calcular gap_y con mejor distribución
        let max_gap_y = (self.screen_height - self.gap_size - 100.0).max(150.0);
        let gap_y = 80.0 + ((self.frame * 17 + self.score as u64 * 31) % max_gap_y as u64) as f32;
        let pipe_width = 80.0;
        let x = self.screen_width + 50.0;
        
        // Actualizar tracking del último pipe
        self.last_pipe_x = x;
        
        // Pipe superior
        let top_id = self.world.spawn_at("PipeTop", x, 0.0);
        self.world.add_sprite(top_id, Sprite::new(pipe_width, gap_y, (0, 128, 0)));
        self.world.add_collider(top_id, Collider::new(pipe_width, gap_y));
        if let Some(e) = self.world.entities.get_mut(&top_id) {
            e.tag = "pipe_top".to_string();
        }
        
        // Pipe inferior
        let bottom_y = gap_y + self.gap_size;
        let bottom_height = self.screen_height - bottom_y - 50.0; // -50 por el suelo
        let bottom_id = self.world.spawn_at("PipeBottom", x, bottom_y);
        self.world.add_sprite(bottom_id, Sprite::new(pipe_width, bottom_height, (0, 128, 0)));
        self.world.add_collider(bottom_id, Collider::new(pipe_width, bottom_height));
        if let Some(e) = self.world.entities.get_mut(&bottom_id) {
            e.tag = "pipe_bottom".to_string();
        }
        
        self.pipe_ids.push(top_id);
        self.pipe_ids.push(bottom_id);
    }
    
    /// Verificar colisiones
    fn check_collisions(&mut self) {
        // Verificar colisión del pájaro con pipes o suelo
        let bird_collisions = self.collision.get_collisions(self.bird_id);
        
        for other_id in bird_collisions {
            if let Some(entity) = self.world.entities.get(&other_id) {
                if entity.tag.starts_with("pipe") || entity.tag == "ground" {
                    self.game_over = true;
                    return;
                }
            }
        }
    }
    
    /// Verificar límites de pantalla
    fn check_bounds(&mut self) {
        if let Some(transform) = self.world.transforms.get(&self.bird_id) {
            // Solo game over si toca el suelo (no el techo)
            let ground_y = self.screen_height - 50.0 - 24.0;  // suelo - altura pájaro
            if transform.y > ground_y {
                self.game_over = true;
                println!("💥 Hit ground! y={}", transform.y);
            }
            // Limitar arriba pero no game over
            if transform.y < 0.0 {
                if let Some(t) = self.world.transforms.get_mut(&self.bird_id) {
                    t.y = 0.0;
                }
                if let Some(v) = self.world.velocities.get_mut(&self.bird_id) {
                    v.vy = 0.0;
                }
            }
        }
    }
    
    /// Renderizar juego
    pub fn render(&self, renderer: &mut Renderer) {
        // Fondo cielo
        renderer.clear(Color::SKY_BLUE);
        
        // Renderizar entidades
        self.render.render(&self.world, renderer);
        
        // UI: Score
        self.draw_score(renderer);
        
        // Mensajes
        if self.game_over {
            self.draw_game_over(renderer);
        } else if !self.started {
            self.draw_start_message(renderer);
        }
    }
    
    /// Dibujar score
    fn draw_score(&self, renderer: &mut Renderer) {
        // Score como rectángulos (número simple)
        let x = 20;
        let y = 20;
        
        // Fondo del score
        renderer.draw_rect(x - 5, y - 5, 60, 30, Color::rgb(0, 0, 0));
        
        // Dígitos del score (representación simple)
        let digits = format!("{:03}", self.score);
        for (i, c) in digits.chars().enumerate() {
            let digit_x = x + (i as i32 * 18);
            self.draw_digit(renderer, digit_x, y, c);
        }
    }
    
    /// Dibujar un dígito simple
    fn draw_digit(&self, renderer: &mut Renderer, x: i32, y: i32, digit: char) {
        let color = Color::WHITE;
        let w = 12u32;
        let h = 20u32;
        
        match digit {
            '0' => {
                renderer.draw_rect_outline(x, y, w, h, color, 2);
            }
            '1' => {
                renderer.draw_rect(x + 5, y, 2, h, color);
            }
            '2' => {
                renderer.draw_rect(x, y, w, 2, color);
                renderer.draw_rect(x + w as i32 - 2, y, 2, h/2, color);
                renderer.draw_rect(x, y + h as i32/2 - 1, w, 2, color);
                renderer.draw_rect(x, y + h as i32/2, 2, h/2, color);
                renderer.draw_rect(x, y + h as i32 - 2, w, 2, color);
            }
            '3' => {
                renderer.draw_rect(x, y, w, 2, color);
                renderer.draw_rect(x + w as i32 - 2, y, 2, h, color);
                renderer.draw_rect(x, y + h as i32/2 - 1, w, 2, color);
                renderer.draw_rect(x, y + h as i32 - 2, w, 2, color);
            }
            '4' => {
                renderer.draw_rect(x, y, 2, h/2, color);
                renderer.draw_rect(x, y + h as i32/2 - 1, w, 2, color);
                renderer.draw_rect(x + w as i32 - 2, y, 2, h, color);
            }
            '5' => {
                renderer.draw_rect(x, y, w, 2, color);
                renderer.draw_rect(x, y, 2, h/2, color);
                renderer.draw_rect(x, y + h as i32/2 - 1, w, 2, color);
                renderer.draw_rect(x + w as i32 - 2, y + h as i32/2, 2, h/2, color);
                renderer.draw_rect(x, y + h as i32 - 2, w, 2, color);
            }
            '6' => {
                renderer.draw_rect(x, y, w, 2, color);
                renderer.draw_rect(x, y, 2, h, color);
                renderer.draw_rect(x, y + h as i32/2 - 1, w, 2, color);
                renderer.draw_rect(x + w as i32 - 2, y + h as i32/2, 2, h/2, color);
                renderer.draw_rect(x, y + h as i32 - 2, w, 2, color);
            }
            '7' => {
                renderer.draw_rect(x, y, w, 2, color);
                renderer.draw_rect(x + w as i32 - 2, y, 2, h, color);
            }
            '8' => {
                renderer.draw_rect_outline(x, y, w, h/2, color, 2);
                renderer.draw_rect_outline(x, y + h as i32/2, w, h/2, color, 2);
            }
            '9' => {
                renderer.draw_rect(x, y, w, 2, color);
                renderer.draw_rect(x, y, 2, h/2, color);
                renderer.draw_rect(x, y + h as i32/2 - 1, w, 2, color);
                renderer.draw_rect(x + w as i32 - 2, y, 2, h, color);
                renderer.draw_rect(x, y + h as i32 - 2, w, 2, color);
            }
            _ => {}
        }
    }
    
    /// Dibujar mensaje de game over
    fn draw_game_over(&self, renderer: &mut Renderer) {
        let cx = (self.screen_width / 2.0) as i32;
        let cy = (self.screen_height / 2.0) as i32;
        
        // Fondo semi-transparente
        renderer.draw_rect(cx - 120, cy - 50, 240, 100, Color::rgb(0, 0, 0));
        
        // Texto "GAME OVER" (como rectángulos)
        renderer.draw_rect(cx - 100, cy - 30, 200, 4, Color::RED);
        renderer.draw_rect(cx - 100, cy + 26, 200, 4, Color::RED);
        
        // Score final
        renderer.draw_rect(cx - 30, cy - 10, 60, 20, Color::WHITE);
    }
    
    /// Dibujar mensaje de inicio
    fn draw_start_message(&self, renderer: &mut Renderer) {
        let cx = (self.screen_width / 2.0) as i32;
        let cy = (self.screen_height / 2.0) as i32;
        
        // Fondo
        renderer.draw_rect(cx - 100, cy - 30, 200, 60, Color::rgb(0, 0, 0));
        
        // Indicador de presionar espacio
        renderer.draw_rect(cx - 80, cy - 10, 160, 4, Color::WHITE);
        renderer.draw_rect(cx - 80, cy + 6, 160, 4, Color::WHITE);
    }
    
    /// Reiniciar juego
    pub fn reset(&mut self) {
        // Limpiar pipes
        for &id in &self.pipe_ids {
            self.world.destroy(id);
        }
        self.pipe_ids.clear();
        
        // Resetear pájaro
        if let Some(transform) = self.world.transforms.get_mut(&self.bird_id) {
            transform.x = 100.0;
            transform.y = self.screen_height / 2.0;
        }
        if let Some(vel) = self.world.velocities.get_mut(&self.bird_id) {
            vel.vx = 0.0;
            vel.vy = 0.0;
        }
        
        // Resetear estado
        self.score = 0;
        self.game_over = false;
        self.started = false;
        self.frame = 0;
        self.last_pipe_x = self.screen_width + 50.0;
    }
    
    /// Redimensionar juego cuando cambia la ventana
    pub fn resize(&mut self, new_width: f32, new_height: f32) {
        self.screen_width = new_width;
        self.screen_height = new_height;
        
        // Actualizar suelo
        if let Some(transform) = self.world.transforms.get_mut(&self.ground_id) {
            transform.y = new_height - 50.0;
        }
        if let Some(sprite) = self.world.sprites.get_mut(&self.ground_id) {
            sprite.width = new_width;
        }
        if let Some(collider) = self.world.colliders.get_mut(&self.ground_id) {
            collider.width = new_width;
        }
    }
}
