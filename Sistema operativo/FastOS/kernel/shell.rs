// ============================================================================
// FastOS Shell
// ============================================================================
// Shell interactivo del sistema
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

use crate::vfs;
use crate::gpu::GpuDriver;
use crate::timer;

/// Estado del shell
pub struct Shell {
    pub input_buffer: [u8; 256],
    pub input_pos: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub width: usize,
    pub height: usize,
    pub char_width: usize,
    pub char_height: usize,
    pub fg_color: u32,
    pub bg_color: u32,
}

impl Shell {
    pub fn new(width: usize, height: usize) -> Self {
        Shell {
            input_buffer: [0; 256],
            input_pos: 0,
            cursor_x: 0,
            cursor_y: 0,
            width,
            height,
            char_width: 8,
            char_height: 16,
            fg_color: 0x00FF00, // Verde
            bg_color: 0x000000, // Negro
        }
    }

    /// Inicializar shell
    pub fn init(&mut self, gpu: &mut GpuDriver) {
        // Limpiar pantalla
        gpu.clear(self.bg_color);
        
        // Mostrar banner
        self.print_str(gpu, "╔══════════════════════════════════════════════════════════════╗\n");
        self.print_str(gpu, "║                    FastOS v0.2.0                             ║\n");
        self.print_str(gpu, "║            GPU-First / Binary-First OS                       ║\n");
        self.print_str(gpu, "╠══════════════════════════════════════════════════════════════╣\n");
        self.print_str(gpu, "║  Stack: ADead-BIB + Rust + wgpu                              ║\n");
        self.print_str(gpu, "║  Autor: Eddi Andree Salazar Matos                            ║\n");
        self.print_str(gpu, "║  Pais: Peru                                                  ║\n");
        self.print_str(gpu, "╚══════════════════════════════════════════════════════════════╝\n");
        self.print_str(gpu, "\nEscribe 'help' para ver comandos disponibles.\n\n");
        
        self.print_prompt(gpu);
    }

    /// Imprimir prompt
    pub fn print_prompt(&mut self, gpu: &mut GpuDriver) {
        let path = vfs::current_path();
        self.print_str(gpu, path);
        self.print_str(gpu, "> ");
    }

    /// Procesar carácter de entrada
    pub fn handle_char(&mut self, gpu: &mut GpuDriver, ch: u8) {
        match ch {
            b'\n' | 13 => {
                self.print_char(gpu, b'\n');
                self.execute_command(gpu);
                self.input_pos = 0;
                self.input_buffer = [0; 256];
                self.print_prompt(gpu);
            }
            8 | 127 => { // Backspace
                if self.input_pos > 0 {
                    self.input_pos -= 1;
                    self.input_buffer[self.input_pos] = 0;
                    // Borrar carácter en pantalla
                    if self.cursor_x > 0 {
                        self.cursor_x -= 1;
                        self.print_char_at(gpu, self.cursor_x, self.cursor_y, b' ');
                    }
                }
            }
            _ if self.input_pos < 255 => {
                self.input_buffer[self.input_pos] = ch;
                self.input_pos += 1;
                self.print_char(gpu, ch);
            }
            _ => {}
        }
    }

