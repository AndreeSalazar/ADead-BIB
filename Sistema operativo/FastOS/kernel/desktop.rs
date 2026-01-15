// ============================================================================
// FastOS Desktop Environment - Windows 11 Style
// ============================================================================

use crate::gpu::GpuDriver;
use crate::mouse;

// Colores Windows 11
const COLOR_BG_TOP: u32 = 0x0078D4;       // Azul claro arriba
const COLOR_BG_BOTTOM: u32 = 0x001F54;    // Azul oscuro abajo
const COLOR_TASKBAR: u32 = 0x1A1A2E;      // Barra oscura semi-transparente
const COLOR_WINDOW_BG: u32 = 0xF3F3F3;    // Fondo ventana claro
const COLOR_WINDOW_TITLE_BG: u32 = 0xFFFFFF; // Barra título blanca
const COLOR_WINDOW_BORDER: u32 = 0xE0E0E0;
const COLOR_WHITE: u32 = 0xFFFFFF;
const COLOR_BLACK: u32 = 0x000000;
const COLOR_ACCENT: u32 = 0x0078D4;       // Azul acento
const COLOR_GRAY: u32 = 0x666666;
const COLOR_ICON_BG: u32 = 0x2D2D44;      // Fondo iconos taskbar

/// Ventana moderna
pub struct Window {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub title: &'static str,
    pub visible: bool,
    pub dragging: bool,
    pub drag_ox: i32,
    pub drag_oy: i32,
}

impl Window {
    pub const fn new(x: i32, y: i32, w: i32, h: i32, title: &'static str) -> Self {
        Window { x, y, width: w, height: h, title, visible: true, dragging: false, drag_ox: 0, drag_oy: 0 }
    }

    pub fn title_contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + 32
    }
}

/// Escritorio moderno
pub struct Desktop {
    pub width: i32,
    pub height: i32,
    pub window: Window,
    pub taskbar_h: i32,
    prev_btn: u8,
}

impl Desktop {
    pub fn new(w: i32, h: i32) -> Self {
        Desktop {
            width: w,
            height: h,
            window: Window::new(w/2 - 200, h/2 - 150, 400, 300, "FastOS"),
            taskbar_h: 48,
            prev_btn: 0,
        }
    }

    pub fn update(&mut self) {
        let (mx, my) = mouse::get_position();
        let btn = mouse::get_buttons();
        let pressed = btn & 1 != 0;
        let was_pressed = self.prev_btn & 1 != 0;

        if self.window.dragging {
            if pressed {
                self.window.x = mx - self.window.drag_ox;
                self.window.y = my - self.window.drag_oy;
                if self.window.y < 0 { self.window.y = 0; }
                if self.window.y > self.height - self.taskbar_h - 32 {
                    self.window.y = self.height - self.taskbar_h - 32;
                }
            } else {
                self.window.dragging = false;
            }
        } else if pressed && !was_pressed && self.window.title_contains(mx, my) {
            self.window.dragging = true;
            self.window.drag_ox = mx - self.window.x;
            self.window.drag_oy = my - self.window.y;
        }

        self.prev_btn = btn;
    }

    pub fn draw(&self, gpu: &mut GpuDriver) {
        self.draw_background(gpu);
        self.draw_dynamic(gpu, 0, 0, 0, 0);
    }

    /// Dibujar solo el fondo estático (una vez)
    pub fn draw_background(&self, gpu: &mut GpuDriver) {
        // Fondo degradado azul
        self.draw_gradient_bg(gpu);
        // Iconos del escritorio
        self.draw_desktop_icons(gpu);
        // Barra de tareas
        self.draw_taskbar(gpu);
    }

    /// Dibujar solo elementos dinámicos (ventana y cursor)
    pub fn draw_dynamic(&self, gpu: &mut GpuDriver, old_mx: i32, old_my: i32, old_wx: i32, old_wy: i32) {
        // Borrar cursor anterior
        self.restore_bg_at_cursor(gpu, old_mx as usize, old_my as usize);
        
        // Borrar ventana anterior si se movió
        if old_wx != self.window.x || old_wy != self.window.y {
            self.restore_bg_at_window(gpu, old_wx, old_wy);
        }
        
        // Dibujar ventana
        if self.window.visible {
            self.draw_window(gpu);
        }
        
        // Dibujar cursor
        self.draw_cursor(gpu);
    }

