// ============================================================
// FastOS — VGA Text Mode Driver (Rust)
// ============================================================
// Safe VGA driver for 80x25 text mode with 16 colors.
// FastOS theme: green on black (like a terminal).
// ============================================================

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

const VGA_BUFFER: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

pub struct VgaWriter {
    col: usize,
    row: usize,
    color: u8,
}

impl VgaWriter {
    pub fn new() -> Self {
        Self {
            col: 0,
            row: 0,
            color: (Color::LightGreen as u8) | ((Color::Black as u8) << 4),
        }
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = (fg as u8) | ((bg as u8) << 4);
    }

    pub fn clear(&mut self) {
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            unsafe {
                let ptr = (VGA_BUFFER + i * 2) as *mut u8;
                *ptr = b' ';
                *ptr.add(1) = self.color;
            }
        }
        self.col = 0;
        self.row = 0;
    }

    pub fn write_char(&mut self, ch: u8) {
        match ch {
            b'\n' => {
                self.col = 0;
                self.row += 1;
            }
            b'\r' => {
                self.col = 0;
            }
            b'\t' => {
                let spaces = 4 - (self.col % 4);
                for _ in 0..spaces {
                    self.write_char(b' ');
                }
                return;
            }
            _ => {
                if self.col >= VGA_WIDTH {
                    self.col = 0;
                    self.row += 1;
                }
                if self.row >= VGA_HEIGHT {
                    self.scroll();
                }
                let offset = (self.row * VGA_WIDTH + self.col) * 2;
                unsafe {
                    let ptr = (VGA_BUFFER + offset) as *mut u8;
                    *ptr = ch;
                    *ptr.add(1) = self.color;
                }
                self.col += 1;
            }
        }

        if self.row >= VGA_HEIGHT {
            self.scroll();
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }

    pub fn write_hex(&mut self, value: u64) {
        self.write_str("0x");
        let mut started = false;
        for i in (0..16).rev() {
            let nibble = ((value >> (i * 4)) & 0xF) as u8;
            if nibble != 0 || started || i == 0 {
                started = true;
                let ch = if nibble < 10 {
                    b'0' + nibble
                } else {
                    b'A' + nibble - 10
                };
                self.write_char(ch);
            }
        }
    }

    pub fn write_dec(&mut self, mut value: u64) {
        if value == 0 {
            self.write_char(b'0');
            return;
        }
        let mut buf = [0u8; 20];
        let mut i = 0;
        while value > 0 {
            buf[i] = b'0' + (value % 10) as u8;
            value /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            self.write_char(buf[i]);
        }
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            let offset = (self.row * VGA_WIDTH + self.col) * 2;
            unsafe {
                let ptr = (VGA_BUFFER + offset) as *mut u8;
                *ptr = b' ';
                *ptr.add(1) = self.color;
            }
        }
    }

    pub fn current_row(&self) -> usize {
        self.row
    }

    pub fn current_col(&self) -> usize {
        self.col
    }

    fn scroll(&mut self) {
        // Move all lines up by 1
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let src = (row * VGA_WIDTH + col) * 2;
                let dst = ((row - 1) * VGA_WIDTH + col) * 2;
                unsafe {
                    let s = (VGA_BUFFER + src) as *const u8;
                    let d = (VGA_BUFFER + dst) as *mut u8;
                    *d = *s;
                    *d.add(1) = *s.add(1);
                }
            }
        }
        // Clear last line
        let last_row = VGA_HEIGHT - 1;
        for col in 0..VGA_WIDTH {
            let offset = (last_row * VGA_WIDTH + col) * 2;
            unsafe {
                let ptr = (VGA_BUFFER + offset) as *mut u8;
                *ptr = b' ';
                *ptr.add(1) = self.color;
            }
        }
        self.row = VGA_HEIGHT - 1;
    }
}
