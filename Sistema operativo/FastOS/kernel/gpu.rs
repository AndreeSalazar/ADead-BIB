// ============================================================================
// FastOS GPU Driver
// ============================================================================
// Acceso directo al framebuffer desde el kernel
// GPU-First philosophy: Todo renderizado pasa por aquí
// ============================================================================

use core::ptr;

/// Estado global del GPU/Framebuffer
pub static mut GPU: Option<GpuDriver> = None;

/// Driver GPU que maneja el framebuffer
pub struct GpuDriver {
    pub buffer: *mut u8,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub bpp: usize,
}

impl GpuDriver {
    /// Inicializar el driver GPU con el framebuffer
    pub fn init(buffer: *mut u8, width: usize, height: usize, pitch: usize, bpp: usize) {
        unsafe {
            GPU = Some(GpuDriver {
                buffer,
                width,
                height,
                pitch,
                bpp,
            });
        }
    }

    /// Obtener referencia al driver
    pub fn get() -> Option<&'static mut GpuDriver> {
        unsafe { GPU.as_mut() }
    }

    /// Limpiar pantalla con un color
    pub fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, color);
            }
        }
    }

    /// Dibujar un pixel
    #[inline]
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = y * self.pitch + x * self.bpp;
        unsafe {
            let pixel = self.buffer.add(offset);
            ptr::write_volatile(pixel, (color & 0xFF) as u8);
            ptr::write_volatile(pixel.add(1), ((color >> 8) & 0xFF) as u8);
            ptr::write_volatile(pixel.add(2), ((color >> 16) & 0xFF) as u8);
        }
    }

    /// Dibujar rectángulo
    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.put_pixel(x + dx, y + dy, color);
            }
        }
    }

    /// Dibujar rectángulo con borde
    pub fn draw_rect_outline(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        // Línea superior
        for dx in 0..w {
            self.put_pixel(x + dx, y, color);
        }
        // Línea inferior
        for dx in 0..w {
            self.put_pixel(x + dx, y + h - 1, color);
        }
        // Línea izquierda
        for dy in 0..h {
            self.put_pixel(x, y + dy, color);
        }
        // Línea derecha
        for dy in 0..h {
            self.put_pixel(x + w - 1, y + dy, color);
        }
    }

    /// Dibujar línea horizontal
    pub fn draw_hline(&mut self, x: usize, y: usize, len: usize, color: u32) {
        for dx in 0..len {
            self.put_pixel(x + dx, y, color);
        }
    }

    /// Dibujar línea vertical
    pub fn draw_vline(&mut self, x: usize, y: usize, len: usize, color: u32) {
        for dy in 0..len {
            self.put_pixel(x, y + dy, color);
        }
    }

    /// Copiar región de memoria (blit)
    pub fn blit(&mut self, src: &[u8], x: usize, y: usize, w: usize, h: usize) {
        for dy in 0..h {
            for dx in 0..w {
                let idx = (dy * w + dx) * 3;
                if idx + 2 < src.len() {
                    let color = (src[idx] as u32)
                        | ((src[idx + 1] as u32) << 8)
                        | ((src[idx + 2] as u32) << 16);
                    self.put_pixel(x + dx, y + dy, color);
                }
            }
        }
    }

    /// Obtener dimensiones
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Obtener dirección del buffer (para programas ADead-BIB)
    pub fn buffer_addr(&self) -> usize {
        self.buffer as usize
    }
}

// Colores predefinidos
pub const COLOR_BLACK: u32 = 0x000000;
pub const COLOR_WHITE: u32 = 0xFFFFFF;
pub const COLOR_RED: u32 = 0xFF0000;
pub const COLOR_GREEN: u32 = 0x00FF00;
pub const COLOR_BLUE: u32 = 0x0000FF;
pub const COLOR_YELLOW: u32 = 0xFFFF00;
pub const COLOR_CYAN: u32 = 0x00FFFF;
pub const COLOR_MAGENTA: u32 = 0xFF00FF;