    /// Ejecutar comando
    fn execute_command(&mut self, gpu: &mut GpuDriver) {
        // Copiar input a buffer local para evitar borrowing issues
        let mut cmd_buf = [0u8; 256];
        let mut arg_buf = [0u8; 128];
        let mut echo_buf = [0u8; 200];
        
        let len = self.input_buffer.iter().position(|&c| c == 0).unwrap_or(self.input_pos);
        cmd_buf[..len].copy_from_slice(&self.input_buffer[..len]);
        
        let cmd_str = core::str::from_utf8(&cmd_buf[..len]).unwrap_or("");
        let mut parts = cmd_str.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        let arg1_str = parts.next().unwrap_or("");
        
        // Copiar arg1 a buffer
        let arg_len = arg1_str.len().min(127);
        arg_buf[..arg_len].copy_from_slice(arg1_str.as_bytes());
        let arg1 = core::str::from_utf8(&arg_buf[..arg_len]).unwrap_or("");
        
        // Copiar echo text
        if cmd_str.len() > 5 {
            let echo_text = &cmd_str[5..];
            let echo_len = echo_text.len().min(199);
            echo_buf[..echo_len].copy_from_slice(echo_text.as_bytes());
        }

        match cmd {
            "help" | "?" => self.cmd_help(gpu),
            "clear" | "cls" => self.cmd_clear(gpu),
            "ls" | "dir" => self.cmd_ls(gpu),
            "cat" | "type" => {
                let arg = core::str::from_utf8(&arg_buf[..arg1.len()]).unwrap_or("");
                self.cmd_cat(gpu, arg);
            }
            "cd" => {
                let arg = core::str::from_utf8(&arg_buf[..arg1.len()]).unwrap_or("");
                self.cmd_cd(gpu, arg);
            }
            "pwd" => self.cmd_pwd(gpu),
            "info" | "about" => self.cmd_info(gpu),
            "mem" | "memory" => self.cmd_mem(gpu),
            "time" => self.cmd_time(gpu),
            "uptime" => self.cmd_uptime(gpu),
            "echo" => {
                if cmd_str.len() > 5 {
                    let echo_len = (cmd_str.len() - 5).min(199);
                    let echo_text = core::str::from_utf8(&echo_buf[..echo_len]).unwrap_or("");
                    self.cmd_echo(gpu, echo_text.trim());
                }
            }
            "reboot" => self.cmd_reboot(),
            "" => {}
            _ => {
                self.print_str(gpu, "Comando no reconocido. Escribe 'help'.\n");
            }
        }
    }

    fn get_input_str(&self) -> &str {
        let len = self.input_buffer.iter().position(|&c| c == 0).unwrap_or(self.input_pos);
        core::str::from_utf8(&self.input_buffer[..len]).unwrap_or("")
    }

