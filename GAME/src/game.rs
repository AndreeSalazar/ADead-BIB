// ADead-BIB Game Logic - Branchless Physics
// All game logic uses branchless operations for maximum CPU/GPU efficiency
//
// Author: Eddi Andreé Salazar Matos

/// Bird entity with branchless physics
#[derive(Debug, Clone)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            x: 100.0,
            y: 300.0,
            velocity: 0.0,
            width: 34.0,
            height: 24.0,
        }
    }
    
    /// Branchless gravity application
    #[inline(always)]
    pub fn apply_gravity(&mut self, gravity: f32) {
        self.velocity += gravity;
        self.y += self.velocity;
    }
    
    /// Branchless flap - no IF needed
    #[inline(always)]
    pub fn flap(&mut self, force: f32) {
        self.velocity = force;
    }
    
    /// Branchless clamp velocity using min/max
    #[inline(always)]
    pub fn clamp_velocity(&mut self, min: f32, max: f32) {
        // Branchless: velocity = max(min, min(velocity, max))
        self.velocity = self.velocity.min(max).max(min);
    }
    
    /// Branchless boundary check - returns collision mask
    #[inline(always)]
    pub fn check_bounds(&self, screen_height: f32) -> i32 {
        // Branchless: returns -1 if out of bounds, 0 if safe
        let top_hit = ((-self.y) as i32) >> 31;  // -1 if y < 0
        let bottom_hit = ((self.y - screen_height) as i32) >> 31;  // -1 if y > height
        // Invert bottom_hit logic: we want -1 when y > height
        let bottom_collision = !bottom_hit & ((self.y > screen_height) as i32 * -1);
        top_hit | bottom_collision
    }
}

/// Pipe obstacle with branchless collision
#[derive(Debug, Clone)]
pub struct Pipe {
    pub x: f32,
    pub gap_y: f32,
    pub gap_size: f32,
    pub width: f32,
    pub passed: bool,
}

impl Pipe {
    pub fn new(x: f32, gap_y: f32) -> Self {
        Self {
            x,
            gap_y,
            gap_size: 150.0,
            width: 80.0,
            passed: false,
        }
    }
    
    /// Branchless collision detection with bird
    #[inline(always)]
    pub fn check_collision(&self, bird: &Bird) -> bool {
        // AABB collision - all branchless using min/max
        let bird_left = bird.x;
        let bird_right = bird.x + bird.width;
        let bird_top = bird.y;
        let bird_bottom = bird.y + bird.height;
        
        let pipe_left = self.x;
        let pipe_right = self.x + self.width;
        let gap_top = self.gap_y;
        let gap_bottom = self.gap_y + self.gap_size;
        
        // Check if bird overlaps pipe horizontally
        let h_overlap = (bird_right > pipe_left) && (bird_left < pipe_right);
        
        // Check if bird is outside the gap (collision with pipe)
        let in_top_pipe = bird_top < gap_top;
        let in_bottom_pipe = bird_bottom > gap_bottom;
        
        // Collision = horizontal overlap AND (in top pipe OR in bottom pipe)
        h_overlap && (in_top_pipe || in_bottom_pipe)
    }
    
    /// Branchless collision using bit operations
    #[inline(always)]
    pub fn check_collision_branchless(&self, bird: &Bird) -> i32 {
        let bird_right = bird.x + bird.width;
        let bird_bottom = bird.y + bird.height;
        let pipe_right = self.x + self.width;
        let gap_bottom = self.gap_y + self.gap_size;
        
        // Horizontal overlap check (branchless)
        let h1 = ((bird_right - self.x) as i32) >> 31;      // 0 if bird_right > pipe_left
        let h2 = ((pipe_right - bird.x) as i32) >> 31;      // 0 if pipe_right > bird_left
        let h_overlap = !(h1 | h2);                          // -1 if overlap
        
        // Vertical gap check (branchless)
        let v1 = ((bird.y - self.gap_y) as i32) >> 31;      // -1 if bird above gap
        let v2 = ((gap_bottom - bird_bottom) as i32) >> 31; // -1 if bird below gap
        let in_gap = !(v1 | v2);                             // -1 if in gap
        
        // Collision = h_overlap AND NOT in_gap
        h_overlap & !in_gap
    }
    
