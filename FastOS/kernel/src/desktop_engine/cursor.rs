// ============================================================
// FastOS — Cursor Renderer (Fase 5)
// ============================================================
// Software cursor rendering with save/restore of background
// pixels underneath the cursor sprite.
// ============================================================

use crate::drivers::framebuffer;

/// Cursor size
const CURSOR_W: u32 = 12;
const CURSOR_H: u32 = 19;

/// Cursor sprite (1 = white, 2 = black outline, 0 = transparent)
static CURSOR_SPRITE: [u8; (CURSOR_W * CURSOR_H) as usize] = [
    2,0,0,0,0,0,0,0,0,0,0,0,
    2,2,0,0,0,0,0,0,0,0,0,0,
    2,1,2,0,0,0,0,0,0,0,0,0,
    2,1,1,2,0,0,0,0,0,0,0,0,
    2,1,1,1,2,0,0,0,0,0,0,0,
    2,1,1,1,1,2,0,0,0,0,0,0,
    2,1,1,1,1,1,2,0,0,0,0,0,
    2,1,1,1,1,1,1,2,0,0,0,0,
    2,1,1,1,1,1,1,1,2,0,0,0,
    2,1,1,1,1,1,1,1,1,2,0,0,
    2,1,1,1,1,1,1,1,1,1,2,0,
    2,1,1,1,1,1,1,1,1,1,1,2,
    2,1,1,1,1,1,2,2,2,2,2,2,
    2,1,1,1,2,1,2,0,0,0,0,0,
    2,1,1,2,0,2,1,2,0,0,0,0,
    2,1,2,0,0,2,1,2,0,0,0,0,
    2,2,0,0,0,0,2,1,2,0,0,0,
    2,0,0,0,0,0,2,1,2,0,0,0,
    0,0,0,0,0,0,0,2,0,0,0,0,
];

/// Saved background under cursor
static mut SAVED_BG: [u32; (CURSOR_W * CURSOR_H) as usize] = [0; (CURSOR_W * CURSOR_H) as usize];
static mut CURSOR_X: u32 = 0;
static mut CURSOR_Y: u32 = 0;
static mut CURSOR_VISIBLE: bool = false;
static mut CURSOR_DRAWN: bool = false;

/// Initialize cursor at center of screen
pub fn init() {
    unsafe {
        CURSOR_X = framebuffer::width() / 2;
        CURSOR_Y = framebuffer::height() / 2;
        CURSOR_VISIBLE = true;
        CURSOR_DRAWN = false;
    }
}

/// Move cursor to absolute position
pub fn set_position(x: u32, y: u32) {
    unsafe {
        // Erase old cursor
        if CURSOR_DRAWN {
            erase();
        }
        CURSOR_X = x;
        CURSOR_Y = y;
        // Clamp to screen
        let sw = framebuffer::width();
        let sh = framebuffer::height();
        if CURSOR_X >= sw { CURSOR_X = sw - 1; }
        if CURSOR_Y >= sh { CURSOR_Y = sh - 1; }
        // Draw at new position
        if CURSOR_VISIBLE {
            draw();
        }
    }
}

/// Move cursor by relative delta
pub fn move_by(dx: i32, dy: i32) {
    unsafe {
        let nx = (CURSOR_X as i32 + dx).max(0) as u32;
        let ny = (CURSOR_Y as i32 + dy).max(0) as u32;
        set_position(nx, ny);
    }
}

/// Get current cursor position
pub fn position() -> (u32, u32) {
    unsafe { (CURSOR_X, CURSOR_Y) }
}

/// Show cursor
pub fn show() {
    unsafe {
        CURSOR_VISIBLE = true;
        if !CURSOR_DRAWN {
            draw();
        }
    }
}

/// Hide cursor
pub fn hide() {
    unsafe {
        if CURSOR_DRAWN {
            erase();
        }
        CURSOR_VISIBLE = false;
    }
}

/// Draw cursor sprite, saving background pixels
fn draw() {
    unsafe {
        let cx = CURSOR_X;
        let cy = CURSOR_Y;

        for row in 0..CURSOR_H {
            for col in 0..CURSOR_W {
                let px = cx + col;
                let py = cy + row;
                let idx = (row * CURSOR_W + col) as usize;
                let sprite_val = CURSOR_SPRITE[idx];

                // Save background
                SAVED_BG[idx] = framebuffer::get_pixel(px, py);

                // Draw cursor pixel
                match sprite_val {
                    1 => framebuffer::put_pixel(px, py, 0xFFFFFFFF), // White fill
                    2 => framebuffer::put_pixel(px, py, 0xFF000000), // Black outline
                    _ => {} // Transparent — don't draw
                }
            }
        }
        CURSOR_DRAWN = true;
    }
}

/// Erase cursor by restoring saved background
fn erase() {
    unsafe {
        let cx = CURSOR_X;
        let cy = CURSOR_Y;

        for row in 0..CURSOR_H {
            for col in 0..CURSOR_W {
                let px = cx + col;
                let py = cy + row;
                let idx = (row * CURSOR_W + col) as usize;
                let sprite_val = CURSOR_SPRITE[idx];

                if sprite_val != 0 {
                    framebuffer::put_pixel(px, py, SAVED_BG[idx]);
                }
            }
        }
        CURSOR_DRAWN = false;
    }
}
