// ============================================================================
// FastOS - VGA Text Mode Driver
// ============================================================================
// Driver para modo texto VGA (80x25)
// Escribe directamente al buffer de video en 0xB8000
// Sin dependencias externas - puro Rust no_std
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

use core::fmt;
use core::ptr;

/// Dimensiones del buffer VGA
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_BUFFER: usize = 0xB8000;

/// Color por defecto (verde sobre negro)
const DEFAULT_COLOR: u8 = 0x0A; // Light green on black

/// Estado global del writer
static mut COLUMN: usize = 0;
static mut ROW: usize = 0;
static mut CURRENT_COLOR: u8 = DEFAULT_COLOR;

/// Cambiar color actual
pub fn set_color(color: u8) {
    unsafe {
        CURRENT_COLOR = color;
    }
}

/// Escribir caracter en posición específica
#[inline]
fn write_char_at(row: usize, col: usize, c: u8, color: u8) {
    let offset = (row * BUFFER_WIDTH + col) * 2;
    let vga = VGA_BUFFER as *mut u8;
    unsafe {
        ptr::write_volatile(vga.add(offset), c);
        ptr::write_volatile(vga.add(offset + 1), color);
    }
}

/// Leer caracter de posición específica
#[inline]
fn read_char_at(row: usize, col: usize) -> (u8, u8) {
    let offset = (row * BUFFER_WIDTH + col) * 2;
    let vga = VGA_BUFFER as *const u8;
    unsafe {
        let c = ptr::read_volatile(vga.add(offset));
        let color = ptr::read_volatile(vga.add(offset + 1));
        (c, color)
    }
}

/// Scroll pantalla hacia arriba
fn scroll() {
    for row in 1..BUFFER_HEIGHT {
        for col in 0..BUFFER_WIDTH {
            let (c, color) = read_char_at(row, col);
            write_char_at(row - 1, col, c, color);
        }
    }
    // Limpiar última fila
    for col in 0..BUFFER_WIDTH {
        write_char_at(BUFFER_HEIGHT - 1, col, b' ', DEFAULT_COLOR);
    }
}

/// Escribir un byte
pub fn write_byte(byte: u8) {
    unsafe {
        match byte {
            b'\n' => {
                COLUMN = 0;
                ROW += 1;
                if ROW >= BUFFER_HEIGHT {
                    scroll();
                    ROW = BUFFER_HEIGHT - 1;
                }
            }
            byte => {
                if COLUMN >= BUFFER_WIDTH {
                    COLUMN = 0;
                    ROW += 1;
                    if ROW >= BUFFER_HEIGHT {
                        scroll();
                        ROW = BUFFER_HEIGHT - 1;
                    }
                }
                write_char_at(ROW, COLUMN, byte, CURRENT_COLOR);
                COLUMN += 1;
            }
        }
    }
}

/// Escribir string
pub fn write_string(s: &str) {
    for byte in s.bytes() {
        match byte {
            0x20..=0x7e | b'\n' => write_byte(byte),
            _ => write_byte(0xFE), // Caracter especial
        }
    }
}

/// Imprimir línea completa (80 caracteres)
pub fn print_line(s: &str) {
    write_string(s);
    write_byte(b'\n');
}

/// Limpiar pantalla
pub fn clear_screen() {
    for row in 0..BUFFER_HEIGHT {
        for col in 0..BUFFER_WIDTH {
            write_char_at(row, col, b' ', DEFAULT_COLOR);
        }
    }
    unsafe {
        COLUMN = 0;
        ROW = 0;
    }
}

/// Inicializar VGA
pub fn init() {
    clear_screen();
}

/// Writer para fmt::Write
pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_string(s);
        Ok(())
    }
}

/// Función interna para print
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = Writer;
    writer.write_fmt(args).unwrap();
}

/// Macro print! para el kernel
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

/// Macro println! para el kernel
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
