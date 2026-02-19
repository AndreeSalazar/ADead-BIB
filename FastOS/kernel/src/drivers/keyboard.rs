// ============================================================
// FastOS — PS/2 Keyboard Driver (IRQ-based)
// ============================================================
// Scancode Set 1 → ASCII translation.
// Works via IRQ 1 (INT 33) or polling.
// ============================================================

use crate::arch::x86_64::port;

const PS2_DATA: u16 = 0x60;
const PS2_STATUS: u16 = 0x64;

/// Keyboard state
static mut SHIFT_HELD: bool = false;
static mut CTRL_HELD: bool = false;
static mut ALT_HELD: bool = false;
static mut CAPS_LOCK: bool = false;

/// Circular key buffer for IRQ-driven input
const KEY_BUFFER_SIZE: usize = 64;
static mut KEY_BUFFER: [u8; KEY_BUFFER_SIZE] = [0; KEY_BUFFER_SIZE];
static mut KEY_READ: usize = 0;
static mut KEY_WRITE: usize = 0;

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

/// Special key codes (returned as high byte)
pub mod key {
    pub const ESC: u8 = 27;
    pub const BACKSPACE: u8 = 8;
    pub const TAB: u8 = 9;
    pub const ENTER: u8 = 13;
    pub const F1: u8 = 0x80;
    pub const F2: u8 = 0x81;
    pub const F3: u8 = 0x82;
    pub const F4: u8 = 0x83;
    pub const F5: u8 = 0x84;
    pub const F6: u8 = 0x85;
    pub const F7: u8 = 0x86;
    pub const F8: u8 = 0x87;
    pub const F9: u8 = 0x88;
    pub const F10: u8 = 0x89;
    pub const F11: u8 = 0x8A;
    pub const F12: u8 = 0x8B;
    pub const UP: u8 = 0x90;
    pub const DOWN: u8 = 0x91;
    pub const LEFT: u8 = 0x92;
    pub const RIGHT: u8 = 0x93;
    pub const HOME: u8 = 0x94;
    pub const END: u8 = 0x95;
    pub const PGUP: u8 = 0x96;
    pub const PGDN: u8 = 0x97;
    pub const INSERT: u8 = 0x98;
    pub const DELETE: u8 = 0x99;
}

/// Initialize keyboard driver
pub fn init() {
    // Flush any pending scancodes
    while port::inb(PS2_STATUS) & 0x01 != 0 {
        port::inb(PS2_DATA);
    }
}

/// Called from IRQ 1 handler — process scancode and buffer key
pub fn irq_handler() {
    let scancode = port::inb(PS2_DATA);

    // Key release (bit 7 set)
    if scancode & 0x80 != 0 {
        let released = scancode & 0x7F;
        unsafe {
            match released {
                0x2A | 0x36 => SHIFT_HELD = false,
                0x1D => CTRL_HELD = false,
                0x38 => ALT_HELD = false,
                _ => {}
            }
        }
        return;
    }

    // Key press
    unsafe {
        match scancode {
            0x2A | 0x36 => { SHIFT_HELD = true; return; }
            0x1D => { CTRL_HELD = true; return; }
            0x38 => { ALT_HELD = true; return; }
            0x3A => { CAPS_LOCK = !CAPS_LOCK; return; }
            _ => {}
        }

        // Translate scancode to ASCII
        let idx = scancode as usize;
        if idx >= 128 { return; }

        let ch = if SHIFT_HELD {
            SCANCODE_SHIFT[idx]
        } else {
            let mut c = SCANCODE_TABLE[idx];
            // Apply caps lock to letters
            if CAPS_LOCK && c >= b'a' && c <= b'z' {
                c -= 32;
            }
            c
        };

        if ch != 0 {
            buffer_key(ch);
        }
    }
}

/// Push a key into the circular buffer
fn buffer_key(ch: u8) {
    unsafe {
        let next = (KEY_WRITE + 1) % KEY_BUFFER_SIZE;
        if next != KEY_READ {
            KEY_BUFFER[KEY_WRITE] = ch;
            KEY_WRITE = next;
        }
    }
}

/// Read a key from the buffer (non-blocking, returns 0 if empty)
pub fn read_key() -> u8 {
    unsafe {
        if KEY_READ == KEY_WRITE { return 0; }
        let ch = KEY_BUFFER[KEY_READ];
        KEY_READ = (KEY_READ + 1) % KEY_BUFFER_SIZE;
        ch
    }
}

/// Check if a key is available
pub fn has_key() -> bool {
    unsafe { KEY_READ != KEY_WRITE }
}

/// Read a key (blocking — waits until a key is pressed)
pub fn wait_key() -> u8 {
    loop {
        let k = read_key();
        if k != 0 { return k; }
        crate::arch::x86_64::cpu::hlt();
    }
}

/// Poll keyboard directly (blocking, no IRQ needed)
pub fn poll_char() -> u8 {
    loop {
        let status = port::inb(PS2_STATUS);
        if status & 0x01 == 0 { continue; }

        let scancode = port::inb(PS2_DATA);
        if scancode & 0x80 != 0 {
            let released = scancode & 0x7F;
            unsafe {
                if released == 0x2A || released == 0x36 { SHIFT_HELD = false; }
            }
            continue;
        }

        unsafe {
            if scancode == 0x2A || scancode == 0x36 { SHIFT_HELD = true; continue; }
        }

        let idx = scancode as usize;
        if idx >= 128 { continue; }

        let ch = unsafe {
            if SHIFT_HELD { SCANCODE_SHIFT[idx] } else { SCANCODE_TABLE[idx] }
        };

        if ch != 0 { return ch; }
    }
}

/// Get modifier state
pub fn is_shift() -> bool { unsafe { SHIFT_HELD } }
pub fn is_ctrl() -> bool { unsafe { CTRL_HELD } }
pub fn is_alt() -> bool { unsafe { ALT_HELD } }
pub fn is_caps() -> bool { unsafe { CAPS_LOCK } }
