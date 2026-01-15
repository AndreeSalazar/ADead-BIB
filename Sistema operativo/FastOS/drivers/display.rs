// ============================================================================
// FastOS - Display Driver
// ============================================================================
// Driver de pantalla (framebuffer)
// Soporta VGA text mode y framebuffer gráfico
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

/// Información del framebuffer
pub struct FramebufferInfo {
    pub address: u64,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u8,
}

/// Driver de display
pub struct Display {
    info: Option<FramebufferInfo>,
    mode: DisplayMode,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DisplayMode {
    TextMode,
    Framebuffer,
}

impl Display {
    /// Crear nuevo display (modo texto por defecto)
    pub const fn new() -> Self {
        Display {
            info: None,
            mode: DisplayMode::TextMode,
        }
    }

    /// Inicializar con framebuffer
    pub fn init_framebuffer(&mut self, info: FramebufferInfo) {
        self.info = Some(info);
        self.mode = DisplayMode::Framebuffer;
    }

    /// Obtener modo actual
    pub fn mode(&self) -> DisplayMode {
        self.mode
    }

    /// Dibujar pixel (solo en modo framebuffer)
    pub fn draw_pixel(&self, x: u32, y: u32, color: u32) {
        if let Some(ref info) = self.info {
            if x < info.width && y < info.height {
                let offset = (y * info.pitch + x * (info.bpp as u32 / 8)) as usize;
                let addr = info.address as *mut u32;
                unsafe {
                    *addr.add(offset / 4) = color;
                }
            }
        }
    }

    /// Dibujar rectángulo
    pub fn draw_rect(&self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.draw_pixel(x + dx, y + dy, color);
            }
        }
    }

    /// Limpiar pantalla
    pub fn clear(&self, color: u32) {
        if let Some(ref info) = self.info {
            self.draw_rect(0, 0, info.width, info.height, color);
        }
    }

    /// Obtener dimensiones
    pub fn dimensions(&self) -> (u32, u32) {
        if let Some(ref info) = self.info {
            (info.width, info.height)
        } else {
            (80, 25) // VGA text mode
        }
    }
}

/// Colores comunes (ARGB)
pub mod colors {
    pub const BLACK: u32 = 0xFF000000;
    pub const WHITE: u32 = 0xFFFFFFFF;
    pub const RED: u32 = 0xFFFF0000;
    pub const GREEN: u32 = 0xFF00FF00;
    pub const BLUE: u32 = 0xFF0000FF;
    pub const YELLOW: u32 = 0xFFFFFF00;
    pub const CYAN: u32 = 0xFF00FFFF;
    pub const MAGENTA: u32 = 0xFFFF00FF;
    pub const GRAY: u32 = 0xFF808080;
    pub const DARK_GRAY: u32 = 0xFF404040;
    pub const LIGHT_GRAY: u32 = 0xFFC0C0C0;
}