    fn split_command<'a>(&self, cmd: &'a str) -> [&'a str; 4] {
        let mut parts = [""; 4];
        for (i, part) in cmd.split_whitespace().take(4).enumerate() {
            parts[i] = part;
        }
        parts
    }

    // Comandos

    fn cmd_help(&mut self, gpu: &mut GpuDriver) {
        self.print_str(gpu, "\n╔═══════════════════════════════════════╗\n");
        self.print_str(gpu, "║         FastOS - Comandos             ║\n");
        self.print_str(gpu, "╠═══════════════════════════════════════╣\n");
        self.print_str(gpu, "║  help     - Mostrar esta ayuda        ║\n");
        self.print_str(gpu, "║  clear    - Limpiar pantalla          ║\n");
        self.print_str(gpu, "║  ls       - Listar archivos           ║\n");
        self.print_str(gpu, "║  cat FILE - Ver contenido             ║\n");
        self.print_str(gpu, "║  cd DIR   - Cambiar directorio        ║\n");
        self.print_str(gpu, "║  pwd      - Directorio actual         ║\n");
        self.print_str(gpu, "║  info     - Info del sistema          ║\n");
        self.print_str(gpu, "║  mem      - Estado de memoria         ║\n");
        self.print_str(gpu, "║  time     - Hora actual               ║\n");
        self.print_str(gpu, "║  uptime   - Tiempo encendido          ║\n");
        self.print_str(gpu, "║  echo TXT - Imprimir texto            ║\n");
        self.print_str(gpu, "║  reboot   - Reiniciar sistema         ║\n");
        self.print_str(gpu, "╚═══════════════════════════════════════╝\n\n");
    }

    fn cmd_clear(&mut self, gpu: &mut GpuDriver) {
        gpu.clear(self.bg_color);
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    fn cmd_ls(&mut self, gpu: &mut GpuDriver) {
        self.print_str(gpu, "\n");
        for entry in vfs::list_current() {
            let type_char = if entry.file_type == vfs::FileType::Directory { "DIR " } else { "FILE" };
            self.print_str(gpu, type_char);
            self.print_str(gpu, "  ");
            self.print_str(gpu, entry.name_str());
            if entry.file_type == vfs::FileType::File {
                self.print_str(gpu, "  (");
                self.print_num(gpu, entry.size);
                self.print_str(gpu, " bytes)");
            }
            self.print_str(gpu, "\n");
        }
        self.print_str(gpu, "\n");
    }

    fn cmd_cat(&mut self, gpu: &mut GpuDriver, filename: &str) {
        if filename.is_empty() {
            self.print_str(gpu, "Uso: cat <archivo>\n");
            return;
        }
        
        match vfs::read_file(filename) {
            Some(content) => {
                self.print_str(gpu, "\n");
                if let Ok(text) = core::str::from_utf8(content) {
                    self.print_str(gpu, text);
                }
                self.print_str(gpu, "\n");
            }
            None => {
                self.print_str(gpu, "Archivo no encontrado: ");
                self.print_str(gpu, filename);
                self.print_str(gpu, "\n");
            }
        }
    }

    fn cmd_cd(&mut self, gpu: &mut GpuDriver, dir: &str) {
        if dir.is_empty() {
            let _ = vfs::change_dir("/");
        } else if let Err(e) = vfs::change_dir(dir) {
            self.print_str(gpu, e);
            self.print_str(gpu, "\n");
        }
    }

    fn cmd_pwd(&mut self, gpu: &mut GpuDriver) {
        self.print_str(gpu, vfs::current_path());
        self.print_str(gpu, "\n");
    }

    fn cmd_info(&mut self, gpu: &mut GpuDriver) {
        self.print_str(gpu, "\n╔═══════════════════════════════════════╗\n");
        self.print_str(gpu, "║           FastOS v0.2.0               ║\n");
        self.print_str(gpu, "╠═══════════════════════════════════════╣\n");
        self.print_str(gpu, "║  Stack: ADead-BIB + Rust + wgpu       ║\n");
        self.print_str(gpu, "║  Arquitectura: x86_64                 ║\n");
        self.print_str(gpu, "║  Kernel: Monolitico                   ║\n");
        self.print_str(gpu, "╠═══════════════════════════════════════╣\n");
        self.print_str(gpu, "║  Autor: Eddi Andree Salazar Matos     ║\n");
        self.print_str(gpu, "║  Pais: Peru                           ║\n");
        self.print_str(gpu, "╚═══════════════════════════════════════╝\n\n");
    }

    fn cmd_mem(&mut self, gpu: &mut GpuDriver) {
        self.print_str(gpu, "\nMemoria:\n");
        self.print_str(gpu, "  Total: ");
        self.print_num(gpu, crate::heap::total());
        self.print_str(gpu, " bytes\n");
        self.print_str(gpu, "  Usada: ");
        self.print_num(gpu, crate::heap::used());
        self.print_str(gpu, " bytes\n");
        self.print_str(gpu, "  Libre: ");
        self.print_num(gpu, crate::heap::free());
        self.print_str(gpu, " bytes\n\n");
    }

    fn cmd_time(&mut self, gpu: &mut GpuDriver) {
        let (h, m, s) = timer::read_rtc_time();
        self.print_str(gpu, "Hora: ");
        self.print_num(gpu, h as usize);
        self.print_str(gpu, ":");
        if m < 10 { self.print_str(gpu, "0"); }
        self.print_num(gpu, m as usize);
        self.print_str(gpu, ":");
        if s < 10 { self.print_str(gpu, "0"); }
        self.print_num(gpu, s as usize);
        self.print_str(gpu, "\n");
    }

    fn cmd_uptime(&mut self, gpu: &mut GpuDriver) {
        let secs = timer::get_uptime();
        let mins = secs / 60;
        let hours = mins / 60;
        self.print_str(gpu, "Uptime: ");
        self.print_num(gpu, hours as usize);
        self.print_str(gpu, "h ");
        self.print_num(gpu, (mins % 60) as usize);
        self.print_str(gpu, "m ");
        self.print_num(gpu, (secs % 60) as usize);
        self.print_str(gpu, "s\n");
    }

    fn cmd_echo(&mut self, gpu: &mut GpuDriver, text: &str) {
        self.print_str(gpu, text);
        self.print_str(gpu, "\n");
    }

    fn cmd_reboot(&self) {
        unsafe {
            // Triple fault para reiniciar
            core::arch::asm!("int 0xFF");
        }
    }

    // Funciones de impresión

    pub fn print_str(&mut self, gpu: &mut GpuDriver, s: &str) {
        for ch in s.bytes() {
            self.print_char(gpu, ch);
        }
    }

    pub fn print_char(&mut self, gpu: &mut GpuDriver, ch: u8) {
        match ch {
            b'\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
                self.check_scroll(gpu);
            }
            b'\r' => {
                self.cursor_x = 0;
            }
            b'\t' => {
                self.cursor_x = (self.cursor_x + 4) & !3;
            }
            _ => {
                self.print_char_at(gpu, self.cursor_x, self.cursor_y, ch);
                self.cursor_x += 1;
                if self.cursor_x * self.char_width >= self.width {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                    self.check_scroll(gpu);
                }
            }
        }
    }

    fn print_char_at(&self, gpu: &mut GpuDriver, x: usize, y: usize, ch: u8) {
        let px = x * self.char_width;
        let py = y * self.char_height;
        
        // Fuente 8x16 simplificada
        let glyph = get_glyph(ch);
        for row in 0..16 {
            let bits = glyph[row];
            for col in 0..8 {
                let color = if (bits >> (7 - col)) & 1 == 1 {
                    self.fg_color
                } else {
                    self.bg_color
                };
                gpu.put_pixel(px + col, py + row, color);
            }
        }
    }

    fn print_num(&mut self, gpu: &mut GpuDriver, n: usize) {
        if n == 0 {
            self.print_char(gpu, b'0');
            return;
        }
        
        let mut buf = [0u8; 20];
        let mut i = 0;
        let mut num = n;
        
        while num > 0 {
            buf[i] = b'0' + (num % 10) as u8;
            num /= 10;
            i += 1;
        }
        
        while i > 0 {
            i -= 1;
            self.print_char(gpu, buf[i]);
        }
    }

    fn check_scroll(&mut self, gpu: &mut GpuDriver) {
        let max_rows = self.height / self.char_height;
        if self.cursor_y >= max_rows {
            // Scroll simple: limpiar y mover cursor arriba
            gpu.clear(self.bg_color);
            self.cursor_y = 0;
        }
    }
}

