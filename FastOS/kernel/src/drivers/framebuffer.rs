// ============================================================
// FastOS — Framebuffer Driver
// ============================================================
// Linear framebuffer for graphical output.
// Supports BIOS VBE (1024x768x32) and VGA text mode fallback.
// Provides pixel, rect, line, circle, text rendering primitives.
// ============================================================

use crate::boot::BootInfo;

/// Framebuffer state (global singleton)
static mut FB: FramebufferState = FramebufferState::empty();

struct FramebufferState {
    addr: u64,
    width: u32,
    height: u32,
    pitch: u32,
    bpp: u32,
    initialized: bool,
}

impl FramebufferState {
    const fn empty() -> Self {
        FramebufferState {
            addr: 0, width: 0, height: 0, pitch: 0, bpp: 0, initialized: false,
        }
    }
}

/// Common colors (0xAARRGGBB format)
pub mod color {
    pub const BLACK: u32       = 0xFF000000;
    pub const WHITE: u32       = 0xFFFFFFFF;
    pub const RED: u32         = 0xFFFF0000;
    pub const GREEN: u32       = 0xFF00FF00;
    pub const BLUE: u32        = 0xFF0000FF;
    pub const YELLOW: u32      = 0xFFFFFF00;
    pub const CYAN: u32        = 0xFF00FFFF;
    pub const MAGENTA: u32     = 0xFFFF00FF;
    pub const GREY: u32        = 0xFF808080;
    pub const DARK_GREY: u32   = 0xFF404040;
    pub const LIGHT_GREY: u32  = 0xFFC0C0C0;

    // Windows 11 palette
    pub const ACCENT: u32      = 0xFF0078D4;  // Windows Blue
    pub const BACKGROUND: u32  = 0xFFF3F3F3;  // Light grey bg
    pub const SURFACE: u32     = 0xFFFFFFFF;  // White surface
    pub const TEXT_PRIMARY: u32 = 0xFF1A1A1A; // Near black
    pub const TEXT_SECONDARY: u32 = 0xFF666666; // Grey
    pub const BORDER: u32      = 0xFFD1D1D1;  // Subtle grey
    pub const TASKBAR: u32     = 0xE01C1C1C;  // Dark semi-transparent
    pub const CLOSE_RED: u32   = 0xFFE81123;  // Close button
    pub const SUCCESS: u32     = 0xFF107C10;  // Green
    pub const WARNING: u32     = 0xFFFFB900;  // Yellow
}

/// Initialize framebuffer from boot info
pub fn init(boot_info: &BootInfo) {
    unsafe {
        FB.addr = boot_info.framebuffer_addr;
        FB.width = boot_info.framebuffer_width;
        FB.height = boot_info.framebuffer_height;
        FB.bpp = boot_info.framebuffer_bpp;
        FB.pitch = boot_info.framebuffer_pitch;
        FB.initialized = true;
    }
}

/// Get framebuffer dimensions
pub fn width() -> u32 { unsafe { FB.width } }
pub fn height() -> u32 { unsafe { FB.height } }
pub fn is_initialized() -> bool { unsafe { FB.initialized } }

// ============================================================
// Pixel Operations
// ============================================================

/// Put a single pixel at (x, y) with color
#[inline]
pub fn put_pixel(x: u32, y: u32, col: u32) {
    unsafe {
        if !FB.initialized || x >= FB.width || y >= FB.height { return; }
        let bpp_bytes = FB.bpp / 8;
        let offset = (y * FB.pitch + x * bpp_bytes) as u64;
        let ptr = (FB.addr + offset) as *mut u8;
        if bpp_bytes == 4 {
            core::ptr::write_volatile(ptr as *mut u32, col);
        } else if bpp_bytes == 3 {
            // 24bpp: write 3 bytes (BGR)
            core::ptr::write_volatile(ptr, col as u8);           // B
            core::ptr::write_volatile(ptr.add(1), (col >> 8) as u8);  // G
            core::ptr::write_volatile(ptr.add(2), (col >> 16) as u8); // R
        }
    }
}

/// Get pixel color at (x, y)
#[inline]
pub fn get_pixel(x: u32, y: u32) -> u32 {
    unsafe {
        if !FB.initialized || x >= FB.width || y >= FB.height { return 0; }
        let bpp_bytes = FB.bpp / 8;
        let offset = (y * FB.pitch + x * bpp_bytes) as u64;
        let ptr = (FB.addr + offset) as *const u8;
        if bpp_bytes == 4 {
            core::ptr::read_volatile(ptr as *const u32)
        } else if bpp_bytes == 3 {
            let b = core::ptr::read_volatile(ptr) as u32;
            let g = core::ptr::read_volatile(ptr.add(1)) as u32;
            let r = core::ptr::read_volatile(ptr.add(2)) as u32;
            0xFF000000 | (r << 16) | (g << 8) | b
        } else {
            0
        }
    }
}