    fn restore_bg_at_cursor(&self, gpu: &mut GpuDriver, x: usize, y: usize) {
        // Restaurar área del cursor con color del fondo
        let h = (self.height - self.taskbar_h) as usize;
        for dy in 0..16usize {
            let py = y + dy;
            if py < h {
                let t = py as u32 * 255 / h as u32;
                let r = ((0x00 * (255 - t) + 0x00 * t) / 255) as u32;
                let g = ((0x78 * (255 - t) + 0x1F * t) / 255) as u32;
                let b = ((0xD4 * (255 - t) + 0x54 * t) / 255) as u32;
                let color = (r << 16) | (g << 8) | b;
                for dx in 0..16usize {
                    if x + dx < self.width as usize {
                        gpu.put_pixel(x + dx, py, color);
                    }
                }
            }
        }
    }

    fn restore_bg_at_window(&self, gpu: &mut GpuDriver, wx: i32, wy: i32) {
        let x = wx.max(0) as usize;
        let y = wy.max(0) as usize;
        let w = (self.window.width + 10) as usize;
        let h = (self.window.height + 10) as usize;
        let screen_h = (self.height - self.taskbar_h) as usize;
        
        for dy in 0..h {
            let py = y + dy;
            if py < screen_h {
                let t = py as u32 * 255 / screen_h as u32;
                let r = ((0x00 * (255 - t) + 0x00 * t) / 255) as u32;
                let g = ((0x78 * (255 - t) + 0x1F * t) / 255) as u32;
                let b = ((0xD4 * (255 - t) + 0x54 * t) / 255) as u32;
                let color = (r << 16) | (g << 8) | b;
                for dx in 0..w {
                    if x + dx < self.width as usize {
                        gpu.put_pixel(x + dx, py, color);
                    }
                }
            }
        }
    }

    fn draw_gradient_bg(&self, gpu: &mut GpuDriver) {
        let h = self.height - self.taskbar_h;
        for y in 0..h as usize {
            let t = y as u32 * 255 / h as u32;
            let r = ((0x00 * (255 - t) + 0x00 * t) / 255) as u32;
            let g = ((0x78 * (255 - t) + 0x1F * t) / 255) as u32;
            let b = ((0xD4 * (255 - t) + 0x54 * t) / 255) as u32;
            let color = (r << 16) | (g << 8) | b;
            gpu.draw_hline(0, y, self.width as usize, color);
        }
    }

    fn draw_desktop_icons(&self, gpu: &mut GpuDriver) {
        // Icono 1: FastOS
        self.draw_icon(gpu, 30, 30, "FastOS", 0x0078D4);
        // Icono 2: Terminal
        self.draw_icon(gpu, 30, 120, "Terminal", 0x1E1E1E);
        // Icono 3: Archivos
        self.draw_icon(gpu, 30, 210, "Archivos", 0xFFB900);
        // Icono 4: Config
        self.draw_icon(gpu, 30, 300, "Config", 0x666666);
    }

    fn draw_icon(&self, gpu: &mut GpuDriver, x: usize, y: usize, label: &str, color: u32) {
        // Cuadro del icono
        gpu.draw_rect(x, y, 64, 64, color);
        // Borde redondeado simulado
        gpu.put_pixel(x, y, COLOR_BG_TOP);
        gpu.put_pixel(x + 63, y, COLOR_BG_TOP);
        gpu.put_pixel(x, y + 63, COLOR_BG_TOP);
        gpu.put_pixel(x + 63, y + 63, COLOR_BG_TOP);
        // Texto
        let tx = x + 32 - (label.len() * 4);
        self.draw_text(gpu, label, tx, y + 70, COLOR_WHITE);
    }

