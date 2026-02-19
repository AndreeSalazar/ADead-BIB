// ============================================================
// FastOS — Graphical File Manager (Fase 7)
// ============================================================
// Simple file browser using the framebuffer and window manager.
// ============================================================

use crate::drivers::framebuffer::{self, color};
use crate::desktop_engine::wm;

/// Open a file manager window
pub fn open() -> wm::WinId {
    let id = wm::create_window("Files", 150, 80, 460, 340);
    draw_content(id);
    id
}

fn draw_content(win_id: wm::WinId) {
    if let Some((cx, cy, cw, ch)) = wm::get_content_rect(win_id) {
        // Background
        framebuffer::fill_rect(cx, cy, cw, ch, color::SURFACE);

        // Toolbar
        framebuffer::fill_rect(cx, cy, cw, 28, color::BACKGROUND);
        framebuffer::draw_hline(cx, cy + 28, cw, color::BORDER);
        framebuffer::draw_string(cx + 8, cy + 6, "< Back", color::TEXT_SECONDARY, color::BACKGROUND);
        framebuffer::draw_string(cx + 64, cy + 6, "> Fwd", color::TEXT_SECONDARY, color::BACKGROUND);

        // Path bar
        framebuffer::fill_rect(cx + 120, cy + 4, cw - 128, 20, color::WHITE);
        framebuffer::draw_rect(cx + 120, cy + 4, cw - 128, 20, color::BORDER);
        framebuffer::draw_string(cx + 126, cy + 6, "C:\\FastOS\\", color::TEXT_PRIMARY, color::WHITE);

        // File list
        let entries = [
            ("#", "system",    "Folder", color::YELLOW),
            ("#", "kernel",    "Folder", color::YELLOW),
            ("#", "boot",      "Folder", color::YELLOW),
            (".", "config.ini","1.2 KB", color::LIGHT_GREY),
            (".", "readme.txt","4.5 KB", color::LIGHT_GREY),
            (".", "fastos.bin","128 KB", color::LIGHT_GREY),
        ];

        let list_y = cy + 36;
        let row_h = 24u32;

        for (i, (icon, name, info, icon_col)) in entries.iter().enumerate() {
            let ry = list_y + i as u32 * row_h;
            if ry + row_h > cy + ch { break; }

            // Alternating row background
            let bg = if i % 2 == 0 { color::SURFACE } else { 0xFFF8F8F8 };
            framebuffer::fill_rect(cx, ry, cw, row_h, bg);

            // Icon
            framebuffer::draw_string(cx + 8, ry + 4, icon, *icon_col, bg);
            // Name
            framebuffer::draw_string(cx + 28, ry + 4, name, color::TEXT_PRIMARY, bg);
            // Info
            framebuffer::draw_string(cx + cw - 80, ry + 4, info, color::TEXT_SECONDARY, bg);
        }

        // Status bar
        let sy = cy + ch - 20;
        framebuffer::fill_rect(cx, sy, cw, 20, color::BACKGROUND);
        framebuffer::draw_string(cx + 8, sy + 2, "6 items", color::TEXT_SECONDARY, color::BACKGROUND);
    }
}
