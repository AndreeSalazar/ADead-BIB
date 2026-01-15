// ============================================================================
// Renderer - ADead-BIB Engine
// ============================================================================
// Renderizado con softbuffer (compatible con Vulkan futuro)

use std::rc::Rc;
use std::num::NonZeroU32;
use winit::window::Window as WinitWindow;

/// Color RGBA
#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    /// Convertir a u32 para buffer
    #[inline]
    pub fn to_u32(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    
    // Colores predefinidos
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    pub const SKY_BLUE: Color = Color::rgb(135, 206, 235);
    pub const GRASS_GREEN: Color = Color::rgb(34, 139, 34);
    pub const PIPE_GREEN: Color = Color::rgb(0, 128, 0);
    pub const BIRD_YELLOW: Color = Color::rgb(255, 215, 0);
}

/// Renderer principal
pub struct Renderer {
    context: softbuffer::Context<Rc<WinitWindow>>,
    surface: softbuffer::Surface<Rc<WinitWindow>, Rc<WinitWindow>>,
    width: u32,
    height: u32,
    buffer: Vec<u32>,
}

impl Renderer {
    /// Crear nuevo renderer
    pub fn new(window: &Rc<WinitWindow>, width: u32, height: u32) -> Self {
        let context = softbuffer::Context::new(window.clone()).expect("Failed to create context");
        let surface = softbuffer::Surface::new(&context, window.clone()).expect("Failed to create surface");
        
        Self {
            context,
            surface,
            width,
            height,
            buffer: vec![0; (width * height) as usize],
        }
    }
    
    /// Limpiar pantalla con color
    pub fn clear(&mut self, color: Color) {
        let c = color.to_u32();
        self.buffer.fill(c);
    }
    
    /// Dibujar pixel
    #[inline]
    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = (y as u32 * self.width + x as u32) as usize;
            self.buffer[idx] = color.to_u32();
        }
    }
    
    /// Dibujar rectángulo relleno
    pub fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        let c = color.to_u32();
        let x_start = x.max(0) as u32;
        let y_start = y.max(0) as u32;
        let x_end = ((x + w as i32) as u32).min(self.width);
        let y_end = ((y + h as i32) as u32).min(self.height);
        
        for py in y_start..y_end {
            for px in x_start..x_end {
                let idx = (py * self.width + px) as usize;
                self.buffer[idx] = c;
            }
        }
    }
    
    /// Dibujar rectángulo con borde
    pub fn draw_rect_outline(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color, thickness: u32) {
        // Top
        self.draw_rect(x, y, w, thickness, color);
        // Bottom
        self.draw_rect(x, y + h as i32 - thickness as i32, w, thickness, color);
        // Left
        self.draw_rect(x, y, thickness, h, color);
        // Right
        self.draw_rect(x + w as i32 - thickness as i32, y, thickness, h, color);
    }
    
    /// Dibujar círculo relleno
    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        let c = color.to_u32();
        let r2 = radius * radius;
        
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= r2 {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                        let idx = (py as u32 * self.width + px as u32) as usize;
                        self.buffer[idx] = c;
                    }
                }
            }
        }
    }
    
    /// Dibujar línea (Bresenham)
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;
        
        loop {
            self.draw_pixel(x, y, color);
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
    
    /// Presentar frame
    pub fn present(&mut self) {
        self.surface.resize(
            NonZeroU32::new(self.width).unwrap(),
            NonZeroU32::new(self.height).unwrap(),
        ).expect("Failed to resize surface");
        
        let mut buffer = self.surface.buffer_mut().expect("Failed to get buffer");
        buffer.copy_from_slice(&self.buffer);
        buffer.present().expect("Failed to present");
    }
    
    /// Obtener dimensiones
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