    fn draw_window(&self, gpu: &mut GpuDriver) {
        let x = self.window.x as usize;
        let y = self.window.y as usize;
        let w = self.window.width as usize;
        let h = self.window.height as usize;

        // Sombra suave
        gpu.draw_rect(x + 8, y + 8, w, h, 0x00000040);
        
        // Fondo ventana
        gpu.draw_rect(x, y, w, h, COLOR_WINDOW_BG);
        
        // Barra de título
        gpu.draw_rect(x, y, w, 32, COLOR_WINDOW_TITLE_BG);
        
        // Título
        self.draw_text(gpu, self.window.title, x + 12, y + 10, COLOR_BLACK);
        
        // Botones de ventana (minimizar, maximizar, cerrar)
        // Cerrar (rojo)
        gpu.draw_rect(x + w - 46, y, 46, 32, 0xE81123);
        self.draw_text(gpu, "X", x + w - 28, y + 10, COLOR_WHITE);
        
        // Maximizar
        gpu.draw_rect(x + w - 92, y, 46, 32, COLOR_WINDOW_TITLE_BG);
        self.draw_text(gpu, "[]", x + w - 78, y + 10, COLOR_GRAY);
        
        // Minimizar
        gpu.draw_rect(x + w - 138, y, 46, 32, COLOR_WINDOW_TITLE_BG);
        self.draw_text(gpu, "-", x + w - 118, y + 10, COLOR_GRAY);
        
        // Línea separadora
        gpu.draw_hline(x, y + 32, w, COLOR_WINDOW_BORDER);
        
        // Contenido
        let cy = y + 50;
        self.draw_text(gpu, "Bienvenido a FastOS!", x + 20, cy, COLOR_BLACK);
        self.draw_text(gpu, "GPU-First / Binary-First OS", x + 20, cy + 25, COLOR_GRAY);
        self.draw_text(gpu, "Stack: ADead-BIB + Rust + wgpu", x + 20, cy + 50, COLOR_ACCENT);
        
        self.draw_text(gpu, "Autor: Eddi Andree Salazar Matos", x + 20, cy + 90, COLOR_BLACK);
        self.draw_text(gpu, "Pais: Peru", x + 20, cy + 115, COLOR_BLACK);
        
        // Bandera de Perú
        gpu.draw_rect(x + 20, cy + 145, 120, 20, 0xFF0000);
        gpu.draw_rect(x + 20, cy + 165, 120, 20, COLOR_WHITE);
        gpu.draw_rect(x + 20, cy + 185, 120, 20, 0xFF0000);
        
        self.draw_text(gpu, "[OK] Modulos cargados", x + 160, cy + 165, 0x00AA00);
    }

    fn draw_taskbar(&self, gpu: &mut GpuDriver) {
        let y = (self.height - self.taskbar_h) as usize;
        let w = self.width as usize;
        let h = self.taskbar_h as usize;

        // Fondo oscuro semi-transparente
        gpu.draw_rect(0, y, w, h, COLOR_TASKBAR);
        
        // Iconos centrados
        let center = w / 2;
        let icon_size = 40;
        let gap = 8;
        let total = 4 * icon_size + 3 * gap;
        let start = center - total / 2;
        
        // Icono Inicio (Windows)
        gpu.draw_rect(start, y + 4, icon_size, icon_size, COLOR_ICON_BG);
        self.draw_text(gpu, "W", start + 14, y + 14, COLOR_WHITE);
        
        // Icono Buscar
        gpu.draw_rect(start + icon_size + gap, y + 4, icon_size, icon_size, COLOR_ICON_BG);
        self.draw_text(gpu, "?", start + icon_size + gap + 14, y + 14, COLOR_WHITE);
        
        // Icono Archivos
        gpu.draw_rect(start + 2*(icon_size + gap), y + 4, icon_size, icon_size, COLOR_ICON_BG);
        self.draw_text(gpu, "F", start + 2*(icon_size + gap) + 14, y + 14, 0xFFB900);
        
        // Icono Terminal
        gpu.draw_rect(start + 3*(icon_size + gap), y + 4, icon_size, icon_size, COLOR_ICON_BG);
        self.draw_text(gpu, ">", start + 3*(icon_size + gap) + 14, y + 14, 0x00FF00);
        
        // Reloj a la derecha
        self.draw_text(gpu, "12:00", w - 60, y + 16, COLOR_WHITE);
    }

