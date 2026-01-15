// ============================================================================
// Input Manager - ADead-BIB Engine
// ============================================================================

use std::collections::HashSet;

/// Códigos de teclas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Space,
    Enter,
    Escape,
    Up,
    Down,
    Left,
    Right,
    W, A, S, D,
    R,
    P,
    Q,
    Key1, Key2, Key3, Key4, Key5,
    Key6, Key7, Key8, Key9, Key0,
}

/// Estado del input
pub struct Input {
    keys_pressed: HashSet<KeyCode>,
    keys_just_pressed: HashSet<KeyCode>,
    keys_just_released: HashSet<KeyCode>,
    mouse_x: f32,
    mouse_y: f32,
    mouse_left: bool,
    mouse_right: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            keys_just_pressed: HashSet::new(),
            keys_just_released: HashSet::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_left: false,
            mouse_right: false,
        }
    }
    
    /// Llamar al inicio de cada frame
    pub fn begin_frame(&mut self) {
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
    }
    
    /// Registrar tecla presionada
    pub fn key_down(&mut self, key: KeyCode) {
        if !self.keys_pressed.contains(&key) {
            self.keys_just_pressed.insert(key);
        }
        self.keys_pressed.insert(key);
    }
    
    /// Registrar tecla liberada
    pub fn key_up(&mut self, key: KeyCode) {
        if self.keys_pressed.contains(&key) {
            self.keys_just_released.insert(key);
        }
        self.keys_pressed.remove(&key);
    }
    
    /// ¿Tecla está presionada?
    #[inline]
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }
    
    /// ¿Tecla acaba de ser presionada este frame?
    #[inline]
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keys_just_pressed.contains(&key)
    }
    
    /// ¿Tecla acaba de ser liberada este frame?
    #[inline]
    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keys_just_released.contains(&key)
    }
    
    /// Actualizar posición del mouse
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }
    
    /// Obtener posición del mouse
    #[inline]
    pub fn mouse_position(&self) -> (f32, f32) {
        (self.mouse_x, self.mouse_y)
    }
    
    /// Estado del botón izquierdo
    pub fn set_mouse_left(&mut self, pressed: bool) {
        self.mouse_left = pressed;
    }
    
    /// Estado del botón derecho
    pub fn set_mouse_right(&mut self, pressed: bool) {
        self.mouse_right = pressed;
    }
    
    #[inline]
    pub fn is_mouse_left_pressed(&self) -> bool {
        self.mouse_left
    }
    
    #[inline]
    pub fn is_mouse_right_pressed(&self) -> bool {
        self.mouse_right
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
