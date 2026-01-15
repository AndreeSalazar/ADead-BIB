// ============================================================================
// FastOS - Keyboard Driver
// ============================================================================
// Driver de teclado PS/2
// Convierte scancodes a caracteres ASCII
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

use spin::Mutex;

/// Buffer de entrada del teclado
pub static INPUT_BUFFER: Mutex<InputBuffer> = Mutex::new(InputBuffer::new());

/// Buffer circular para entrada
pub struct InputBuffer {
    buffer: [u8; 256],
    read_pos: usize,
    write_pos: usize,
}

impl InputBuffer {
    const fn new() -> Self {
        InputBuffer {
            buffer: [0; 256],
            read_pos: 0,
            write_pos: 0,
        }
    }

    /// Agregar caracter al buffer
    pub fn push(&mut self, c: u8) {
        let next = (self.write_pos + 1) % 256;
        if next != self.read_pos {
            self.buffer[self.write_pos] = c;
            self.write_pos = next;
        }
    }

    /// Leer caracter del buffer
    pub fn pop(&mut self) -> Option<u8> {
        if self.read_pos == self.write_pos {
            None
        } else {
            let c = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % 256;
            Some(c)
        }
    }
}

/// Tabla de scancodes a ASCII (US layout, sin shift)
static SCANCODE_TO_ASCII: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 8,   // 0-14
    b'\t', b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n', // 15-28
    0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',          // 29-41
    0, b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0,             // 42-54
    b'*', 0, b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,                                // 55-70
    0, 0, 0, 0, b'-', 0, 0, 0, b'+', 0, 0, 0, 0, 0, 0, 0,                                // 71-86
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,                                      // 87-102
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,                                      // 103-118
    0, 0, 0, 0, 0, 0, 0, 0, 0,                                                           // 119-127
];

/// Estado del teclado
static KEYBOARD_STATE: Mutex<KeyboardState> = Mutex::new(KeyboardState::new());

struct KeyboardState {
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
}

impl KeyboardState {
    const fn new() -> Self {
        KeyboardState {
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
        }
    }
}

/// Inicializar driver de teclado
pub fn init() {
    // El teclado ya está inicializado por el BIOS
    // Solo necesitamos habilitar la IRQ1 (hecho en interrupts.rs)
}

/// Manejar scancode del teclado
pub fn handle_scancode(scancode: u8) {
    let mut state = KEYBOARD_STATE.lock();
    
    // Key release (bit 7 set)
    if scancode & 0x80 != 0 {
        let released = scancode & 0x7F;
        match released {
            0x2A | 0x36 => state.shift_pressed = false, // Shift
            0x1D => state.ctrl_pressed = false,          // Ctrl
            0x38 => state.alt_pressed = false,           // Alt
            _ => {}
        }
        return;
    }
    
    // Key press
    match scancode {
        0x2A | 0x36 => {
            state.shift_pressed = true;
            return;
        }
        0x1D => {
            state.ctrl_pressed = true;
            return;
        }
        0x38 => {
            state.alt_pressed = true;
            return;
        }
        _ => {}
    }
    
    // Convertir a ASCII
    if scancode < 128 {
        let mut ascii = SCANCODE_TO_ASCII[scancode as usize];
        
        if ascii != 0 {
            // Aplicar shift
            if state.shift_pressed {
                ascii = apply_shift(ascii);
            }
            
            // Imprimir caracter
            if ascii == b'\n' {
                crate::println!();
                process_command();
                crate::print!("> ");
            } else if ascii == 8 {
                // Backspace
                // TODO: implementar backspace
            } else {
                crate::print!("{}", ascii as char);
                INPUT_BUFFER.lock().push(ascii);
            }
        }
    }
}

/// Aplicar shift a un caracter
fn apply_shift(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - 32, // Mayúsculas
        b'1' => b'!',
        b'2' => b'@',
        b'3' => b'#',
        b'4' => b'$',
        b'5' => b'%',
        b'6' => b'^',
        b'7' => b'&',
        b'8' => b'*',
        b'9' => b'(',
        b'0' => b')',
        b'-' => b'_',
        b'=' => b'+',
        b'[' => b'{',
        b']' => b'}',
        b'\\' => b'|',
        b';' => b':',
        b'\'' => b'"',
        b',' => b'<',
        b'.' => b'>',
        b'/' => b'?',
        b'`' => b'~',
        _ => c,
    }
}

/// Procesar comando ingresado
fn process_command() {
    let mut buffer = INPUT_BUFFER.lock();
    let mut cmd = [0u8; 64];
    let mut len = 0;
    
    while let Some(c) = buffer.pop() {
        if len < 64 {
            cmd[len] = c;
            len += 1;
        }
    }
    
    if len == 0 {
        return;
    }
    
    // Convertir a string
    let cmd_str = core::str::from_utf8(&cmd[..len]).unwrap_or("");
    
    // Ejecutar comando
    match cmd_str.trim() {
        "help" => {
            crate::println!("Comandos disponibles:");
            crate::println!("  help    - Mostrar esta ayuda");
            crate::println!("  clear   - Limpiar pantalla");
            crate::println!("  info    - Informacion del sistema");
            crate::println!("  mem     - Estado de memoria");
            crate::println!("  reboot  - Reiniciar sistema");
        }
        "clear" => {
            crate::vga::WRITER.lock().clear_screen();
        }
        "info" => {
            crate::println!("FastOS v0.1.0");
            crate::println!("ADead-BIB + Rust + wgpu");
            crate::println!("Author: Eddi Andree Salazar Matos");
            crate::println!("Architecture: x86_64");
        }
        "mem" => {
            let (used, free) = crate::memory::stats();
            crate::println!("Memoria usada: {} bytes", used);
            crate::println!("Memoria libre: {} bytes", free);
        }
        "reboot" => {
            crate::println!("Reiniciando...");
            unsafe {
                // Triple fault para reiniciar
                core::arch::asm!("int 0", options(noreturn));
            }
        }
        "" => {}
        _ => {
            crate::println!("Comando no reconocido: {}", cmd_str.trim());
            crate::println!("Escribe 'help' para ver comandos disponibles.");
        }
    }
}