    fn draw_cursor(&self, gpu: &mut GpuDriver) {
        let (mx, my) = mouse::get_position();
        let x = mx as usize;
        let y = my as usize;

        // Cursor moderno (flecha)
        for i in 0..16usize {
            let len = if i < 12 { 12 - i } else { 0 };
            for j in 0..len {
                gpu.put_pixel(x + j, y + i, COLOR_WHITE);
            }
        }
        // Borde
        for i in 0..12usize {
            gpu.put_pixel(x + 12 - i, y + i, COLOR_BLACK);
        }
        gpu.draw_vline(x, y, 12, COLOR_BLACK);
    }

    fn draw_text(&self, gpu: &mut GpuDriver, text: &str, x: usize, y: usize, color: u32) {
        let mut cx = x;
        for c in text.bytes() {
            self.draw_char(gpu, cx, y, c, color);
            cx += 8;
        }
    }

    fn draw_char(&self, gpu: &mut GpuDriver, x: usize, y: usize, c: u8, color: u32) {
        let bitmap = get_font(c);
        for (row, &bits) in bitmap.iter().enumerate() {
            for col in 0..8 {
                if bits & (0x80 >> col) != 0 {
                    gpu.put_pixel(x + col, y + row, color);
                }
            }
        }
    }
}

fn get_font(c: u8) -> [u8; 8] {
    match c {
        b'A' => [0x18, 0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x00],
        b'B' => [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00],
        b'C' => [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00],
        b'D' => [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00],
        b'E' => [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7E, 0x00],
        b'F' => [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x60, 0x00],
        b'G' => [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00],
        b'H' => [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
        b'I' => [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
        b'K' => [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00],
        b'L' => [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00],
        b'M' => [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00],
        b'N' => [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00],
        b'O' => [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        b'P' => [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00],
        b'R' => [0x7C, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0x66, 0x00],
        b'S' => [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
        b'T' => [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
        b'U' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        b'V' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
        b'W' => [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00],
        b'X' => [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00],
        b'a' => [0x00, 0x00, 0x3C, 0x06, 0x3E, 0x66, 0x3E, 0x00],
        b'b' => [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x7C, 0x00],
        b'c' => [0x00, 0x00, 0x3C, 0x66, 0x60, 0x66, 0x3C, 0x00],
        b'd' => [0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x3E, 0x00],
        b'e' => [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x60, 0x3C, 0x00],
        b'f' => [0x1C, 0x36, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x00],
        b'g' => [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x3C],
        b'h' => [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
        b'i' => [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00],
        b'k' => [0x60, 0x60, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0x00],
        b'l' => [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
        b'm' => [0x00, 0x00, 0x66, 0x7F, 0x7F, 0x6B, 0x63, 0x00],
        b'n' => [0x00, 0x00, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
        b'o' => [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00],
        b'p' => [0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60],
        b'r' => [0x00, 0x00, 0x7C, 0x66, 0x60, 0x60, 0x60, 0x00],
        b's' => [0x00, 0x00, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x00],
        b't' => [0x30, 0x30, 0x7C, 0x30, 0x30, 0x36, 0x1C, 0x00],
        b'u' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3E, 0x00],
        b'v' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
        b'w' => [0x00, 0x00, 0x63, 0x6B, 0x7F, 0x7F, 0x36, 0x00],
        b'x' => [0x00, 0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x00],
        b'y' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x3E, 0x06, 0x3C],
        b'z' => [0x00, 0x00, 0x7E, 0x0C, 0x18, 0x30, 0x7E, 0x00],
        b'0' => [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00],
        b'1' => [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],
        b'2' => [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x30, 0x7E, 0x00],
        b' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        b'!' => [0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x00],
        b'.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00],
        b':' => [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00],
        b'-' => [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00],
        b'+' => [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00],
        b'/' => [0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x00, 0x00],
        b'@' => [0x3C, 0x66, 0x6E, 0x6A, 0x6E, 0x60, 0x3C, 0x00],
        b'[' => [0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00],
        b']' => [0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00],
        b'>' => [0x60, 0x30, 0x18, 0x0C, 0x18, 0x30, 0x60, 0x00],
        b'?' => [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x00, 0x18, 0x00],
        _ => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}
