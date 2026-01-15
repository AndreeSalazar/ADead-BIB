// ============================================================================
// Components - ADead-BIB ECS
// ============================================================================
// Componentes reutilizables para entidades

/// Componente de transformación (posición, rotación, escala)
#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
    
    pub fn with_scale(mut self, sx: f32, sy: f32) -> Self {
        self.scale_x = sx;
        self.scale_y = sy;
        self
    }
    
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

/// Componente de velocidad
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

impl Velocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        Self { vx, vy }
    }
    
    pub fn zero() -> Self {
        Self { vx: 0.0, vy: 0.0 }
    }
}

/// Componente de sprite/renderizado
#[derive(Debug, Clone)]
pub struct Sprite {
    pub width: f32,
    pub height: f32,
    pub color: (u8, u8, u8),
    pub visible: bool,
}

impl Sprite {
    pub fn new(width: f32, height: f32, color: (u8, u8, u8)) -> Self {
        Self {
            width,
            height,
            color,
            visible: true,
        }
    }
    
    pub fn rect(width: f32, height: f32) -> Self {
        Self::new(width, height, (255, 255, 255))
    }
}

/// Componente de colisión (AABB)
#[derive(Debug, Clone, Copy)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub is_trigger: bool,
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            offset_x: 0.0,
            offset_y: 0.0,
            is_trigger: false,
        }
    }
    
    pub fn trigger(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            offset_x: 0.0,
            offset_y: 0.0,
            is_trigger: true,
        }
    }
    
    /// Verificar colisión AABB
    pub fn check_collision(&self, pos_a: &Transform, other: &Collider, pos_b: &Transform) -> bool {
        let a_left = pos_a.x + self.offset_x;
        let a_right = a_left + self.width;
        let a_top = pos_a.y + self.offset_y;
        let a_bottom = a_top + self.height;
        
        let b_left = pos_b.x + other.offset_x;
        let b_right = b_left + other.width;
        let b_top = pos_b.y + other.offset_y;
        let b_bottom = b_top + other.height;
        
        a_left < b_right && a_right > b_left && a_top < b_bottom && a_bottom > b_top
    }
}

/// Componente de física
#[derive(Debug, Clone, Copy)]
pub struct RigidBody {
    pub mass: f32,
    pub gravity_scale: f32,
    pub drag: f32,
    pub is_kinematic: bool,
}

impl RigidBody {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            gravity_scale: 1.0,
            drag: 0.0,
            is_kinematic: false,
        }
    }
    
    pub fn kinematic() -> Self {
        Self {
            mass: 1.0,
            gravity_scale: 0.0,
            drag: 0.0,
            is_kinematic: true,
        }
    }
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::new(1.0)
    }
}

/// Componente de salud
#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }
    
    pub fn damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }
    
    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }
    
    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }
    
    pub fn percentage(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

/// Componente de puntuación
#[derive(Debug, Clone, Copy, Default)]
pub struct Score {
    pub value: u32,
    pub multiplier: f32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            value: 0,
            multiplier: 1.0,
        }
    }
    
    pub fn add(&mut self, points: u32) {
        self.value += (points as f32 * self.multiplier) as u32;
    }
    
    pub fn reset(&mut self) {
        self.value = 0;
    }
}
