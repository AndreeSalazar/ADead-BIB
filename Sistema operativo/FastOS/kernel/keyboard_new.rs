// ============================================================================
// FastOS Keyboard Driver (PS/2)
// ============================================================================
// Driver de teclado con buffer de entrada
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};

/// Tamaño del buffer de teclado
const BUFFER_SIZE: usize = 256;

/// Buffer circular de caracteres
static mut KEY_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static BUFFER_HEAD: AtomicUsize = AtomicUsize::new(0);
static BUFFER_TAIL: AtomicUsize = AtomicUsize::new(0);

/// Estado de teclas modificadoras
static mut SHIFT_PRESSED: bool = false;
static mut CTRL_PRESSED: bool = false;
static mut ALT_PRESSED: bool = false;
static mut CAPS_LOCK: bool = false;

/// Mapa de scancodes a caracteres (US layout)
const SCANCODE_MAP: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 8,   // 0-14
    b'\t', b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n', // 15-28
    0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',         // 29-41
    0, b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0,            // 42-54
    b'*', 0, b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,                                        // 55-67
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,                                     // 68-83
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,                                     // 84-99
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,                                     // 100-115
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,                                                 // 116-127
];

/// Mapa de scancodes con Shift
const SCANCODE_MAP_SHIFT: [u8; 128] = [
    0, 27, b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 8,
    b'\t', b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', b'\n',
    0, b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', b'~',
    0, b'|', b'Z', b'X', b'C', b'V', b'B', b'N', b'M', b'<', b'>', b'?', 0,
    b'*', 0, b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// Inicializar teclado
pub fn init() {
    // El teclado PS/2 ya está inicializado por el BIOS
    // Solo limpiamos el buffer
    BUFFER_HEAD.store(0, Ordering::SeqCst);
    BUFFER_TAIL.store(0, Ordering::SeqCst);
}

/// Manejar scancode (llamado desde IRQ1)
pub fn handle_scancode(scancode: u8) {
    let released = scancode & 0x80 != 0;
    let code = scancode & 0x7F;

    unsafe {
        match code {
            0x2A | 0x36 => SHIFT_PRESSED = !released, // Left/Right Shift
            0x1D => CTRL_PRESSED = !released,         // Ctrl
            0x38 => ALT_PRESSED = !released,          // Alt
            0x3A if !released => CAPS_LOCK = !CAPS_LOCK, // Caps Lock toggle
            _ if !released => {
                // Obtener carácter
                let ch = if SHIFT_PRESSED || CAPS_LOCK {
                    SCANCODE_MAP_SHIFT[code as usize]
                } else {
                    SCANCODE_MAP[code as usize]
                };

                if ch != 0 {
                    push_char(ch);
                }
            }
            _ => {}
        }
    }
}

/// Agregar carácter al buffer
fn push_char(ch: u8) {
    let head = BUFFER_HEAD.load(Ordering::SeqCst);
    let next_head = (head + 1) % BUFFER_SIZE;
    
    if next_head != BUFFER_TAIL.load(Ordering::SeqCst) {
        unsafe { KEY_BUFFER[head] = ch; }
        BUFFER_HEAD.store(next_head, Ordering::SeqCst);
    }
}

/// Leer carácter del buffer (no bloqueante)
pub fn read_char() -> Option<u8> {
    let tail = BUFFER_TAIL.load(Ordering::SeqCst);
    let head = BUFFER_HEAD.load(Ordering::SeqCst);
    
    if tail == head {
        return None;
    }
    
    let ch = unsafe { KEY_BUFFER[tail] };
    BUFFER_TAIL.store((tail + 1) % BUFFER_SIZE, Ordering::SeqCst);
    Some(ch)
}

/// Leer carácter (bloqueante)
pub fn getchar() -> u8 {
    loop {
        if let Some(ch) = read_char() {
            return ch;
        }
        unsafe { core::arch::asm!("hlt"); }
    }
}

/// Verificar si hay caracteres disponibles
pub fn has_char() -> bool {
    BUFFER_HEAD.load(Ordering::SeqCst) != BUFFER_TAIL.load(Ordering::SeqCst)
}

/// Leer línea completa
pub fn read_line(buffer: &mut [u8]) -> usize {
    let mut pos = 0;
    
    loop {
        let ch = getchar();
        
        match ch {
            b'\n' | 13 => {
                if pos < buffer.len() {
                    buffer[pos] = 0;
                }
                return pos;
            }
            8 | 127 => { // Backspace
                if pos > 0 {
                    pos -= 1;
                }
            }
            _ if pos < buffer.len() - 1 => {
                buffer[pos] = ch;
                pos += 1;
            }
            _ => {}
        }
    }
}

/// Obtener estado de modificadores
pub fn get_modifiers() -> (bool, bool, bool) {
    unsafe { (SHIFT_PRESSED, CTRL_PRESSED, ALT_PRESSED) }
}