/// Alpha-blend a pixel onto the framebuffer
#[inline]
pub fn blend_pixel(x: u32, y: u32, col: u32) {
    let alpha = (col >> 24) & 0xFF;
    if alpha == 0xFF {
        put_pixel(x, y, col);
        return;
    }
    if alpha == 0 { return; }

    let dst = get_pixel(x, y);
    let inv_alpha = 255 - alpha;

    let r = (((col >> 16) & 0xFF) * alpha + ((dst >> 16) & 0xFF) * inv_alpha) / 255;
    let g = (((col >> 8) & 0xFF) * alpha + ((dst >> 8) & 0xFF) * inv_alpha) / 255;
    let b = ((col & 0xFF) * alpha + (dst & 0xFF) * inv_alpha) / 255;

    put_pixel(x, y, 0xFF000000 | (r << 16) | (g << 8) | b);
}

// ============================================================
// Shape Primitives
// ============================================================

/// Fill a rectangle with solid color
pub fn fill_rect(x: u32, y: u32, w: u32, h: u32, col: u32) {
    for row in 0..h {
        let py = y + row;
        if py >= unsafe { FB.height } { break; }
        for col_idx in 0..w {
            let px = x + col_idx;
            if px >= unsafe { FB.width } { break; }
            put_pixel(px, py, col);
        }
    }
}

/// Draw a rectangle outline
pub fn draw_rect(x: u32, y: u32, w: u32, h: u32, col: u32) {
    draw_hline(x, y, w, col);
    draw_hline(x, y + h.saturating_sub(1), w, col);
    draw_vline(x, y, h, col);
    draw_vline(x + w.saturating_sub(1), y, h, col);
}

/// Draw a horizontal line
pub fn draw_hline(x: u32, y: u32, length: u32, col: u32) {
    for i in 0..length {
        put_pixel(x + i, y, col);
    }
}

/// Draw a vertical line
pub fn draw_vline(x: u32, y: u32, length: u32, col: u32) {
    for i in 0..length {
        put_pixel(x, y + i, col);
    }
}