/// Obtener glyph de carácter (fuente 8x16 básica)
fn get_glyph(ch: u8) -> [u8; 16] {
    // Fuente muy básica - solo caracteres imprimibles
    match ch {
        b'A'..=b'Z' => FONT_UPPER[(ch - b'A') as usize],
        b'a'..=b'z' => FONT_LOWER[(ch - b'a') as usize],
        b'0'..=b'9' => FONT_DIGITS[(ch - b'0') as usize],
        b' ' => [0; 16],
        b'.' => [0,0,0,0,0,0,0,0,0,0,0,0,0x18,0x18,0,0],
        b',' => [0,0,0,0,0,0,0,0,0,0,0,0x18,0x18,0x08,0x10,0],
        b':' => [0,0,0,0,0x18,0x18,0,0,0,0x18,0x18,0,0,0,0,0],
        b';' => [0,0,0,0,0x18,0x18,0,0,0,0x18,0x18,0x08,0x10,0,0,0],
        b'-' => [0,0,0,0,0,0,0,0x7E,0,0,0,0,0,0,0,0],
        b'_' => [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0xFF,0],
        b'=' => [0,0,0,0,0,0x7E,0,0,0x7E,0,0,0,0,0,0,0],
        b'+' => [0,0,0,0,0x18,0x18,0x18,0x7E,0x18,0x18,0x18,0,0,0,0,0],
        b'/' => [0,0,0x02,0x04,0x08,0x10,0x20,0x40,0x80,0,0,0,0,0,0,0],
        b'\\' => [0,0,0x80,0x40,0x20,0x10,0x08,0x04,0x02,0,0,0,0,0,0,0],
        b'(' => [0,0x04,0x08,0x10,0x10,0x10,0x10,0x10,0x10,0x08,0x04,0,0,0,0,0],
        b')' => [0,0x20,0x10,0x08,0x08,0x08,0x08,0x08,0x08,0x10,0x20,0,0,0,0,0],
        b'[' => [0,0x1E,0x10,0x10,0x10,0x10,0x10,0x10,0x10,0x10,0x1E,0,0,0,0,0],
        b']' => [0,0x78,0x08,0x08,0x08,0x08,0x08,0x08,0x08,0x08,0x78,0,0,0,0,0],
        b'<' => [0,0,0x02,0x04,0x08,0x10,0x20,0x10,0x08,0x04,0x02,0,0,0,0,0],
        b'>' => [0,0,0x40,0x20,0x10,0x08,0x04,0x08,0x10,0x20,0x40,0,0,0,0,0],
        b'!' => [0,0,0x18,0x18,0x18,0x18,0x18,0x18,0,0x18,0x18,0,0,0,0,0],
        b'?' => [0,0,0x3C,0x42,0x02,0x04,0x08,0x10,0,0x10,0x10,0,0,0,0,0],
        b'\'' => [0,0x18,0x18,0x08,0x10,0,0,0,0,0,0,0,0,0,0,0],
        b'"' => [0,0x66,0x66,0x22,0x44,0,0,0,0,0,0,0,0,0,0,0],
        b'*' => [0,0,0,0x42,0x24,0x18,0xFF,0x18,0x24,0x42,0,0,0,0,0,0],
        b'#' => [0,0,0x24,0x24,0x7E,0x24,0x24,0x7E,0x24,0x24,0,0,0,0,0,0],
        0xC9 => [0xFF,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80], // ╔
        0xBB => [0xFF,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01], // ╗
        0xC8 => [0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0xFF], // ╚
        0xBC => [0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0xFF], // ╝
        0xCD => [0,0,0,0,0,0,0,0xFF,0,0,0,0,0,0,0,0], // ═
        0xBA => [0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18], // ║
        0xCC => [0x80,0x80,0x80,0x80,0x80,0x80,0x80,0xFF,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80], // ╠
        0xB9 => [0x01,0x01,0x01,0x01,0x01,0x01,0x01,0xFF,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01], // ╣
        _ => [0x00,0x7E,0x42,0x42,0x42,0x42,0x42,0x42,0x42,0x42,0x7E,0x00,0x00,0x00,0x00,0x00], // Default box
    }
}

