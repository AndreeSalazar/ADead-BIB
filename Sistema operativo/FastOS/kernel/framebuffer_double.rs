// ============================================================================
// FastOS Double Buffering
// ============================================================================
// Buffer doble para eliminar parpadeo en el renderizado
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

/// Tamaño máximo del back buffer (1920x1080x4 = ~8MB)
const MAX_BUFFER_SIZE: usize = 1920 * 1080 * 4;

/// Back buffer estático
static mut BACK_BUFFER: [u8; MAX_BUFFER_SIZE] = [0; MAX_BUFFER_SIZE];

/// Estado del double buffering
pub struct DoubleBuffer {
    front: *mut u8,
    back: *mut u8,
    width: usize,
    height: usize,
    pitch: usize,
    bpp: usize,
    size: usize,
    initialized: bool,
}

impl DoubleBuffer {
    pub const fn new() -> Self {
        DoubleBuffer {
            front: core::ptr::null_mut(),
            back: core::ptr::null_mut(),
            width: 0,
            height: 0,
            pitch: 0,
            bpp: 4,
            size: 0,
            initialized: false,
        }
    }

    /// Inicializar double buffering
    pub fn init(&mut self, front_buffer: *mut u8, width: usize, height: usize, pitch: usize, bpp: usize) {
        self.front = front_buffer;
        self.width = width;
        self.height = height;
        self.pitch = pitch;
        self.bpp = bpp;
        self.size = pitch * height;
        
        // Usar buffer estático como back buffer
        self.back = unsafe { BACK_BUFFER.as_mut_ptr() };
        self.initialized = true;
    }

    /// Obtener puntero al back buffer para dibujar
    pub fn back_buffer(&mut self) -> Option<&mut [u8]> {
        if !self.initialized || self.size == 0 {
            return None;
        }
        Some(unsafe { core::slice::from_raw_parts_mut(self.back, self.size) })
    }

    /// Copiar back buffer al front buffer (swap)
    pub fn swap(&mut self) {
        if !self.initialized || self.front.is_null() || self.back.is_null() {
            return;
        }
        
        unsafe {
            // Copiar back → front
            core::ptr::copy_nonoverlapping(self.back, self.front, self.size);
        }
    }

    /// Dibujar un píxel en el back buffer
    #[inline]
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height || self.back.is_null() {
            return;
        }
        let offset = y * self.pitch + x * self.bpp;
        if offset + 2 < self.size {
            unsafe {
                let ptr = self.back.add(offset);
                *ptr = (color & 0xFF) as u8;
                *ptr.add(1) = ((color >> 8) & 0xFF) as u8;
                *ptr.add(2) = ((color >> 16) & 0xFF) as u8;
            }
        }
    }

    /// Dibujar rectángulo en el back buffer
    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.put_pixel(x + dx, y + dy, color);
            }
        }
    }

    /// Dibujar línea horizontal (optimizada)
    pub fn draw_hline(&mut self, x: usize, y: usize, w: usize, color: u32) {
        if y >= self.height || self.back.is_null() {
            return;
        }
        let start_x = x.min(self.width);
        let end_x = (x + w).min(self.width);
        
        for px in start_x..end_x {
            self.put_pixel(px, y, color);
        }
    }

    /// Limpiar back buffer
    pub fn clear(&mut self, color: u32) {
        self.draw_rect(0, 0, self.width, self.height, color);
    }

    /// Obtener dimensiones
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Verificar si está inicializado
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Instancia global del double buffer
pub static mut DOUBLE_BUFFER: DoubleBuffer = DoubleBuffer::new();

/// Inicializar double buffering global
pub fn init(front_buffer: *mut u8, width: usize, height: usize, pitch: usize, bpp: usize) {
    unsafe {
        DOUBLE_BUFFER.init(front_buffer, width, height, pitch, bpp);
    }
}

/// Obtener referencia al double buffer
pub fn get() -> Option<&'static mut DoubleBuffer> {
    unsafe {
        if DOUBLE_BUFFER.is_initialized() {
            Some(&mut DOUBLE_BUFFER)
        } else {
            None
        }
    }
}

/// Swap buffers
pub fn swap() {
    unsafe {
        DOUBLE_BUFFER.swap();
    }
}
