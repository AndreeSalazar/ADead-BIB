// ============================================================
// FastOS — Keyboard Driver (Rust)
// ============================================================
// PS/2 keyboard input via port 0x60.
// Scancode Set 1 → ASCII translation.
// ============================================================

const PS2_DATA: u16 = 0x60;
const PS2_STATUS: u16 = 0x64;

extern "C" {
    fn fastos_inb(port: u16) -> u8;
}

fn inb(port: u16) -> u8 {
    unsafe { fastos_inb(port) }
}

// Scancode Set 1 → ASCII (lowercase, no shift)
static SCANCODE_TABLE: [u8; 128] = [
    0, 27, // 0x00: none, 0x01: ESC
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', // 0x02-0x0B
    b'-', b'=', 8, // 0x0C: -, 0x0D: =, 0x0E: backspace
    b'\t', // 0x0F: tab
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', // 0x10-0x19
    b'[', b']', b'\n', // 0x1A: [, 0x1B: ], 0x1C: enter
    0, // 0x1D: left ctrl
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', // 0x1E-0x26
    b';', b'\'', b'`', // 0x27-0x29
    0, // 0x2A: left shift
    b'\\', // 0x2B
    b'z', b'x', b'c', b'v', b'b', b'n', b'm', // 0x2C-0x32
    b',', b'.', b'/', // 0x33-0x35
    0, // 0x36: right shift
    b'*', // 0x37: keypad *
    0, // 0x38: left alt
    b' ', // 0x39: space
    0, // 0x3A: caps lock
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x3B-0x44: F1-F10
    0, 0, // 0x45: num lock, 0x46: scroll lock
    0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x47-0x4F: keypad
    0, 0, 0, 0, 0, 0, 0, // 0x50-0x56
    0, 0, // 0x57-0x58: F11, F12
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // padding
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

// Shifted scancode table
static SCANCODE_SHIFT: [u8; 128] = [
    0, 27,
    b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')',
    b'_', b'+', 8,
    b'\t',
    b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P',
    b'{', b'}', b'\n',
    0,
    b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L',
    b':', b'"', b'~',
    0,
    b'|',
    b'Z', b'X', b'C', b'V', b'B', b'N', b'M',
    b'<', b'>', b'?',
    0, b'*', 0, b' ', 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

pub struct Keyboard {
    shift_held: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { shift_held: false }
    }

    /// Read a single keypress (blocking). Returns ASCII char or 0 for special keys.
    pub fn read_char(&mut self) -> u8 {
        loop {
            // Wait for data available
            let status = inb(PS2_STATUS);
            if status & 0x01 == 0 {
                continue;
            }

            let scancode = inb(PS2_DATA);

            // Key release (bit 7 set)
            if scancode & 0x80 != 0 {
                let released = scancode & 0x7F;
                if released == 0x2A || released == 0x36 {
                    self.shift_held = false;
                }
                continue;
            }

            // Shift press
            if scancode == 0x2A || scancode == 0x36 {
                self.shift_held = true;
                continue;
            }

            // Translate scancode to ASCII
            let idx = scancode as usize;
            if idx >= 128 {
                continue;
            }

            let ch = if self.shift_held {
                SCANCODE_SHIFT[idx]
            } else {
                SCANCODE_TABLE[idx]
            };

            if ch != 0 {
                return ch;
            }
        }
    }

    /// Read a line of input into a buffer. Returns number of chars read.
    pub fn read_line<'a>(&mut self, buf: &'a mut [u8]) -> usize {
        let mut pos = 0;
        loop {
            let ch = self.read_char();
            match ch {
                b'\n' => return pos,
                8 => {
                    // Backspace
                    if pos > 0 {
                        pos -= 1;
                    }
                }
                _ => {
                    if pos < buf.len() {
                        buf[pos] = ch;
                        pos += 1;
                    }
                }
            }
        }
    }
}