// Fuentes básicas
const FONT_UPPER: [[u8; 16]; 26] = [
    [0,0,0x18,0x24,0x42,0x42,0x7E,0x42,0x42,0x42,0,0,0,0,0,0], // A
    [0,0,0x7C,0x42,0x42,0x7C,0x42,0x42,0x42,0x7C,0,0,0,0,0,0], // B
    [0,0,0x3C,0x42,0x40,0x40,0x40,0x40,0x42,0x3C,0,0,0,0,0,0], // C
    [0,0,0x78,0x44,0x42,0x42,0x42,0x42,0x44,0x78,0,0,0,0,0,0], // D
    [0,0,0x7E,0x40,0x40,0x7C,0x40,0x40,0x40,0x7E,0,0,0,0,0,0], // E
    [0,0,0x7E,0x40,0x40,0x7C,0x40,0x40,0x40,0x40,0,0,0,0,0,0], // F
    [0,0,0x3C,0x42,0x40,0x40,0x4E,0x42,0x42,0x3C,0,0,0,0,0,0], // G
    [0,0,0x42,0x42,0x42,0x7E,0x42,0x42,0x42,0x42,0,0,0,0,0,0], // H
    [0,0,0x3C,0x18,0x18,0x18,0x18,0x18,0x18,0x3C,0,0,0,0,0,0], // I
    [0,0,0x1E,0x04,0x04,0x04,0x04,0x44,0x44,0x38,0,0,0,0,0,0], // J
    [0,0,0x42,0x44,0x48,0x70,0x48,0x44,0x42,0x42,0,0,0,0,0,0], // K
    [0,0,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x7E,0,0,0,0,0,0], // L
    [0,0,0x42,0x66,0x5A,0x42,0x42,0x42,0x42,0x42,0,0,0,0,0,0], // M
    [0,0,0x42,0x62,0x52,0x4A,0x46,0x42,0x42,0x42,0,0,0,0,0,0], // N
    [0,0,0x3C,0x42,0x42,0x42,0x42,0x42,0x42,0x3C,0,0,0,0,0,0], // O
    [0,0,0x7C,0x42,0x42,0x7C,0x40,0x40,0x40,0x40,0,0,0,0,0,0], // P
    [0,0,0x3C,0x42,0x42,0x42,0x42,0x4A,0x44,0x3A,0,0,0,0,0,0], // Q
    [0,0,0x7C,0x42,0x42,0x7C,0x48,0x44,0x42,0x42,0,0,0,0,0,0], // R
    [0,0,0x3C,0x42,0x40,0x3C,0x02,0x02,0x42,0x3C,0,0,0,0,0,0], // S
    [0,0,0x7E,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0,0,0,0,0,0], // T
    [0,0,0x42,0x42,0x42,0x42,0x42,0x42,0x42,0x3C,0,0,0,0,0,0], // U
    [0,0,0x42,0x42,0x42,0x42,0x42,0x24,0x24,0x18,0,0,0,0,0,0], // V
    [0,0,0x42,0x42,0x42,0x42,0x5A,0x5A,0x66,0x42,0,0,0,0,0,0], // W
    [0,0,0x42,0x42,0x24,0x18,0x18,0x24,0x42,0x42,0,0,0,0,0,0], // X
    [0,0,0x42,0x42,0x24,0x18,0x18,0x18,0x18,0x18,0,0,0,0,0,0], // Y
    [0,0,0x7E,0x02,0x04,0x08,0x10,0x20,0x40,0x7E,0,0,0,0,0,0], // Z
];

