// ============================================================================
// FastOS Desktop Environment
// ============================================================================

use crate::gpu::GpuDriver;
use crate::mouse;

// Colores del escritorio
const COLOR_DESKTOP: u32 = 0x008080;      // Teal (fondo)
const COLOR_TASKBAR: u32 = 0xC0C0C0;      // Gris claro
const COLOR_TASKBAR_DARK: u32 = 0x808080; // Gris oscuro
const COLOR_WINDOW_BG: u32 = 0xC0C0C0;    // Fondo ventana
const COLOR_WINDOW_TITLE: u32 = 0x000080; // Azul título
const COLOR_WHITE: u32 = 0xFFFFFF;
const COLOR_BLACK: u32 = 0x000000;
const COLOR_BUTTON: u32 = 0xC0C0C0;
const COLOR_CURSOR: u32 = 0xFFFFFF;

/// Ventana del escritorio
pub struct Window {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub title: &'static str,
    pub visible: bool,
    pub dragging: bool,
    pub drag_offset_x: i32,
    pub drag_offset_y: i32,
}

impl Window {
    pub const fn new(x: i32, y: i32, w: i32, h: i32, title: &'static str) -> Self {
        Window {
            x, y, width: w, height: h, title,
            visible: true,
            dragging: false,
            drag_offset_x: 0,
            drag_offset_y: 0,
        }
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.width &&
        py >= self.y && py < self.y + self.height
    }

    pub fn title_bar_contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.width &&
        py >= self.y && py < self.y + 20
    }
}

/// Estado del escritorio
pub struct Desktop {
    pub width: i32,
    pub height: i32,
    pub windows: [Window; 2],
    pub window_count: usize,
    pub taskbar_height: i32,
    prev_mouse_x: i32,
    prev_mouse_y: i32,
    prev_buttons: u8,
}

impl Desktop {
    pub fn new(w: i32, h: i32) -> Self {
        Desktop {
            width: w,
            height: h,
            windows: [
                Window::new(100, 80, 400, 300, "FastOS - Bienvenido"),
                Window::new(300, 150, 350, 250, "Acerca de"),
            ],
            window_count: 2,
            taskbar_height: 40,
            prev_mouse_x: w / 2,
            prev_mouse_y: h / 2,
            prev_buttons: 0,
        }
    }

    /// Actualizar lógica del escritorio
    pub fn update(&mut self) {
        let (mx, my) = mouse::get_position();
        let buttons = mouse::get_buttons();
        let left_pressed = buttons & 0x01 != 0;
        let left_was_pressed = self.prev_buttons & 0x01 != 0;

        // Manejar arrastre de ventanas
        for i in 0..self.window_count {
            let win = &mut self.windows[i];
            
            if win.dragging {
                if left_pressed {
                    win.x = mx - win.drag_offset_x;
                    win.y = my - win.drag_offset_y;
                    // Limitar a pantalla
                    if win.y < 0 { win.y = 0; }
                    if win.y > self.height - self.taskbar_height - 20 {
                        win.y = self.height - self.taskbar_height - 20;
                    }
                } else {
                    win.dragging = false;
                }
            } else if left_pressed && !left_was_pressed {
                if win.title_bar_contains(mx, my) {
                    win.dragging = true;
                    win.drag_offset_x = mx - win.x;
                    win.drag_offset_y = my - win.y;
                }
            }
        }

        self.prev_mouse_x = mx;
        self.prev_mouse_y = my;
        self.prev_buttons = buttons;
    }

    /// Dibujar todo el escritorio
    pub fn draw(&self, gpu: &mut GpuDriver) {
        // Fondo del escritorio
        gpu.clear(COLOR_DESKTOP);

        // Dibujar ventanas
        for i in 0..self.window_count {
            self.draw_window(gpu, &self.windows[i], i);
        }

        // Barra de tareas
        self.draw_taskbar(gpu);

        // Cursor del mouse
        self.draw_cursor(gpu);
    }

    fn draw_window(&self, gpu: &mut GpuDriver, win: &Window, idx: usize) {
        if !win.visible { return; }

        let x = win.x as usize;
        let y = win.y as usize;
        let w = win.width as usize;
        let h = win.height as usize;

        // Sombra
        gpu.draw_rect(x + 4, y + 4, w, h, 0x404040);

        // Fondo de ventana
        gpu.draw_rect(x, y, w, h, COLOR_WINDOW_BG);

        // Barra de título
        gpu.draw_rect(x, y, w, 20, COLOR_WINDOW_TITLE);

        // Título
        self.draw_text_at(gpu, win.title, x + 5, y + 6, COLOR_WHITE);

        // Botón cerrar
        gpu.draw_rect(x + w - 18, y + 3, 14, 14, COLOR_BUTTON);
        self.draw_text_at(gpu, "X", x + w - 14, y + 5, COLOR_BLACK);

        // Borde 3D
        gpu.draw_hline(x, y, w, COLOR_WHITE);
        gpu.draw_vline(x, y, h, COLOR_WHITE);
        gpu.draw_hline(x, y + h - 1, w, COLOR_TASKBAR_DARK);
        gpu.draw_vline(x + w - 1, y, h, COLOR_TASKBAR_DARK);

        // Contenido de la ventana
        let content_y = y + 25;
        if idx == 0 {
            self.draw_text_at(gpu, "Bienvenido a FastOS!", x + 20, content_y + 10, COLOR_BLACK);
            self.draw_text_at(gpu, "GPU-First / Binary-First OS", x + 20, content_y + 30, COLOR_BLACK);
            self.draw_text_at(gpu, "Stack: ADead-BIB + Rust + wgpu", x + 20, content_y + 50, COLOR_BLACK);
            self.draw_text_at(gpu, "", x + 20, content_y + 80, COLOR_BLACK);
            self.draw_text_at(gpu, "Mueve las ventanas con el mouse!", x + 20, content_y + 100, 0x000080);
            
            // Bandera pequeña
            gpu.draw_rect(x + 20, content_y + 130, 100, 15, 0xFF0000);
            gpu.draw_rect(x + 20, content_y + 145, 100, 15, COLOR_WHITE);
            gpu.draw_rect(x + 20, content_y + 160, 100, 15, 0xFF0000);
        } else {
            self.draw_text_at(gpu, "FastOS v0.1.0", x + 20, content_y + 10, COLOR_BLACK);
            self.draw_text_at(gpu, "Autor: Eddi Andree Salazar Matos", x + 20, content_y + 35, COLOR_BLACK);
            self.draw_text_at(gpu, "Pais: Peru", x + 20, content_y + 55, COLOR_BLACK);
            self.draw_text_at(gpu, "Email: eddi.salazar.dev@gmail.com", x + 20, content_y + 80, COLOR_BLACK);
            self.draw_text_at(gpu, "", x + 20, content_y + 110, COLOR_BLACK);
            self.draw_text_at(gpu, "[OK] Modulos cargados", x + 20, content_y + 130, 0x008000);
        }
    }

