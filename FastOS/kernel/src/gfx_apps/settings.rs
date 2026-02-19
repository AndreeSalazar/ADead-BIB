// ============================================================
// FastOS — Graphical Settings (Fase 7)
// ============================================================
// System settings panel with display, sound, and about sections.
// ============================================================

use crate::drivers::framebuffer::{self, color};
use crate::desktop_engine::wm;

/// Open a settings window
pub fn open() -> wm::WinId {
    let id = wm::create_window("Settings", 180, 100, 440, 320);
    draw_content(id);
    id
}

fn draw_content(win_id: wm::WinId) {
    if let Some((cx, cy, cw, ch)) = wm::get_content_rect(win_id) {
        framebuffer::fill_rect(cx, cy, cw, ch, color::SURFACE);

        // Sidebar
        let sidebar_w = 120u32;
        framebuffer::fill_rect(cx, cy, sidebar_w, ch, color::BACKGROUND);
        framebuffer::draw_vline(cx + sidebar_w, cy, ch, color::BORDER);

        let items = ["Display", "Sound", "Network", "About"];
        for (i, item) in items.iter().enumerate() {
            let iy = cy + 8 + i as u32 * 32;
            let bg = if i == 0 { color::ACCENT } else { color::BACKGROUND };
            let fg = if i == 0 { color::WHITE } else { color::TEXT_PRIMARY };
            framebuffer::fill_rect(cx + 4, iy, sidebar_w - 8, 26, bg);
            framebuffer::draw_string(cx + 12, iy + 5, item, fg, bg);
        }

        // Content panel — Display settings
        let px = cx + sidebar_w + 12;
        let py = cy + 8;

        framebuffer::draw_string(px, py, "Display", color::TEXT_PRIMARY, color::SURFACE);
        framebuffer::draw_hline(px, py + 20, cw - sidebar_w - 24, color::BORDER);

        // Resolution
        framebuffer::draw_string(px, py + 30, "Resolution:", color::TEXT_SECONDARY, color::SURFACE);
        let w = framebuffer::width();
        let h = framebuffer::height();
        let mut buf = [0u8; 20];
        let n = fmt_resolution(&mut buf, w, h);
        draw_bytes(px + 100, py + 30, &buf[..n], color::TEXT_PRIMARY, color::SURFACE);

        // Color depth
        framebuffer::draw_string(px, py + 54, "Color:", color::TEXT_SECONDARY, color::SURFACE);
        framebuffer::draw_string(px + 100, py + 54, "32-bit ARGB", color::TEXT_PRIMARY, color::SURFACE);

        // Brightness slider
        framebuffer::draw_string(px, py + 78, "Brightness:", color::TEXT_SECONDARY, color::SURFACE);
        let slider_x = px + 100;
        let slider_y = py + 82;
        framebuffer::fill_rect(slider_x, slider_y, 150, 6, color::BORDER);
        framebuffer::fill_rect(slider_x, slider_y, 112, 6, color::ACCENT);
        framebuffer::fill_circle((slider_x + 112) as i32, (slider_y + 3) as i32, 6, color::ACCENT);

        // Theme
        framebuffer::draw_string(px, py + 110, "Theme:", color::TEXT_SECONDARY, color::SURFACE);
        framebuffer::fill_rounded_rect(px + 100, py + 106, 60, 22, 4, 0xFF2D2D2D);
        framebuffer::draw_string(px + 108, py + 109, "Dark", color::WHITE, 0xFF2D2D2D);
        framebuffer::fill_rounded_rect(px + 168, py + 106, 60, 22, 4, color::BACKGROUND);
        framebuffer::draw_string(px + 176, py + 109, "Light", color::TEXT_PRIMARY, color::BACKGROUND);
    }
}

fn fmt_resolution(buf: &mut [u8], w: u32, h: u32) -> usize {
    let mut pos = fmt_u32(&mut buf[0..], w);
    buf[pos] = b'x';
    pos += 1;
    pos += fmt_u32(&mut buf[pos..], h);
    pos
}

fn fmt_u32(buf: &mut [u8], val: u32) -> usize {
    if val == 0 { buf[0] = b'0'; return 1; }
    let mut tmp = [0u8; 10];
    let mut n = val;
    let mut i = 0;
    while n > 0 { tmp[i] = b'0' + (n % 10) as u8; n /= 10; i += 1; }
    for j in 0..i { buf[j] = tmp[i - 1 - j]; }
    i
}

fn draw_bytes(x: u32, y: u32, bytes: &[u8], fg: u32, bg: u32) {
    let mut cx = x;
    for &b in bytes {
        framebuffer::draw_char(cx, y, b, fg, bg);
        cx += 8;
    }
}