const FONT_LOWER: [[u8; 16]; 26] = [
    [0,0,0,0,0,0x3C,0x02,0x3E,0x42,0x3E,0,0,0,0,0,0], // a
    [0,0,0x40,0x40,0x40,0x7C,0x42,0x42,0x42,0x7C,0,0,0,0,0,0], // b
    [0,0,0,0,0,0x3C,0x42,0x40,0x42,0x3C,0,0,0,0,0,0], // c
    [0,0,0x02,0x02,0x02,0x3E,0x42,0x42,0x42,0x3E,0,0,0,0,0,0], // d
    [0,0,0,0,0,0x3C,0x42,0x7E,0x40,0x3C,0,0,0,0,0,0], // e
    [0,0,0x0C,0x10,0x10,0x7C,0x10,0x10,0x10,0x10,0,0,0,0,0,0], // f
    [0,0,0,0,0,0x3E,0x42,0x42,0x3E,0x02,0x3C,0,0,0,0,0], // g
    [0,0,0x40,0x40,0x40,0x7C,0x42,0x42,0x42,0x42,0,0,0,0,0,0], // h
    [0,0,0x18,0,0,0x38,0x18,0x18,0x18,0x3C,0,0,0,0,0,0], // i
    [0,0,0x04,0,0,0x04,0x04,0x04,0x04,0x44,0x38,0,0,0,0,0], // j
    [0,0,0x40,0x40,0x44,0x48,0x70,0x48,0x44,0x42,0,0,0,0,0,0], // k
    [0,0,0x38,0x18,0x18,0x18,0x18,0x18,0x18,0x3C,0,0,0,0,0,0], // l
    [0,0,0,0,0,0x76,0x49,0x49,0x49,0x49,0,0,0,0,0,0], // m
    [0,0,0,0,0,0x7C,0x42,0x42,0x42,0x42,0,0,0,0,0,0], // n
    [0,0,0,0,0,0x3C,0x42,0x42,0x42,0x3C,0,0,0,0,0,0], // o
    [0,0,0,0,0,0x7C,0x42,0x42,0x7C,0x40,0x40,0,0,0,0,0], // p
    [0,0,0,0,0,0x3E,0x42,0x42,0x3E,0x02,0x02,0,0,0,0,0], // q
    [0,0,0,0,0,0x5C,0x62,0x40,0x40,0x40,0,0,0,0,0,0], // r
    [0,0,0,0,0,0x3E,0x40,0x3C,0x02,0x7C,0,0,0,0,0,0], // s
    [0,0,0x10,0x10,0x10,0x7C,0x10,0x10,0x10,0x0C,0,0,0,0,0,0], // t
    [0,0,0,0,0,0x42,0x42,0x42,0x42,0x3E,0,0,0,0,0,0], // u
    [0,0,0,0,0,0x42,0x42,0x42,0x24,0x18,0,0,0,0,0,0], // v
    [0,0,0,0,0,0x41,0x49,0x49,0x49,0x36,0,0,0,0,0,0], // w
    [0,0,0,0,0,0x42,0x24,0x18,0x24,0x42,0,0,0,0,0,0], // x
    [0,0,0,0,0,0x42,0x42,0x42,0x3E,0x02,0x3C,0,0,0,0,0], // y
    [0,0,0,0,0,0x7E,0x04,0x18,0x20,0x7E,0,0,0,0,0,0], // z
];

