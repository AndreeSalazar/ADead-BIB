// ============================================================
// FastOS — PS/2 Mouse Driver (IRQ-based)
// ============================================================
// 3-byte PS/2 mouse protocol via IRQ 12 (INT 44).
// Provides cursor position and button state.
// ============================================================

use crate::arch::x86_64::port;

const PS2_DATA: u16 = 0x60;
const PS2_CMD: u16 = 0x64;
const PS2_STATUS: u16 = 0x64;

/// Mouse state
static mut MOUSE_X: i32 = 512;  // Start at center (1024/2)
static mut MOUSE_Y: i32 = 384;  // Start at center (768/2)
static mut MOUSE_BUTTONS: u8 = 0;
static mut MOUSE_CYCLE: u8 = 0;
static mut MOUSE_BYTES: [u8; 3] = [0; 3];
static mut MOUSE_INITIALIZED: bool = false;

/// Screen bounds (set during init)
static mut SCREEN_WIDTH: i32 = 1024;
static mut SCREEN_HEIGHT: i32 = 768;

/// Initialize PS/2 mouse
pub fn init(width: u32, height: u32) {
    unsafe {
        SCREEN_WIDTH = width as i32;
        SCREEN_HEIGHT = height as i32;
        MOUSE_X = SCREEN_WIDTH / 2;
        MOUSE_Y = SCREEN_HEIGHT / 2;
    }

    // Wait for controller ready
    wait_write();
    port::outb(PS2_CMD, 0xA8); // Enable auxiliary device (mouse)

    // Enable IRQ 12
    wait_write();
    port::outb(PS2_CMD, 0x20); // Read command byte
    wait_read();
    let status = port::inb(PS2_DATA);
    wait_write();
    port::outb(PS2_CMD, 0x60); // Write command byte
    wait_write();
    port::outb(PS2_DATA, status | 0x02); // Enable IRQ 12

    // Set defaults
    mouse_write(0xF6); // Set defaults
    mouse_read(); // ACK

    // Enable data reporting
    mouse_write(0xF4); // Enable
    mouse_read(); // ACK

    unsafe { MOUSE_INITIALIZED = true; }
}

/// Wait for PS/2 controller input buffer to be ready
fn wait_write() {
    for _ in 0..100_000 {
        if port::inb(PS2_STATUS) & 0x02 == 0 { return; }
    }
}

/// Wait for PS/2 controller output buffer to have data
fn wait_read() {
    for _ in 0..100_000 {
        if port::inb(PS2_STATUS) & 0x01 != 0 { return; }
    }
}

/// Write a byte to the mouse (via PS/2 controller)
fn mouse_write(data: u8) {
    wait_write();
    port::outb(PS2_CMD, 0xD4); // Send to auxiliary device
    wait_write();
    port::outb(PS2_DATA, data);
}

/// Read a byte from the mouse
fn mouse_read() -> u8 {
    wait_read();
    port::inb(PS2_DATA)
}

/// Called from IRQ 12 handler — process mouse packet byte
pub fn irq_handler() {
    let byte = port::inb(PS2_DATA);

    unsafe {
        match MOUSE_CYCLE {
            0 => {
                // First byte: buttons + overflow + sign bits
                // Bit 3 must be set (alignment bit)
                if byte & 0x08 != 0 {
                    MOUSE_BYTES[0] = byte;
                    MOUSE_CYCLE = 1;
                }
            }
            1 => {
                // Second byte: X movement
                MOUSE_BYTES[1] = byte;
                MOUSE_CYCLE = 2;
            }
            2 => {
                // Third byte: Y movement — complete packet
                MOUSE_BYTES[2] = byte;
                MOUSE_CYCLE = 0;
                process_packet();
            }
            _ => { MOUSE_CYCLE = 0; }
        }
    }
}

/// Process a complete 3-byte mouse packet
fn process_packet() {
    unsafe {
        let flags = MOUSE_BYTES[0];
        let mut dx = MOUSE_BYTES[1] as i32;
        let mut dy = MOUSE_BYTES[2] as i32;

        // Apply sign extension
        if flags & 0x10 != 0 { dx |= !0xFF; } // X sign bit
        if flags & 0x20 != 0 { dy |= !0xFF; } // Y sign bit

        // Check overflow — discard if overflow
        if flags & 0xC0 != 0 { return; }

        // Update position (PS/2 Y is inverted)
        MOUSE_X += dx;
        MOUSE_Y -= dy;

        // Clamp to screen bounds
        if MOUSE_X < 0 { MOUSE_X = 0; }
        if MOUSE_Y < 0 { MOUSE_Y = 0; }
        if MOUSE_X >= SCREEN_WIDTH { MOUSE_X = SCREEN_WIDTH - 1; }
        if MOUSE_Y >= SCREEN_HEIGHT { MOUSE_Y = SCREEN_HEIGHT - 1; }

        // Update button state
        MOUSE_BUTTONS = flags & 0x07;
    }
}

/// Get current mouse X position
pub fn x() -> i32 { unsafe { MOUSE_X } }

/// Get current mouse Y position
pub fn y() -> i32 { unsafe { MOUSE_Y } }

/// Check if left button is pressed
pub fn left_button() -> bool { unsafe { MOUSE_BUTTONS & 0x01 != 0 } }

/// Check if right button is pressed
pub fn right_button() -> bool { unsafe { MOUSE_BUTTONS & 0x02 != 0 } }

/// Check if middle button is pressed
pub fn middle_button() -> bool { unsafe { MOUSE_BUTTONS & 0x04 != 0 } }

/// Get raw button state
pub fn buttons() -> u8 { unsafe { MOUSE_BUTTONS } }

/// Check if mouse is initialized
pub fn is_initialized() -> bool { unsafe { MOUSE_INITIALIZED } }

/// Set mouse position (e.g., for warping cursor)
pub fn set_position(x: i32, y: i32) {
    unsafe {
        MOUSE_X = x.clamp(0, SCREEN_WIDTH - 1);
        MOUSE_Y = y.clamp(0, SCREEN_HEIGHT - 1);
    }
}