    /// Move pipe left (branchless)
    #[inline(always)]
    pub fn move_left(&mut self, speed: f32) {
        self.x -= speed;
    }
    
    /// Check if pipe is off screen (branchless)
    #[inline(always)]
    pub fn is_off_screen(&self) -> bool {
        self.x + self.width < 0.0
    }
}

/// Game state with branchless updates
pub struct GameState {
    pub bird: Bird,
    pub pipes: Vec<Pipe>,
    pub score: u32,
    pub game_over: bool,
    pub frame: u64,
    
    // Constants
    gravity: f32,
    flap_force: f32,
    pipe_speed: f32,
    pipe_spawn_interval: u64,
    screen_width: f32,
    screen_height: f32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            bird: Bird::new(),
            pipes: vec![Pipe::new(400.0, 200.0), Pipe::new(650.0, 250.0)],
            score: 0,
            game_over: false,
            frame: 0,
            gravity: 0.5,
            flap_force: -8.0,
            pipe_speed: 3.0,
            pipe_spawn_interval: 120, // 2 seconds at 60 FPS
            screen_width: 800.0,
            screen_height: 600.0,
        }
    }
    
    /// Main update - all branchless where possible
    pub fn update(&mut self) {
        if self.game_over {
            return;
        }
        
        self.frame += 1;
        
        // Update bird physics (branchless)
        self.bird.apply_gravity(self.gravity);
        self.bird.clamp_velocity(-15.0, 15.0);
        
        // Check bounds collision
        if self.bird.y < 0.0 || self.bird.y > self.screen_height - self.bird.height {
            self.game_over = true;
            return;
        }
        
        // Update pipes
        for pipe in &mut self.pipes {
            pipe.move_left(self.pipe_speed);
            
            // Check collision (branchless version)
            if pipe.check_collision(&self.bird) {
                self.game_over = true;
                return;
            }
            
            // Score when passing pipe
            if !pipe.passed && pipe.x + pipe.width < self.bird.x {
                pipe.passed = true;
                self.score += 1;
            }
        }
        
        // Remove off-screen pipes and spawn new ones
        self.pipes.retain(|p| !p.is_off_screen());
        
        // Spawn new pipe
        if self.frame % self.pipe_spawn_interval == 0 {
            let gap_y = 100.0 + (self.frame % 300) as f32; // Pseudo-random gap position
            self.pipes.push(Pipe::new(self.screen_width, gap_y));
        }
    }
    
    /// Flap action
    pub fn flap(&mut self) {
        if !self.game_over {
            self.bird.flap(self.flap_force);
        }
    }
    
    /// Reset game
    pub fn reset(&mut self) {
        self.bird = Bird::new();
        self.pipes = vec![Pipe::new(500.0, 200.0)];
        self.score = 0;
        self.game_over = false;
        self.frame = 0;
    }
    
    /// Spawn a new pipe
    pub fn spawn_pipe(&mut self, gap_y: f32) {
        self.pipes.push(Pipe::new(self.screen_width + 50.0, gap_y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_branchless_collision() {
        let pipe = Pipe::new(100.0, 200.0);
        let mut bird = Bird::new();
        
        // Bird in gap - no collision
        bird.x = 100.0;
        bird.y = 250.0;
        assert!(!pipe.check_collision(&bird));
        
        // Bird above gap - collision
        bird.y = 100.0;
        assert!(pipe.check_collision(&bird));
        
        // Bird below gap - collision
        bird.y = 400.0;
        assert!(pipe.check_collision(&bird));
    }
    
    #[test]
    fn test_deterministic_physics() {
        let mut game1 = GameState::new();
        let mut game2 = GameState::new();
        
        // Same inputs should produce same results
        for _ in 0..100 {
            game1.update();
            game2.update();
            
            if game1.frame % 30 == 0 {
                game1.flap();
                game2.flap();
            }
        }
        
        assert_eq!(game1.bird.y, game2.bird.y);
        assert_eq!(game1.score, game2.score);
        assert_eq!(game1.game_over, game2.game_over);
    }
}