const FONT_DIGITS: [[u8; 16]; 10] = [
    [0,0,0x3C,0x42,0x46,0x4A,0x52,0x62,0x42,0x3C,0,0,0,0,0,0], // 0
    [0,0,0x18,0x28,0x08,0x08,0x08,0x08,0x08,0x3E,0,0,0,0,0,0], // 1
    [0,0,0x3C,0x42,0x02,0x0C,0x10,0x20,0x40,0x7E,0,0,0,0,0,0], // 2
    [0,0,0x3C,0x42,0x02,0x1C,0x02,0x02,0x42,0x3C,0,0,0,0,0,0], // 3
    [0,0,0x04,0x0C,0x14,0x24,0x44,0x7E,0x04,0x04,0,0,0,0,0,0], // 4
    [0,0,0x7E,0x40,0x40,0x7C,0x02,0x02,0x42,0x3C,0,0,0,0,0,0], // 5
    [0,0,0x1C,0x20,0x40,0x7C,0x42,0x42,0x42,0x3C,0,0,0,0,0,0], // 6
    [0,0,0x7E,0x02,0x04,0x08,0x10,0x10,0x10,0x10,0,0,0,0,0,0], // 7
    [0,0,0x3C,0x42,0x42,0x3C,0x42,0x42,0x42,0x3C,0,0,0,0,0,0], // 8
    [0,0,0x3C,0x42,0x42,0x42,0x3E,0x02,0x04,0x38,0,0,0,0,0,0], // 9
];