/// Draw a line using Bresenham's algorithm
pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, col: u32) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 {
            put_pixel(x0 as u32, y0 as u32, col);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

/// Fill a circle using midpoint algorithm
pub fn fill_circle(cx: i32, cy: i32, radius: i32, col: u32) {
    let mut x = radius;
    let mut y = 0i32;
    let mut err = 1 - radius;

    while x >= y {
        draw_hline_signed(cx - x, cy + y, 2 * x + 1, col);
        draw_hline_signed(cx - x, cy - y, 2 * x + 1, col);
        draw_hline_signed(cx - y, cy + x, 2 * y + 1, col);
        draw_hline_signed(cx - y, cy - x, 2 * y + 1, col);
        y += 1;
        if err < 0 {
            err += 2 * y + 1;
        } else {
            x -= 1;
            err += 2 * (y - x) + 1;
        }
    }
}

fn draw_hline_signed(x: i32, y: i32, length: i32, col: u32) {
    if y < 0 { return; }
    let start = if x < 0 { 0 } else { x as u32 };
    let end = (x + length) as u32;
    for px in start..end {
        put_pixel(px, y as u32, col);
    }
}

/// Draw a rounded rectangle
pub fn fill_rounded_rect(x: u32, y: u32, w: u32, h: u32, radius: u32, col: u32) {
    // Fill center
    fill_rect(x + radius, y, w - 2 * radius, h, col);
    // Fill left/right strips
    fill_rect(x, y + radius, radius, h - 2 * radius, col);
    fill_rect(x + w - radius, y + radius, radius, h - 2 * radius, col);
    // Fill corners with circles
    let r = radius as i32;
    fill_quarter_circle(x + radius, y + radius, r, 0, col);         // top-left
    fill_quarter_circle(x + w - radius - 1, y + radius, r, 1, col); // top-right
    fill_quarter_circle(x + radius, y + h - radius - 1, r, 2, col); // bottom-left
    fill_quarter_circle(x + w - radius - 1, y + h - radius - 1, r, 3, col); // bottom-right
}

fn fill_quarter_circle(cx: u32, cy: u32, r: i32, quarter: u8, col: u32) {
    for dy in 0..=r {
        for dx in 0..=r {
            if dx * dx + dy * dy <= r * r {
                let (px, py) = match quarter {
                    0 => (cx as i32 - dx, cy as i32 - dy), // top-left
                    1 => (cx as i32 + dx, cy as i32 - dy), // top-right
                    2 => (cx as i32 - dx, cy as i32 + dy), // bottom-left
                    _ => (cx as i32 + dx, cy as i32 + dy), // bottom-right
                };
                if px >= 0 && py >= 0 {
                    put_pixel(px as u32, py as u32, col);
                }
            }
        }
    }
}

// ============================================================
// Text Rendering (8x16 Bitmap Font)
// ============================================================

/// Font dimensions
pub const FONT_WIDTH: u32 = 8;
pub const FONT_HEIGHT: u32 = 16;

/// Built-in 8x16 bitmap font (CP437 compatible, first 128 chars)
/// Each character is 16 bytes (one byte per row, MSB = leftmost pixel)
static FONT_8X16: [u8; 128 * 16] = {
    let mut font = [0u8; 128 * 16];

    // Space (32)
    // Already zeros

    // We define key printable ASCII characters inline
    // This is a minimal font — full CP437 would be 4096 bytes
    // For now, define A-Z, a-z, 0-9, and common symbols

    // Helper: character data will be populated at runtime via init_font()
    font
};

/// Runtime font data (populated from a more complete source or hardcoded)
static mut RUNTIME_FONT: [u8; 256 * 16] = [0u8; 256 * 16];
static mut FONT_INITIALIZED: bool = false;

/// Initialize the bitmap font with basic glyphs
pub fn init_font() {
    unsafe {
        if FONT_INITIALIZED { return; }

        // Generate a simple procedural font for printable ASCII
        // Each char is 8 pixels wide, 16 pixels tall
        for ch in 32u8..127 {
            let base = (ch as usize) * 16;
            match ch {
                // Numbers 0-9
                b'0' => { let d = [0x3C,0x66,0x6E,0x76,0x66,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'1' => { let d = [0x18,0x38,0x18,0x18,0x18,0x18,0x7E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'2' => { let d = [0x3C,0x66,0x06,0x0C,0x18,0x30,0x7E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'3' => { let d = [0x3C,0x66,0x06,0x1C,0x06,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'4' => { let d = [0x0C,0x1C,0x3C,0x6C,0x7E,0x0C,0x0C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'5' => { let d = [0x7E,0x60,0x7C,0x06,0x06,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'6' => { let d = [0x3C,0x60,0x7C,0x66,0x66,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'7' => { let d = [0x7E,0x06,0x0C,0x18,0x18,0x18,0x18,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'8' => { let d = [0x3C,0x66,0x66,0x3C,0x66,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'9' => { let d = [0x3C,0x66,0x66,0x3E,0x06,0x0C,0x38,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }

                // Uppercase A-Z
                b'A' => { let d = [0x18,0x3C,0x66,0x66,0x7E,0x66,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'B' => { let d = [0x7C,0x66,0x66,0x7C,0x66,0x66,0x7C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'C' => { let d = [0x3C,0x66,0x60,0x60,0x60,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'D' => { let d = [0x78,0x6C,0x66,0x66,0x66,0x6C,0x78,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'E' => { let d = [0x7E,0x60,0x60,0x7C,0x60,0x60,0x7E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'F' => { let d = [0x7E,0x60,0x60,0x7C,0x60,0x60,0x60,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'G' => { let d = [0x3C,0x66,0x60,0x6E,0x66,0x66,0x3E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'H' => { let d = [0x66,0x66,0x66,0x7E,0x66,0x66,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'I' => { let d = [0x3C,0x18,0x18,0x18,0x18,0x18,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'J' => { let d = [0x1E,0x0C,0x0C,0x0C,0x0C,0x6C,0x38,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'K' => { let d = [0x66,0x6C,0x78,0x70,0x78,0x6C,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'L' => { let d = [0x60,0x60,0x60,0x60,0x60,0x60,0x7E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'M' => { let d = [0x63,0x77,0x7F,0x6B,0x63,0x63,0x63,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'N' => { let d = [0x66,0x76,0x7E,0x7E,0x6E,0x66,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'O' => { let d = [0x3C,0x66,0x66,0x66,0x66,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'P' => { let d = [0x7C,0x66,0x66,0x7C,0x60,0x60,0x60,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'Q' => { let d = [0x3C,0x66,0x66,0x66,0x6A,0x6C,0x36,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'R' => { let d = [0x7C,0x66,0x66,0x7C,0x6C,0x66,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'S' => { let d = [0x3C,0x66,0x60,0x3C,0x06,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'T' => { let d = [0x7E,0x18,0x18,0x18,0x18,0x18,0x18,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'U' => { let d = [0x66,0x66,0x66,0x66,0x66,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'V' => { let d = [0x66,0x66,0x66,0x66,0x66,0x3C,0x18,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'W' => { let d = [0x63,0x63,0x63,0x6B,0x7F,0x77,0x63,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'X' => { let d = [0x66,0x66,0x3C,0x18,0x3C,0x66,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'Y' => { let d = [0x66,0x66,0x66,0x3C,0x18,0x18,0x18,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'Z' => { let d = [0x7E,0x06,0x0C,0x18,0x30,0x60,0x7E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }

                // Lowercase a-z
                b'a' => { let d = [0x00,0x00,0x3C,0x06,0x3E,0x66,0x3E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'b' => { let d = [0x60,0x60,0x7C,0x66,0x66,0x66,0x7C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'c' => { let d = [0x00,0x00,0x3C,0x66,0x60,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'd' => { let d = [0x06,0x06,0x3E,0x66,0x66,0x66,0x3E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'e' => { let d = [0x00,0x00,0x3C,0x66,0x7E,0x60,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'f' => { let d = [0x1C,0x36,0x30,0x7C,0x30,0x30,0x30,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'g' => { let d = [0x00,0x00,0x3E,0x66,0x66,0x3E,0x06,0x3C,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'h' => { let d = [0x60,0x60,0x7C,0x66,0x66,0x66,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'i' => { let d = [0x18,0x00,0x38,0x18,0x18,0x18,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'j' => { let d = [0x0C,0x00,0x1C,0x0C,0x0C,0x0C,0x6C,0x38,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'k' => { let d = [0x60,0x60,0x66,0x6C,0x78,0x6C,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'l' => { let d = [0x38,0x18,0x18,0x18,0x18,0x18,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'm' => { let d = [0x00,0x00,0x66,0x7F,0x7F,0x6B,0x63,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'n' => { let d = [0x00,0x00,0x7C,0x66,0x66,0x66,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'o' => { let d = [0x00,0x00,0x3C,0x66,0x66,0x66,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'p' => { let d = [0x00,0x00,0x7C,0x66,0x66,0x7C,0x60,0x60,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'q' => { let d = [0x00,0x00,0x3E,0x66,0x66,0x3E,0x06,0x06,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'r' => { let d = [0x00,0x00,0x7C,0x66,0x60,0x60,0x60,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b's' => { let d = [0x00,0x00,0x3E,0x60,0x3C,0x06,0x7C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b't' => { let d = [0x30,0x30,0x7C,0x30,0x30,0x36,0x1C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'u' => { let d = [0x00,0x00,0x66,0x66,0x66,0x66,0x3E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'v' => { let d = [0x00,0x00,0x66,0x66,0x66,0x3C,0x18,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'w' => { let d = [0x00,0x00,0x63,0x6B,0x7F,0x7F,0x36,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'x' => { let d = [0x00,0x00,0x66,0x3C,0x18,0x3C,0x66,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'y' => { let d = [0x00,0x00,0x66,0x66,0x66,0x3E,0x06,0x3C,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'z' => { let d = [0x00,0x00,0x7E,0x0C,0x18,0x30,0x7E,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }

                // Symbols
                b'!' => { let d = [0x18,0x18,0x18,0x18,0x18,0x00,0x18,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'.' => { let d = [0x00,0x00,0x00,0x00,0x00,0x00,0x18,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b',' => { let d = [0x00,0x00,0x00,0x00,0x00,0x18,0x18,0x30,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b':' => { let d = [0x00,0x00,0x18,0x00,0x00,0x18,0x00,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'-' => { let d = [0x00,0x00,0x00,0x7E,0x00,0x00,0x00,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'_' => { let d = [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x7E,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'/' => { let d = [0x02,0x06,0x0C,0x18,0x30,0x60,0x40,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'(' => { let d = [0x0C,0x18,0x30,0x30,0x30,0x18,0x0C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b')' => { let d = [0x30,0x18,0x0C,0x0C,0x0C,0x18,0x30,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'=' => { let d = [0x00,0x00,0x7E,0x00,0x7E,0x00,0x00,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'+' => { let d = [0x00,0x18,0x18,0x7E,0x18,0x18,0x00,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'*' => { let d = [0x00,0x66,0x3C,0xFF,0x3C,0x66,0x00,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'>' => { let d = [0x30,0x18,0x0C,0x06,0x0C,0x18,0x30,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'<' => { let d = [0x0C,0x18,0x30,0x60,0x30,0x18,0x0C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'@' => { let d = [0x3C,0x66,0x6E,0x6A,0x6E,0x60,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'#' => { let d = [0x24,0x24,0x7E,0x24,0x7E,0x24,0x24,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'$' => { let d = [0x18,0x3E,0x60,0x3C,0x06,0x7C,0x18,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'%' => { let d = [0x62,0x66,0x0C,0x18,0x30,0x66,0x46,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'[' => { let d = [0x3C,0x30,0x30,0x30,0x30,0x30,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b']' => { let d = [0x3C,0x0C,0x0C,0x0C,0x0C,0x0C,0x3C,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }
                b'?' => { let d = [0x3C,0x66,0x06,0x0C,0x18,0x00,0x18,0x00,0,0,0,0,0,0,0,0]; RUNTIME_FONT[base..base+16].copy_from_slice(&d); }

                // Default: small filled block for undefined chars
                _ => {
                    RUNTIME_FONT[base + 2] = 0x3C;
                    RUNTIME_FONT[base + 3] = 0x3C;
                    RUNTIME_FONT[base + 4] = 0x3C;
                    RUNTIME_FONT[base + 5] = 0x3C;
                }
            }
        }

        FONT_INITIALIZED = true;
    }
}

/// Draw a single character at pixel position (x, y)
pub fn draw_char(x: u32, y: u32, ch: u8, fg: u32, bg: u32) {
    unsafe {
        if !FONT_INITIALIZED { init_font(); }

        let idx = (ch as usize) * 16;
        for row in 0..8u32 {
            let byte = RUNTIME_FONT[idx + row as usize];
            for col in 0..8u32 {
                let pixel_color = if byte & (0x80 >> col) != 0 { fg } else { bg };
                put_pixel(x + col, y + row * 2, pixel_color);
                put_pixel(x + col, y + row * 2 + 1, pixel_color); // Double height for 8x16
            }
        }
    }
}

/// Draw a string at pixel position (x, y)
pub fn draw_string(x: u32, y: u32, s: &str, fg: u32, bg: u32) {
    let mut cx = x;
    for byte in s.bytes() {
        if byte == b'\n' {
            // Newline not handled in single-line draw
            continue;
        }
        draw_char(cx, y, byte, fg, bg);
        cx += FONT_WIDTH;
    }
}

/// Draw a string with transparent background (only foreground pixels)
pub fn draw_string_transparent(x: u32, y: u32, s: &str, fg: u32) {
    unsafe {
        if !FONT_INITIALIZED { init_font(); }
    }

    let mut cx = x;
    for byte in s.bytes() {
        if byte == b'\n' { continue; }
        unsafe {
            let idx = (byte as usize) * 16;
            for row in 0..8u32 {
                let font_byte = RUNTIME_FONT[idx + row as usize];
                for col in 0..8u32 {
                    if font_byte & (0x80 >> col) != 0 {
                        put_pixel(cx + col, y + row * 2, fg);
                        put_pixel(cx + col, y + row * 2 + 1, fg);
                    }
                }
            }
        }
        cx += FONT_WIDTH;
    }
}

// ============================================================
// Screen Operations
// ============================================================

/// Clear entire screen with a color
pub fn clear(col: u32) {
    unsafe {
        if !FB.initialized { return; }
        for y in 0..FB.height {
            for x in 0..FB.width {
                put_pixel(x, y, col);
            }
        }
    }
}

/// Scroll the screen up by `lines` pixel rows
pub fn scroll_up(lines: u32, bg: u32) {
    unsafe {
        if !FB.initialized { return; }
        let bytes_per_row = FB.pitch;
        let src_offset = (lines * bytes_per_row) as usize;
        let total_bytes = ((FB.height - lines) * bytes_per_row) as usize;

        // Copy rows up
        let base = FB.addr as *mut u8;
        core::ptr::copy(base.add(src_offset), base, total_bytes);

        // Clear bottom rows
        let clear_start = total_bytes;
        let clear_bytes = (lines * bytes_per_row) as usize;
        let clear_ptr = base.add(clear_start) as *mut u32;
        for i in 0..(clear_bytes / 4) {
            core::ptr::write_volatile(clear_ptr.add(i), bg);
        }
    }
}