    fn draw_taskbar(&self, gpu: &mut GpuDriver) {
        let y = (self.height - self.taskbar_height) as usize;
        let w = self.width as usize;
        let h = self.taskbar_height as usize;

        // Fondo
        gpu.draw_rect(0, y, w, h, COLOR_TASKBAR);

        // Borde superior 3D
        gpu.draw_hline(0, y, w, COLOR_WHITE);

        // Botón Inicio
        gpu.draw_rect(4, y + 4, 80, h - 8, COLOR_BUTTON);
        gpu.draw_hline(4, y + 4, 80, COLOR_WHITE);
        gpu.draw_vline(4, y + 4, h - 8, COLOR_WHITE);
        gpu.draw_hline(4, y + h - 5, 80, COLOR_TASKBAR_DARK);
        gpu.draw_vline(83, y + 4, h - 8, COLOR_TASKBAR_DARK);
        self.draw_text_at(gpu, "Inicio", 20, y + 14, COLOR_BLACK);

        // Reloj (simulado)
        self.draw_text_at(gpu, "12:00", w - 50, y + 14, COLOR_BLACK);
    }

    fn draw_cursor(&self, gpu: &mut GpuDriver) {
        let (mx, my) = mouse::get_position();
        let x = mx as usize;
        let y = my as usize;

        // Cursor tipo flecha
        for i in 0..12 {
            gpu.draw_hline(x, y + i, 12 - i, COLOR_CURSOR);
        }
        // Borde negro
        for i in 0..12 {
            gpu.put_pixel(x + 12 - i, y + i, COLOR_BLACK);
        }
        gpu.draw_vline(x, y, 12, COLOR_BLACK);
    }

    fn draw_text_at(&self, gpu: &mut GpuDriver, text: &str, x: usize, y: usize, color: u32) {
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

// Font bitmap 8x8
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
        b'J' => [0x1E, 0x0C, 0x0C, 0x0C, 0x6C, 0x6C, 0x38, 0x00],
        b'K' => [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00],
        b'L' => [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00],
        b'M' => [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00],
        b'N' => [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00],
        b'O' => [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        b'P' => [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00],
        b'Q' => [0x3C, 0x66, 0x66, 0x66, 0x6A, 0x6C, 0x36, 0x00],
        b'R' => [0x7C, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0x66, 0x00],
        b'S' => [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
        b'T' => [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
        b'U' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        b'V' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
        b'W' => [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00],
        b'X' => [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00],
        b'Y' => [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00],
        b'Z' => [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00],
        b'a' => [0x00, 0x00, 0x3C, 0x06, 0x3E, 0x66, 0x3E, 0x00],
        b'b' => [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x7C, 0x00],
        b'c' => [0x00, 0x00, 0x3C, 0x66, 0x60, 0x66, 0x3C, 0x00],
        b'd' => [0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x3E, 0x00],
        b'e' => [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x60, 0x3C, 0x00],
        b'f' => [0x1C, 0x36, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x00],
        b'g' => [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x3C],
        b'h' => [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
        b'i' => [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00],
        b'j' => [0x0C, 0x00, 0x1C, 0x0C, 0x0C, 0x6C, 0x6C, 0x38],
        b'k' => [0x60, 0x60, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0x00],
        b'l' => [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
        b'm' => [0x00, 0x00, 0x66, 0x7F, 0x7F, 0x6B, 0x63, 0x00],
        b'n' => [0x00, 0x00, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
        b'o' => [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00],
        b'p' => [0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60],
        b'q' => [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x06],
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
        b'3' => [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00],
        b'4' => [0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x0C, 0x00],
        b'5' => [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00],
        b'6' => [0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00],
        b'7' => [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00],
        b'8' => [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00],
        b'9' => [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00],
        b' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        b'!' => [0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x00],
        b'.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00],
        b',' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30],
        b':' => [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00],
        b'-' => [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00],
        b'+' => [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00],
        b'/' => [0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x00, 0x00],
        b'@' => [0x3C, 0x66, 0x6E, 0x6A, 0x6E, 0x60, 0x3C, 0x00],
        b'[' => [0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00],
        b']' => [0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00],
        b'(' => [0x0C, 0x18, 0x30, 0x30, 0x30, 0x18, 0x0C, 0x00],
        b')' => [0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x18, 0x30, 0x00],
        _ => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}
