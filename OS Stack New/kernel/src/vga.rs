// ============================================================
// ADead-OS — VGA Text Mode Driver (Rust)
// ============================================================
// Safe Rust wrapper for VGA text mode (0xB8000).
// This is the SECURITY layer — Rust ensures no buffer overflows,
// no out-of-bounds writes, no undefined behavior.
//
// ADead-BIB handles the hardware setup.
// Rust handles the safe logic.
// ============================================================

/// VGA text mode colors (4-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGrey = 0x7,
    DarkGrey = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_BUFFER: usize = 0xB8000;

/// Combine foreground and background colors into a VGA attribute byte.
fn color_code(fg: Color, bg: Color) -> u8 {
    (bg as u8) << 4 | (fg as u8)
}

/// VGA entry: character + attribute packed into u16.
fn vga_entry(ch: u8, color: u8) -> u16 {
    (color as u16) << 8 | (ch as u16)
}

/// Safe VGA text mode writer.
///
/// Tracks cursor position and color, ensures all writes
/// stay within the 80x25 buffer bounds.
pub struct VgaWriter {
    col: usize,
    row: usize,
    color: u8,
}

impl VgaWriter {
    /// Create a new VGA writer at position (0, 0) with white-on-black.
    pub fn new() -> Self {
        Self {
            col: 0,
            row: 0,
            color: color_code(Color::White, Color::Black),
        }
    }

    /// Set the current text color.
    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = color_code(fg, bg);
    }

    /// Clear the entire screen with the current background color.
    pub fn clear(&mut self) {
        let blank = vga_entry(b' ', self.color);
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            self.write_vga(i, blank);
        }
        self.col = 0;
        self.row = 0;
    }

    /// Write a single character at the current cursor position.
    pub fn write_char(&mut self, ch: u8) {
        match ch {
            b'\n' => {
                self.col = 0;
                self.row += 1;
                if self.row >= VGA_HEIGHT {
                    self.scroll();
                }
            }
            b'\r' => {
                self.col = 0;
            }
            b'\t' => {
                let spaces = 4 - (self.col % 4);
                for _ in 0..spaces {
                    self.write_char(b' ');
                }
            }
            _ => {
                if self.col >= VGA_WIDTH {
                    self.col = 0;
                    self.row += 1;
                    if self.row >= VGA_HEIGHT {
                        self.scroll();
                    }
                }
                let offset = self.row * VGA_WIDTH + self.col;
                self.write_vga(offset, vga_entry(ch, self.color));
                self.col += 1;
            }
        }
    }

    /// Write a string to the VGA buffer.
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }

    /// Write a u64 value as hexadecimal.
    pub fn write_hex(&mut self, value: u64) {
        let hex_chars = b"0123456789ABCDEF";
        let mut started = false;
        for i in (0..16).rev() {
            let nibble = ((value >> (i * 4)) & 0xF) as usize;
            if nibble != 0 || started || i == 0 {
                self.write_char(hex_chars[nibble]);
                started = true;
            }
        }
    }

    /// Write a u64 value as decimal.
    pub fn write_dec(&mut self, value: u64) {
        if value == 0 {
            self.write_char(b'0');
            return;
        }
        let mut buf = [0u8; 20];
        let mut pos = 0;
        let mut v = value;
        while v > 0 {
            buf[pos] = b'0' + (v % 10) as u8;
            v /= 10;
            pos += 1;
        }
        for i in (0..pos).rev() {
            self.write_char(buf[i]);
        }
    }

    /// Scroll the screen up by one line.
    fn scroll(&mut self) {
        // Move all lines up by one
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let src = row * VGA_WIDTH + col;
                let dst = (row - 1) * VGA_WIDTH + col;
                let entry = self.read_vga(src);
                self.write_vga(dst, entry);
            }
        }
        // Clear the last line
        let blank = vga_entry(b' ', self.color);
        for col in 0..VGA_WIDTH {
            let offset = (VGA_HEIGHT - 1) * VGA_WIDTH + col;
            self.write_vga(offset, blank);
        }
        self.row = VGA_HEIGHT - 1;
    }

    /// Write a u16 entry to the VGA buffer at the given offset.
    /// Bounds-checked for safety.
    fn write_vga(&self, offset: usize, value: u16) {
        if offset < VGA_WIDTH * VGA_HEIGHT {
            let ptr = VGA_BUFFER as *mut u16;
            unsafe {
                ptr.add(offset).write_volatile(value);
            }
        }
    }

    /// Read a u16 entry from the VGA buffer at the given offset.
    fn read_vga(&self, offset: usize) -> u16 {
        if offset < VGA_WIDTH * VGA_HEIGHT {
            let ptr = VGA_BUFFER as *const u16;
            unsafe { ptr.add(offset).read_volatile() }
        } else {
            0
        }
    }
}
